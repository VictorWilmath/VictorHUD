//! State shared between the background services, the UI and the VR loop.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::services::desktop::{CaptureTarget, MonitorInfo, WindowInfo};
use crate::services::media::{MediaCommand, MediaSnapshot};
use crate::services::vrchat_log::VrchatState;
use crate::services::vrcx::VrcxSnapshot;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hand {
    Left,
    Right,
}

impl Hand {
    pub fn other(self) -> Hand {
        match self {
            Hand::Left => Hand::Right,
            Hand::Right => Hand::Left,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Hand::Left => "Left",
            Hand::Right => "Right",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelKind {
    Watch,
    Media,
    Desktop,
    Vrcx,
    Notifications,
    Settings,
}

impl PanelKind {
    pub fn title(self) -> &'static str {
        match self {
            PanelKind::Watch => "Watch",
            PanelKind::Media => "Media",
            PanelKind::Desktop => "Desktop",
            PanelKind::Vrcx => "VRCX",
            PanelKind::Notifications => "Notifications",
            PanelKind::Settings => "Settings",
        }
    }
}

/// Where a panel lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Placement {
    /// Fixed in the room.
    World,
    /// Locked to a spot in your field of view.
    Head,
    /// Rides along on the watch wrist.
    Wrist,
}

impl Placement {
    pub fn label(self) -> &'static str {
        match self {
            Placement::World => "Pinned in world",
            Placement::Head => "Pinned to view",
            Placement::Wrist => "On wrist",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PanelId(pub u32);

impl PanelId {
    pub const WATCH: PanelId = PanelId(0);
}

#[derive(Clone, Debug, PartialEq)]
pub enum PanelAction {
    Close,
    SetPlacement(Placement),
    ToggleLock,
    ToggleClickThrough,
    AdjustOpacity(f32),
    AdjustSize(f32),
    SetCaptureTarget(CaptureTarget),
}

/// Everything the UI can ask the app to do.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Media(MediaCommand),
    SelectMediaSession(String),
    OpenPanel(PanelKind),
    Panel(PanelId, PanelAction),
    SetWatchExpanded(bool),
    SetKeepOnWrist(bool),
    SetDoNotDisturb(bool),
    SetPanelsHidden(bool),
    MakeAllPanelsInteractive,
    ResetWatchPosition,
    SwitchVirtualDesktop { forward: bool },
    ShowKeyboard(PanelId),
    RefreshWindows,
    CopyToClipboard(String),
    OpenUrl(String),
    MarkNotificationsRead,
    ClearNotifications,
    DismissNotification(u64),
    SendTestNotification,
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Headset,
    LeftHand,
    RightHand,
    Tracker,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeviceBattery {
    pub kind: DeviceKind,
    pub label: String,
    pub percent: f32,
    pub charging: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SessionInfo {
    /// The running VR game, if any.
    pub app_name: Option<String>,
    pub app_started: Option<SystemTime>,
    pub steamvr_started: Option<SystemTime>,
}

impl SessionInfo {
    /// Label and elapsed time for the watch: the game if one is running, SteamVR otherwise.
    pub fn display(&self, now: SystemTime) -> (String, Option<Duration>) {
        match (&self.app_name, self.app_started) {
            (Some(name), started) => (name.clone(), started.and_then(|t| now.duration_since(t).ok())),
            (None, _) => (
                "SteamVR".to_string(),
                self.steamvr_started.and_then(|t| now.duration_since(t).ok()),
            ),
        }
    }
}

/// A decoded RGBA image (album art, notification icons). `id` identifies the content so each
/// egui context uploads it once.
#[derive(Clone, PartialEq)]
pub struct Artwork {
    pub id: u64,
    pub size: [usize; 2],
    pub rgba: Vec<u8>,
}

impl std::fmt::Debug for Artwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Artwork").field("id", &self.id).field("size", &self.size).finish()
    }
}

impl Artwork {
    /// Decodes PNG/JPEG/BMP/WebP bytes, downscaling large images to at most `max_side` pixels.
    pub fn decode(bytes: &[u8], max_side: u32) -> Option<Arc<Artwork>> {
        let image = image::load_from_memory(bytes).ok()?;
        let image = if image.width() > max_side || image.height() > max_side {
            image.thumbnail(max_side, max_side)
        } else {
            image
        };
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        Some(Arc::new(Artwork { id: hash_bytes(bytes), size, rgba: rgba.into_raw() }))
    }
}

pub fn hash_bytes(bytes: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: u64,
    pub source: String,
    pub title: String,
    pub body: String,
    pub received: DateTime<Local>,
    pub timeout: Duration,
    pub icon: Option<Arc<Artwork>>,
    pub read: bool,
}

#[derive(Clone, Debug, Default)]
pub struct NotificationCenter {
    pub history: VecDeque<Notification>,
    next_id: u64,
}

impl NotificationCenter {
    pub fn push(&mut self, mut notification: Notification, limit: usize) -> u64 {
        self.next_id += 1;
        notification.id = self.next_id;
        self.history.push_front(notification);
        while self.history.len() > limit {
            self.history.pop_back();
        }
        self.next_id
    }

    pub fn unread(&self) -> usize {
        self.history.iter().filter(|n| !n.read).count()
    }

    pub fn mark_all_read(&mut self) {
        for notification in &mut self.history {
            notification.read = true;
        }
    }

    pub fn dismiss(&mut self, id: u64) {
        self.history.retain(|n| n.id != id);
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}

/// Live data shown by the panels.
#[derive(Clone, Debug, Default)]
pub struct HudState {
    pub session: SessionInfo,
    pub batteries: Vec<DeviceBattery>,
    pub media: MediaSnapshot,
    /// Media session the user picked; falls back to whatever Windows considers current.
    pub preferred_media_session: Option<String>,
    pub vrchat: VrchatState,
    pub vrcx: VrcxSnapshot,
    pub notifications: NotificationCenter,
    pub monitors: Vec<MonitorInfo>,
    pub windows: Vec<WindowInfo>,
    pub watch_expanded: bool,
    pub panels_hidden: bool,
    pub any_click_through: bool,
    /// Short human-readable problems (port in use, VR runtime missing, ...).
    pub warnings: Vec<String>,
}

impl HudState {
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        let warning = warning.into();
        if !self.warnings.contains(&warning) {
            self.warnings.push(warning);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notification(title: &str) -> Notification {
        Notification {
            id: 0,
            source: "Test".into(),
            title: title.into(),
            body: String::new(),
            received: Local::now(),
            timeout: Duration::from_secs(3),
            icon: None,
            read: false,
        }
    }

    #[test]
    fn notification_history_is_bounded_and_newest_first() {
        let mut center = NotificationCenter::default();
        for i in 0..5 {
            center.push(notification(&format!("n{i}")), 3);
        }
        let titles: Vec<_> = center.history.iter().map(|n| n.title.as_str()).collect();
        assert_eq!(titles, ["n4", "n3", "n2"]);
        assert_eq!(center.unread(), 3);
        center.mark_all_read();
        assert_eq!(center.unread(), 0);
        let id = center.history[0].id;
        center.dismiss(id);
        assert_eq!(center.history.len(), 2);
    }

    #[test]
    fn session_falls_back_to_steamvr_uptime() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        let now = start + Duration::from_secs(90);
        let session = SessionInfo { app_name: None, app_started: None, steamvr_started: Some(start) };
        assert_eq!(session.display(now), ("SteamVR".to_string(), Some(Duration::from_secs(90))));

        let session = SessionInfo {
            app_name: Some("VRChat".into()),
            app_started: Some(start + Duration::from_secs(30)),
            steamvr_started: Some(start),
        };
        assert_eq!(session.display(now), ("VRChat".to_string(), Some(Duration::from_secs(60))));
    }
}
