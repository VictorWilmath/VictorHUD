//! What's running in VR and for how long, plus device batteries.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use windows::Win32::Foundation::{CloseHandle, FILETIME};
use windows::Win32::System::ProcessStatus::EnumProcesses;
use windows::Win32::System::Threading::{GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

use crate::state::{DeviceBattery, DeviceKind, SessionInfo};
use crate::vr::{self, DeviceIndex, Vr, sys};

const SESSION_INTERVAL: Duration = Duration::from_secs(2);
const BATTERY_INTERVAL: Duration = Duration::from_secs(15);

pub struct SessionTracker {
    scene_pid: u32,
    info: SessionInfo,
    next_session: Instant,
    next_battery: Instant,
}

impl SessionTracker {
    pub fn new() -> Self {
        let info = SessionInfo { steamvr_started: steamvr_start_time(), ..Default::default() };
        Self { scene_pid: 0, info, next_session: Instant::now(), next_battery: Instant::now() }
    }

    /// Re-checks the scene app soon (after SteamVR says it changed).
    pub fn refresh_soon(&mut self) {
        self.next_session = Instant::now();
    }

    /// Returns new session info and batteries when they changed.
    pub fn update(&mut self, vr: &Vr) -> (Option<SessionInfo>, Option<Vec<DeviceBattery>>) {
        let now = Instant::now();
        let mut session = None;
        if now >= self.next_session {
            self.next_session = now + SESSION_INTERVAL;
            let pid = vr.apps.scene_process_id();
            if pid != self.scene_pid {
                self.scene_pid = pid;
                let mut info = self.info.clone();
                if pid == 0 {
                    info.app_name = None;
                    info.app_started = None;
                } else {
                    let key = vr.apps.app_key_for_process(pid);
                    info.app_name = key
                        .as_deref()
                        .and_then(|key| vr.apps.app_name(key))
                        .or_else(|| super::desktop::process_path(pid).map(|path| crate::util::pretty_app_name(&path)));
                    info.app_started = process_start_time(pid);
                    log::info!("now playing {:?} (pid {pid}, key {key:?})", info.app_name);
                }
                if info.steamvr_started.is_none() {
                    info.steamvr_started = steamvr_start_time();
                }
                if info != self.info {
                    self.info = info.clone();
                    session = Some(info);
                }
            }
        }
        let batteries = (now >= self.next_battery).then(|| {
            self.next_battery = now + BATTERY_INTERVAL;
            batteries(vr)
        });
        (session, batteries)
    }
}

fn batteries(vr: &Vr) -> Vec<DeviceBattery> {
    let mut list = Vec::new();
    let mut trackers = 0;
    for device in 0..vr::MAX_DEVICES as DeviceIndex {
        if !vr.is_connected(device) {
            continue;
        }
        let class = vr.device_class(device);
        let kind = match class {
            sys::ETrackedDeviceClass_TrackedDeviceClass_HMD => DeviceKind::Headset,
            sys::ETrackedDeviceClass_TrackedDeviceClass_Controller => match vr.controller_role(device) {
                sys::ETrackedControllerRole_TrackedControllerRole_LeftHand => DeviceKind::LeftHand,
                sys::ETrackedControllerRole_TrackedControllerRole_RightHand => DeviceKind::RightHand,
                _ => continue,
            },
            sys::ETrackedDeviceClass_TrackedDeviceClass_GenericTracker => DeviceKind::Tracker,
            _ => continue,
        };
        let Some((level, charging)) = vr.battery(device) else { continue };
        let label = match kind {
            DeviceKind::Headset => "Headset".to_string(),
            DeviceKind::LeftHand => "Left controller".to_string(),
            DeviceKind::RightHand => "Right controller".to_string(),
            DeviceKind::Tracker => {
                trackers += 1;
                format!("Tracker {trackers}")
            }
        };
        list.push(DeviceBattery { kind, label, percent: level.clamp(0.0, 1.0), charging });
    }
    list
}

fn filetime_to_system_time(time: FILETIME) -> Option<SystemTime> {
    let ticks = ((time.dwHighDateTime as u64) << 32) | time.dwLowDateTime as u64;
    let unix_ticks = ticks.checked_sub(11_644_473_600 * 10_000_000)?;
    Some(UNIX_EPOCH + Duration::from_nanos(unix_ticks * 100))
}

pub fn process_start_time(pid: u32) -> Option<SystemTime> {
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?;
    let (mut created, mut exited, mut kernel, mut user) = Default::default();
    let result = unsafe { GetProcessTimes(handle, &mut created, &mut exited, &mut kernel, &mut user) };
    unsafe {
        let _ = CloseHandle(handle);
    }
    result.ok()?;
    filetime_to_system_time(created)
}

/// When `vrserver.exe` started, i.e. how long SteamVR has been running.
fn steamvr_start_time() -> Option<SystemTime> {
    let mut pids = vec![0u32; 4096];
    let mut bytes = 0u32;
    unsafe { EnumProcesses(pids.as_mut_ptr(), (pids.len() * 4) as u32, &mut bytes) }.ok()?;
    pids.truncate(bytes as usize / 4);
    pids.into_iter()
        .filter(|&pid| pid != 0)
        .find(|&pid| {
            super::desktop::process_path(pid).is_some_and(|path| {
                path.rsplit('\\').next().is_some_and(|exe| exe.eq_ignore_ascii_case("vrserver.exe"))
            })
        })
        .and_then(process_start_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_filetimes() {
        // 2021-01-01T00:00:00Z
        let ticks: u64 = 132_539_328_000_000_000;
        let time = FILETIME { dwLowDateTime: ticks as u32, dwHighDateTime: (ticks >> 32) as u32 };
        let unix = filetime_to_system_time(time).unwrap().duration_since(UNIX_EPOCH).unwrap();
        assert_eq!(unix.as_secs(), 1_609_459_200);
    }

    #[test]
    fn this_process_has_a_start_time() {
        let started = process_start_time(std::process::id()).unwrap();
        assert!(started <= SystemTime::now());
    }
}
