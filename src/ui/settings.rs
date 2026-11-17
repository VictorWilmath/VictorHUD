//! Settings panel. Edits `Config` in place; the host persists changes shortly after.

use egui::{Color32, RichText, ScrollArea, Sense, Slider, Ui, vec2};

use super::UiCtx;
use super::icons::{self, Icon};
use super::theme::{self, Palette};
use super::widgets;
use crate::config::{Config, ToastPlacement};
use crate::state::{Action, Hand};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Section {
    #[default]
    General,
    Watch,
    Interaction,
    Desktop,
    Media,
    Notifications,
    Vrcx,
    About,
}

const SECTIONS: [(Section, Icon, &str); 8] = [
    (Section::General, Icon::Gear, "General"),
    (Section::Watch, Icon::Watch, "Watch"),
    (Section::Interaction, Icon::Pointer, "Controllers"),
    (Section::Desktop, Icon::Monitor, "Desktops"),
    (Section::Media, Icon::Music, "Media"),
    (Section::Notifications, Icon::Bell, "Notifications"),
    (Section::Vrcx, Icon::Users, "VRCX"),
    (Section::About, Icon::Globe, "About"),
];

const ACCENTS: [[u8; 3]; 6] =
    [[120, 140, 255], [168, 120, 255], [255, 110, 170], [255, 160, 80], [70, 210, 160], [80, 190, 255]];

pub fn draw(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let section_id = egui::Id::unique("settings-section");
    let mut section: Section = ui.data(|d| d.get_temp(section_id)).unwrap_or_default();
    let before = ctx.config.clone();

    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(176.0);
            ui.spacing_mut().item_spacing.y = 4.0;
            for (value, icon, label) in SECTIONS {
                if nav_item(ui, &palette, icon, label, section == value).clicked() {
                    section = value;
                }
            }
        });
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ScrollArea::vertical().id_salt(("settings", section as u8)).auto_shrink([false, false]).show(ui, |ui| {
                ui.set_width(ui.available_width() - 8.0);
                match section {
                    Section::General => general(ui, ctx),
                    Section::Watch => watch(ui, ctx),
                    Section::Interaction => interaction(ui, ctx),
                    Section::Desktop => desktop(ui, ctx),
                    Section::Media => media(ui, ctx),
                    Section::Notifications => notifications(ui, ctx),
                    Section::Vrcx => vrcx(ui, ctx),
                    Section::About => about(ui, ctx),
                }
            });
        });
    });
    ui.data_mut(|d| d.insert_temp(section_id, section));
    if *ctx.config != before {
        ctx.config_changed = true;
    }
}

fn nav_item(ui: &mut Ui, palette: &Palette, icon: Icon, label: &str, selected: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::click());
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, 12.0, palette.accent_soft);
    } else if response.hovered() {
        painter.rect_filled(rect, 12.0, palette.card_hover);
    }
    let color = if selected { palette.accent } else { palette.text_dim };
    let icon_rect = egui::Rect::from_center_size(egui::pos2(rect.min.x + 22.0, rect.center().y), vec2(18.0, 18.0));
    icons::paint(painter, icon_rect, icon, color);
    painter.text(
        egui::pos2(rect.min.x + 42.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(15.0),
        if selected { palette.text } else { palette.text_dim },
    );
    response
}

fn group(ui: &mut Ui, palette: &Palette, title: &str, add: impl FnOnce(&mut Ui)) {
    widgets::section_label(ui, palette, title);
    theme::card_frame(palette).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.spacing_mut().item_spacing.y = 10.0;
        add(ui);
    });
    ui.add_space(6.0);
}

/// Fixed-width, left-aligned label column shared by slider and colour rows.
fn row_label(ui: &mut Ui, palette: &Palette, label: &str) {
    ui.allocate_ui_with_layout(vec2(170.0, 22.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_width(170.0);
        ui.add(egui::Label::new(RichText::new(label).color(palette.text)).truncate());
    });
}

fn slider_row(ui: &mut Ui, palette: &Palette, label: &str, slider: Slider<'_>) {
    ui.horizontal(|ui| {
        row_label(ui, palette, label);
        // Leave room for the value box egui draws after the slider.
        ui.spacing_mut().slider_width = (ui.available_width() - 96.0).max(100.0);
        ui.add(slider);
    });
}

fn hint(ui: &mut Ui, palette: &Palette, text: &str) {
    ui.add(egui::Label::new(RichText::new(text).size(12.5).color(palette.text_faint)).wrap());
}

fn general(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let mut start_with_steamvr = ctx.config.general.start_with_steamvr;
    group(ui, &palette, "Clock", |ui| {
        let general = &mut ctx.config.general;
        widgets::toggle_row(ui, &palette, "24-hour clock", "", &mut general.clock_24h);
        widgets::toggle_row(ui, &palette, "Show seconds", "", &mut general.show_seconds);
        widgets::toggle_row(ui, &palette, "Show date", "", &mut general.show_date);
    });
    group(ui, &palette, "Appearance", |ui| {
        ui.horizontal(|ui| {
            row_label(ui, &palette, "Accent colour");
            for accent in ACCENTS {
                let (rect, response) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::click());
                let color = Color32::from_rgb(accent[0], accent[1], accent[2]);
                ui.painter().circle_filled(rect.center(), 12.0, color);
                if ctx.config.general.accent == accent {
                    ui.painter().circle_stroke(rect.center(), 14.0, egui::Stroke::new(2.0, Color32::WHITE));
                }
                if response.clicked() {
                    ctx.config.general.accent = accent;
                }
            }
        });
        slider_row(
            ui,
            &palette,
            "Text sharpness",
            Slider::new(&mut ctx.config.general.ui_scale, 1.0..=3.0).step_by(0.25).suffix("×"),
        );
        hint(ui, &palette, "Higher values render panels at a higher resolution. Costs a little GPU time.");
        widgets::toggle_row(
            ui,
            &palette,
            "Performance mode",
            "Redraw panels less often while nothing is moving",
            &mut ctx.config.general.performance_mode,
        );
    });
    group(ui, &palette, "Startup", |ui| {
        widgets::toggle_row(
            ui,
            &palette,
            "Start with SteamVR",
            "Launch VictorHUD automatically",
            &mut start_with_steamvr,
        );
        if ui.button(RichText::new("Quit VictorHUD").color(palette.bad)).clicked() {
            ctx.act(Action::Quit);
        }
    });
    ctx.config.general.start_with_steamvr = start_with_steamvr;
}

fn watch(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    group(ui, &palette, "Placement", |ui| {
        ui.horizontal(|ui| {
            row_label(ui, &palette, "Watch hand");
            for hand in [Hand::Left, Hand::Right] {
                if widgets::chip(ui, &palette, hand.label(), ctx.config.watch.hand == hand).clicked() {
                    ctx.config.watch.hand = hand;
                }
            }
        });
        let mut keep = ctx.config.watch.keep_on_wrist;
        if widgets::toggle_row(ui, &palette, "Stay on wrist", "Show the watch even when your arm is down", &mut keep) {
            ctx.act(Action::SetKeepOnWrist(keep));
        }
        let mut width_cm = ctx.config.watch.width_m * 100.0;
        slider_row(ui, &palette, "Size", Slider::new(&mut width_cm, 8.0..=22.0).suffix(" cm").fixed_decimals(1));
        ctx.config.watch.width_m = width_cm / 100.0;
        slider_row(ui, &palette, "Opacity", Slider::new(&mut ctx.config.watch.opacity, 0.3..=1.0).fixed_decimals(2));
        ui.horizontal(|ui| {
            if ui.button("Reset position").clicked() {
                ctx.act(Action::ResetWatchPosition);
            }
            hint(ui, &palette, "Grab the watch with your other hand to move it.");
        });
    });
    group(ui, &palette, "Raise to look", |ui| {
        let watch = &mut ctx.config.watch;
        slider_row(
            ui,
            &palette,
            "Wrist tilt tolerance",
            Slider::new(&mut watch.facing_angle, 15.0..=75.0).suffix("°").fixed_decimals(0),
        );
        slider_row(
            ui,
            &palette,
            "Gaze tolerance",
            Slider::new(&mut watch.view_angle, 15.0..=70.0).suffix("°").fixed_decimals(0),
        );
        slider_row(
            ui,
            &palette,
            "Max distance",
            Slider::new(&mut watch.max_distance, 0.4..=1.0).suffix(" m").fixed_decimals(2),
        );
        slider_row(
            ui,
            &palette,
            "Show after",
            Slider::new(&mut watch.show_delay, 0.0..=0.8).suffix(" s").fixed_decimals(2),
        );
        slider_row(
            ui,
            &palette,
            "Hide after",
            Slider::new(&mut watch.hide_delay, 0.0..=2.0).suffix(" s").fixed_decimals(2),
        );
        hint(
            ui,
            &palette,
            "Raise your wrist like you're checking the time. Loosen the tolerances if the watch is hard to bring up.",
        );
    });
}

fn interaction(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    group(ui, &palette, "Laser", |ui| {
        let interaction = &mut ctx.config.interaction;
        slider_row(
            ui,
            &palette,
            "Max length",
            Slider::new(&mut interaction.laser_max_length, 1.0..=10.0).suffix(" m").fixed_decimals(1),
        );
        widgets::toggle_row(ui, &palette, "Haptics", "Tick on hover and click", &mut interaction.haptics);
        slider_row(
            ui,
            &palette,
            "Haptic strength",
            Slider::new(&mut interaction.haptic_strength, 0.0..=1.0).fixed_decimals(2),
        );
    });
    group(ui, &palette, "Scrolling", |ui| {
        let interaction = &mut ctx.config.interaction;
        slider_row(
            ui,
            &palette,
            "Scroll speed",
            Slider::new(&mut interaction.scroll_speed, 2.0..=40.0).fixed_decimals(0),
        );
        widgets::toggle_row(ui, &palette, "Invert scrolling", "", &mut interaction.invert_scroll);
        widgets::toggle_row(
            ui,
            &palette,
            "Move the mouse on hover",
            "Pointing at a desktop moves the Windows cursor, so hover effects work",
            &mut interaction.hover_moves_cursor,
        );
    });
    group(ui, &palette, "Controls", |ui| {
        hint(
            ui,
            &palette,
            "Trigger: click   ·   Grip: move a panel (grab with both hands to resize)   ·   Thumbstick: scroll   ·   \
             B / Y or menu: toggle the watch. Rebind anything in SteamVR → Controller bindings → VictorHUD.",
        );
    });
}

fn desktop(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let monitors = &ctx.state.monitors;
    group(ui, &palette, "Capture", |ui| {
        let desktop = &mut ctx.config.desktop;
        slider_row(ui, &palette, "Frame rate limit", Slider::new(&mut desktop.max_fps, 10..=120).suffix(" fps"));
        widgets::toggle_row(ui, &palette, "Show mouse cursor", "", &mut desktop.show_cursor);
        slider_row(
            ui,
            &palette,
            "New panel width",
            Slider::new(&mut desktop.default_width_m, 0.4..=3.0).suffix(" m").fixed_decimals(2),
        );
        if !monitors.is_empty() {
            ui.horizontal_wrapped(|ui| {
                row_label(ui, &palette, "Open first");
                for monitor in monitors {
                    if widgets::chip(ui, &palette, &monitor.label, desktop.default_monitor == monitor.index).clicked() {
                        desktop.default_monitor = monitor.index;
                    }
                }
            });
        }
    });
    hint(ui, &palette, "Open several desktop panels from the watch to see more than one display at once.");
}

fn media(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    group(ui, &palette, "Media", |ui| {
        let media = &mut ctx.config.media;
        widgets::toggle_row(ui, &palette, "Show album art", "", &mut media.show_artwork);
        slider_row(ui, &palette, "Volume step", Slider::new(&mut media.volume_step, 0.01..=0.2).fixed_decimals(2));
    });
    hint(
        ui,
        &palette,
        "Media controls work with any app that shows up in the Windows media flyout: Spotify, browsers, YouTube Music, VLC, and more.",
    );
}

fn notifications(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let mut dnd = ctx.config.notifications.do_not_disturb;
    group(ui, &palette, "Popups", |ui| {
        let n = &mut ctx.config.notifications;
        widgets::toggle_row(ui, &palette, "Do not disturb", "Keep notifications in the inbox without popups", &mut dnd);
        ui.horizontal(|ui| {
            row_label(ui, &palette, "Show popups");
            for (placement, label) in [(ToastPlacement::Head, "In view"), (ToastPlacement::Wrist, "Above the watch")] {
                if widgets::chip(ui, &palette, label, n.placement == placement).clicked() {
                    n.placement = placement;
                }
            }
        });
        slider_row(
            ui,
            &palette,
            "Default duration",
            Slider::new(&mut n.default_duration, 1.0..=15.0).suffix(" s").fixed_decimals(1),
        );
        slider_row(ui, &palette, "Keep history", Slider::new(&mut n.history_limit, 10..=500));
    });
    if dnd != ctx.config.notifications.do_not_disturb {
        ctx.act(Action::SetDoNotDisturb(dnd));
    }
    group(ui, &palette, "XSOverlay-compatible receiver", |ui| {
        let n = &mut ctx.config.notifications;
        widgets::toggle_row(
            ui,
            &palette,
            "Accept notifications",
            "From VRCX, VRCVideoCacher, OSC tools and others",
            &mut n.enabled,
        );
        ui.horizontal(|ui| {
            row_label(ui, &palette, "UDP port");
            ui.add(egui::DragValue::new(&mut n.udp_port).range(1024..=65535));
            ui.add_space(12.0);
            ui.label("WebSocket port");
            ui.add(egui::DragValue::new(&mut n.websocket_port).range(1024..=65535));
        });
        hint(
            ui,
            &palette,
            "Port and on/off changes apply the next time VictorHUD starts. Close XSOverlay first if it uses the same ports.",
        );
    });
}

fn vrcx(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    group(ui, &palette, "VRCX", |ui| {
        let status = match &state.vrcx.status {
            crate::services::vrcx::VrcxStatus::Ready => ("Connected (read-only)", palette.good),
            crate::services::vrcx::VrcxStatus::Starting => ("Starting…", palette.text_dim),
            crate::services::vrcx::VrcxStatus::NotFound(_) => ("Database not found", palette.warn),
            crate::services::vrcx::VrcxStatus::Error(_) => ("Error reading database", palette.bad),
        };
        ui.horizontal(|ui| {
            row_label(ui, &palette, "Status");
            widgets::pill(ui, status.0, status.1);
        });
        path_field(
            ui,
            &palette,
            "Database file",
            &mut ctx.config.vrcx.database_path,
            crate::config::default_vrcx_db_path(),
        );
        path_field(
            ui,
            &palette,
            "VRChat log folder",
            &mut ctx.config.vrcx.vrchat_log_dir,
            crate::config::default_vrchat_log_dir(),
        );
        hint(
            ui,
            &palette,
            "VictorHUD only reads VRCX's database and never writes to it. Path changes apply after a restart.",
        );
    });
}

fn path_field(
    ui: &mut Ui,
    palette: &Palette,
    label: &str,
    value: &mut Option<std::path::PathBuf>,
    default: Option<std::path::PathBuf>,
) {
    ui.horizontal(|ui| {
        row_label(ui, palette, label);
        let mut text = value.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
        let placeholder =
            default.map(|p| p.display().to_string()).unwrap_or_else(|| "Not available on this system".into());
        let response = ui.add(
            egui::TextEdit::singleline(&mut text)
                .hint_text(RichText::new(placeholder).color(palette.text_faint))
                .desired_width(ui.available_width()),
        );
        if response.changed() {
            *value = if text.trim().is_empty() { None } else { Some(text.trim().into()) };
        }
    });
}

fn about(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    group(ui, &palette, "VictorHUD", |ui| {
        ui.label(RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).strong());
        ui.label(RichText::new("A wrist-first SteamVR overlay, written in Rust.").color(palette.text_dim));
        let config_path = Config::path();
        ui.horizontal(|ui| {
            row_label(ui, &palette, "Settings file");
            ui.add(
                egui::Label::new(RichText::new(config_path.display().to_string()).size(12.5).color(palette.text_dim))
                    .truncate(),
            );
        });
        ui.horizontal(|ui| {
            if ui.button("Copy settings path").clicked() {
                ctx.act(Action::CopyToClipboard(config_path.display().to_string()));
            }
            if ui.button("Send test notification").clicked() {
                ctx.act(Action::SendTestNotification);
            }
        });
    });
}
