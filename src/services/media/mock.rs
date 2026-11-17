//! Demo media sessions for the desktop preview and for platforms without system media controls.

use std::sync::Arc;
use std::time::{Duration, Instant};

use super::{MediaCommand, MediaSession, MediaSnapshot, PlaybackStatus, RepeatMode};
use crate::state::Artwork;

struct Track {
    title: &'static str,
    artist: &'static str,
    album: &'static str,
    length: u64,
    colors: ([u8; 3], [u8; 3]),
}

const SPOTIFY_TRACKS: &[Track] = &[
    Track {
        title: "Midnight City",
        artist: "M83",
        album: "Hurry Up, We're Dreaming",
        length: 243,
        colors: ([255, 94, 98], [72, 52, 212]),
    },
    Track {
        title: "Instant Crush",
        artist: "Daft Punk, Julian Casablancas",
        album: "Random Access Memories",
        length: 337,
        colors: ([20, 20, 30], [212, 175, 55]),
    },
    Track {
        title: "Resonance",
        artist: "HOME",
        album: "Odyssey",
        length: 212,
        colors: ([255, 120, 200], [40, 200, 255]),
    },
];

pub struct MockMedia {
    track: usize,
    position: Duration,
    playing: bool,
    shuffle: bool,
    repeat: RepeatMode,
    last_tick: Instant,
    master_volume: f32,
    muted: bool,
    spotify_volume: f32,
    artworks: Vec<Arc<Artwork>>,
    video_position: Duration,
}

impl Default for MockMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl MockMedia {
    pub fn new() -> Self {
        Self {
            track: 0,
            position: Duration::from_secs(83),
            playing: true,
            shuffle: false,
            repeat: RepeatMode::Off,
            last_tick: Instant::now(),
            master_volume: 0.62,
            muted: false,
            spotify_volume: 0.8,
            artworks: SPOTIFY_TRACKS
                .iter()
                .enumerate()
                .map(|(i, t)| gradient_artwork(i as u64 + 1, t.colors.0, t.colors.1))
                .collect(),
            video_position: Duration::from_secs(412),
        }
    }

    fn advance(&mut self) {
        let now = Instant::now();
        if self.playing {
            self.position += now - self.last_tick;
            let length = Duration::from_secs(SPOTIFY_TRACKS[self.track].length);
            if self.position >= length {
                self.position = Duration::ZERO;
                if self.repeat != RepeatMode::Track {
                    self.track = (self.track + 1) % SPOTIFY_TRACKS.len();
                }
            }
        }
        self.last_tick = now;
    }

    pub fn apply(&mut self, command: MediaCommand) {
        self.advance();
        match command {
            MediaCommand::PlayPause { session } if session == "Spotify.exe" => self.playing = !self.playing,
            MediaCommand::Next { session } if session == "Spotify.exe" => {
                self.track = (self.track + 1) % SPOTIFY_TRACKS.len();
                self.position = Duration::ZERO;
            }
            MediaCommand::Previous { session } if session == "Spotify.exe" => {
                if self.position > Duration::from_secs(3) {
                    self.position = Duration::ZERO;
                } else {
                    self.track = (self.track + SPOTIFY_TRACKS.len() - 1) % SPOTIFY_TRACKS.len();
                    self.position = Duration::ZERO;
                }
            }
            MediaCommand::Seek { session, position } if session == "Spotify.exe" => self.position = position,
            MediaCommand::Seek { position, .. } => self.video_position = position,
            MediaCommand::SetShuffle { shuffle, .. } => self.shuffle = shuffle,
            MediaCommand::SetRepeat { repeat, .. } => self.repeat = repeat,
            MediaCommand::SetAppVolume { volume, .. } => self.spotify_volume = volume.clamp(0.0, 1.0),
            MediaCommand::SetMasterVolume(volume) => self.master_volume = volume.clamp(0.0, 1.0),
            MediaCommand::SetMuted(muted) => self.muted = muted,
            _ => {}
        }
    }

    pub fn snapshot(&mut self) -> MediaSnapshot {
        self.advance();
        let track = &SPOTIFY_TRACKS[self.track];
        let now = Instant::now();
        let spotify = MediaSession {
            id: "Spotify.exe".into(),
            app_name: "Spotify".into(),
            title: track.title.into(),
            artist: track.artist.into(),
            album: track.album.into(),
            status: if self.playing { PlaybackStatus::Playing } else { PlaybackStatus::Paused },
            position: Some(self.position),
            duration: Some(Duration::from_secs(track.length)),
            sampled_at: now,
            can_play_pause: true,
            can_next: true,
            can_previous: true,
            can_seek: true,
            shuffle: Some(self.shuffle),
            repeat: Some(self.repeat),
            artwork: Some(self.artworks[self.track].clone()),
            app_volume: Some(self.spotify_volume),
        };
        let video = MediaSession {
            id: "308046B0AF4A39CB".into(),
            app_name: "Firefox".into(),
            title: "Building a VR overlay in Rust - Part 3".into(),
            artist: "YouTube".into(),
            album: String::new(),
            status: PlaybackStatus::Paused,
            position: Some(self.video_position),
            duration: Some(Duration::from_secs(1520)),
            sampled_at: now,
            can_play_pause: true,
            can_next: false,
            can_previous: false,
            can_seek: true,
            shuffle: None,
            repeat: None,
            artwork: None,
            app_volume: Some(1.0),
        };
        MediaSnapshot {
            sessions: vec![spotify, video],
            current: Some("Spotify.exe".into()),
            master_volume: Some(self.master_volume),
            muted: Some(self.muted),
            error: None,
        }
    }
}

/// A soft diagonal gradient used as stand-in album art.
pub fn gradient_artwork(id: u64, a: [u8; 3], b: [u8; 3]) -> Arc<Artwork> {
    const SIZE: usize = 96;
    let mut rgba = Vec::with_capacity(SIZE * SIZE * 4);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let t = (x + y) as f32 / (2 * SIZE - 2) as f32;
            let glow = 1.0 - (((x as f32 - 30.0).powi(2) + (y as f32 - 28.0).powi(2)).sqrt() / 90.0).min(1.0);
            for c in 0..3 {
                let base = a[c] as f32 * (1.0 - t) + b[c] as f32 * t;
                rgba.push((base + glow * 60.0).min(255.0) as u8);
            }
            rgba.push(255);
        }
    }
    Arc::new(Artwork { id: 0xA27_0000 + id, size: [SIZE, SIZE], rgba })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_change_the_demo_state() {
        let mut media = MockMedia::new();
        let first = media.snapshot().sessions[0].title.clone();
        media.apply(MediaCommand::Next { session: "Spotify.exe".into() });
        assert_ne!(media.snapshot().sessions[0].title, first);
        media.apply(MediaCommand::PlayPause { session: "Spotify.exe".into() });
        assert_eq!(media.snapshot().sessions[0].status, PlaybackStatus::Paused);
        media.apply(MediaCommand::SetMasterVolume(2.0));
        assert_eq!(media.snapshot().master_volume, Some(1.0));
    }
}
