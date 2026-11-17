//! Media sessions (anything that shows up in the Windows media flyout: Spotify, browsers, players).

pub mod mock;

use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::state::Artwork;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlaybackStatus {
    #[default]
    Stopped,
    Playing,
    Paused,
    Changing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepeatMode {
    Off,
    Track,
    List,
}

impl RepeatMode {
    pub fn next(self) -> RepeatMode {
        match self {
            RepeatMode::Off => RepeatMode::List,
            RepeatMode::List => RepeatMode::Track,
            RepeatMode::Track => RepeatMode::Off,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaSession {
    /// Stable identifier for the session (the app's AppUserModelID).
    pub id: String,
    /// Friendly name of the app playing it, e.g. "Spotify" or "Firefox".
    pub app_name: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub status: PlaybackStatus,
    pub position: Option<Duration>,
    pub duration: Option<Duration>,
    /// When `position` was sampled, so the UI can advance it smoothly while playing.
    pub sampled_at: Instant,
    pub can_play_pause: bool,
    pub can_next: bool,
    pub can_previous: bool,
    pub can_seek: bool,
    pub shuffle: Option<bool>,
    pub repeat: Option<RepeatMode>,
    pub artwork: Option<Arc<Artwork>>,
    /// Per-app volume (0..1) when the app's audio session could be found.
    pub app_volume: Option<f32>,
}

impl MediaSession {
    pub fn live_position(&self, now: Instant) -> Option<Duration> {
        let position = self.position?;
        let position = if self.status == PlaybackStatus::Playing {
            position + now.saturating_duration_since(self.sampled_at)
        } else {
            position
        };
        Some(match self.duration {
            Some(duration) if duration > Duration::ZERO => position.min(duration),
            _ => position,
        })
    }

    pub fn progress(&self, now: Instant) -> Option<f32> {
        let duration = self.duration.filter(|d| !d.is_zero())?;
        Some(self.live_position(now)?.as_secs_f32() / duration.as_secs_f32())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaSnapshot {
    pub sessions: Vec<MediaSession>,
    /// The session Windows currently considers active.
    pub current: Option<String>,
    pub master_volume: Option<f32>,
    pub muted: Option<bool>,
    pub error: Option<String>,
}

impl MediaSnapshot {
    /// The session to show: the user's pick if still present, then Windows' current session,
    /// then anything playing, then the first session.
    pub fn active(&self, preferred: Option<&str>) -> Option<&MediaSession> {
        preferred
            .and_then(|id| self.sessions.iter().find(|s| s.id == id))
            .or_else(|| self.current.as_deref().and_then(|id| self.sessions.iter().find(|s| s.id == id)))
            .or_else(|| self.sessions.iter().find(|s| s.status == PlaybackStatus::Playing))
            .or_else(|| self.sessions.first())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaCommand {
    PlayPause { session: String },
    Next { session: String },
    Previous { session: String },
    Seek { session: String, position: Duration },
    SetShuffle { session: String, shuffle: bool },
    SetRepeat { session: String, repeat: RepeatMode },
    SetAppVolume { session: String, volume: f32 },
    SetMasterVolume(f32),
    SetMuted(bool),
}

/// Maps a media session's AppUserModelID to a readable app name.
pub fn friendly_app_name(aumid: &str) -> String {
    let lower = aumid.to_ascii_lowercase();
    let known = [
        ("spotify", "Spotify"),
        ("chrome", "Chrome"),
        ("msedge", "Edge"),
        ("308046b0af4a39cb", "Firefox"),
        ("firefox", "Firefox"),
        ("opera", "Opera"),
        ("brave", "Brave"),
        ("vivaldi", "Vivaldi"),
        ("discord", "Discord"),
        ("vlc", "VLC"),
        ("zunemusic", "Media Player"),
        ("zunevideo", "Movies & TV"),
        ("applemusic", "Apple Music"),
        ("itunes", "iTunes"),
        ("tidal", "TIDAL"),
        ("deezer", "Deezer"),
        ("amazonmusic", "Amazon Music"),
        ("foobar2000", "foobar2000"),
        ("musicbee", "MusicBee"),
        ("aimp", "AIMP"),
        ("plex", "Plex"),
        ("mpv", "mpv"),
        ("vrchat", "VRChat"),
    ];
    if let Some((_, name)) = known.iter().find(|(needle, _)| lower.contains(needle)) {
        return name.to_string();
    }
    // Packaged apps look like `Publisher.App_hash!Entry`; executables like `app.exe`.
    let base = aumid.split('!').next().unwrap_or(aumid);
    let base = base.split('_').next().unwrap_or(base);
    let base = base.rsplit('.').find(|part| !part.eq_ignore_ascii_case("exe")).unwrap_or(base);
    crate::util::pretty_app_name(base)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(id: &str, status: PlaybackStatus) -> MediaSession {
        MediaSession {
            id: id.into(),
            app_name: friendly_app_name(id),
            title: "Song".into(),
            artist: "Artist".into(),
            album: String::new(),
            status,
            position: Some(Duration::from_secs(10)),
            duration: Some(Duration::from_secs(100)),
            sampled_at: Instant::now(),
            can_play_pause: true,
            can_next: true,
            can_previous: true,
            can_seek: true,
            shuffle: None,
            repeat: None,
            artwork: None,
            app_volume: None,
        }
    }

    #[test]
    fn friendly_names() {
        assert_eq!(friendly_app_name("Spotify.exe"), "Spotify");
        assert_eq!(friendly_app_name("308046B0AF4A39CB"), "Firefox");
        assert_eq!(friendly_app_name("MSEdge"), "Edge");
        assert_eq!(friendly_app_name("Microsoft.ZuneMusic_8wekyb3d8bbwe!Microsoft.ZuneMusic"), "Media Player");
        assert_eq!(friendly_app_name("Contoso.CoolPlayer_abc123!App"), "CoolPlayer");
        assert_eq!(friendly_app_name("someplayer.exe"), "Someplayer");
    }

    #[test]
    fn active_session_preference_order() {
        let snapshot = MediaSnapshot {
            sessions: vec![session("a.exe", PlaybackStatus::Paused), session("b.exe", PlaybackStatus::Playing)],
            current: None,
            ..Default::default()
        };
        assert_eq!(snapshot.active(None).unwrap().id, "b.exe");
        assert_eq!(snapshot.active(Some("a.exe")).unwrap().id, "a.exe");
        assert_eq!(snapshot.active(Some("gone.exe")).unwrap().id, "b.exe");
        let snapshot = MediaSnapshot { current: Some("a.exe".into()), ..snapshot };
        assert_eq!(snapshot.active(None).unwrap().id, "a.exe");
    }

    #[test]
    fn live_position_advances_only_while_playing() {
        let mut s = session("a.exe", PlaybackStatus::Playing);
        let later = s.sampled_at + Duration::from_secs(5);
        assert_eq!(s.live_position(later), Some(Duration::from_secs(15)));
        let much_later = s.sampled_at + Duration::from_secs(500);
        assert_eq!(s.live_position(much_later), Some(Duration::from_secs(100)));
        s.status = PlaybackStatus::Paused;
        assert_eq!(s.live_position(later), Some(Duration::from_secs(10)));
        assert!((s.progress(later).unwrap() - 0.1).abs() < 1e-6);
    }
}
