//! VictorHUD — a wrist-first SteamVR overlay written in Rust.
//!
//! Raise your wrist like you're checking a watch to see the time, how long you've been in your
//! current game, and what's playing (with controls). The watch menu opens desktop panels you can
//! click and scroll with your controllers, a VRCX panel for VRChat, a media panel, a notification
//! inbox and settings. Any panel can be pinned in the world, pinned to your view, or attached to
//! your wrist.
//!
//! # Layout
//!
//! - `ui`: every panel, drawn with egui. The same code runs in VR and in the `--preview` window.
//! - `hud`, `state`, `config`: shared state, actions and persisted settings.
//! - `interaction`: the watch-raise gesture, laser hit testing, placement and scrolling math.
//! - `services`: background data (media, VRChat logs, VRCX, the notification receiver).
//! - `vr`: OpenVR, loaded from the installed SteamVR at runtime, so no C++ SDK, CMake or
//!   `openvr_api.dll` is needed. `vr/sys.rs` holds the generated bindings.
//! - `win`: the SteamVR host loop, Direct3D 11 rendering, desktop capture, synthetic input and
//!   Windows media/volume control.
//!
//! # Building
//!
//! Any platform (the panels in a normal window, on sample data):
//!
//! ```text
//! cargo run -- --preview          # sample data
//! cargo run -- --preview --live   # real media / VRChat / VRCX data (Windows)
//! ```
//!
//! Cross-compiling the Windows build from Linux works with the `x86_64-pc-windows-gnu` target
//! and mingw-w64.
//!
//! # Controls
//!
//! Trigger clicks, grip grabs and moves panels (grab with both hands to resize), the thumbstick
//! scrolls, and B/Y (or the menu button) toggles the watch. All of it can be rebound under
//! SteamVR → Settings → Controllers → Manage controller bindings → VictorHUD.
//!
//! # Notifications
//!
//! VictorHUD accepts XSOverlay-style notifications on UDP port 42069 and the WebSocket API on
//! port 42070, so tools that already notify XSOverlay (VRCX, VRCVideoCacher, OSC apps) work
//! without changes. Close XSOverlay first if it's running, since both use the same ports.
//!
//! Settings (`config.toml`) and the log (`VictorHUD.log`) live in `%APPDATA%\VictorHUD\config`.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
// Off Windows only the preview builds, so the VR-only helpers go unused there.
#![cfg_attr(not(windows), allow(dead_code))]

mod config;
mod hud;
mod interaction;
#[cfg(feature = "preview")]
mod preview;
mod services;
mod state;
mod ui;
mod util;
#[cfg(windows)]
mod vr;
#[cfg(windows)]
mod win;

use std::io::Write;

const HELP: &str = "\
VictorHUD — a wrist-first SteamVR overlay

USAGE:
    VictorHUD [OPTIONS]

OPTIONS:
    (none)          Run the overlay (start SteamVR first)
    --install       Register with SteamVR so VictorHUD starts with it, then exit
    --uninstall     Remove the SteamVR registration, then exit
    --preview       Show every panel in a desktop window using sample data
    --live          With --preview: use real media, VRChat and VRCX data
    --verbose       Also log to the console
    -h, --help      Show this help
    -V, --version   Show the version
";

#[derive(Debug, Default, PartialEq)]
struct Args {
    install: bool,
    uninstall: bool,
    preview: bool,
    live: bool,
    verbose: bool,
    help: bool,
    version: bool,
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Args, String> {
    let mut parsed = Args::default();
    for arg in args {
        match arg.as_str() {
            "--install" => parsed.install = true,
            "--uninstall" => parsed.uninstall = true,
            "--preview" => parsed.preview = true,
            "--live" => parsed.live = true,
            "--verbose" | "-v" => parsed.verbose = true,
            "--help" | "-h" | "/?" => parsed.help = true,
            "--version" | "-V" => parsed.version = true,
            other => return Err(format!("unknown option `{other}`")),
        }
    }
    if parsed.live && !parsed.preview {
        return Err("--live only works together with --preview".into());
    }
    Ok(parsed)
}

fn main() {
    let args = match parse_args(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(err) => {
            attach_console();
            eprintln!("error: {err}\n\n{HELP}");
            std::process::exit(2);
        }
    };
    if args.help || args.version {
        attach_console();
        if args.version {
            println!("VictorHUD {}", env!("CARGO_PKG_VERSION"));
        } else {
            print!("{HELP}");
        }
        return;
    }
    if args.install || args.uninstall || args.verbose {
        attach_console();
    }
    init_logging(args.verbose || cfg!(debug_assertions));
    log::info!("VictorHUD {} starting", env!("CARGO_PKG_VERSION"));

    let result = run(&args);
    if let Err(err) = result {
        log::error!("{err:#}");
        attach_console();
        eprintln!("VictorHUD: {err:#}");
        #[cfg(windows)]
        if !args.install && !args.uninstall && !args.preview {
            show_error(&format!("{err:#}"));
        }
        std::process::exit(1);
    }
}

fn run(args: &Args) -> anyhow::Result<()> {
    if args.preview {
        #[cfg(feature = "preview")]
        {
            let mode = if args.live { services::DataMode::Live } else { services::DataMode::Demo };
            return preview::run(mode);
        }
        #[cfg(not(feature = "preview"))]
        anyhow::bail!("this build was compiled without the `preview` feature");
    }
    #[cfg(windows)]
    {
        if args.install || args.uninstall {
            return win::install(args.install);
        }
        win::run()
    }
    #[cfg(not(windows))]
    {
        anyhow::bail!("the VR overlay runs on Windows with SteamVR; use --preview to see the panels on this system")
    }
}

#[cfg(windows)]
fn show_error(message: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
    use windows::core::{HSTRING, w};
    unsafe {
        MessageBoxW(None, &HSTRING::from(message), w!("VictorHUD"), MB_OK | MB_ICONERROR);
    }
}

/// Lets a GUI-subsystem build print to the terminal it was started from.
fn attach_console() {
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

struct Tee {
    file: Option<std::fs::File>,
    console: bool,
}

impl Write for Tee {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(file) = &mut self.file {
            let _ = file.write_all(buf);
        }
        if self.console {
            let _ = std::io::stderr().write_all(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(file) = &mut self.file {
            file.flush()?;
        }
        Ok(())
    }
}

fn init_logging(console: bool) {
    let dir = config::data_dir();
    let _ = std::fs::create_dir_all(&dir);
    let file = std::fs::File::create(dir.join("VictorHUD.log")).ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,wgpu=warn,naga=warn,eframe=warn"))
        .format_timestamp_millis()
        .target(env_logger::Target::Pipe(Box::new(Tee { file, console })))
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Result<Args, String> {
        parse_args(list.iter().map(|s| s.to_string()))
    }

    #[test]
    fn parses_flags() {
        assert_eq!(args(&[]).unwrap(), Args::default());
        let parsed = args(&["--preview", "--live"]).unwrap();
        assert!(parsed.preview && parsed.live);
        assert!(args(&["--install"]).unwrap().install);
        assert!(args(&["--bogus"]).is_err());
        assert!(args(&["--live"]).is_err());
    }
}
