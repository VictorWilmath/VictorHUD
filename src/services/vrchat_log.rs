//! Follows VRChat's `output_log_*.txt` to know which instance you're in and who is there.
//! This is the same source VRCX uses, so it works whether or not VRCX is running.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use crossbeam_channel::Sender;

use super::ServiceEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceAccess {
    Public,
    FriendsPlus,
    Friends,
    InvitePlus,
    Invite,
    GroupPublic,
    GroupPlus,
    Group,
}

impl InstanceAccess {
    pub fn label(self) -> &'static str {
        match self {
            InstanceAccess::Public => "Public",
            InstanceAccess::FriendsPlus => "Friends+",
            InstanceAccess::Friends => "Friends",
            InstanceAccess::InvitePlus => "Invite+",
            InstanceAccess::Invite => "Invite",
            InstanceAccess::GroupPublic => "Group Public",
            InstanceAccess::GroupPlus => "Group+",
            InstanceAccess::Group => "Group",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InstanceInfo {
    /// Full location string, e.g. `wrld_…:12345~friends(usr_…)~region(eu)`.
    pub location: String,
    pub world_id: String,
    pub instance_name: String,
    pub world_name: Option<String>,
    pub access: InstanceAccess,
    pub region: String,
    pub joined_at: DateTime<Local>,
}

impl InstanceInfo {
    pub fn from_location(location: &str, joined_at: DateTime<Local>) -> Option<Self> {
        let location = location.trim();
        let (world_id, rest) = location.split_once(':')?;
        if !world_id.starts_with("wrld_") {
            return None;
        }
        let mut parts = rest.split('~');
        let instance_name = parts.next().unwrap_or_default().to_string();
        let mut access = InstanceAccess::Public;
        let mut can_request_invite = false;
        let mut group_access = None;
        let mut region = "us".to_string();
        for part in parts {
            let (key, value) = match part.split_once('(') {
                Some((key, value)) => (key, value.trim_end_matches(')')),
                None => (part, ""),
            };
            match key {
                "hidden" => access = InstanceAccess::FriendsPlus,
                "friends" => access = InstanceAccess::Friends,
                "private" => access = InstanceAccess::Invite,
                "canRequestInvite" => can_request_invite = true,
                "group" => access = InstanceAccess::Group,
                "groupAccessType" => group_access = Some(value.to_string()),
                "region" => region = value.to_string(),
                _ => {}
            }
        }
        if access == InstanceAccess::Invite && can_request_invite {
            access = InstanceAccess::InvitePlus;
        }
        if access == InstanceAccess::Group {
            access = match group_access.as_deref() {
                Some("public") => InstanceAccess::GroupPublic,
                Some("plus") => InstanceAccess::GroupPlus,
                _ => InstanceAccess::Group,
            };
        }
        Some(Self {
            location: location.to_string(),
            world_id: world_id.to_string(),
            instance_name,
            world_name: None,
            access,
            region: region_label(&region).to_string(),
            joined_at,
        })
    }

    /// Link that opens this instance on the VRChat website (works for joinable instances).
    pub fn launch_url(&self) -> String {
        format!("https://vrchat.com/home/launch?worldId={}&instanceId={}", self.world_id, self.location.split_once(':').map(|(_, i)| i).unwrap_or(""))
    }

    pub fn world_url(&self) -> String {
        format!("https://vrchat.com/home/world/{}", self.world_id)
    }
}

fn region_label(code: &str) -> &str {
    match code {
        "us" => "US West",
        "use" => "US East",
        "eu" => "Europe",
        "jp" => "Japan",
        other => other,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerEntry {
    pub display_name: String,
    pub user_id: Option<String>,
    pub joined_at: DateTime<Local>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VrchatState {
    pub log_file: Option<PathBuf>,
    pub local_user: Option<String>,
    pub instance: Option<InstanceInfo>,
    /// Players in the current instance, in join order (includes you).
    pub players: Vec<PlayerEntry>,
    pub last_activity: Option<DateTime<Local>>,
}

/// Incremental parser for VRChat log lines.
#[derive(Default)]
pub struct LogParser {
    pub state: VrchatState,
}

impl LogParser {
    /// Feeds one line; returns whether the state changed.
    pub fn feed_line(&mut self, line: &str) -> bool {
        let Some((timestamp, message)) = split_line(line) else {
            return false;
        };
        let message = message.trim();
        let message = message
            .strip_prefix("[Behaviour]")
            .or_else(|| message.strip_prefix("[NetworkManager]"))
            .or_else(|| message.strip_prefix("[RoomManager]"))
            .map(str::trim)
            .unwrap_or(message);

        if let Some(rest) = message.strip_prefix("User Authenticated:") {
            let (name, _) = split_name_and_id(rest.trim());
            self.state.local_user = Some(name);
            return true;
        }
        if let Some(location) = message.strip_prefix("Joining ") {
            if location.starts_with("wrld_") {
                if let Some(instance) = InstanceInfo::from_location(location, timestamp) {
                    self.state.instance = Some(instance);
                    self.state.players.clear();
                    self.state.last_activity = Some(timestamp);
                    return true;
                }
            }
        }
        let room_name = message
            .strip_prefix("Joining or Creating Room:")
            .or_else(|| message.strip_prefix("Entering Room:"));
        if let Some(name) = room_name {
            if let Some(instance) = &mut self.state.instance {
                instance.world_name = Some(name.trim().to_string());
                self.state.last_activity = Some(timestamp);
                return true;
            }
            return false;
        }
        if message.starts_with("OnLeftRoom") {
            self.state.instance = None;
            self.state.players.clear();
            self.state.last_activity = Some(timestamp);
            return true;
        }
        if let Some(rest) = message.strip_prefix("OnPlayerJoined") {
            let rest = rest.trim_start_matches(':').trim();
            if rest.is_empty() {
                return false;
            }
            let (display_name, user_id) = split_name_and_id(rest);
            let already_here = self.state.players.iter().any(|p| match (&p.user_id, &user_id) {
                (Some(a), Some(b)) => a == b,
                _ => p.display_name == display_name,
            });
            if !already_here {
                self.state.players.push(PlayerEntry { display_name, user_id, joined_at: timestamp });
            }
            self.state.last_activity = Some(timestamp);
            return true;
        }
        if let Some(rest) = message.strip_prefix("OnPlayerLeft") {
            let rest = rest.trim_start_matches(':').trim();
            if rest.is_empty() || rest.starts_with("Room") {
                return false;
            }
            let (display_name, user_id) = split_name_and_id(rest);
            let before = self.state.players.len();
            self.state.players.retain(|p| match (&p.user_id, &user_id) {
                (Some(a), Some(b)) => a != b,
                _ => p.display_name != display_name,
            });
            self.state.last_activity = Some(timestamp);
            return self.state.players.len() != before;
        }
        false
    }
}

/// Splits `2024.06.15 21:04:33 Log        -  message` into a local timestamp and the message.
fn split_line(line: &str) -> Option<(DateTime<Local>, &str)> {
    let stamp = line.get(..19)?;
    let naive = NaiveDateTime::parse_from_str(stamp, "%Y.%m.%d %H:%M:%S").ok()?;
    let timestamp = Local.from_local_datetime(&naive).earliest()?;
    let (_, message) = line[19..].split_once(" -  ")?;
    Some((timestamp, message))
}

/// `Name (usr_123)` -> (`Name`, Some(`usr_123`)); `Name` -> (`Name`, None).
fn split_name_and_id(text: &str) -> (String, Option<String>) {
    if let Some(open) = text.rfind(" (usr_") {
        if text.ends_with(')') {
            let id = &text[open + 2..text.len() - 1];
            return (text[..open].trim().to_string(), Some(id.to_string()));
        }
    }
    (text.trim().to_string(), None)
}

/// Newest `output_log_*.txt` in `dir`.
pub fn newest_log_file(dir: &Path) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .filter(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.starts_with("output_log") && name.ends_with(".txt")
        })
        .filter_map(|entry| Some((entry.metadata().ok()?.modified().ok()?, entry.path())))
        .max_by_key(|(modified, _)| *modified)
        .map(|(_, path)| path)
}

/// Follows the newest log file and sends a snapshot whenever something changes.
pub fn spawn(dir: PathBuf, events: Sender<ServiceEvent>, shutdown: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("vrchat-log".into())
        .spawn(move || {
            let mut current: Option<PathBuf> = None;
            let mut parser = LogParser::default();
            let mut offset = 0u64;
            let mut pending = Vec::new();
            while !shutdown.load(Ordering::Relaxed) {
                if let Some(newest) = newest_log_file(&dir) {
                    if current.as_ref() != Some(&newest) {
                        log::info!("following VRChat log {}", newest.display());
                        parser = LogParser::default();
                        parser.state.log_file = Some(newest.clone());
                        offset = 0;
                        pending.clear();
                        current = Some(newest);
                        let _ = events.send(ServiceEvent::Vrchat(parser.state.clone()));
                    }
                }
                if let Some(path) = &current {
                    match read_new_bytes(path, &mut offset) {
                        Ok(bytes) if !bytes.is_empty() => {
                            pending.extend_from_slice(&bytes);
                            let mut changed = false;
                            while let Some(newline) = pending.iter().position(|&b| b == b'\n') {
                                let line: Vec<u8> = pending.drain(..=newline).collect();
                                let line = String::from_utf8_lossy(&line);
                                changed |= parser.feed_line(line.trim_end_matches(['\r', '\n']));
                            }
                            if changed {
                                let _ = events.send(ServiceEvent::Vrchat(parser.state.clone()));
                            }
                        }
                        Ok(_) => {}
                        Err(err) => log::debug!("reading VRChat log: {err}"),
                    }
                }
                std::thread::sleep(Duration::from_millis(750));
            }
        })
        .expect("spawning vrchat-log thread");
}

fn read_new_bytes(path: &Path, offset: &mut u64) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let len = file.metadata()?.len();
    if len < *offset {
        *offset = 0;
    }
    file.seek(SeekFrom::Start(*offset))?;
    let mut buffer = Vec::with_capacity((len - *offset) as usize);
    file.read_to_end(&mut buffer)?;
    *offset += buffer.len() as u64;
    Ok(buffer)
}

/// A short, realistic log used by the preview and tests.
pub fn demo_log() -> &'static str {
    "2026.11.16 21:02:11 Log        -  [Behaviour] User Authenticated: VictorWilmath (usr_0b1c2d3e-0000-4000-8000-000000000001)\n\
2026.11.16 21:40:02 Log        -  [Behaviour] Joining wrld_4432ea9b-729c-46e3-8eaf-846aa0a37fdd:48213~hidden(usr_0b1c2d3e-0000-4000-8000-000000000001)~region(use)\n\
2026.11.16 21:40:02 Log        -  [Behaviour] Joining or Creating Room: The Black Cat\n\
2026.11.16 21:40:09 Log        -  [Behaviour] OnPlayerJoined VictorWilmath (usr_0b1c2d3e-0000-4000-8000-000000000001)\n\
2026.11.16 21:40:09 Log        -  [Behaviour] OnPlayerJoined Kazuha (usr_aa000000-0000-4000-8000-000000000002)\n\
2026.11.16 21:40:09 Log        -  [Behaviour] OnPlayerJoined Moonbeam (usr_aa000000-0000-4000-8000-000000000003)\n\
2026.11.16 21:52:44 Log        -  [Behaviour] OnPlayerJoined Pixel Fox (usr_aa000000-0000-4000-8000-000000000004)\n\
2026.11.16 22:05:13 Log        -  [Behaviour] OnPlayerJoined Ren (usr_aa000000-0000-4000-8000-000000000005)\n\
2026.11.16 22:31:50 Log        -  [Behaviour] OnPlayerLeft Moonbeam (usr_aa000000-0000-4000-8000-000000000003)\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(log: &str) -> VrchatState {
        let mut parser = LogParser::default();
        for line in log.lines() {
            parser.feed_line(line);
        }
        parser.state
    }

    #[test]
    fn tracks_instance_world_and_players() {
        let state = parse(demo_log());
        assert_eq!(state.local_user.as_deref(), Some("VictorWilmath"));
        let instance = state.instance.unwrap();
        assert_eq!(instance.world_name.as_deref(), Some("The Black Cat"));
        assert_eq!(instance.access, InstanceAccess::FriendsPlus);
        assert_eq!(instance.region, "US East");
        assert_eq!(instance.instance_name, "48213");
        let names: Vec<_> = state.players.iter().map(|p| p.display_name.as_str()).collect();
        assert_eq!(names, ["VictorWilmath", "Kazuha", "Pixel Fox", "Ren"]);
        assert_eq!(state.players[1].user_id.as_deref(), Some("usr_aa000000-0000-4000-8000-000000000002"));
    }

    #[test]
    fn joining_a_new_world_resets_players() {
        let log = format!(
            "{}2026.11.16 23:00:00 Log        -  [Behaviour] OnLeftRoom\n\
2026.11.16 23:00:05 Log        -  [Behaviour] Joining wrld_ffff0000-0000-0000-0000-000000000000:1~private(usr_x)~canRequestInvite~region(eu)\n\
2026.11.16 23:00:05 Log        -  [Behaviour] Entering Room: Quiet Room\n",
            demo_log()
        );
        let state = parse(&log);
        let instance = state.instance.unwrap();
        assert_eq!(instance.world_name.as_deref(), Some("Quiet Room"));
        assert_eq!(instance.access, InstanceAccess::InvitePlus);
        assert_eq!(instance.region, "Europe");
        assert!(state.players.is_empty());
    }

    #[test]
    fn legacy_lines_without_user_ids() {
        let log = "2023.01.02 10:00:00 Log        -  [Behaviour] Joining wrld_1:2\n\
2023.01.02 10:00:01 Log        -  [Behaviour] OnPlayerJoined Old Friend\n\
2023.01.02 10:00:02 Log        -  [Behaviour] OnPlayerJoined Another One\n\
2023.01.02 10:05:00 Log        -  [Behaviour] OnPlayerLeft Old Friend\n";
        let state = parse(log);
        assert_eq!(state.instance.unwrap().access, InstanceAccess::Public);
        assert_eq!(state.players.len(), 1);
        assert_eq!(state.players[0].display_name, "Another One");
        assert_eq!(state.players[0].user_id, None);
    }

    #[test]
    fn group_instances() {
        let now = Local::now();
        let public = InstanceInfo::from_location("wrld_a:1~group(grp_x)~groupAccessType(public)~region(jp)", now).unwrap();
        assert_eq!(public.access, InstanceAccess::GroupPublic);
        assert_eq!(public.region, "Japan");
        let plus = InstanceInfo::from_location("wrld_a:1~group(grp_x)~groupAccessType(plus)", now).unwrap();
        assert_eq!(plus.access, InstanceAccess::GroupPlus);
        let friends = InstanceInfo::from_location("wrld_a:1~friends(usr_x)", now).unwrap();
        assert_eq!(friends.access, InstanceAccess::Friends);
        assert!(InstanceInfo::from_location("offline", now).is_none());
    }

    #[test]
    fn ignores_noise() {
        let mut parser = LogParser::default();
        assert!(!parser.feed_line(""));
        assert!(!parser.feed_line("garbage line"));
        assert!(!parser.feed_line("2026.11.16 21:02:11 Debug      -  [Always] Some unrelated output"));
    }

    #[test]
    fn follows_appended_lines_and_newest_file() {
        let dir = tempfile::tempdir().unwrap();
        let older = dir.path().join("output_log_2026-11-15_20-00-00.txt");
        std::fs::write(&older, "old").unwrap();
        std::thread::sleep(Duration::from_millis(20));
        let newer = dir.path().join("output_log_2026-11-16_21-00-00.txt");
        std::fs::write(&newer, &demo_log()[..200]).unwrap();
        assert_eq!(newest_log_file(dir.path()).as_deref(), Some(newer.as_path()));

        let mut offset = 0;
        let first = read_new_bytes(&newer, &mut offset).unwrap();
        assert_eq!(first.len(), 200);
        std::fs::write(&newer, demo_log()).unwrap();
        let rest = read_new_bytes(&newer, &mut offset).unwrap();
        assert_eq!(first.len() + rest.len(), demo_log().len());
    }
}
