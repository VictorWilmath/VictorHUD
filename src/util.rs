use std::time::Duration;

/// `1:05:09` for long durations, `5:09` for short ones.
pub fn format_duration(duration: Duration) -> String {
    let total = duration.as_secs();
    let (hours, minutes, seconds) = (total / 3600, (total / 60) % 60, total % 60);
    if hours > 0 { format!("{hours}:{minutes:02}:{seconds:02}") } else { format!("{minutes}:{seconds:02}") }
}

/// `2h 14m`, `14m`, or `just now`, for "how long ago" labels.
pub fn format_ago(duration: Duration) -> String {
    let secs = duration.as_secs();
    match secs {
        0..=59 => "just now".to_string(),
        60..=3599 => format!("{}m ago", secs / 60),
        3600..=86_399 => format!("{}h {}m ago", secs / 3600, (secs / 60) % 60),
        _ => format!("{}d ago", secs / 86_400),
    }
}

/// Removes Unity-style rich text tags (`<b>`, `<color=#fff>`, ...) that XSOverlay senders use.
pub fn strip_rich_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if c == '<'
            && let Some(end) = text[i..].find('>')
        {
            let tag = &text[i + 1..i + end];
            let name = tag.trim_start_matches('/').split(['=', ' ']).next().unwrap_or("");
            if matches!(
                name.to_ascii_lowercase().as_str(),
                "b" | "i" | "u" | "s" | "color" | "size" | "br" | "material" | "quad" | "mark"
            ) {
                if name.eq_ignore_ascii_case("br") {
                    out.push('\n');
                }
                while let Some(&(j, _)) = chars.peek() {
                    if j > i + end {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
        }
        out.push(c);
    }
    out
}

/// Opens a URL with the system's default handler.
pub fn open_url(url: &str) {
    if !(url.starts_with("https://") || url.starts_with("http://") || url.starts_with("vrcx://")) {
        log::warn!("refusing to open non-web url {url}");
        return;
    }
    #[cfg(windows)]
    {
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
        use windows::core::{HSTRING, w};
        let result = unsafe { ShellExecuteW(None, w!("open"), &HSTRING::from(url), None, None, SW_SHOWNORMAL) };
        // ShellExecuteW reports success with a value above 32.
        if result.0 as usize <= 32 {
            log::warn!("could not open {url} (ShellExecute returned {})", result.0 as usize);
        }
    }
    #[cfg(not(windows))]
    log::info!("would open {url} (only supported on Windows)");
}

/// Title-cases an executable-style name: `spotify.exe` -> `Spotify`.
pub fn pretty_app_name(raw: &str) -> String {
    let base = raw.rsplit(['\\', '/']).next().unwrap_or(raw);
    let base = base.strip_suffix(".exe").or_else(|| base.strip_suffix(".EXE")).unwrap_or(base);
    let mut chars = base.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations() {
        assert_eq!(format_duration(Duration::from_secs(5)), "0:05");
        assert_eq!(format_duration(Duration::from_secs(309)), "5:09");
        assert_eq!(format_duration(Duration::from_secs(3909)), "1:05:09");
        assert_eq!(format_ago(Duration::from_secs(30)), "just now");
        assert_eq!(format_ago(Duration::from_secs(600)), "10m ago");
        assert_eq!(format_ago(Duration::from_secs(7500)), "2h 5m ago");
        assert_eq!(format_ago(Duration::from_secs(200_000)), "2d ago");
    }

    #[test]
    fn strips_rich_text_but_keeps_angle_brackets_in_prose() {
        assert_eq!(strip_rich_text("<color=#00ff00>Friend</color> is <b>online</b>"), "Friend is online");
        assert_eq!(strip_rich_text("line<br>two"), "line\ntwo");
        assert_eq!(strip_rich_text("1 < 2 and 3 > 2"), "1 < 2 and 3 > 2");
        assert_eq!(strip_rich_text("<3 you"), "<3 you");
    }

    #[test]
    fn app_names() {
        assert_eq!(pretty_app_name("spotify.exe"), "Spotify");
        assert_eq!(pretty_app_name(r"C:\Games\VRChat\VRChat.exe"), "VRChat");
        assert_eq!(pretty_app_name(""), "");
    }
}
