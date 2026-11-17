//! Everything that only exists on Windows: the SteamVR host, Direct3D rendering, desktop
//! capture, synthetic input and system media.

mod app;
mod d3d;
pub mod desktop;
mod input;
mod manifest;
pub mod media;
mod painter;
mod panel;
mod session;

use anyhow::Context as _;
use windows::Win32::Foundation::{GlobalFree, HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData};
use windows::Win32::System::Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalUnlock};
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize};
use windows::Win32::UI::HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext};

/// Process-wide setup shared by the overlay and the live preview.
pub fn init_process() {
    unsafe {
        // Physical pixels everywhere, so captures and synthetic clicks line up on scaled displays.
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        if let Err(err) = RoInitialize(RO_INIT_MULTITHREADED) {
            log::debug!("RoInitialize: {err}");
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    init_process();
    app::run()
}

/// `--install` / `--uninstall`: (un)registers VictorHUD with SteamVR.
pub fn install(enable: bool) -> anyhow::Result<()> {
    let vr = crate::vr::VrUtility::init().context("connecting to SteamVR (is it installed?)")?;
    let paths = manifest::write()?;
    if enable {
        manifest::register(&vr.apps, &paths, true)?;
        println!("VictorHUD is registered and will start together with SteamVR.");
        println!("Turn that off any time under Settings > General in VictorHUD.");
    } else {
        manifest::unregister(&vr.apps, &paths)?;
        println!("VictorHUD was removed from SteamVR.");
    }
    Ok(())
}

pub fn set_clipboard(text: &str) -> anyhow::Result<()> {
    const CF_UNICODETEXT: u32 = 13;
    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        OpenClipboard(None).context("opening the clipboard")?;
        let result = (|| -> anyhow::Result<()> {
            EmptyClipboard()?;
            let memory: HGLOBAL = GlobalAlloc(GMEM_MOVEABLE, wide.len() * 2)?;
            let pointer = GlobalLock(memory) as *mut u16;
            if pointer.is_null() {
                let _ = GlobalFree(Some(memory));
                anyhow::bail!("couldn't lock clipboard memory");
            }
            std::ptr::copy_nonoverlapping(wide.as_ptr(), pointer, wide.len());
            let _ = GlobalUnlock(memory);
            if let Err(err) = SetClipboardData(CF_UNICODETEXT, Some(HANDLE(memory.0))) {
                let _ = GlobalFree(Some(memory));
                return Err(err.into());
            }
            Ok(())
        })();
        let _ = CloseClipboard();
        result
    }
}
