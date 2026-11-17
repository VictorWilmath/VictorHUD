//! Finds and loads the installed SteamVR runtime, doing what Valve's `openvr_api.dll` shim does:
//! read `%LOCALAPPDATA%\openvr\openvrpaths.vrpath`, load `<runtime>\bin\vrclient_x64.dll`, and
//! ask its `IVRClientCore` for the interfaces we need.

use std::ffi::{CStr, CString, c_char, c_void};
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};

use super::sys::{EVRApplicationType, EVRInitError, EVRInitError_VRInitError_None};

const CLIENT_CORE_VERSION: &CStr = c"IVRClientCore_003";

/// `vr::IVRClientCore`. It has no virtual destructor, so these are the first vtable slots. On
/// x64 Windows the C++ member-call ABI is the plain C one with `this` first.
#[repr(C)]
struct ClientCoreVtbl {
    init: unsafe extern "C" fn(
        this: *mut c_void,
        app_type: EVRApplicationType,
        startup_info: *const c_char,
    ) -> EVRInitError,
    cleanup: unsafe extern "C" fn(this: *mut c_void),
    is_interface_version_valid: unsafe extern "C" fn(this: *mut c_void, version: *const c_char) -> EVRInitError,
    get_generic_interface: unsafe extern "C" fn(
        this: *mut c_void,
        name_and_version: *const c_char,
        error: *mut EVRInitError,
    ) -> *mut c_void,
    is_hmd_present: unsafe extern "C" fn(this: *mut c_void) -> bool,
    english_string_for_error: unsafe extern "C" fn(this: *mut c_void, error: EVRInitError) -> *const c_char,
    id_for_error: unsafe extern "C" fn(this: *mut c_void, error: EVRInitError) -> *const c_char,
}

#[repr(C)]
struct ClientCore {
    vtbl: *const ClientCoreVtbl,
}

type FactoryFn = unsafe extern "C" fn(interface_name: *const c_char, return_code: *mut i32) -> *mut c_void;

/// A running connection to SteamVR. Dropping it shuts the connection down.
pub struct Runtime {
    core: *mut ClientCore,
    initialized: bool,
    _library: libloading::Library,
}

impl Runtime {
    pub fn init(app_type: EVRApplicationType) -> anyhow::Result<Self> {
        let runtime_dir = runtime_path()?;
        let dll = runtime_dir.join("bin").join("vrclient_x64.dll");
        if !dll.is_file() {
            bail!("SteamVR looks damaged: {} is missing. Try verifying SteamVR's files in Steam.", dll.display());
        }
        let library = unsafe {
            libloading::os::windows::Library::load_with_flags(
                &dll,
                libloading::os::windows::LOAD_WITH_ALTERED_SEARCH_PATH,
            )
        }
        .with_context(|| format!("loading {}", dll.display()))?;
        let library: libloading::Library = library.into();
        let factory: libloading::Symbol<FactoryFn> = unsafe { library.get(b"VRClientCoreFactory\0") }
            .context("SteamVR's vrclient has no VRClientCoreFactory")?;
        let mut code = 0;
        let core = unsafe { factory(CLIENT_CORE_VERSION.as_ptr(), &mut code) } as *mut ClientCore;
        if core.is_null() {
            bail!("this SteamVR version doesn't provide {CLIENT_CORE_VERSION:?} (code {code}); update SteamVR");
        }
        let mut runtime = Self { core, initialized: false, _library: library };
        let error = unsafe { (runtime.vtbl().init)(core.cast(), app_type, std::ptr::null()) };
        if error != EVRInitError_VRInitError_None {
            return Err(anyhow!("SteamVR refused the connection: {}", runtime.error_message(error)));
        }
        runtime.initialized = true;
        Ok(runtime)
    }

    fn vtbl(&self) -> &ClientCoreVtbl {
        unsafe { &*(*self.core).vtbl }
    }

    pub fn error_message(&self, error: EVRInitError) -> String {
        let text = unsafe { (self.vtbl().english_string_for_error)(self.core.cast(), error) };
        let id = unsafe { (self.vtbl().id_for_error)(self.core.cast(), error) };
        let read = |ptr: *const c_char| {
            (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned())
        };
        match (read(text), read(id)) {
            (Some(text), Some(id)) => format!("{text} ({id})"),
            (Some(text), None) => text,
            (None, Some(id)) => id,
            (None, None) => format!("error {error}"),
        }
    }

    /// Returns the C function table for an interface such as `IVROverlay_028`.
    pub fn fn_table<T>(&self, version: &str) -> anyhow::Result<*const T> {
        let name = CString::new(format!("FnTable:{version}")).expect("interface names have no NUL");
        let mut error = EVRInitError_VRInitError_None;
        let table = unsafe { (self.vtbl().get_generic_interface)(self.core.cast(), name.as_ptr(), &mut error) };
        if table.is_null() || error != EVRInitError_VRInitError_None {
            bail!("SteamVR doesn't provide {version} ({}); update SteamVR", self.error_message(error));
        }
        Ok(table as *const T)
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        if self.initialized {
            unsafe { (self.vtbl().cleanup)(self.core.cast()) };
        }
    }
}

/// The SteamVR install directory, honouring the same overrides as Valve's loader.
pub fn runtime_path() -> anyhow::Result<PathBuf> {
    if let Some(dir) = std::env::var_os("VR_OVERRIDE").filter(|v| !v.is_empty()) {
        return Ok(PathBuf::from(dir));
    }
    let registry = registry_file().ok_or_else(|| anyhow!("can't locate %LOCALAPPDATA%"))?;
    let text = std::fs::read_to_string(&registry).map_err(|_| {
        anyhow!("SteamVR isn't installed, or hasn't been started yet on this PC ({} not found)", registry.display())
    })?;
    let runtime = parse_runtime_path(&text).with_context(|| format!("reading {}", registry.display()))?;
    if !runtime.is_dir() {
        bail!("SteamVR's install folder {} no longer exists; start SteamVR once to fix this", runtime.display());
    }
    Ok(runtime)
}

fn registry_file() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("VR_PATHREG_OVERRIDE").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(path));
    }
    std::env::var_os("LOCALAPPDATA").map(|dir| Path::new(&dir).join("openvr").join("openvrpaths.vrpath"))
}

fn parse_runtime_path(json: &str) -> anyhow::Result<PathBuf> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    value
        .get("runtime")
        .and_then(|list| list.as_array())
        .and_then(|list| list.iter().find_map(|entry| entry.as_str()))
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("no SteamVR runtime is registered"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_first_runtime_entry() {
        let json = r#"{
            "config": ["C:\\Program Files (x86)\\Steam\\config"],
            "external_drivers": null,
            "jsonid": "vrpathreg",
            "log": ["C:\\Program Files (x86)\\Steam\\logs"],
            "runtime": ["C:\\Program Files (x86)\\Steam\\steamapps\\common\\SteamVR", "D:\\Other"],
            "version": 1
        }"#;
        assert_eq!(
            parse_runtime_path(json).unwrap(),
            PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\SteamVR")
        );
        assert!(parse_runtime_path(r#"{"runtime": []}"#).is_err());
        assert!(parse_runtime_path("not json").is_err());
    }
}
