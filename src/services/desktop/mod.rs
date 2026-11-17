//! What a desktop panel can show.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CaptureTarget {
    /// A monitor, by its index in [`MonitorInfo`] enumeration order.
    Monitor(usize),
    /// A top-level window, by HWND.
    Window(u64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MonitorInfo {
    pub index: usize,
    /// GDI device name, e.g. `\\.\DISPLAY1`.
    pub device_name: String,
    pub label: String,
    /// `[x, y, width, height]` in virtual-desktop pixels.
    pub rect: [i32; 4],
    pub primary: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowInfo {
    pub hwnd: u64,
    pub title: String,
    pub process: String,
}

/// Monitors shown in the preview and in tests.
pub fn demo_monitors() -> Vec<MonitorInfo> {
    vec![
        MonitorInfo {
            index: 0,
            device_name: r"\\.\DISPLAY1".into(),
            label: "Display 1".into(),
            rect: [0, 0, 2560, 1440],
            primary: true,
        },
        MonitorInfo {
            index: 1,
            device_name: r"\\.\DISPLAY2".into(),
            label: "Display 2".into(),
            rect: [2560, 180, 1920, 1080],
            primary: false,
        },
    ]
}

pub fn demo_windows() -> Vec<WindowInfo> {
    vec![
        WindowInfo { hwnd: 0x1001, title: "VRCX".into(), process: "VRCX.exe".into() },
        WindowInfo { hwnd: 0x1002, title: "Discord".into(), process: "Discord.exe".into() },
        WindowInfo { hwnd: 0x1003, title: "YouTube - Firefox".into(), process: "firefox.exe".into() },
    ]
}

pub fn target_label(target: &CaptureTarget, monitors: &[MonitorInfo], windows: &[WindowInfo]) -> String {
    match target {
        CaptureTarget::Monitor(index) => monitors
            .iter()
            .find(|m| m.index == *index)
            .map(|m| m.label.clone())
            .unwrap_or_else(|| format!("Display {}", index + 1)),
        CaptureTarget::Window(hwnd) => windows
            .iter()
            .find(|w| w.hwnd == *hwnd)
            .map(|w| w.title.clone())
            .unwrap_or_else(|| "Window".into()),
    }
}
