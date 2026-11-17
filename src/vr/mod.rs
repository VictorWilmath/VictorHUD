//! Thin safe layer over the OpenVR C function tables VictorHUD uses.

mod loader;
#[rustfmt::skip]
pub mod sys;

use std::ffi::{CStr, CString, c_char, c_void};

use anyhow::bail;

use crate::interaction::placement::Pose;
use sys::*;

pub type OverlayHandle = VROverlayHandle_t;
pub type DeviceIndex = TrackedDeviceIndex_t;

pub const MAX_DEVICES: usize = k_unMaxTrackedDeviceCount as usize;
pub const HMD: DeviceIndex = k_unTrackedDeviceIndex_Hmd as DeviceIndex;

const SYSTEM_VERSION: &str = "IVRSystem_026";
const OVERLAY_VERSION: &str = "IVROverlay_028";
const INPUT_VERSION: &str = "IVRInput_011";
const APPLICATIONS_VERSION: &str = "IVRApplications_008";

macro_rules! call {
    ($table:expr, $name:ident ( $($arg:expr),* $(,)? )) => {
        unsafe { ($table.$name.expect(concat!("OpenVR function ", stringify!($name))))($($arg),*) }
    };
}

fn cstring(text: &str) -> CString {
    CString::new(text.replace('\0', "")).expect("NULs were removed")
}

/// OpenVR declares most string parameters as `char *` even though it never writes to them.
fn arg(text: &CString) -> *mut c_char {
    text.as_ptr() as *mut c_char
}

pub struct Vr {
    system: *const VR_IVRSystem_FnTable,
    overlay: *const VR_IVROverlay_FnTable,
    input: *const VR_IVRInput_FnTable,
    pub apps: Apps,
    _runtime: loader::Runtime,
}

/// A connection that only manages SteamVR's app registry (`--install` / `--uninstall`). It
/// doesn't need a headset.
pub struct VrUtility {
    pub apps: Apps,
    _runtime: loader::Runtime,
}

impl VrUtility {
    pub fn init() -> anyhow::Result<Self> {
        let runtime = loader::Runtime::init(EVRApplicationType_VRApplication_Utility)?;
        Ok(Self { apps: Apps(runtime.fn_table(APPLICATIONS_VERSION)?), _runtime: runtime })
    }
}

impl Vr {
    pub fn init() -> anyhow::Result<Self> {
        let runtime = loader::Runtime::init(EVRApplicationType_VRApplication_Overlay)?;
        Ok(Self {
            system: runtime.fn_table(SYSTEM_VERSION)?,
            overlay: runtime.fn_table(OVERLAY_VERSION)?,
            input: runtime.fn_table(INPUT_VERSION)?,
            apps: Apps(runtime.fn_table(APPLICATIONS_VERSION)?),
            _runtime: runtime,
        })
    }

    fn sys(&self) -> &VR_IVRSystem_FnTable {
        unsafe { &*self.system }
    }

    fn ovl(&self) -> &VR_IVROverlay_FnTable {
        unsafe { &*self.overlay }
    }

    fn inp(&self) -> &VR_IVRInput_FnTable {
        unsafe { &*self.input }
    }

    // ---- IVRSystem ----

    pub fn poll_event(&self) -> Option<VREvent_t> {
        let mut event: VREvent_t = unsafe { std::mem::zeroed() };
        call!(self.sys(), PollNextEvent(&mut event, size_of::<VREvent_t>() as u32)).then_some(event)
    }

    /// Poses of every tracked device, predicted `seconds_ahead` into the future.
    pub fn device_poses(&self, seconds_ahead: f32) -> [TrackedDevicePose_t; MAX_DEVICES] {
        let mut poses: [TrackedDevicePose_t; MAX_DEVICES] = unsafe { std::mem::zeroed() };
        call!(
            self.sys(),
            GetDeviceToAbsoluteTrackingPose(
                ETrackingUniverseOrigin_TrackingUniverseStanding,
                seconds_ahead,
                poses.as_mut_ptr(),
                MAX_DEVICES as u32,
            )
        );
        poses
    }

    pub fn device_class(&self, device: DeviceIndex) -> ETrackedDeviceClass {
        call!(self.sys(), GetTrackedDeviceClass(device))
    }

    pub fn controller_role(&self, device: DeviceIndex) -> ETrackedControllerRole {
        call!(self.sys(), GetControllerRoleForTrackedDeviceIndex(device))
    }

    pub fn controller_for_role(&self, role: ETrackedControllerRole) -> Option<DeviceIndex> {
        let index = call!(self.sys(), GetTrackedDeviceIndexForControllerRole(role));
        ((index as usize) < MAX_DEVICES && self.is_connected(index)).then_some(index)
    }

    pub fn is_connected(&self, device: DeviceIndex) -> bool {
        call!(self.sys(), IsTrackedDeviceConnected(device))
    }

    pub fn float_property(&self, device: DeviceIndex, property: ETrackedDeviceProperty) -> Option<f32> {
        let mut error = ETrackedPropertyError_TrackedProp_Success;
        let value = call!(self.sys(), GetFloatTrackedDeviceProperty(device, property, &mut error));
        (error == ETrackedPropertyError_TrackedProp_Success).then_some(value)
    }

    pub fn bool_property(&self, device: DeviceIndex, property: ETrackedDeviceProperty) -> Option<bool> {
        let mut error = ETrackedPropertyError_TrackedProp_Success;
        let value = call!(self.sys(), GetBoolTrackedDeviceProperty(device, property, &mut error));
        (error == ETrackedPropertyError_TrackedProp_Success).then_some(value)
    }

    /// Battery level in 0..=1 and whether it's charging, for devices that report one.
    pub fn battery(&self, device: DeviceIndex) -> Option<(f32, bool)> {
        let level = self.float_property(device, ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float)?;
        let charging = self.bool_property(device, ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool).unwrap_or(false);
        Some((level, charging))
    }

    /// DXGI adapter index of the GPU driving the headset.
    pub fn dxgi_adapter_index(&self) -> i32 {
        let mut index = -1;
        call!(self.sys(), GetDXGIOutputInfo(&mut index));
        index
    }

    // ---- IVROverlay ----

    fn overlay_result(&self, error: EVROverlayError, what: &str) -> anyhow::Result<()> {
        if error == EVROverlayError_VROverlayError_None {
            return Ok(());
        }
        let name = call!(self.ovl(), GetOverlayErrorNameFromEnum(error));
        let name = if name.is_null() {
            format!("error {error}")
        } else {
            unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned()
        };
        bail!("{what} failed: {name}")
    }

    pub fn create_overlay(&self, key: &str, name: &str) -> anyhow::Result<OverlayHandle> {
        let (key, name) = (cstring(key), cstring(name));
        let mut handle = k_ulOverlayHandleInvalid as OverlayHandle;
        let error = call!(self.ovl(), CreateOverlay(arg(&key), arg(&name), &mut handle));
        self.overlay_result(error, "creating an overlay")?;
        Ok(handle)
    }

    pub fn create_dashboard_overlay(&self, key: &str, name: &str) -> anyhow::Result<(OverlayHandle, OverlayHandle)> {
        let (key, name) = (cstring(key), cstring(name));
        let (mut main, mut thumbnail) = (0, 0);
        let error = call!(self.ovl(), CreateDashboardOverlay(arg(&key), arg(&name), &mut main, &mut thumbnail));
        self.overlay_result(error, "creating the dashboard tab")?;
        Ok((main, thumbnail))
    }

    pub fn destroy_overlay(&self, handle: OverlayHandle) {
        call!(self.ovl(), DestroyOverlay(handle));
    }

    /// Hands a shared D3D11 texture to the compositor.
    pub fn set_overlay_texture(&self, handle: OverlayHandle, texture: *mut c_void) -> anyhow::Result<()> {
        let mut info = Texture_t {
            handle: texture,
            eType: ETextureType_TextureType_DirectX,
            eColorSpace: EColorSpace_ColorSpace_Gamma,
        };
        let error = call!(self.ovl(), SetOverlayTexture(handle, &mut info));
        self.overlay_result(error, "updating an overlay texture")
    }

    pub fn set_overlay_width(&self, handle: OverlayHandle, meters: f32) {
        call!(self.ovl(), SetOverlayWidthInMeters(handle, meters));
    }

    pub fn set_overlay_alpha(&self, handle: OverlayHandle, alpha: f32) {
        call!(self.ovl(), SetOverlayAlpha(handle, alpha.clamp(0.0, 1.0)));
    }

    pub fn set_overlay_sort_order(&self, handle: OverlayHandle, order: u32) {
        call!(self.ovl(), SetOverlaySortOrder(handle, order));
    }

    pub fn set_overlay_flag(&self, handle: OverlayHandle, flag: VROverlayFlags, enabled: bool) {
        call!(self.ovl(), SetOverlayFlag(handle, flag, enabled));
    }

    /// Places an overlay in the standing tracking space.
    pub fn set_overlay_transform_absolute(&self, handle: OverlayHandle, pose: Pose) {
        let mut matrix = HmdMatrix34_t { m: pose.to_rows() };
        call!(
            self.ovl(),
            SetOverlayTransformAbsolute(handle, ETrackingUniverseOrigin_TrackingUniverseStanding, &mut matrix)
        );
    }

    /// Attaches an overlay to a tracked device so it moves with it without per-frame updates.
    pub fn set_overlay_transform_device(&self, handle: OverlayHandle, device: DeviceIndex, pose: Pose) {
        self.set_overlay_transform_device_rows(handle, device, pose.to_rows());
    }

    /// Like [`Self::set_overlay_transform_device`] with a full 3×4 matrix, which may scale.
    pub fn set_overlay_transform_device_rows(&self, handle: OverlayHandle, device: DeviceIndex, rows: [[f32; 4]; 3]) {
        let mut matrix = HmdMatrix34_t { m: rows };
        call!(self.ovl(), SetOverlayTransformTrackedDeviceRelative(handle, device, &mut matrix));
    }

    /// Uploads RGBA pixels from memory. Only meant for small, rarely changing images.
    pub fn set_overlay_raw(
        &self,
        handle: OverlayHandle,
        rgba: &mut [u8],
        width: u32,
        height: u32,
    ) -> anyhow::Result<()> {
        assert_eq!(rgba.len(), (width * height * 4) as usize);
        let error = call!(self.ovl(), SetOverlayRaw(handle, rgba.as_mut_ptr().cast(), width, height, 4));
        self.overlay_result(error, "updating an overlay image")
    }

    pub fn show_overlay(&self, handle: OverlayHandle) {
        call!(self.ovl(), ShowOverlay(handle));
    }

    pub fn hide_overlay(&self, handle: OverlayHandle) {
        call!(self.ovl(), HideOverlay(handle));
    }

    pub fn poll_overlay_event(&self, handle: OverlayHandle) -> Option<VREvent_t> {
        let mut event: VREvent_t = unsafe { std::mem::zeroed() };
        call!(self.ovl(), PollNextOverlayEvent(handle, &mut event, size_of::<VREvent_t>() as u32)).then_some(event)
    }

    /// Lets SteamVR's own dashboard laser drive this overlay with mouse events.
    pub fn enable_mouse_input(&self, handle: OverlayHandle, width: f32, height: f32) {
        call!(self.ovl(), SetOverlayInputMethod(handle, VROverlayInputMethod_Mouse));
        let mut scale = HmdVector2_t { v: [width, height] };
        call!(self.ovl(), SetOverlayMouseScale(handle, &mut scale));
        self.set_overlay_flag(handle, VROverlayFlags_SendVRSmoothScrollEvents, true);
    }

    /// Opens SteamVR's keyboard in minimal mode; each key arrives as a `KeyboardCharInput` event
    /// on `handle`'s event queue.
    pub fn show_keyboard(&self, handle: OverlayHandle, description: &str, user_value: u64) -> anyhow::Result<()> {
        let description = cstring(description);
        let existing = cstring("");
        let error = call!(
            self.ovl(),
            ShowKeyboardForOverlay(
                handle,
                EGamepadTextInputMode_k_EGamepadTextInputModeNormal,
                EGamepadTextInputLineMode_k_EGamepadTextInputLineModeSingleLine,
                EKeyboardFlags_KeyboardFlag_Minimal,
                arg(&description),
                1024,
                arg(&existing),
                user_value,
            )
        );
        self.overlay_result(error, "opening the SteamVR keyboard")
    }

    pub fn hide_keyboard(&self) {
        call!(self.ovl(), HideKeyboard());
    }

    // ---- IVRInput ----

    fn input_result(&self, error: EVRInputError, what: &str) -> anyhow::Result<()> {
        if error == EVRInputError_VRInputError_None { Ok(()) } else { bail!("{what} failed (input error {error})") }
    }

    pub fn set_action_manifest(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let path = cstring(&path.to_string_lossy());
        let error = call!(self.inp(), SetActionManifestPath(arg(&path)));
        self.input_result(error, "loading the controller action manifest")
    }

    pub fn action_set(&self, name: &str) -> anyhow::Result<VRActionSetHandle_t> {
        let name_c = cstring(name);
        let mut handle = 0;
        let error = call!(self.inp(), GetActionSetHandle(arg(&name_c), &mut handle));
        self.input_result(error, name)?;
        Ok(handle)
    }

    pub fn action(&self, name: &str) -> anyhow::Result<VRActionHandle_t> {
        let name_c = cstring(name);
        let mut handle = 0;
        let error = call!(self.inp(), GetActionHandle(arg(&name_c), &mut handle));
        self.input_result(error, name)?;
        Ok(handle)
    }

    pub fn input_source(&self, path: &str) -> anyhow::Result<VRInputValueHandle_t> {
        let path_c = cstring(path);
        let mut handle = 0;
        let error = call!(self.inp(), GetInputSourceHandle(arg(&path_c), &mut handle));
        self.input_result(error, path)?;
        Ok(handle)
    }

    /// Refreshes action state, activating `set` once per hand. A hand marked exclusive stops
    /// sending the inputs bound in `set` to the game (used while that hand's laser is on a panel,
    /// so clicking doesn't also shoot or teleport).
    pub fn update_actions(&self, set: VRActionSetHandle_t, hands: &[(VRInputValueHandle_t, bool)]) {
        let mut active: Vec<VRActiveActionSet_t> = hands
            .iter()
            .map(|&(device, exclusive)| VRActiveActionSet_t {
                ulActionSet: set,
                ulRestrictedToDevice: device,
                ulSecondaryActionSet: k_ulInvalidActionSetHandle as VRActionSetHandle_t,
                unPadding: 0,
                nPriority: if exclusive { k_nActionSetOverlayGlobalPriorityMin } else { 0 },
            })
            .collect();
        call!(
            self.inp(),
            UpdateActionState(active.as_mut_ptr(), size_of::<VRActiveActionSet_t>() as u32, active.len() as u32)
        );
    }

    pub fn digital(&self, action: VRActionHandle_t, device: VRInputValueHandle_t) -> InputDigitalActionData_t {
        let mut data: InputDigitalActionData_t = unsafe { std::mem::zeroed() };
        call!(
            self.inp(),
            GetDigitalActionData(action, &mut data, size_of::<InputDigitalActionData_t>() as u32, device)
        );
        data
    }

    pub fn analog(&self, action: VRActionHandle_t, device: VRInputValueHandle_t) -> InputAnalogActionData_t {
        let mut data: InputAnalogActionData_t = unsafe { std::mem::zeroed() };
        call!(self.inp(), GetAnalogActionData(action, &mut data, size_of::<InputAnalogActionData_t>() as u32, device));
        data
    }

    /// Like [`Self::pose`], sampled `seconds` from now so it lines up with [`Self::device_poses`].
    pub fn pose_at(&self, action: VRActionHandle_t, device: VRInputValueHandle_t, seconds: f32) -> Option<Pose> {
        let mut data: InputPoseActionData_t = unsafe { std::mem::zeroed() };
        call!(
            self.inp(),
            GetPoseActionDataRelativeToNow(
                action,
                ETrackingUniverseOrigin_TrackingUniverseStanding,
                seconds,
                &mut data,
                size_of::<InputPoseActionData_t>() as u32,
                device,
            )
        );
        (data.bActive && data.pose.bPoseIsValid).then(|| Pose::from_rows(data.pose.mDeviceToAbsoluteTracking.m))
    }

    pub fn haptic(
        &self,
        action: VRActionHandle_t,
        device: VRInputValueHandle_t,
        seconds: f32,
        frequency: f32,
        amplitude: f32,
    ) {
        call!(
            self.inp(),
            TriggerHapticVibrationAction(action, 0.0, seconds, frequency, amplitude.clamp(0.0, 1.0), device)
        );
    }
}

/// `IVRApplications`: SteamVR's registry of installed apps and the running scene app.
pub struct Apps(*const VR_IVRApplications_FnTable);

impl Apps {
    fn app(&self) -> &VR_IVRApplications_FnTable {
        unsafe { &*self.0 }
    }

    fn app_result(&self, error: EVRApplicationError, what: &str) -> anyhow::Result<()> {
        if error == EVRApplicationError_VRApplicationError_None {
            Ok(())
        } else {
            bail!("{what} failed (application error {error})")
        }
    }

    pub fn add_app_manifest(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let path = cstring(&path.to_string_lossy());
        let error = call!(self.app(), AddApplicationManifest(arg(&path), false));
        self.app_result(error, "registering VictorHUD with SteamVR")
    }

    pub fn remove_app_manifest(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let path = cstring(&path.to_string_lossy());
        let error = call!(self.app(), RemoveApplicationManifest(arg(&path)));
        self.app_result(error, "removing VictorHUD from SteamVR")
    }

    pub fn is_app_installed(&self, key: &str) -> bool {
        let key = cstring(key);
        call!(self.app(), IsApplicationInstalled(arg(&key)))
    }

    pub fn set_auto_launch(&self, key: &str, enabled: bool) -> anyhow::Result<()> {
        let key = cstring(key);
        let error = call!(self.app(), SetApplicationAutoLaunch(arg(&key), enabled));
        self.app_result(error, "changing auto-launch")
    }

    pub fn auto_launch(&self, key: &str) -> bool {
        let key = cstring(key);
        call!(self.app(), GetApplicationAutoLaunch(arg(&key)))
    }

    /// Tells SteamVR this process is the app with `key`, so its controller bindings apply.
    pub fn identify_app(&self, key: &str) -> anyhow::Result<()> {
        let key = cstring(key);
        let error = call!(self.app(), IdentifyApplication(std::process::id(), arg(&key)));
        self.app_result(error, "identifying with SteamVR")
    }

    pub fn scene_process_id(&self) -> u32 {
        call!(self.app(), GetCurrentSceneProcessId())
    }

    pub fn app_key_for_process(&self, pid: u32) -> Option<String> {
        let mut buffer = [0 as c_char; k_unMaxApplicationKeyLength as usize];
        let error = call!(self.app(), GetApplicationKeyByProcessId(pid, buffer.as_mut_ptr(), buffer.len() as u32));
        (error == EVRApplicationError_VRApplicationError_None)
            .then(|| unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
            .filter(|key| !key.is_empty())
    }

    pub fn app_name(&self, key: &str) -> Option<String> {
        let key = cstring(key);
        let mut buffer = [0 as c_char; 256];
        let mut error = EVRApplicationError_VRApplicationError_None;
        call!(
            self.app(),
            GetApplicationPropertyString(
                arg(&key),
                EVRApplicationProperty_VRApplicationProperty_Name_String,
                buffer.as_mut_ptr(),
                buffer.len() as u32,
                &mut error,
            )
        );
        (error == EVRApplicationError_VRApplicationError_None)
            .then(|| unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
            .filter(|name| !name.is_empty())
    }
}

/// Turns a raw keyboard event payload into text.
pub fn keyboard_text(event: &VREvent_t) -> String {
    let raw = unsafe { event.data.keyboard.cNewInput };
    let bytes: Vec<u8> = raw.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}
