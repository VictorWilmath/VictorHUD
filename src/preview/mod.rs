//! `--preview`: every panel in a normal desktop window, running on sample data (or live data with
//! `--live` on Windows). Useful for working on the UI without putting on a headset.

use std::time::{Duration, SystemTime};

use chrono::Local;
use egui::{Align, Color32, Layout, RichText, Sense, Ui, vec2};

use crate::hud::Hud;
use crate::services::desktop::CaptureTarget;
use crate::services::{DataMode, ServiceEvent};
use crate::state::{Action, DeviceBattery, DeviceKind, Notification, PanelAction, PanelId, PanelKind, SessionInfo};
use crate::ui::{self, DesktopView, PanelInfo, UiCtx, theme};

pub fn run(mode: DataMode) -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("VictorHUD — preview")
            .with_inner_size([1480.0, 920.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native("VictorHUD", options, Box::new(move |cc| Ok(Box::new(PreviewApp::new(cc, mode)))))
        .map_err(|err| anyhow::anyhow!("preview window failed: {err}"))
}

struct PreviewApp {
    hud: Hud,
    mode: DataMode,
    panels: Vec<PanelInfo>,
    focused: Option<PanelId>,
    next_id: u32,
    applied_accent: Option<[u8; 3]>,
    desktop_texture: Option<egui::TextureHandle>,
    status: Option<(String, std::time::Instant)>,
}

impl PreviewApp {
    fn new(_cc: &eframe::CreationContext<'_>, mode: DataMode) -> Self {
        let mut hud = Hud::new(crate::config::Config::path(), mode);
        if mode == DataMode::Demo {
            seed_demo_state(&mut hud);
        }
        let mut app = Self {
            hud,
            mode,
            panels: Vec::new(),
            focused: None,
            next_id: 1,
            applied_accent: None,
            desktop_texture: None,
            status: None,
        };
        app.open(PanelKind::Media);
        app
    }

    fn open(&mut self, kind: PanelKind) {
        if let Some(existing) = self.panels.iter().find(|p| p.kind == kind && kind != PanelKind::Desktop) {
            self.focused = Some(existing.id);
            return;
        }
        let id = PanelId(self.next_id);
        self.next_id += 1;
        let mut info = PanelInfo::new(id, kind);
        if kind == PanelKind::Desktop {
            info.capture = Some(CaptureTarget::Monitor(self.hud.config.desktop.default_monitor));
        }
        self.panels.push(info);
        self.focused = Some(id);
    }

    fn flash(&mut self, message: impl Into<String>) {
        self.status = Some((message.into(), std::time::Instant::now()));
    }

    fn handle(&mut self, ctx: &egui::Context, action: Action) {
        let Some(action) = self.hud.handle(action) else { return };
        match action {
            Action::OpenPanel(kind) => self.open(kind),
            Action::Panel(id, panel_action) => {
                let Some(index) = self.panels.iter().position(|p| p.id == id) else { return };
                let panel = &mut self.panels[index];
                match panel_action {
                    PanelAction::Close => {
                        self.panels.remove(index);
                        if self.focused == Some(id) {
                            self.focused = self.panels.last().map(|p| p.id);
                        }
                    }
                    PanelAction::SetPlacement(placement) => {
                        panel.placement = placement;
                        let label = placement.label();
                        self.flash(format!("In VR this panel would now be: {label}"));
                    }
                    PanelAction::ToggleLock => panel.locked = !panel.locked,
                    PanelAction::ToggleClickThrough => panel.click_through = !panel.click_through,
                    PanelAction::AdjustOpacity(delta) => panel.opacity = cycle_opacity(panel.opacity, delta),
                    PanelAction::AdjustSize(_) => self.flash("Panel size only changes in VR"),
                    PanelAction::SetCaptureTarget(target) => panel.capture = Some(target),
                }
                self.hud.state.any_click_through = self.panels.iter().any(|p| p.click_through);
            }
            Action::MakeAllPanelsInteractive => {
                for panel in &mut self.panels {
                    panel.click_through = false;
                }
                self.hud.state.any_click_through = false;
            }
            Action::CopyToClipboard(text) => {
                ctx.copy_text(text);
                self.flash("Copied to clipboard");
            }
            Action::RefreshWindows => {
                if self.mode == DataMode::Demo {
                    self.hud.state.windows = crate::services::desktop::demo_windows();
                }
            }
            Action::SwitchVirtualDesktop { forward } => {
                self.flash(if forward { "Next virtual desktop" } else { "Previous virtual desktop" })
            }
            Action::ResetWatchPosition => self.flash("Watch position reset"),
            Action::ShowKeyboard(_) => self.flash("The SteamVR keyboard opens in VR"),
            Action::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            other => log::debug!("preview ignores {other:?}"),
        }
    }

    fn desktop_view(&mut self, ctx: &egui::Context) -> DesktopView {
        let texture = self.desktop_texture.get_or_insert_with(|| {
            ctx.load_texture("demo-desktop", demo_desktop_image(), egui::TextureOptions::LINEAR)
        });
        DesktopView {
            texture: Some(texture.id()),
            size: [1920, 1080],
            cursor: Some([0.46, 0.41]),
            status: None,
            virtual_desktops: true,
        }
    }
}

fn cycle_opacity(current: f32, delta: f32) -> f32 {
    let next = current + delta;
    if next < 0.35 { 1.0 } else { next.clamp(0.2, 1.0) }
}

impl eframe::App for PreviewApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if self.hud.update() {
            ctx.request_repaint();
        }
        if self.applied_accent != Some(self.hud.config.general.accent) {
            theme::apply(&ctx, &theme::Palette::new(self.hud.config.general.accent));
            self.applied_accent = Some(self.hud.config.general.accent);
        }
        let desktop = self.desktop_view(&ctx);

        let full = ui.max_rect();
        ui.painter().rect_filled(full, 0.0, Color32::from_rgb(9, 10, 15));

        let mut ui_ctx = UiCtx::new(&self.hud.state, &mut self.hud.config, false);
        let palette = ui_ctx.palette;
        let mut focus_change = None;

        egui::Frame::new().inner_margin(egui::Margin::same(24)).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("VictorHUD").size(22.0).strong());
                let note = match self.mode {
                    DataMode::Demo => "Desktop preview  ·  sample data",
                    DataMode::Live => "Desktop preview  ·  live data",
                };
                ui.label(RichText::new(note).color(palette.text_faint));
                if let Some((message, at)) = &self.status
                    && at.elapsed() < Duration::from_secs(3)
                {
                    ui.add_space(16.0);
                    ui.label(RichText::new(message).color(palette.accent));
                }
            });
            ui.add_space(12.0);

            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui::WATCH_WIDTH);
                    let watch = PanelInfo::new(PanelId::WATCH, PanelKind::Watch);
                    ui::draw_panel(ui, &mut ui_ctx, &watch, None);
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("In VR this sits on your wrist and appears when you look at it.")
                            .size(12.5)
                            .color(palette.text_faint),
                    );
                });
                ui.add_space(28.0);

                ui.vertical(|ui| {
                    if ui_ctx.state.panels_hidden {
                        ui.label(RichText::new("Panels are hidden. Use “Show all” on the watch.").color(palette.text_dim));
                        return;
                    }
                    ui.horizontal(|ui| {
                        for panel in &self.panels {
                            let selected = self.focused == Some(panel.id);
                            let label = match panel.kind {
                                PanelKind::Desktop => format!(
                                    "Desktop · {}",
                                    panel
                                        .capture
                                        .map(|t| crate::services::desktop::target_label(
                                            &t,
                                            &ui_ctx.state.monitors,
                                            &ui_ctx.state.windows
                                        ))
                                        .unwrap_or_default()
                                ),
                                kind => kind.title().to_string(),
                            };
                            if ui::widgets::chip(ui, &palette, &label, selected).clicked() {
                                focus_change = Some(panel.id);
                            }
                        }
                        if self.panels.is_empty() {
                            ui.label(RichText::new("Open a panel from the watch menu (More).").color(palette.text_faint));
                        }
                    });
                    ui.add_space(10.0);
                    let Some(panel) = self.focused.and_then(|id| self.panels.iter().find(|p| p.id == id)) else {
                        return;
                    };
                    let size = ui::panel_size(panel.kind, Some(16.0 / 9.0));
                    egui::ScrollArea::both().id_salt("preview-panel").show(ui, |ui| {
                        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
                        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(Layout::top_down(Align::Min)));
                        child.set_clip_rect(rect.intersect(ui.clip_rect()));
                        child.multiply_opacity(panel.opacity);
                        let view = (panel.kind == PanelKind::Desktop).then_some(&desktop);
                        ui::draw_panel(&mut child, &mut ui_ctx, panel, view);
                        if panel.click_through {
                            ui.painter().rect_filled(rect, 26.0, Color32::from_black_alpha(60));
                        }
                    });
                });
            });
        });

        if let Some((notification, alpha)) = self.hud_toast() {
            let width = ui::TOAST_SIZE.x;
            egui::Area::new(egui::Id::new("toast"))
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(full.center().x - width / 2.0, full.min.y + 18.0))
                .show(&ctx, |ui| {
                    ui.set_width(width);
                    ui.set_height(ui::TOAST_SIZE.y);
                    ui.multiply_opacity(alpha);
                    ui::notifications::draw_toast(ui, &palette, &notification);
                });
            ctx.request_repaint();
        }

        let actions = std::mem::take(&mut ui_ctx.actions);
        let config_changed = ui_ctx.config_changed;
        drop(ui_ctx);
        if config_changed {
            self.hud.mark_config_dirty();
        }
        if let Some(id) = focus_change {
            self.focused = Some(id);
        }
        for action in actions {
            self.handle(&ctx, action);
        }
        ctx.request_repaint_after(Duration::from_millis(250));
    }

    fn on_exit(&mut self) {
        self.hud.save_now();
    }
}

impl PreviewApp {
    fn hud_toast(&mut self) -> Option<(Notification, f32)> {
        self.hud.current_toast().map(|(n, alpha)| (n.clone(), alpha))
    }
}

fn seed_demo_state(hud: &mut Hud) {
    let now = SystemTime::now();
    hud.state.session = SessionInfo {
        app_name: Some("VRChat".into()),
        app_started: now.checked_sub(Duration::from_secs(72 * 60 + 18)),
        steamvr_started: now.checked_sub(Duration::from_secs(98 * 60 + 41)),
    };
    hud.state.batteries = vec![
        DeviceBattery { kind: DeviceKind::LeftHand, label: "Left controller".into(), percent: 0.64, charging: false },
        DeviceBattery { kind: DeviceKind::RightHand, label: "Right controller".into(), percent: 0.18, charging: false },
    ];
    let sender = hud.services.sender();
    let demo = [
        ("VRCX", "Moonbeam is online", "Last seen in The Great Pug"),
        ("Discord", "Kazuha", "you coming to the movie world tonight?"),
    ];
    for (source, title, body) in demo {
        let _ = sender.send(ServiceEvent::Notification(Notification {
            id: 0,
            source: source.into(),
            title: title.into(),
            body: body.into(),
            received: Local::now(),
            timeout: Duration::from_secs(5),
            icon: None,
            read: false,
        }));
    }
}

/// A stand-in desktop: wallpaper, two windows and a taskbar.
fn demo_desktop_image() -> egui::ColorImage {
    let (w, h) = (960usize, 540usize);
    let mut pixels = vec![Color32::BLACK; w * h];
    let mut fill = |x0: usize, y0: usize, x1: usize, y1: usize, color: Color32| {
        for y in y0.min(h)..y1.min(h) {
            for x in x0.min(w)..x1.min(w) {
                pixels[y * w + x] = color;
            }
        }
    };
    for y in 0..h {
        for x in 0..w {
            let t = y as f32 / h as f32;
            let s = x as f32 / w as f32;
            let r = (30.0 + 60.0 * s + 20.0 * t) as u8;
            let g = (24.0 + 30.0 * (1.0 - t)) as u8;
            let b = (70.0 + 90.0 * (1.0 - s * 0.5) - 30.0 * t) as u8;
            fill(x, y, x + 1, y + 1, Color32::from_rgb(r, g, b));
        }
    }
    fill(110, 70, 600, 400, Color32::from_rgb(244, 245, 248));
    fill(110, 70, 600, 98, Color32::from_rgb(222, 224, 232));
    fill(130, 120, 400, 136, Color32::from_rgb(60, 64, 80));
    for row in 0..8 {
        let y = 160 + row * 26;
        let width = 380 - (row * 37) % 140;
        fill(130, y, 130 + width, y + 10, Color32::from_rgb(180, 184, 198));
    }
    fill(520, 180, 880, 470, Color32::from_rgb(43, 45, 57));
    fill(520, 180, 880, 206, Color32::from_rgb(32, 33, 42));
    for row in 0..7 {
        let y = 226 + row * 34;
        fill(540, y, 564, y + 24, Color32::from_rgb(120, 140, 255));
        fill(576, y + 4, 576 + 150 + (row * 41) % 120, y + 12, Color32::from_rgb(200, 204, 220));
    }
    fill(0, 508, w, h, Color32::from_rgb(20, 22, 30));
    for i in 0..6 {
        let x = 380 + i * 36;
        fill(x, 516, x + 24, 532, Color32::from_rgb(90 + i as u8 * 20, 120, 220 - i as u8 * 15));
    }
    egui::ColorImage::new([w, h], pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opacity_cycles_back_to_full() {
        assert!((cycle_opacity(1.0, -0.2) - 0.8).abs() < 1e-6);
        assert!((cycle_opacity(0.4, -0.2) - 1.0).abs() < 1e-6);
    }
}
