//! Background data sources. Each runs on its own thread and reports through one channel.

pub mod desktop;
pub mod media;
pub mod notify_server;
pub mod vrchat_log;
pub mod vrcx;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};

use crate::config::Config;
use crate::state::{HudState, Notification};
use media::{MediaCommand, MediaSnapshot};

#[derive(Debug)]
pub enum ServiceEvent {
    Media(MediaSnapshot),
    Vrchat(vrchat_log::VrchatState),
    Vrcx(vrcx::VrcxSnapshot),
    Notification(Notification),
    Warning(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataMode {
    /// Real system media, VRChat logs and the VRCX database.
    Live,
    /// Built-in sample data (desktop preview, screenshots).
    Demo,
}

pub struct Services {
    pub events: Receiver<ServiceEvent>,
    events_tx: Sender<ServiceEvent>,
    media: Sender<MediaCommand>,
    shutdown: Arc<AtomicBool>,
}

impl Services {
    pub fn start(config: &Config, mode: DataMode) -> Self {
        let (events_tx, events) = crossbeam_channel::unbounded();
        let (media_tx, media_rx) = crossbeam_channel::unbounded();
        let shutdown = Arc::new(AtomicBool::new(false));

        match mode {
            DataMode::Live => {
                drop(media_rx);
                let _ = events_tx.send(ServiceEvent::Media(MediaSnapshot {
                    error: Some("System media controls aren't connected yet".into()),
                    ..Default::default()
                }));

                if let Some(dir) = config.vrcx.resolved_log_dir() {
                    vrchat_log::spawn(dir, events_tx.clone(), shutdown.clone());
                }
                if let Some(path) = config.vrcx.resolved_database_path() {
                    vrcx::spawn(path, events_tx.clone(), shutdown.clone());
                } else {
                    let _ = events_tx.send(ServiceEvent::Vrcx(vrcx::VrcxSnapshot {
                        status: vrcx::VrcxStatus::NotFound(PathBuf::from("VRCX.sqlite3")),
                        ..Default::default()
                    }));
                }
            }
            DataMode::Demo => {
                spawn_mock_media(media_rx, events_tx.clone(), shutdown.clone());
                let mut parser = vrchat_log::LogParser::default();
                for line in vrchat_log::demo_log().lines() {
                    parser.feed_line(line);
                }
                let _ = events_tx.send(ServiceEvent::Vrchat(parser.state));
                let demo_db = std::env::temp_dir().join("victorhud-demo-vrcx.sqlite3");
                match vrcx::write_demo_database(&demo_db) {
                    Ok(()) => vrcx::spawn(demo_db, events_tx.clone(), shutdown.clone()),
                    Err(err) => log::warn!("demo VRCX database: {err}"),
                }
            }
        }

        if config.notifications.enabled {
            notify_server::spawn(
                config.notifications.udp_port,
                config.notifications.websocket_port,
                config.notifications.default_duration,
                events_tx.clone(),
                shutdown.clone(),
            );
        }

        Self { events, events_tx, media: media_tx, shutdown }
    }

    pub fn send_media(&self, command: MediaCommand) {
        let _ = self.media.send(command);
    }

    /// Lets the host inject events (e.g. a test notification).
    pub fn sender(&self) -> Sender<ServiceEvent> {
        self.events_tx.clone()
    }

    /// Drains pending events into `state`. Returns whether anything changed, plus any new
    /// notifications so the host can show toasts.
    pub fn apply_pending(&self, state: &mut HudState, config: &Config) -> (bool, Vec<u64>) {
        let mut changed = false;
        let mut new_notifications = Vec::new();
        while let Ok(event) = self.events.try_recv() {
            changed = true;
            match event {
                ServiceEvent::Media(snapshot) => state.media = snapshot,
                ServiceEvent::Vrchat(vrchat) => state.vrchat = vrchat,
                ServiceEvent::Vrcx(snapshot) => state.vrcx = snapshot,
                ServiceEvent::Notification(notification) => {
                    let id = state.notifications.push(notification, config.notifications.history_limit);
                    new_notifications.push(id);
                }
                ServiceEvent::Warning(warning) => state.add_warning(warning),
            }
        }
        (changed, new_notifications)
    }
}

impl Drop for Services {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

fn spawn_mock_media(commands: Receiver<MediaCommand>, events: Sender<ServiceEvent>, shutdown: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("media-demo".into())
        .spawn(move || {
            let mut media = media::mock::MockMedia::new();
            while !shutdown.load(Ordering::Relaxed) {
                while let Ok(command) = commands.try_recv() {
                    media.apply(command);
                }
                if events.send(ServiceEvent::Media(media.snapshot())).is_err() {
                    return;
                }
                match commands.recv_timeout(Duration::from_millis(1000)) {
                    Ok(command) => media.apply(command),
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected) => return,
                }
            }
        })
        .expect("spawning media-demo thread");
}
