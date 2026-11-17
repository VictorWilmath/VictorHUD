//! Host-independent application core: config persistence, live state, services, toasts and the
//! actions that behave the same in VR and in the desktop preview.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use chrono::Local;

use crate::config::Config;
use crate::services::{DataMode, ServiceEvent, Services};
use crate::state::{Action, HudState, Notification, PanelKind};

const SAVE_DELAY: Duration = Duration::from_millis(800);

pub struct Hud {
    pub config: Config,
    pub config_path: PathBuf,
    pub state: HudState,
    pub services: Services,
    toasts: VecDeque<u64>,
    toast_started: Option<Instant>,
    dirty_since: Option<Instant>,
}

impl Hud {
    pub fn new(config_path: PathBuf, mode: DataMode) -> Self {
        let config = Config::load_or_default(&config_path);
        let services = Services::start(&config, mode);
        let mut state = HudState::default();
        if mode == DataMode::Demo {
            state.monitors = crate::services::desktop::demo_monitors();
            state.windows = crate::services::desktop::demo_windows();
        }
        Self { config, config_path, state, services, toasts: VecDeque::new(), toast_started: None, dirty_since: None }
    }

    /// Pulls in service events, queues toasts and saves settings that changed a moment ago.
    /// Returns whether anything visible changed.
    pub fn update(&mut self) -> bool {
        let (changed, new_notifications) = self.services.apply_pending(&mut self.state, &self.config);
        if !self.config.notifications.do_not_disturb {
            self.toasts.extend(new_notifications);
        }
        if let Some(since) = self.dirty_since
            && since.elapsed() >= SAVE_DELAY
        {
            self.save_now();
        }
        changed
    }

    pub fn mark_config_dirty(&mut self) {
        self.dirty_since.get_or_insert_with(Instant::now);
    }

    pub fn save_now(&mut self) {
        self.dirty_since = None;
        if let Err(err) = self.config.save(&self.config_path) {
            log::warn!("saving settings: {err:#}");
            self.state.add_warning(format!("Couldn't save settings: {err}"));
        }
    }

    /// The notification to show as a popup right now, if any.
    pub fn current_toast(&mut self) -> Option<(&Notification, f32)> {
        loop {
            let id = *self.toasts.front()?;
            let Some(notification) = self.state.notifications.history.iter().find(|n| n.id == id) else {
                self.toasts.pop_front();
                self.toast_started = None;
                continue;
            };
            let started = *self.toast_started.get_or_insert_with(Instant::now);
            let elapsed = started.elapsed();
            if elapsed >= notification.timeout {
                self.toasts.pop_front();
                self.toast_started = None;
                continue;
            }
            let fade = 0.25;
            let t = elapsed.as_secs_f32();
            let remaining = (notification.timeout - elapsed).as_secs_f32();
            let alpha = (t / fade).min(remaining / fade).clamp(0.0, 1.0);
            return Some((notification, alpha));
        }
    }

    /// Handles actions that work the same everywhere. Anything host specific is handed back.
    pub fn handle(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Media(command) => self.services.send_media(command),
            Action::SelectMediaSession(id) => self.state.preferred_media_session = Some(id),
            Action::SetWatchExpanded(expanded) => self.state.watch_expanded = expanded,
            Action::SetKeepOnWrist(keep) => {
                self.config.watch.keep_on_wrist = keep;
                self.mark_config_dirty();
            }
            Action::SetDoNotDisturb(dnd) => {
                self.config.notifications.do_not_disturb = dnd;
                if dnd {
                    self.toasts.clear();
                    self.toast_started = None;
                }
                self.mark_config_dirty();
            }
            Action::SetPanelsHidden(hidden) => self.state.panels_hidden = hidden,
            Action::MarkNotificationsRead => self.state.notifications.mark_all_read(),
            Action::ClearNotifications => {
                self.state.notifications.clear();
                self.toasts.clear();
            }
            Action::DismissNotification(id) => self.state.notifications.dismiss(id),
            Action::SendTestNotification => {
                let _ = self.services.sender().send(ServiceEvent::Notification(test_notification()));
            }
            Action::OpenUrl(url) => crate::util::open_url(&url),
            Action::OpenPanel(PanelKind::Notifications) => {
                self.state.notifications.mark_all_read();
                return Some(Action::OpenPanel(PanelKind::Notifications));
            }
            other => return Some(other),
        }
        None
    }
}

impl Drop for Hud {
    fn drop(&mut self) {
        if self.dirty_since.is_some() {
            self.save_now();
        }
    }
}

pub fn test_notification() -> Notification {
    Notification {
        id: 0,
        source: "VictorHUD".into(),
        title: "Test notification".into(),
        body: "If you can read this, notifications are working.".into(),
        received: Local::now(),
        timeout: Duration::from_secs(4),
        icon: None,
        read: false,
    }
}
