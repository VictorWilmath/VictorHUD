//! Read-only access to VRCX's local database (`%APPDATA%\VRCX\VRCX.sqlite3`).
//!
//! VRCX keeps per-account tables named `{prefix}_feed_gps`, `{prefix}_feed_online_offline`, ...
//! where the prefix is the logged-in user id without `-` and `_`. VictorHUD never writes to it.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::{DateTime, Local};
use crossbeam_channel::Sender;
use rusqlite::{Connection, OpenFlags, OptionalExtension};

use super::ServiceEvent;

const FEED_LIMIT: usize = 150;
const RECENT_WORLD_LIMIT: usize = 25;
/// Friends whose last "online" event is older than this are not assumed to still be online.
const ONLINE_STALE_AFTER: chrono::Duration = chrono::Duration::hours(12);

#[derive(Clone, Debug, Default, PartialEq)]
pub enum VrcxStatus {
    #[default]
    Starting,
    NotFound(PathBuf),
    Error(String),
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedKind {
    Location,
    Online,
    Offline,
    Status,
}

impl FeedKind {
    pub fn label(self) -> &'static str {
        match self {
            FeedKind::Location => "Location",
            FeedKind::Online => "Online",
            FeedKind::Offline => "Offline",
            FeedKind::Status => "Status",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FeedEntry {
    pub created_at: Option<DateTime<Local>>,
    pub kind: FeedKind,
    pub user_id: String,
    pub display_name: String,
    /// World name for location/online events, status text for status events.
    pub detail: String,
    pub location: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FriendEntry {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: Option<String>,
    /// Estimated from VRCX's feed, so only as fresh as VRCX's last update.
    pub likely_online: bool,
    pub last_world: Option<String>,
    pub last_location: Option<String>,
    pub last_seen: Option<DateTime<Local>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecentWorld {
    pub visited_at: Option<DateTime<Local>>,
    pub world_id: String,
    pub world_name: String,
    pub location: String,
    pub time_spent: Option<Duration>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VrcxSnapshot {
    pub status: VrcxStatus,
    pub db_path: Option<PathBuf>,
    pub user_prefix: Option<String>,
    pub feed: Vec<FeedEntry>,
    pub friends: Vec<FriendEntry>,
    pub recent_worlds: Vec<RecentWorld>,
    pub memos: HashMap<String, String>,
    pub updated_at: Option<DateTime<Local>>,
}

impl VrcxSnapshot {
    pub fn is_friend(&self, user_id: &str) -> bool {
        self.friends.iter().any(|f| f.user_id == user_id)
    }

    pub fn online_friends(&self) -> impl Iterator<Item = &FriendEntry> {
        self.friends.iter().filter(|f| f.likely_online)
    }
}

/// `usr_12ab-34cd` -> `usr12ab34cd`, the table prefix VRCX uses.
pub fn table_prefix(user_id: &str) -> String {
    user_id.chars().filter(|c| *c != '-' && *c != '_').collect()
}

fn parse_time(text: &str) -> Option<DateTime<Local>> {
    DateTime::parse_from_rfc3339(text.trim())
        .map(|t| t.with_timezone(&Local))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(text.trim(), "%Y-%m-%d %H:%M:%S")
                .map(|naive| naive.and_utc().with_timezone(&Local))
        })
        .ok()
}

fn table_names(conn: &Connection) -> rusqlite::Result<HashSet<String>> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table'")?;
    let names = stmt.query_map([], |row| row.get::<_, String>(0))?.collect::<Result<_, _>>()?;
    Ok(names)
}

/// Finds the logged-in account's table prefix: VRCX's own "last user" setting if present,
/// otherwise the account whose GPS feed has the newest entry.
fn detect_prefix(conn: &Connection, tables: &HashSet<String>) -> Option<String> {
    if tables.contains("configs") {
        let last_user: Option<String> = conn
            .query_row("SELECT value FROM configs WHERE key = 'config:lastuserloggedin'", [], |row| row.get(0))
            .optional()
            .ok()
            .flatten();
        if let Some(user) = last_user {
            let prefix = table_prefix(user.trim_matches('"'));
            if tables.contains(&format!("{prefix}_feed_gps")) {
                return Some(prefix);
            }
        }
    }
    let mut best: Option<(String, String)> = None;
    for table in tables.iter().filter(|t| t.ends_with("_feed_gps") && t.starts_with("usr")) {
        let prefix = table.trim_end_matches("_feed_gps").to_string();
        let newest: Option<String> = conn
            .query_row(&format!("SELECT MAX(created_at) FROM \"{table}\""), [], |row| row.get(0))
            .ok()
            .flatten();
        let newest = newest.unwrap_or_default();
        if best.as_ref().is_none_or(|(_, b)| newest > *b) {
            best = Some((prefix, newest));
        }
    }
    best.map(|(prefix, _)| prefix)
}

pub fn open_read_only(path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX)?;
    conn.busy_timeout(Duration::from_secs(2))?;
    Ok(conn)
}

/// Reads everything the VRCX panel shows in one pass.
pub fn read_snapshot(path: &Path) -> VrcxSnapshot {
    let mut snapshot = VrcxSnapshot { db_path: Some(path.to_path_buf()), ..Default::default() };
    if !path.exists() {
        snapshot.status = VrcxStatus::NotFound(path.to_path_buf());
        return snapshot;
    }
    let result = (|| -> rusqlite::Result<()> {
        let conn = open_read_only(path)?;
        let tables = table_names(&conn)?;
        let prefix = detect_prefix(&conn, &tables);
        if let Some(prefix) = &prefix {
            snapshot.feed = read_feed(&conn, &tables, prefix).unwrap_or_else(|err| {
                log::warn!("VRCX feed: {err}");
                Vec::new()
            });
            snapshot.friends = read_friends(&conn, &tables, prefix).unwrap_or_else(|err| {
                log::warn!("VRCX friends: {err}");
                Vec::new()
            });
        }
        if tables.contains("memos") {
            let mut stmt = conn.prepare("SELECT user_id, memo FROM memos WHERE memo IS NOT NULL AND memo != ''")?;
            snapshot.memos = stmt
                .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?
                .filter_map(Result::ok)
                .collect();
        }
        if tables.contains("gamelog_location") {
            snapshot.recent_worlds = read_recent_worlds(&conn).unwrap_or_else(|err| {
                log::warn!("VRCX worlds: {err}");
                Vec::new()
            });
        }
        snapshot.user_prefix = prefix;
        Ok(())
    })();
    snapshot.status = match result {
        Ok(()) => VrcxStatus::Ready,
        Err(err) => VrcxStatus::Error(err.to_string()),
    };
    snapshot.updated_at = Some(Local::now());
    snapshot
}

fn read_feed(conn: &Connection, tables: &HashSet<String>, prefix: &str) -> rusqlite::Result<Vec<FeedEntry>> {
    let mut parts = Vec::new();
    if tables.contains(&format!("{prefix}_feed_gps")) {
        parts.push(format!(
            "SELECT created_at, 'GPS', user_id, display_name, IFNULL(world_name, ''), IFNULL(location, '') FROM \"{prefix}_feed_gps\""
        ));
    }
    if tables.contains(&format!("{prefix}_feed_online_offline")) {
        parts.push(format!(
            "SELECT created_at, type, user_id, display_name, IFNULL(world_name, ''), IFNULL(location, '') FROM \"{prefix}_feed_online_offline\""
        ));
    }
    if tables.contains(&format!("{prefix}_feed_status")) {
        parts.push(format!(
            "SELECT created_at, 'Status', user_id, display_name, \
             IFNULL(status, '') || CASE WHEN IFNULL(status_description, '') != '' THEN ' - ' || status_description ELSE '' END, '' \
             FROM \"{prefix}_feed_status\""
        ));
    }
    if parts.is_empty() {
        return Ok(Vec::new());
    }
    let sql = format!("{} ORDER BY 1 DESC LIMIT {FEED_LIMIT}", parts.join(" UNION ALL "));
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| {
        let kind: String = row.get(1)?;
        Ok(FeedEntry {
            created_at: parse_time(&row.get::<_, String>(0)?),
            kind: match kind.as_str() {
                "GPS" => FeedKind::Location,
                "Online" => FeedKind::Online,
                "Offline" => FeedKind::Offline,
                _ => FeedKind::Status,
            },
            user_id: row.get(2)?,
            display_name: row.get(3)?,
            detail: row.get(4)?,
            location: row.get(5)?,
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

fn read_friends(conn: &Connection, tables: &HashSet<String>, prefix: &str) -> rusqlite::Result<Vec<FriendEntry>> {
    let current = format!("{prefix}_friend_log_current");
    if !tables.contains(&current) {
        return Ok(Vec::new());
    }
    let mut stmt = conn.prepare(&format!("SELECT user_id, display_name, trust_level FROM \"{current}\""))?;
    let mut friends: Vec<FriendEntry> = stmt
        .query_map([], |row| {
            Ok(FriendEntry {
                user_id: row.get(0)?,
                display_name: row.get(1)?,
                trust_level: row.get(2)?,
                likely_online: false,
                last_world: None,
                last_location: None,
                last_seen: None,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    // Latest presence event and latest location per friend.
    let mut presence: HashMap<String, (String, Option<DateTime<Local>>)> = HashMap::new();
    let online_offline = format!("{prefix}_feed_online_offline");
    if tables.contains(&online_offline) {
        let mut stmt = conn.prepare(&format!(
            "SELECT user_id, type, MAX(created_at) FROM \"{online_offline}\" GROUP BY user_id"
        ))?;
        for row in stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)))? {
            let (user, kind, at) = row?;
            presence.insert(user, (kind, parse_time(&at)));
        }
    }
    let mut locations: HashMap<String, (String, String, Option<DateTime<Local>>)> = HashMap::new();
    let gps = format!("{prefix}_feed_gps");
    if tables.contains(&gps) {
        let mut stmt = conn.prepare(&format!(
            "SELECT user_id, IFNULL(world_name, ''), IFNULL(location, ''), MAX(created_at) FROM \"{gps}\" GROUP BY user_id"
        ))?;
        for row in stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?))
        })? {
            let (user, world, location, at) = row?;
            locations.insert(user, (world, location, parse_time(&at)));
        }
    }

    let now = Local::now();
    for friend in &mut friends {
        let presence = presence.get(&friend.user_id);
        let location = locations.get(&friend.user_id);
        let last_presence_at = presence.and_then(|(_, at)| *at);
        let last_gps_at = location.and_then(|(_, _, at)| *at);
        friend.last_seen = last_presence_at.max(last_gps_at);
        let online_by_presence = presence.is_some_and(|(kind, _)| kind == "Online");
        let moved_after_presence = match (last_gps_at, last_presence_at) {
            (Some(gps), Some(presence)) => gps > presence,
            (Some(_), None) => true,
            _ => false,
        };
        let fresh = friend.last_seen.is_some_and(|seen| now - seen < ONLINE_STALE_AFTER);
        friend.likely_online = fresh && (online_by_presence || moved_after_presence);
        if let Some((world, location, _)) = location {
            friend.last_world = (!world.is_empty()).then(|| world.clone());
            friend.last_location = (!location.is_empty()).then(|| location.clone());
        }
    }
    friends.sort_by(|a, b| {
        b.likely_online
            .cmp(&a.likely_online)
            .then_with(|| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()))
    });
    Ok(friends)
}

fn read_recent_worlds(conn: &Connection) -> rusqlite::Result<Vec<RecentWorld>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT created_at, IFNULL(world_id, ''), IFNULL(world_name, ''), IFNULL(location, ''), IFNULL(time, 0) \
         FROM gamelog_location ORDER BY id DESC LIMIT {RECENT_WORLD_LIMIT}"
    ))?;
    let rows = stmt.query_map([], |row| {
        let millis: i64 = row.get(4)?;
        Ok(RecentWorld {
            visited_at: parse_time(&row.get::<_, String>(0)?),
            world_id: row.get(1)?,
            world_name: row.get(2)?,
            location: row.get(3)?,
            time_spent: (millis > 0).then(|| Duration::from_millis(millis as u64)),
        })
    })?;
    Ok(rows.filter_map(Result::ok).collect())
}

/// Polls the database and sends a snapshot every few seconds.
pub fn spawn(path: PathBuf, events: Sender<ServiceEvent>, shutdown: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("vrcx-db".into())
        .spawn(move || {
            let mut last: Option<VrcxSnapshot> = None;
            while !shutdown.load(Ordering::Relaxed) {
                let mut snapshot = read_snapshot(&path);
                let unchanged = last.as_ref().is_some_and(|prev| {
                    prev.status == snapshot.status
                        && prev.feed == snapshot.feed
                        && prev.friends == snapshot.friends
                        && prev.recent_worlds == snapshot.recent_worlds
                        && prev.memos == snapshot.memos
                });
                if !unchanged {
                    if let VrcxStatus::Error(err) = &snapshot.status {
                        log::warn!("VRCX database: {err}");
                    }
                    let _ = events.send(ServiceEvent::Vrcx(snapshot.clone()));
                }
                snapshot.updated_at = None;
                last = Some(snapshot);
                for _ in 0..20 {
                    if shutdown.load(Ordering::Relaxed) {
                        return;
                    }
                    std::thread::sleep(Duration::from_millis(500));
                }
            }
        })
        .expect("spawning vrcx-db thread");
}

/// Writes a small database with VRCX's schema and believable rows, for the preview and tests.
pub fn write_demo_database(path: &Path) -> rusqlite::Result<()> {
    let _ = std::fs::remove_file(path);
    let conn = Connection::open(path)?;
    let prefix = table_prefix("usr_0b1c2d3e-0000-4000-8000-000000000001");
    conn.execute_batch(&format!(
        "CREATE TABLE configs (key TEXT PRIMARY KEY, value TEXT);
         CREATE TABLE {prefix}_feed_gps (id INTEGER PRIMARY KEY, created_at TEXT, user_id TEXT, display_name TEXT, location TEXT, world_name TEXT, previous_location TEXT, time INTEGER, group_name TEXT);
         CREATE TABLE {prefix}_feed_status (id INTEGER PRIMARY KEY, created_at TEXT, user_id TEXT, display_name TEXT, status TEXT, status_description TEXT, previous_status TEXT, previous_status_description TEXT);
         CREATE TABLE {prefix}_feed_online_offline (id INTEGER PRIMARY KEY, created_at TEXT, user_id TEXT, display_name TEXT, type TEXT, location TEXT, world_name TEXT, time INTEGER, group_name TEXT);
         CREATE TABLE {prefix}_friend_log_current (user_id TEXT PRIMARY KEY, display_name TEXT, trust_level TEXT);
         CREATE TABLE memos (user_id TEXT PRIMARY KEY, edited_at TEXT, memo TEXT);
         CREATE TABLE gamelog_location (id INTEGER PRIMARY KEY, created_at TEXT, location TEXT, world_id TEXT, world_name TEXT, time INTEGER, group_name TEXT);
         INSERT INTO configs VALUES ('config:lastuserloggedin', 'usr_0b1c2d3e-0000-4000-8000-000000000001');"
    ))?;

    let now = chrono::Utc::now();
    let at = |minutes_ago: i64| (now - chrono::Duration::minutes(minutes_ago)).to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let friends = [
        ("usr_aa000000-0000-4000-8000-000000000002", "Kazuha", "Trusted User"),
        ("usr_aa000000-0000-4000-8000-000000000003", "Moonbeam", "Known User"),
        ("usr_aa000000-0000-4000-8000-000000000004", "Pixel Fox", "Trusted User"),
        ("usr_aa000000-0000-4000-8000-000000000005", "Ren", "User"),
        ("usr_aa000000-0000-4000-8000-000000000006", "Astra", "Known User"),
        ("usr_aa000000-0000-4000-8000-000000000007", "Juniper", "Trusted User"),
    ];
    for (id, name, trust) in friends {
        conn.execute(&format!("INSERT INTO {prefix}_friend_log_current VALUES (?1, ?2, ?3)"), (id, name, trust))?;
    }
    let presence = [
        (240, friends[0].0, "Kazuha", "Online"),
        (190, friends[4].0, "Astra", "Online"),
        (95, friends[1].0, "Moonbeam", "Online"),
        (61, friends[4].0, "Astra", "Offline"),
        (12, friends[5].0, "Juniper", "Online"),
    ];
    for (ago, id, name, kind) in presence {
        conn.execute(
            &format!("INSERT INTO {prefix}_feed_online_offline (created_at, user_id, display_name, type, location, world_name, time) VALUES (?1, ?2, ?3, ?4, '', '', 0)"),
            (at(ago), id, name, kind),
        )?;
    }
    let gps = [
        (70, friends[0].0, "Kazuha", "wrld_4432ea9b-729c-46e3-8eaf-846aa0a37fdd:48213~hidden(usr_x)~region(use)", "The Black Cat"),
        (52, friends[1].0, "Moonbeam", "wrld_4432ea9b-729c-46e3-8eaf-846aa0a37fdd:48213~hidden(usr_x)~region(use)", "The Black Cat"),
        (35, friends[2].0, "Pixel Fox", "wrld_4432ea9b-729c-46e3-8eaf-846aa0a37fdd:48213~hidden(usr_x)~region(use)", "The Black Cat"),
        (9, friends[1].0, "Moonbeam", "wrld_b2d24c29-1ded-4990-a90d-dd6dcc440300:771~friends(usr_y)~region(eu)", "Midnight Rooftop"),
        (4, friends[5].0, "Juniper", "private", ""),
    ];
    for (ago, id, name, location, world) in gps {
        conn.execute(
            &format!("INSERT INTO {prefix}_feed_gps (created_at, user_id, display_name, location, world_name, previous_location, time) VALUES (?1, ?2, ?3, ?4, ?5, '', 0)"),
            (at(ago), id, name, location, world),
        )?;
    }
    conn.execute(
        &format!("INSERT INTO {prefix}_feed_status (created_at, user_id, display_name, status, status_description, previous_status, previous_status_description) VALUES (?1, ?2, ?3, 'join me', 'come say hi', 'active', '')"),
        (at(20), friends[2].0, "Pixel Fox"),
    )?;
    conn.execute("INSERT INTO memos VALUES (?1, ?2, 'Met at the Rust meetup world')", (friends[0].0, at(5000)))?;
    let worlds = [
        (300, "wrld_11111111-2222-4333-8444-555555555555", "Movie & Chill", 5_400_000),
        (110, "wrld_66666666-7777-4888-9999-000000000000", "Just B Club", 3_600_000),
        (82, "wrld_4432ea9b-729c-46e3-8eaf-846aa0a37fdd", "The Black Cat", 0),
    ];
    for (ago, world_id, world_name, time) in worlds {
        conn.execute(
            "INSERT INTO gamelog_location (created_at, location, world_id, world_name, time, group_name) VALUES (?1, ?2, ?3, ?4, ?5, '')",
            (at(ago), format!("{world_id}:1"), world_id, world_name, time),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_matches_vrcx() {
        assert_eq!(table_prefix("usr_0b1c2d3e-0000-4000"), "usr0b1c2d3e00004000");
    }

    #[test]
    fn reads_feed_friends_memos_and_worlds() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("VRCX.sqlite3");
        write_demo_database(&path).unwrap();
        let snapshot = read_snapshot(&path);
        assert_eq!(snapshot.status, VrcxStatus::Ready);
        assert_eq!(snapshot.user_prefix.as_deref(), Some("usr0b1c2d3e00004000800000000000001"));

        // Newest first, all three feed tables merged.
        assert_eq!(snapshot.feed.len(), 11);
        assert_eq!(snapshot.feed[0].display_name, "Juniper");
        assert_eq!(snapshot.feed[0].kind, FeedKind::Location);
        assert!(snapshot.feed.windows(2).all(|w| w[0].created_at >= w[1].created_at));
        assert!(snapshot.feed.iter().any(|e| e.kind == FeedKind::Status && e.detail == "join me - come say hi"));

        let online: Vec<_> = snapshot.online_friends().map(|f| f.display_name.as_str()).collect();
        // Astra went offline; Ren never showed up; Pixel Fox moved recently without a presence event.
        assert_eq!(online, ["Juniper", "Kazuha", "Moonbeam", "Pixel Fox"]);
        let moonbeam = snapshot.friends.iter().find(|f| f.display_name == "Moonbeam").unwrap();
        assert_eq!(moonbeam.last_world.as_deref(), Some("Midnight Rooftop"));
        assert!(snapshot.is_friend("usr_aa000000-0000-4000-8000-000000000005"));

        assert_eq!(snapshot.memos.len(), 1);
        assert_eq!(snapshot.recent_worlds.len(), 3);
        assert_eq!(snapshot.recent_worlds[0].world_name, "The Black Cat");
        assert_eq!(snapshot.recent_worlds[1].time_spent, Some(Duration::from_secs(3600)));
    }

    #[test]
    fn falls_back_to_newest_account_tables_without_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("VRCX.sqlite3");
        write_demo_database(&path).unwrap();
        {
            let conn = Connection::open(&path).unwrap();
            conn.execute("DELETE FROM configs", []).unwrap();
            conn.execute_batch(
                "CREATE TABLE usrstale_feed_gps (id INTEGER PRIMARY KEY, created_at TEXT, user_id TEXT, display_name TEXT, location TEXT, world_name TEXT, previous_location TEXT, time INTEGER);
                 INSERT INTO usrstale_feed_gps (created_at, user_id, display_name, location, world_name, previous_location, time) VALUES ('2020-01-01T00:00:00.000Z', 'u', 'Old', '', '', '', 0);",
            )
            .unwrap();
        }
        let snapshot = read_snapshot(&path);
        assert_eq!(snapshot.user_prefix.as_deref(), Some("usr0b1c2d3e00004000800000000000001"));
    }

    #[test]
    fn missing_database_is_reported() {
        let snapshot = read_snapshot(Path::new("/definitely/not/here/VRCX.sqlite3"));
        assert!(matches!(snapshot.status, VrcxStatus::NotFound(_)));
    }
}
