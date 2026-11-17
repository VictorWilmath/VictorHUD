//! System media sessions (whatever shows in the Windows volume flyout: Spotify, browsers, media
//! players) through `GlobalSystemMediaTransportControlsSessionManager`, plus master and per-app
//! volume through Core Audio.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use windows::Foundation::TimeSpan;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as Session,
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties as MediaProperties,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as Status,
};
use windows::Media::MediaPlaybackAutoRepeatMode;
use windows::Storage::Streams::DataReader;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
    IAudioSessionControl2, IAudioSessionManager2, IMMDeviceEnumerator, ISimpleAudioVolume, MMDeviceEnumerator,
    eMultimedia, eRender,
};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize};
use windows::core::Interface;

use crate::services::ServiceEvent;
use crate::services::media::{
    MediaCommand, MediaSession, MediaSnapshot, PlaybackStatus, RepeatMode, friendly_app_name,
};
use crate::state::Artwork;

const POLL_INTERVAL: Duration = Duration::from_millis(500);
/// Re-send unchanged snapshots this often so the UI's extrapolated positions stay in sync.
const RESYNC_INTERVAL: Duration = Duration::from_secs(5);
const AUDIO_REFRESH: Duration = Duration::from_secs(10);
/// Seconds between 1601-01-01 (Windows FILETIME epoch) and 1970-01-01.
const FILETIME_UNIX_OFFSET: i64 = 11_644_473_600;

pub fn spawn(commands: Receiver<MediaCommand>, events: Sender<ServiceEvent>, shutdown: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("media".into())
        .spawn(move || {
            if let Err(err) = unsafe { RoInitialize(RO_INIT_MULTITHREADED) } {
                log::debug!("RoInitialize on the media thread: {err}");
            }
            let mut service = MediaService::new();
            while !shutdown.load(Ordering::Relaxed) {
                let snapshot = service.snapshot();
                if service.should_send(&snapshot) && events.send(ServiceEvent::Media(snapshot)).is_err() {
                    return;
                }
                match commands.recv_timeout(POLL_INTERVAL) {
                    Ok(command) => {
                        service.apply(command);
                        while let Ok(command) = commands.try_recv() {
                            service.apply(command);
                        }
                        // Give the app a moment to update its state before reading it back.
                        std::thread::sleep(Duration::from_millis(150));
                        service.force_send = true;
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }
        })
        .expect("spawning media thread");
}

struct CachedArt {
    artwork: Option<Arc<Artwork>>,
    attempts: u32,
    last_attempt: Instant,
}

struct Audio {
    endpoint: IAudioEndpointVolume,
    sessions: IAudioSessionManager2,
    opened: Instant,
}

struct MediaService {
    manager: Option<SessionManager>,
    manager_error: Option<String>,
    audio: Option<Audio>,
    audio_retry: Option<Instant>,
    art: HashMap<String, CachedArt>,
    last_sent: Option<MediaSnapshot>,
    last_sent_at: Option<Instant>,
    force_send: bool,
}

impl MediaService {
    fn new() -> Self {
        let (manager, manager_error) = match SessionManager::RequestAsync().and_then(|op| op.join()) {
            Ok(manager) => (Some(manager), None),
            Err(err) => {
                log::warn!("system media sessions unavailable: {err}");
                (None, Some("Windows media controls aren't available on this system".to_string()))
            }
        };
        Self {
            manager,
            manager_error,
            audio: None,
            audio_retry: None,
            art: HashMap::new(),
            last_sent: None,
            last_sent_at: None,
            force_send: true,
        }
    }

    fn should_send(&mut self, snapshot: &MediaSnapshot) -> bool {
        let changed = match &self.last_sent {
            Some(previous) => !same_ignoring_timing(previous, snapshot),
            None => true,
        };
        let stale = self.last_sent_at.is_none_or(|at| at.elapsed() >= RESYNC_INTERVAL);
        if changed || stale || self.force_send {
            self.force_send = false;
            self.last_sent = Some(snapshot.clone());
            self.last_sent_at = Some(Instant::now());
            true
        } else {
            false
        }
    }

    fn audio(&mut self) -> Option<&Audio> {
        let expired = self.audio.as_ref().is_some_and(|audio| audio.opened.elapsed() >= AUDIO_REFRESH);
        if expired {
            self.audio = None;
        }
        if self.audio.is_none() && self.audio_retry.is_none_or(|at| Instant::now() >= at) {
            match open_audio() {
                Ok(audio) => self.audio = Some(audio),
                Err(err) => {
                    log::debug!("default audio device unavailable: {err}");
                    self.audio_retry = Some(Instant::now() + Duration::from_secs(5));
                }
            }
        }
        self.audio.as_ref()
    }

    fn sessions(&self) -> Vec<Session> {
        self.manager
            .as_ref()
            .and_then(|manager| manager.GetSessions().ok())
            .map(|list| list.into_iter().collect())
            .unwrap_or_default()
    }

    fn find(&self, id: &str) -> Option<Session> {
        self.sessions().into_iter().find(|s| s.SourceAppUserModelId().map(|aumid| aumid == id).unwrap_or(false))
    }

    fn snapshot(&mut self) -> MediaSnapshot {
        let mut snapshot = MediaSnapshot { error: self.manager_error.clone(), ..Default::default() };
        if let Some(manager) = &self.manager {
            snapshot.current = manager
                .GetCurrentSession()
                .ok()
                .and_then(|session| session.SourceAppUserModelId().ok())
                .map(|id| id.to_string());
        }
        let sessions = self.sessions();
        let mut seen = Vec::new();
        for session in &sessions {
            match self.read_session(session) {
                Ok(read) => {
                    seen.push(art_key(&read));
                    snapshot.sessions.push(read);
                }
                Err(err) => log::debug!("skipping a media session: {err}"),
            }
        }
        self.art.retain(|key, _| seen.contains(key));

        let ids: Vec<String> = snapshot.sessions.iter().map(|s| s.id.clone()).collect();
        if let Some(audio) = self.audio() {
            unsafe {
                snapshot.master_volume = audio.endpoint.GetMasterVolumeLevelScalar().ok();
                snapshot.muted = audio.endpoint.GetMute().ok().map(|m| m.as_bool());
            }
            let volumes: Vec<Option<f32>> = ids
                .iter()
                .map(|id| app_volumes(audio, id).first().and_then(|volume| unsafe { volume.GetMasterVolume() }.ok()))
                .collect();
            for (session, volume) in snapshot.sessions.iter_mut().zip(volumes) {
                session.app_volume = volume;
            }
        }
        snapshot
    }

    fn read_session(&mut self, session: &Session) -> windows::core::Result<MediaSession> {
        let id = session.SourceAppUserModelId()?.to_string();
        let properties = session.TryGetMediaPropertiesAsync().and_then(|op| op.join()).ok();
        let text = |get: &dyn Fn(&MediaProperties) -> windows::core::Result<windows::core::HSTRING>| {
            properties.as_ref().and_then(|p| get(p).ok()).map(|s| s.to_string()).unwrap_or_default()
        };
        let title = text(&|p| p.Title());
        let artist = {
            let artist = text(&|p| p.Artist());
            if artist.is_empty() { text(&|p| p.AlbumArtist()) } else { artist }
        };
        let album = text(&|p| p.AlbumTitle());

        let info = session.GetPlaybackInfo()?;
        let status = match info.PlaybackStatus()? {
            Status::Playing => PlaybackStatus::Playing,
            Status::Paused => PlaybackStatus::Paused,
            Status::Changing | Status::Opened => PlaybackStatus::Changing,
            _ => PlaybackStatus::Stopped,
        };
        let controls = info.Controls()?;
        let enabled = |value: windows::core::Result<bool>| value.unwrap_or(false);
        let shuffle = if enabled(controls.IsShuffleEnabled()) {
            info.IsShuffleActive().and_then(|r| r.Value()).ok()
        } else {
            None
        };
        let repeat = if enabled(controls.IsRepeatEnabled()) {
            info.AutoRepeatMode().and_then(|r| r.Value()).ok().map(|mode| match mode {
                MediaPlaybackAutoRepeatMode::Track => RepeatMode::Track,
                MediaPlaybackAutoRepeatMode::List => RepeatMode::List,
                _ => RepeatMode::Off,
            })
        } else {
            None
        };

        let (position, duration) = match session.GetTimelineProperties() {
            Ok(timeline) => {
                let start = timeline.StartTime().map(ticks).unwrap_or(0);
                let end = timeline.EndTime().map(ticks).unwrap_or(0);
                let mut position = timeline.Position().map(ticks).unwrap_or(0) - start;
                if status == PlaybackStatus::Playing
                    && let Ok(updated) = timeline.LastUpdatedTime()
                    && updated.UniversalTime > 0
                {
                    // Apps only report the position when it jumps; extrapolate from that moment.
                    position += (now_filetime() - updated.UniversalTime).max(0);
                }
                let length = end - start;
                if length > 0 {
                    (Some(ticks_to_duration(position.clamp(0, length))), Some(ticks_to_duration(length)))
                } else {
                    (None, None)
                }
            }
            Err(_) => (None, None),
        };

        let mut read = MediaSession {
            app_name: friendly_app_name(&id),
            id,
            title,
            artist,
            album,
            status,
            position,
            duration,
            sampled_at: Instant::now(),
            can_play_pause: enabled(controls.IsPlayPauseToggleEnabled())
                || enabled(controls.IsPlayEnabled())
                || enabled(controls.IsPauseEnabled()),
            can_next: enabled(controls.IsNextEnabled()),
            can_previous: enabled(controls.IsPreviousEnabled()),
            can_seek: enabled(controls.IsPlaybackPositionEnabled()) && duration.is_some(),
            shuffle,
            repeat,
            artwork: None,
            app_volume: None,
        };
        read.artwork = self.artwork(&read, properties.as_ref());
        Ok(read)
    }

    /// Album art is fetched once per track (retrying briefly, since apps often set it late).
    fn artwork(&mut self, session: &MediaSession, properties: Option<&MediaProperties>) -> Option<Arc<Artwork>> {
        let key = art_key(session);
        if let Some(cached) = self.art.get_mut(&key) {
            let retry = cached.artwork.is_none()
                && cached.attempts < 4
                && cached.last_attempt.elapsed() >= Duration::from_millis(1500);
            if !retry {
                return cached.artwork.clone();
            }
            cached.attempts += 1;
            cached.last_attempt = Instant::now();
            cached.artwork = properties.and_then(read_thumbnail);
            return cached.artwork.clone();
        }
        let artwork = properties.and_then(read_thumbnail);
        self.art.insert(key, CachedArt { artwork: artwork.clone(), attempts: 1, last_attempt: Instant::now() });
        artwork
    }

    fn apply(&mut self, command: MediaCommand) {
        let result: anyhow::Result<()> = (|| {
            match &command {
                MediaCommand::PlayPause { session } => {
                    if let Some(s) = self.find(session) {
                        s.TryTogglePlayPauseAsync()?.join()?;
                    }
                }
                MediaCommand::Next { session } => {
                    if let Some(s) = self.find(session) {
                        s.TrySkipNextAsync()?.join()?;
                    }
                }
                MediaCommand::Previous { session } => {
                    if let Some(s) = self.find(session) {
                        s.TrySkipPreviousAsync()?.join()?;
                    }
                }
                MediaCommand::Seek { session, position } => {
                    if let Some(s) = self.find(session) {
                        let start = s.GetTimelineProperties().and_then(|t| t.StartTime()).map(ticks).unwrap_or(0);
                        let target = start + (position.as_nanos() / 100) as i64;
                        s.TryChangePlaybackPositionAsync(target)?.join()?;
                    }
                }
                MediaCommand::SetShuffle { session, shuffle } => {
                    if let Some(s) = self.find(session) {
                        s.TryChangeShuffleActiveAsync(*shuffle)?.join()?;
                    }
                }
                MediaCommand::SetRepeat { session, repeat } => {
                    if let Some(s) = self.find(session) {
                        let mode = match repeat {
                            RepeatMode::Off => MediaPlaybackAutoRepeatMode::None,
                            RepeatMode::Track => MediaPlaybackAutoRepeatMode::Track,
                            RepeatMode::List => MediaPlaybackAutoRepeatMode::List,
                        };
                        s.TryChangeAutoRepeatModeAsync(mode)?.join()?;
                    }
                }
                MediaCommand::SetAppVolume { session, volume } => {
                    if let Some(audio) = self.audio() {
                        for control in app_volumes(audio, session) {
                            unsafe { control.SetMasterVolume(volume.clamp(0.0, 1.0), std::ptr::null()) }?;
                        }
                    }
                }
                MediaCommand::SetMasterVolume(volume) => {
                    if let Some(audio) = self.audio() {
                        unsafe { audio.endpoint.SetMasterVolumeLevelScalar(volume.clamp(0.0, 1.0), std::ptr::null()) }?;
                    }
                }
                MediaCommand::SetMuted(muted) => {
                    if let Some(audio) = self.audio() {
                        unsafe { audio.endpoint.SetMute(*muted, std::ptr::null()) }?;
                    }
                }
            }
            Ok(())
        })();
        if let Err(err) = result {
            log::info!("media command {command:?} failed: {err:#}");
        }
    }
}

fn art_key(session: &MediaSession) -> String {
    format!("{}\n{}\n{}\n{}", session.id, session.title, session.artist, session.album)
}

fn read_thumbnail(properties: &MediaProperties) -> Option<Arc<Artwork>> {
    let reference = properties.Thumbnail().ok()?;
    let stream = reference.OpenReadAsync().ok()?.join().ok()?;
    let size = stream.Size().ok()?;
    if size == 0 || size > 16 * 1024 * 1024 {
        return None;
    }
    let reader = DataReader::CreateDataReader(&stream).ok()?;
    reader.LoadAsync(size as u32).ok()?.join().ok()?;
    let mut bytes = vec![0u8; size as usize];
    reader.ReadBytes(&mut bytes).ok()?;
    Artwork::decode(&bytes, 256)
}

fn open_audio() -> windows::core::Result<Audio> {
    unsafe {
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        let endpoint: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;
        let sessions: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
        Ok(Audio { endpoint, sessions, opened: Instant::now() })
    }
}

/// Volume controls of every audio session that belongs to the app behind media session `id`.
fn app_volumes(audio: &Audio, id: &str) -> Vec<ISimpleAudioVolume> {
    let mut found = Vec::new();
    let Ok(list) = (unsafe { audio.sessions.GetSessionEnumerator() }) else { return found };
    let count = unsafe { list.GetCount() }.unwrap_or(0);
    for index in 0..count {
        let Ok(control) = (unsafe { list.GetSession(index) }) else { continue };
        let Ok(control2) = control.cast::<IAudioSessionControl2>() else { continue };
        let pid = unsafe { control2.GetProcessId() }.unwrap_or(0);
        if pid == 0 {
            continue;
        }
        let Some(exe) = super::desktop::process_path(pid) else { continue };
        if session_matches_process(id, &exe)
            && let Ok(volume) = control.cast::<ISimpleAudioVolume>()
        {
            found.push(volume);
        }
    }
    found
}

/// Whether a media session's AppUserModelID belongs to the process at `exe_path`.
pub fn session_matches_process(aumid: &str, exe_path: &str) -> bool {
    let exe = exe_path.rsplit(['\\', '/']).next().unwrap_or(exe_path).to_ascii_lowercase();
    let stem = exe.strip_suffix(".exe").unwrap_or(&exe);
    if stem.is_empty() {
        return false;
    }
    let aumid_lower = aumid.to_ascii_lowercase();
    aumid_lower.contains(stem) || friendly_app_name(aumid).eq_ignore_ascii_case(&crate::util::pretty_app_name(stem))
}

fn ticks(span: TimeSpan) -> i64 {
    span.Duration
}

fn ticks_to_duration(ticks: i64) -> Duration {
    Duration::from_nanos(ticks.max(0) as u64 * 100)
}

fn now_filetime() -> i64 {
    let since_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    (since_unix.as_secs() as i64 + FILETIME_UNIX_OFFSET) * 10_000_000 + since_unix.subsec_nanos() as i64 / 100
}

/// Equal apart from sample timestamps and the small position drift those cause.
fn same_ignoring_timing(a: &MediaSnapshot, b: &MediaSnapshot) -> bool {
    if a.current != b.current
        || a.master_volume != b.master_volume
        || a.muted != b.muted
        || a.error != b.error
        || a.sessions.len() != b.sessions.len()
    {
        return false;
    }
    a.sessions.iter().zip(&b.sessions).all(|(x, y)| {
        let drift = match (x.live_position(Instant::now()), y.live_position(Instant::now())) {
            (Some(p), Some(q)) => p.abs_diff(q) < Duration::from_millis(1500),
            (None, None) => true,
            _ => false,
        };
        drift
            && x.id == y.id
            && x.title == y.title
            && x.artist == y.artist
            && x.album == y.album
            && x.status == y.status
            && x.duration == y.duration
            && x.can_play_pause == y.can_play_pause
            && x.can_next == y.can_next
            && x.can_previous == y.can_previous
            && x.can_seek == y.can_seek
            && x.shuffle == y.shuffle
            && x.repeat == y.repeat
            && x.artwork.as_ref().map(|a| a.id) == y.artwork.as_ref().map(|a| a.id)
            && x.app_volume == y.app_volume
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_media_sessions_to_processes() {
        assert!(session_matches_process("Spotify.exe", r"C:\Users\v\AppData\Roaming\Spotify\Spotify.exe"));
        assert!(session_matches_process("Chrome", r"C:\Program Files\Google\Chrome\Application\chrome.exe"));
        assert!(session_matches_process("MSEdge", r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"));
        assert!(session_matches_process("308046B0AF4A39CB", r"C:\Program Files\Mozilla Firefox\firefox.exe"));
        assert!(!session_matches_process("Spotify.exe", r"C:\Program Files\Mozilla Firefox\firefox.exe"));
        assert!(!session_matches_process("Spotify.exe", ""));
    }

    #[test]
    fn filetime_is_after_2020() {
        // 2020-01-01 in FILETIME ticks.
        assert!(now_filetime() > 132_223_104_000_000_000);
    }
}
