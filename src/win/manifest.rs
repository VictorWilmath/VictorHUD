//! Files SteamVR reads about VictorHUD: the app manifest (so it can start VictorHUD with SteamVR)
//! and the controller action manifest with default bindings.

use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::vr::Apps;

pub const APP_KEY: &str = "victorwilmath.victorhud";

const FILES: [(&str, &str); 5] = [
    ("actions.json", include_str!("../../assets/steamvr/actions.json")),
    ("bindings_knuckles.json", include_str!("../../assets/steamvr/bindings_knuckles.json")),
    ("bindings_oculus_touch.json", include_str!("../../assets/steamvr/bindings_oculus_touch.json")),
    ("bindings_vive_controller.json", include_str!("../../assets/steamvr/bindings_vive_controller.json")),
    ("bindings_holographic_controller.json", include_str!("../../assets/steamvr/bindings_holographic_controller.json")),
];

pub struct Paths {
    pub app_manifest: PathBuf,
    pub actions: PathBuf,
}

fn steamvr_dir() -> PathBuf {
    let config = crate::config::data_dir();
    config.parent().map(Path::to_path_buf).unwrap_or(config).join("steamvr")
}

fn write_if_changed(path: &Path, contents: &str) -> anyhow::Result<()> {
    if std::fs::read_to_string(path).is_ok_and(|existing| existing == contents) {
        return Ok(());
    }
    std::fs::write(path, contents).with_context(|| format!("writing {}", path.display()))
}

/// Writes the manifests next to the settings and returns where they are.
pub fn write() -> anyhow::Result<Paths> {
    let dir = steamvr_dir();
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    for (name, contents) in FILES {
        write_if_changed(&dir.join(name), contents)?;
    }
    let actions = dir.join("actions.json");
    let exe = std::env::current_exe().context("finding VictorHUD.exe")?;
    let app_manifest = dir.join("victorhud.vrmanifest");
    write_if_changed(&app_manifest, &app_manifest_json(&exe, &actions))?;
    Ok(Paths { app_manifest, actions })
}

fn app_manifest_json(exe: &Path, actions: &Path) -> String {
    let manifest = serde_json::json!({
        "source": "builtin",
        "applications": [{
            "app_key": APP_KEY,
            "launch_type": "binary",
            "binary_path_windows": exe.to_string_lossy(),
            "is_dashboard_overlay": true,
            "action_manifest_path": actions.to_string_lossy(),
            "strings": {
                "en_us": {
                    "name": "VictorHUD",
                    "description": "Wrist watch, media controls, desktops and VRCX in VR."
                }
            }
        }]
    });
    serde_json::to_string_pretty(&manifest).expect("static JSON")
}

/// Registers VictorHUD with SteamVR and turns starting with SteamVR on or off.
pub fn register(apps: &Apps, paths: &Paths, auto_launch: bool) -> anyhow::Result<()> {
    apps.add_app_manifest(&paths.app_manifest)?;
    if apps.auto_launch(APP_KEY) != auto_launch {
        apps.set_auto_launch(APP_KEY, auto_launch)?;
    }
    Ok(())
}

pub fn unregister(apps: &Apps, paths: &Paths) -> anyhow::Result<()> {
    if apps.is_app_installed(APP_KEY) {
        let _ = apps.set_auto_launch(APP_KEY, false);
        apps.remove_app_manifest(&paths.app_manifest)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_files_are_valid_json() {
        for (name, contents) in FILES {
            let value: serde_json::Value = serde_json::from_str(contents).unwrap_or_else(|e| panic!("{name}: {e}"));
            if name != "actions.json" {
                assert_eq!(value["app_key"], APP_KEY, "{name}");
            }
        }
    }

    #[test]
    fn bindings_only_use_declared_actions() {
        let actions: serde_json::Value = serde_json::from_str(FILES[0].1).unwrap();
        let declared: Vec<&str> =
            actions["actions"].as_array().unwrap().iter().map(|a| a["name"].as_str().unwrap()).collect();
        let bound: Vec<&str> = actions["default_bindings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| b["binding_url"].as_str().unwrap())
            .collect();
        for (name, contents) in &FILES[1..] {
            assert!(bound.contains(name), "{name} isn't listed in actions.json");
            for output in contents.split("\"output\": \"").skip(1) {
                let action = output.split('"').next().unwrap();
                assert!(declared.contains(&action), "{name} binds unknown action {action}");
            }
        }
    }

    #[test]
    fn app_manifest_points_at_the_exe() {
        let json = app_manifest_json(Path::new(r"C:\Tools\VictorHUD.exe"), Path::new(r"C:\cfg\actions.json"));
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let app = &value["applications"][0];
        assert_eq!(app["app_key"], APP_KEY);
        assert_eq!(app["binary_path_windows"], r"C:\Tools\VictorHUD.exe");
        assert_eq!(app["is_dashboard_overlay"], true);
    }
}
