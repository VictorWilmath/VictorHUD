//! Synthetic mouse and keyboard input, used to drive Windows from desktop panels.
//!
//! Windows doesn't let a normal process send input to elevated (admin) windows, so clicks on
//! those are ignored unless VictorHUD itself runs as administrator.

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
    WHEEL_DELTA,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
}

fn send(inputs: &[INPUT]) {
    let sent = unsafe { SendInput(inputs, size_of::<INPUT>() as i32) };
    if sent as usize != inputs.len() {
        log::debug!("SendInput delivered {sent} of {} events (blocked by an elevated window?)", inputs.len());
    }
}

fn mouse(flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, data: i32) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT { dx, dy, mouseData: data as u32, dwFlags: flags, time: 0, dwExtraInfo: 0 },
        },
    }
}

fn key(vk: VIRTUAL_KEY, scan: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: vk, wScan: scan, dwFlags: flags, time: 0, dwExtraInfo: 0 } },
    }
}

/// Converts a virtual-desktop pixel to SendInput's 0..=65535 absolute range.
fn absolute(x: i32, y: i32) -> (i32, i32) {
    let (left, top, width, height) = unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN),
            GetSystemMetrics(SM_YVIRTUALSCREEN),
            GetSystemMetrics(SM_CXVIRTUALSCREEN).max(2),
            GetSystemMetrics(SM_CYVIRTUALSCREEN).max(2),
        )
    };
    let nx = ((x - left) as i64 * 65535 / (width - 1) as i64) as i32;
    let ny = ((y - top) as i64 * 65535 / (height - 1) as i64) as i32;
    (nx, ny)
}

fn move_input(x: i32, y: i32) -> INPUT {
    let (nx, ny) = absolute(x, y);
    mouse(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK, nx, ny, 0)
}

pub fn cursor_pos() -> Option<(i32, i32)> {
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point) }.ok().map(|_| (point.x, point.y))
}

pub fn move_to(x: i32, y: i32) {
    send(&[move_input(x, y)]);
}

pub fn button(x: i32, y: i32, button: MouseButton, down: bool) {
    let flag = match (button, down) {
        (MouseButton::Left, true) => MOUSEEVENTF_LEFTDOWN,
        (MouseButton::Left, false) => MOUSEEVENTF_LEFTUP,
        (MouseButton::Right, true) => MOUSEEVENTF_RIGHTDOWN,
        (MouseButton::Right, false) => MOUSEEVENTF_RIGHTUP,
    };
    send(&[move_input(x, y), mouse(flag, 0, 0, 0)]);
}

/// Scrolls by whole wheel notches. Positive `vertical` scrolls up, positive `horizontal` right.
pub fn wheel(vertical: i32, horizontal: i32) {
    let mut inputs = Vec::with_capacity(2);
    if vertical != 0 {
        inputs.push(mouse(MOUSEEVENTF_WHEEL, 0, 0, vertical * WHEEL_DELTA as i32));
    }
    if horizontal != 0 {
        inputs.push(mouse(MOUSEEVENTF_HWHEEL, 0, 0, horizontal * WHEEL_DELTA as i32));
    }
    if !inputs.is_empty() {
        send(&inputs);
    }
}

fn tap(vk: VIRTUAL_KEY, extended: bool) -> [INPUT; 2] {
    let flags = if extended { KEYEVENTF_EXTENDEDKEY } else { KEYBD_EVENT_FLAGS(0) };
    [key(vk, 0, flags), key(vk, 0, flags | KEYEVENTF_KEYUP)]
}

/// Types text into the focused window. `\u{8}` is Backspace, `\n` Enter and `\t` Tab, matching
/// what the SteamVR keyboard sends.
pub fn type_text(text: &str) {
    let mut inputs = Vec::new();
    for c in text.chars() {
        match c {
            '\u{8}' => inputs.extend(tap(VK_BACK, false)),
            '\n' | '\r' => inputs.extend(tap(VK_RETURN, false)),
            '\t' => inputs.extend(tap(VK_TAB, false)),
            '\u{1b}' => inputs.extend(tap(VK_ESCAPE, false)),
            c => {
                let mut units = [0u16; 2];
                for &unit in c.encode_utf16(&mut units).iter() {
                    inputs.push(key(VIRTUAL_KEY(0), unit, KEYEVENTF_UNICODE));
                    inputs.push(key(VIRTUAL_KEY(0), unit, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP));
                }
            }
        }
    }
    if !inputs.is_empty() {
        send(&inputs);
    }
}

/// Ctrl+Win+Left/Right: Windows' shortcut for switching virtual desktops.
pub fn switch_virtual_desktop(forward: bool) {
    let arrow = if forward { VK_RIGHT } else { VK_LEFT };
    let flags = KEYBD_EVENT_FLAGS(0);
    send(&[
        key(VK_CONTROL, 0, flags),
        key(VK_LWIN, 0, KEYEVENTF_EXTENDEDKEY),
        key(arrow, 0, KEYEVENTF_EXTENDEDKEY),
        key(arrow, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP),
        key(VK_LWIN, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP),
        key(VK_CONTROL, 0, flags | KEYEVENTF_KEYUP),
    ]);
}
