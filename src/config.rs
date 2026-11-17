use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::interaction::placement::Pose;
use crate::state::{Hand, PanelKind, Placement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub watch: WatchConfig,
    pub interaction: InteractionConfig,
    pub desktop: DesktopConfig,
    pub media: MediaConfig,
    pub notifications: NotificationConfig,
    pub vrcx: VrcxConfig,
    /// Where panels were last left, so reopening one puts it back.
    pub layouts: Vec<SavedLayout>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            watch: WatchConfig::default(),
            interaction: InteractionConfig::default(),
            desktop: DesktopConfig::default(),
            media: MediaConfig::default(),
            notifications: NotificationConfig::default(),
            vrcx: VrcxConfig::default(),
            layouts: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub start_with_steamvr: bool,
    /// Multiplies the physical size of every panel.
    pub ui_scale: f32,
    pub accent: [u8; 3],
    /// Skip re-rendering panels that are hidden or fully transparent.
    pub performance_mode: bool,
    pub clock_24h: bool,
    pub show_seconds: bool,
    pub show_date: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            start_with_steamvr: true,
            ui_scale: 1.0,
            accent: [124, 92, 255],
            performance_mode: true,
            clock_24h: false,
            show_seconds: false,
            show_date: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WatchConfig {
    pub hand: Hand,
    pub keep_on_wrist: bool,
    /// Max angle (degrees) between the watch face normal and the direction to your eyes.
    pub facing_angle: f32,
    /// Max angle (degrees) between where you look and the watch.
    pub view_angle: f32,
    pub max_distance: f32,
    pub show_delay: f32,
    pub hide_delay: f32,
    pub width_m: f32,
    pub opacity: f32,
    /// Watch pose relative to the wrist hand's pointer ("tip") pose. `None` uses the built-in default.
    pub offset_left: Option<Pose>,
    pub offset_right: Option<Pose>,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            hand: Hand::Left,
            keep_on_wrist: false,
            facing_angle: 40.0,
            view_angle: 38.0,
            max_distance: 0.75,
            show_delay: 0.12,
            hide_delay: 0.35,
            width_m: 0.13,
            opacity: 1.0,
            offset_left: None,
            offset_right: None,
        }
    }
}

impl WatchConfig {
    pub fn offset(&self, hand: Hand) -> Pose {
        let custom = match hand {
            Hand::Left => self.offset_left,
            Hand::Right => self.offset_right,
        };
        custom.unwrap_or_else(|| Pose::default_watch_offset(hand))
    }

    pub fn set_offset(&mut self, hand: Hand, pose: Option<Pose>) {
        match hand {
            Hand::Left => self.offset_left = pose,
            Hand::Right => self.offset_right = pose,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InteractionConfig {
    pub laser_color: [u8; 3],
    pub laser_max_length: f32,
    pub haptics: bool,
    pub haptic_strength: f32,
    /// Wheel notches per second at full stick deflection.
    pub scroll_speed: f32,
    pub invert_scroll: bool,
    /// Move the Windows cursor while merely hovering a desktop (otherwise only on click).
    pub hover_moves_cursor: bool,
}

impl Default for InteractionConfig {
    fn default() -> Self {
        Self {
            laser_color: [124, 92, 255],
            laser_max_length: 4.0,
            haptics: true,
            haptic_strength: 0.35,
            scroll_speed: 12.0,
            invert_scroll: false,
            hover_moves_cursor: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DesktopConfig {
    pub max_fps: u32,
    pub show_cursor: bool,
    pub default_width_m: f32,
    /// Monitor opened by default from the watch menu (0 = primary).
    pub default_monitor: usize,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self { max_fps: 60, show_cursor: true, default_width_m: 1.0, default_monitor: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MediaConfig {
    pub show_artwork: bool,
    /// Volume change per button press, 0..1.
    pub volume_step: f32,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self { show_artwork: true, volume_step: 0.05 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastPlacement {
    Head,
    Wrist,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub udp_port: u16,
    pub websocket_port: u16,
    pub placement: ToastPlacement,
    /// Used when the sender doesn't specify a timeout.
    pub default_duration: f32,
    pub do_not_disturb: bool,
    pub history_limit: usize,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            udp_port: 42069,
            websocket_port: 42070,
            placement: ToastPlacement::Head,
            default_duration: 4.0,
            do_not_disturb: false,
            history_limit: 100,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VrcxConfig {
    /// Overrides `%APPDATA%\VRCX\VRCX.sqlite3`.
    pub database_path: Option<PathBuf>,
    /// Overrides `%USERPROFILE%\AppData\LocalLow\VRChat\VRChat`.
    pub vrchat_log_dir: Option<PathBuf>,
}

impl VrcxConfig {
    pub fn resolved_database_path(&self) -> Option<PathBuf> {
        self.database_path.clone().or_else(default_vrcx_db_path)
    }

    pub fn resolved_log_dir(&self) -> Option<PathBuf> {
        self.vrchat_log_dir.clone().or_else(default_vrchat_log_dir)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedLayout {
    pub kind: PanelKind,
    pub placement: Placement,
    pub pose: Pose,
    pub width_m: f32,
    pub opacity: f32,
}

pub fn default_vrcx_db_path() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|appdata| PathBuf::from(appdata).join("VRCX").join("VRCX.sqlite3"))
}

pub fn default_vrchat_log_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").map(|home| {
        PathBuf::from(home).join("AppData").join("LocalLow").join("VRChat").join("VRChat")
    })
}

pub fn data_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "VictorHUD")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("VictorHUD"))
}

impl Config {
    pub fn path() -> PathBuf {
        data_dir().join("config.toml")
    }

    /// Loads the config, falling back to defaults (and keeping a backup) if the file is unreadable.
    pub fn load_or_default(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(text) => match toml::from_str::<Config>(&text) {
                Ok(config) => config.sanitized(),
                Err(err) => {
                    log::warn!("config at {} is invalid ({err}); using defaults", path.display());
                    let _ = fs::copy(path, path.with_extension("toml.bak"));
                    Config::default()
                }
            },
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        let text = toml::to_string_pretty(self)?;
        let tmp = path.with_extension("toml.tmp");
        fs::write(&tmp, text)?;
        fs::rename(&tmp, path)?;
        Ok(())
    }

    /// Clamps values a hand-edited file could push out of range.
    pub fn sanitized(mut self) -> Self {
        self.general.ui_scale = self.general.ui_scale.clamp(0.5, 2.5);
        self.watch.facing_angle = self.watch.facing_angle.clamp(10.0, 80.0);
        self.watch.view_angle = self.watch.view_angle.clamp(10.0, 80.0);
        self.watch.max_distance = self.watch.max_distance.clamp(0.2, 1.5);
        self.watch.show_delay = self.watch.show_delay.clamp(0.0, 2.0);
        self.watch.hide_delay = self.watch.hide_delay.clamp(0.0, 3.0);
        self.watch.width_m = self.watch.width_m.clamp(0.06, 0.3);
        self.watch.opacity = self.watch.opacity.clamp(0.2, 1.0);
        self.interaction.laser_max_length = self.interaction.laser_max_length.clamp(0.5, 10.0);
        self.interaction.haptic_strength = self.interaction.haptic_strength.clamp(0.0, 1.0);
        self.interaction.scroll_speed = self.interaction.scroll_speed.clamp(1.0, 60.0);
        self.desktop.max_fps = self.desktop.max_fps.clamp(5, 144);
        self.desktop.default_width_m = self.desktop.default_width_m.clamp(0.3, 4.0);
        self.media.volume_step = self.media.volume_step.clamp(0.01, 0.25);
        self.notifications.default_duration = self.notifications.default_duration.clamp(1.0, 30.0);
        self.notifications.history_limit = self.notifications.history_limit.clamp(10, 1000);
        self
    }

    pub fn layout_for(&self, kind: PanelKind) -> Option<&SavedLayout> {
        self.layouts.iter().find(|layout| layout.kind == kind)
    }

    pub fn remember_layout(&mut self, layout: SavedLayout) {
        self.layouts.retain(|existing| existing.kind != layout.kind);
        self.layouts.push(layout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_toml() {
        let mut config = Config::default();
        config.watch.hand = Hand::Right;
        config.watch.keep_on_wrist = true;
        config.watch.offset_right = Some(Pose::default_watch_offset(Hand::Right));
        config.remember_layout(SavedLayout {
            kind: PanelKind::Vrcx,
            placement: Placement::Head,
            pose: Pose::IDENTITY,
            width_m: 0.5,
            opacity: 0.9,
        });

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        config.save(&path).unwrap();
        assert_eq!(Config::load_or_default(&path), config);
    }

    #[test]
    fn partial_files_fill_in_defaults() {
        let config: Config = toml::from_str("[watch]\nhand = \"Right\"\n").unwrap();
        assert_eq!(config.watch.hand, Hand::Right);
        assert_eq!(config.watch.facing_angle, WatchConfig::default().facing_angle);
        assert_eq!(config.notifications.udp_port, 42069);
    }

    #[test]
    fn invalid_files_fall_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "this is = = not toml").unwrap();
        assert_eq!(Config::load_or_default(&path), Config::default());
        assert!(path.with_extension("toml.bak").exists());
    }

    #[test]
    fn sanitize_clamps_out_of_range_values() {
        let mut config = Config::default();
        config.general.ui_scale = 40.0;
        config.desktop.max_fps = 0;
        let config = config.sanitized();
        assert_eq!(config.general.ui_scale, 2.5);
        assert_eq!(config.desktop.max_fps, 5);
    }
}
