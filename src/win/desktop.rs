//! Monitor and window capture for desktop panels.
//!
//! Monitors use DXGI Desktop Duplication when they're on the headset's GPU (fast, no border).
//! Windows, and monitors on another GPU, use Windows Graphics Capture, which also captures
//! windows that are covered by other windows.

use std::time::{Duration, Instant};

use anyhow::{Context, anyhow};
use windows::Graphics::Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession};
use windows::Graphics::DirectX::Direct3D11::IDirect3DDevice;
use windows::Graphics::DirectX::DirectXPixelFormat;
use windows::Graphics::SizeInt32;
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dwm::{DWMWA_CLOAKED, DWMWA_EXTENDED_FRAME_BOUNDS, DwmGetWindowAttribute};
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};
use windows::Win32::System::WinRT::Direct3D11::{CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess};
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GW_OWNER, GWL_EXSTYLE, GetWindow, GetWindowLongW, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, SW_RESTORE, SetForegroundWindow, ShowWindow,
    WS_EX_TOOLWINDOW,
};
use windows::core::{BOOL as CoreBool, Interface};

use super::d3d::Gpu;
use crate::services::desktop::{CaptureTarget, MonitorInfo, WindowInfo};

const MONITORINFOF_PRIMARY: u32 = 1;

fn wide_to_string(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

fn rect_to_xywh(rect: RECT) -> [i32; 4] {
    [rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top]
}

struct RawMonitor {
    handle: HMONITOR,
    info: MonitorInfo,
}

fn raw_monitors() -> Vec<RawMonitor> {
    unsafe extern "system" fn callback(monitor: HMONITOR, _: HDC, _: *mut RECT, data: LPARAM) -> CoreBool {
        let list = unsafe { &mut *(data.0 as *mut Vec<RawMonitor>) };
        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;
        if unsafe { GetMonitorInfoW(monitor, &mut info.monitorInfo as *mut MONITORINFO) }.as_bool() {
            let device_name = wide_to_string(&info.szDevice);
            let number = device_name.trim_start_matches(|c: char| !c.is_ascii_digit()).parse::<usize>().ok();
            let index = list.len();
            let label = format!("Display {}", number.unwrap_or(index + 1));
            list.push(RawMonitor {
                handle: monitor,
                info: MonitorInfo {
                    index,
                    device_name,
                    label,
                    rect: rect_to_xywh(info.monitorInfo.rcMonitor),
                    primary: info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY != 0,
                },
            });
        }
        true.into()
    }
    let mut list: Vec<RawMonitor> = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(None, None, Some(callback), LPARAM(&mut list as *mut _ as isize));
    }
    list
}

pub fn monitors() -> Vec<MonitorInfo> {
    raw_monitors().into_iter().map(|m| m.info).collect()
}

fn process_name(pid: u32) -> Option<String> {
    process_path(pid)?.rsplit('\\').next().map(str::to_string)
}

pub fn process_path(pid: u32) -> Option<String> {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?;
    let mut buffer = [0u16; 520];
    let mut len = buffer.len() as u32;
    let result = unsafe {
        QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, windows::core::PWSTR(buffer.as_mut_ptr()), &mut len)
    };
    unsafe {
        let _ = windows::Win32::Foundation::CloseHandle(handle);
    }
    result.ok()?;
    Some(String::from_utf16_lossy(&buffer[..len as usize]))
}

fn is_cloaked(hwnd: HWND) -> bool {
    let mut cloaked: u32 = 0;
    unsafe { DwmGetWindowAttribute(hwnd, DWMWA_CLOAKED, (&mut cloaked as *mut u32).cast(), 4) }.is_ok() && cloaked != 0
}

/// Top-level windows worth capturing, in Z order.
pub fn windows() -> Vec<WindowInfo> {
    unsafe extern "system" fn callback(hwnd: HWND, data: LPARAM) -> CoreBool {
        let list = unsafe { &mut *(data.0 as *mut Vec<WindowInfo>) };
        unsafe {
            if !IsWindowVisible(hwnd).as_bool() || GetWindow(hwnd, GW_OWNER).is_ok() || is_cloaked(hwnd) {
                return true.into();
            }
            if GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 & WS_EX_TOOLWINDOW.0 != 0 {
                return true.into();
            }
            let len = GetWindowTextLengthW(hwnd);
            if len <= 0 {
                return true.into();
            }
            let mut title = vec![0u16; len as usize + 1];
            let copied = GetWindowTextW(hwnd, &mut title);
            let title = String::from_utf16_lossy(&title[..copied.max(0) as usize]);
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid == std::process::id() || title == "Program Manager" {
                return true.into();
            }
            let process = process_name(pid).unwrap_or_default();
            list.push(WindowInfo { hwnd: hwnd.0 as u64, title, process });
        }
        true.into()
    }
    let mut list: Vec<WindowInfo> = Vec::new();
    unsafe {
        let _ = EnumWindows(Some(callback), LPARAM(&mut list as *mut _ as isize));
    }
    list
}

fn hwnd(raw: u64) -> HWND {
    HWND(raw as *mut std::ffi::c_void)
}

/// The window's visible bounds on screen (without the invisible resize border).
pub fn window_rect(raw: u64) -> Option<[i32; 4]> {
    let mut rect = RECT::default();
    unsafe {
        DwmGetWindowAttribute(
            hwnd(raw),
            DWMWA_EXTENDED_FRAME_BOUNDS,
            (&mut rect as *mut RECT).cast(),
            size_of::<RECT>() as u32,
        )
    }
    .ok()?;
    let xywh = rect_to_xywh(rect);
    (xywh[2] > 0 && xywh[3] > 0).then_some(xywh)
}

pub fn window_exists(raw: u64) -> bool {
    unsafe { IsWindow(Some(hwnd(raw))).as_bool() }
}

/// Restores and focuses a captured window so clicks and typing reach it.
pub fn focus_window(raw: u64) {
    unsafe {
        if IsIconic(hwnd(raw)).as_bool() {
            let _ = ShowWindow(hwnd(raw), SW_RESTORE);
        }
        let _ = SetForegroundWindow(hwnd(raw));
    }
}

pub fn is_minimized(raw: u64) -> bool {
    unsafe { IsIconic(hwnd(raw)).as_bool() }
}

/// Screen rectangle `[x, y, w, h]` that a capture target covers, for mapping clicks.
pub fn target_rect(target: CaptureTarget, monitors: &[MonitorInfo]) -> Option<[i32; 4]> {
    match target {
        CaptureTarget::Monitor(index) => monitors.iter().find(|m| m.index == index).map(|m| m.rect),
        CaptureTarget::Window(raw) => window_rect(raw),
    }
}

struct Duplication {
    duplication: IDXGIOutputDuplication,
}

struct GraphicsCapture {
    pool: Direct3D11CaptureFramePool,
    session: GraphicsCaptureSession,
    device: IDirect3DDevice,
    size: SizeInt32,
}

impl Drop for GraphicsCapture {
    fn drop(&mut self) {
        let _ = self.session.Close();
        let _ = self.pool.Close();
    }
}

enum Source {
    Duplication(Duplication),
    Graphics(GraphicsCapture),
}

/// A running capture of one monitor or window into a texture the panel's painter can show.
pub struct Capture {
    pub target: CaptureTarget,
    source: Option<Source>,
    texture: Option<ID3D11Texture2D>,
    view: Option<ID3D11ShaderResourceView>,
    size: [u32; 2],
    /// Bumped every time `view` is replaced, so the painter re-registers it.
    pub generation: u64,
    pub status: Option<String>,
    retry_at: Option<Instant>,
    last_poll: Option<Instant>,
}

impl Capture {
    pub fn new(target: CaptureTarget) -> Self {
        Self {
            target,
            source: None,
            texture: None,
            view: None,
            size: [0, 0],
            generation: 0,
            status: Some("Starting capture…".into()),
            retry_at: None,
            last_poll: None,
        }
    }

    pub fn view(&self) -> Option<(&ID3D11ShaderResourceView, [u32; 2])> {
        self.view.as_ref().map(|view| (view, self.size))
    }

    fn start(&mut self, gpu: &Gpu) -> anyhow::Result<()> {
        self.source = Some(match self.target {
            CaptureTarget::Monitor(index) => {
                let monitors = raw_monitors();
                let monitor = monitors
                    .iter()
                    .find(|m| m.info.index == index)
                    .ok_or_else(|| anyhow!("That display is disconnected"))?;
                match duplicate(gpu, &monitor.info.device_name)? {
                    Some(duplication) => Source::Duplication(duplication),
                    None => Source::Graphics(start_graphics_capture(gpu, item_for_monitor(monitor.handle)?)?),
                }
            }
            CaptureTarget::Window(raw) => {
                if !window_exists(raw) {
                    return Err(anyhow!("That window was closed"));
                }
                Source::Graphics(start_graphics_capture(gpu, item_for_window(raw)?)?)
            }
        });
        Ok(())
    }

    fn fail(&mut self, message: String, retry_in: Duration) {
        log::debug!("capture {:?}: {message}", self.target);
        self.source = None;
        self.status = Some(message);
        self.retry_at = Some(Instant::now() + retry_in);
    }

    /// Copies the newest frame, if there is one. Returns whether the image changed.
    pub fn poll(&mut self, gpu: &Gpu, max_fps: u32) -> bool {
        let now = Instant::now();
        let interval = Duration::from_secs_f32(1.0 / max_fps.max(1) as f32);
        if self.last_poll.is_some_and(|last| now - last < interval) {
            return false;
        }
        self.last_poll = Some(now);
        if self.source.is_none() {
            if self.retry_at.is_some_and(|at| now < at) {
                return false;
            }
            if let Err(err) = self.start(gpu) {
                self.fail(format!("{err:#}"), Duration::from_secs(2));
                return false;
            }
        }
        if let CaptureTarget::Window(raw) = self.target {
            if !window_exists(raw) {
                self.fail("That window was closed".into(), Duration::from_secs(3));
                return false;
            }
            if is_minimized(raw) {
                self.status = Some("This window is minimized. Click here to restore it.".into());
                self.texture = None;
                return self.view.take().is_some();
            }
        }
        let result = match self.source.as_mut() {
            Some(Source::Duplication(duplication)) => next_duplicated_frame(duplication),
            Some(Source::Graphics(capture)) => next_graphics_frame(capture),
            None => return false,
        };
        match result {
            Ok(Some(frame)) => match self.store(gpu, &frame.texture, frame.region) {
                Ok(changed) => {
                    self.status = None;
                    if let Some(release) = frame.release {
                        release();
                    }
                    changed
                }
                Err(err) => {
                    if let Some(release) = frame.release {
                        release();
                    }
                    self.fail(format!("Capture failed: {err:#}"), Duration::from_secs(2));
                    false
                }
            },
            Ok(None) => false,
            Err(FrameError::Lost) => {
                // Resolution change, a fullscreen game taking over, or the UAC/lock screen.
                self.fail("Display changed, reconnecting…".into(), Duration::from_millis(500));
                false
            }
            Err(FrameError::Other(err)) => {
                self.fail(format!("Capture failed: {err:#}"), Duration::from_secs(2));
                false
            }
        }
    }

    /// Copies `region` (or all) of `source` into our own texture, resizing it when needed.
    fn store(&mut self, gpu: &Gpu, source: &ID3D11Texture2D, region: Option<[u32; 2]>) -> anyhow::Result<bool> {
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { source.GetDesc(&mut desc) };
        let [width, height] = region.unwrap_or([desc.Width, desc.Height]);
        let width = width.min(desc.Width).max(1);
        let height = height.min(desc.Height).max(1);
        if self.texture.is_none() || self.size != [width, height] {
            let texture = gpu.texture(width, height, desc.Format, D3D11_BIND_SHADER_RESOURCE, 0)?;
            self.view = Some(gpu.shader_view(&texture)?);
            self.texture = Some(texture);
            self.size = [width, height];
            self.generation += 1;
        }
        let target = self.texture.as_ref().context("no capture texture")?;
        unsafe {
            if width == desc.Width && height == desc.Height {
                gpu.context.CopyResource(target, source);
            } else {
                let region = D3D11_BOX { left: 0, top: 0, front: 0, right: width, bottom: height, back: 1 };
                gpu.context.CopySubresourceRegion(target, 0, 0, 0, 0, source, 0, Some(&region));
            }
        }
        Ok(true)
    }
}

struct Frame {
    texture: ID3D11Texture2D,
    region: Option<[u32; 2]>,
    release: Option<Box<dyn FnOnce()>>,
}

enum FrameError {
    Lost,
    Other(anyhow::Error),
}

/// Starts Desktop Duplication if `device_name`'s output is on our GPU.
fn duplicate(gpu: &Gpu, device_name: &str) -> anyhow::Result<Option<Duplication>> {
    let dxgi_device: IDXGIDevice = gpu.device.cast()?;
    let adapter = unsafe { dxgi_device.GetAdapter() }?;
    let mut index = 0;
    while let Ok(output) = unsafe { adapter.EnumOutputs(index) } {
        index += 1;
        let desc = unsafe { output.GetDesc() }?;
        if wide_to_string(&desc.DeviceName) != device_name {
            continue;
        }
        let output1: IDXGIOutput1 = output.cast()?;
        return match unsafe { output1.DuplicateOutput(&gpu.device) } {
            Ok(duplication) => Ok(Some(Duplication { duplication })),
            Err(err) if err.code() == DXGI_ERROR_NOT_CURRENTLY_AVAILABLE => {
                Err(anyhow!("Too many apps are capturing this display; close one and try again"))
            }
            Err(err) if err.code() == DXGI_ERROR_UNSUPPORTED => Ok(None),
            Err(err) => Err(anyhow!("Can't capture this display right now ({err})")),
        };
    }
    Ok(None)
}

fn next_duplicated_frame(duplication: &mut Duplication) -> Result<Option<Frame>, FrameError> {
    let mut info = DXGI_OUTDUPL_FRAME_INFO::default();
    let mut resource: Option<IDXGIResource> = None;
    match unsafe { duplication.duplication.AcquireNextFrame(0, &mut info, &mut resource) } {
        Ok(()) => {}
        Err(err) if err.code() == DXGI_ERROR_WAIT_TIMEOUT => return Ok(None),
        Err(err) if err.code() == DXGI_ERROR_ACCESS_LOST || err.code() == DXGI_ERROR_INVALID_CALL => {
            return Err(FrameError::Lost);
        }
        Err(err) => return Err(FrameError::Other(err.into())),
    }
    let dup = duplication.duplication.clone();
    let release: Box<dyn FnOnce()> = Box::new(move || unsafe {
        let _ = dup.ReleaseFrame();
    });
    if info.LastPresentTime == 0 || info.AccumulatedFrames == 0 {
        release();
        return Ok(None);
    }
    let texture = match resource.map(|r| r.cast::<ID3D11Texture2D>()) {
        Some(Ok(texture)) => texture,
        _ => {
            release();
            return Ok(None);
        }
    };
    Ok(Some(Frame { texture, region: None, release: Some(release) }))
}

fn item_for_window(raw: u64) -> anyhow::Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    unsafe { interop.CreateForWindow(hwnd(raw)) }.context("Windows won't capture that window")
}

fn item_for_monitor(monitor: HMONITOR) -> anyhow::Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    unsafe { interop.CreateForMonitor(monitor) }.context("Windows won't capture that display")
}

fn start_graphics_capture(gpu: &Gpu, item: GraphicsCaptureItem) -> anyhow::Result<GraphicsCapture> {
    let dxgi_device: IDXGIDevice = gpu.device.cast()?;
    let device: IDirect3DDevice = unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi_device) }?.cast()?;
    let size = item.Size()?;
    let pool =
        Direct3D11CaptureFramePool::CreateFreeThreaded(&device, DirectXPixelFormat::B8G8R8A8UIntNormalized, 2, size)?;
    let session = pool.CreateCaptureSession(&item)?;
    // Both need newer Windows builds; older ones just keep the defaults.
    let _ = session.SetIsCursorCaptureEnabled(false);
    let _ = session.SetIsBorderRequired(false);
    session.StartCapture()?;
    Ok(GraphicsCapture { pool, session, device, size })
}

fn next_graphics_frame(capture: &mut GraphicsCapture) -> Result<Option<Frame>, FrameError> {
    let mut latest = None;
    while let Ok(frame) = capture.pool.TryGetNextFrame() {
        latest = Some(frame);
    }
    let Some(frame) = latest else { return Ok(None) };
    let content = frame.ContentSize().map_err(|e| FrameError::Other(e.into()))?;
    if content.Width <= 0 || content.Height <= 0 {
        return Ok(None);
    }
    if content.Width != capture.size.Width || content.Height != capture.size.Height {
        capture.size = content;
        capture
            .pool
            .Recreate(&capture.device, DirectXPixelFormat::B8G8R8A8UIntNormalized, 2, content)
            .map_err(|e| FrameError::Other(e.into()))?;
    }
    let texture = frame
        .Surface()
        .and_then(|surface| surface.cast::<IDirect3DDxgiInterfaceAccess>())
        .and_then(|access| unsafe { access.GetInterface::<ID3D11Texture2D>() })
        .map_err(|e| FrameError::Other(e.into()))?;
    let release: Box<dyn FnOnce()> = Box::new(move || {
        let _ = frame.Close();
    });
    Ok(Some(Frame { texture, region: Some([content.Width as u32, content.Height as u32]), release: Some(release) }))
}
