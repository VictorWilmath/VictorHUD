//! The wrist watch: clock, session timer, now playing, and the menu that opens everything else.

use egui::{Align, Align2, CornerRadius, FontId, Layout, Margin, RichText, Sense, Stroke, Ui, vec2};

use super::UiCtx;
use super::icons::{self, Icon};
use super::theme::{self, Palette};
use super::widgets::{self, ButtonKind, icon_button};
use crate::services::media::{MediaCommand, MediaSession, PlaybackStatus};
use crate::state::{Action, DeviceBattery, DeviceKind, PanelKind};
use crate::util::format_duration;

pub fn draw(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let frame = egui::Frame::new()
        .fill(palette.panel)
        .stroke(Stroke::new(1.0, palette.outline))
        .corner_radius(CornerRadius::same(30))
        .inner_margin(Margin::same(18));
    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.spacing_mut().item_spacing.y = 10.0;
        clock_row(ui, ctx);
        session_card(ui, ctx);
        media_card(ui, ctx);
        more_toggle(ui, ctx);
        if ctx.state.watch_expanded {
            menu(ui, ctx);
        }
    });
}

fn clock_row(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let general = &ctx.config.general;
    let now = ctx.now;
    let time = if general.clock_24h { now.format("%H:%M") } else { now.format("%-I:%M") }.to_string();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(RichText::new(time).size(56.0).strong().color(palette.text));
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.add_space(12.0);
            if general.show_seconds {
                ui.label(RichText::new(now.format(":%S").to_string()).size(20.0).color(palette.text_dim));
            }
            if !general.clock_24h {
                ui.label(RichText::new(now.format("%p").to_string()).size(17.0).color(palette.text_dim));
            }
        });
        ui.with_layout(Layout::top_down(Align::Max), |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            ui.add_space(8.0);
            if ctx.config.general.show_date {
                ui.label(RichText::new(now.format("%a, %b %-d").to_string()).size(16.0).color(palette.text_dim));
            }
            battery_row(ui, &palette, &ctx.state.batteries);
        });
    });
}

fn battery_row(ui: &mut Ui, palette: &Palette, batteries: &[DeviceBattery]) {
    if batteries.is_empty() {
        return;
    }
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for battery in batteries.iter().rev().take(4) {
            let color = if battery.charging {
                palette.accent
            } else if battery.percent < 0.2 {
                palette.bad
            } else if battery.percent < 0.4 {
                palette.warn
            } else {
                palette.good
            };
            ui.label(RichText::new(format!("{:.0}%", battery.percent * 100.0)).size(12.5).color(palette.text_dim));
            let (rect, response) = ui.allocate_exact_size(vec2(22.0, 11.0), Sense::hover());
            icons::battery(ui.painter(), rect, battery.percent, color, palette.text_faint);
            response.on_hover_text(&battery.label);
            let tag = match battery.kind {
                DeviceKind::Headset => "HMD",
                DeviceKind::LeftHand => "L",
                DeviceKind::RightHand => "R",
                DeviceKind::Tracker => "T",
            };
            ui.label(RichText::new(tag).size(12.0).strong().color(palette.text_faint));
            ui.add_space(6.0);
        }
    });
}

fn session_card(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let session = &ctx.state.session;
    let (label, elapsed) = session.display(ctx.system_now);
    let in_game = session.app_name.is_some();
    theme::card_frame(&palette).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            widgets::icon(ui, Icon::Clock, 26.0, if in_game { palette.good } else { palette.accent });
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(RichText::new(&label).size(16.0).strong());
                let detail = if in_game {
                    match session.steamvr_started.and_then(|t| ctx.system_now.duration_since(t).ok()) {
                        Some(vr) => format!("Session  ·  SteamVR {}", format_duration(vr)),
                        None => "Session".to_string(),
                    }
                } else {
                    "No game running".to_string()
                };
                ui.label(RichText::new(detail).size(12.5).color(palette.text_faint));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let text = elapsed.map(format_duration).unwrap_or_else(|| "--:--".into());
                ui.label(RichText::new(text).size(24.0).family(egui::FontFamily::Monospace).color(palette.text));
            });
        });
    });
}

fn media_card(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    let active = state.media.active(state.preferred_media_session.as_deref());
    theme::card_frame(&palette).show(ui, |ui| {
        ui.set_width(ui.available_width());
        match active {
            Some(session) => now_playing(ui, ctx, session),
            None => {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(44.0, 44.0), Sense::hover());
                    ui.painter().rect_filled(rect, 10.0, palette.card);
                    icons::paint(ui.painter(), rect.shrink(11.0), Icon::Music, palette.text_faint);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.label(RichText::new("Nothing playing").size(15.5).color(palette.text_dim));
                        let detail = state
                            .media
                            .error
                            .clone()
                            .unwrap_or_else(|| "Play music or a video on your PC to control it here".into());
                        ui.label(RichText::new(detail).size(12.5).color(palette.text_faint));
                    });
                });
            }
        }
    });
}

fn now_playing(ui: &mut Ui, ctx: &mut UiCtx, session: &MediaSession) {
    let palette = ctx.palette;
    let state = ctx.state;
    ui.horizontal(|ui| {
        if ctx.config.media.show_artwork {
            let texture = session.artwork.as_ref().map(|art| widgets::artwork_texture(ui.ctx(), art));
            widgets::artwork(ui, &palette, texture.as_ref(), 64.0);
        }
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            let title = if session.title.is_empty() { "Unknown title" } else { &session.title };
            ui.add(egui::Label::new(RichText::new(title).size(16.5).strong()).truncate());
            if !session.artist.is_empty() {
                ui.add(egui::Label::new(RichText::new(&session.artist).size(14.0).color(palette.text_dim)).truncate());
            }
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                widgets::pill(ui, &session.app_name, palette.accent);
                let status = match session.status {
                    PlaybackStatus::Playing => None,
                    PlaybackStatus::Paused => Some("Paused"),
                    PlaybackStatus::Stopped => Some("Stopped"),
                    PlaybackStatus::Changing => Some("Loading"),
                };
                if let Some(status) = status {
                    ui.label(RichText::new(status).size(12.0).color(palette.text_faint));
                }
                let sessions = &state.media.sessions;
                if sessions.len() > 1 {
                    let index = sessions.iter().position(|s| s.id == session.id).unwrap_or(0);
                    let next = &sessions[(index + 1) % sessions.len()];
                    let response = ui
                        .add(
                            egui::Button::new(
                                RichText::new(format!("{}/{}", index + 1, sessions.len()))
                                    .size(12.0)
                                    .color(palette.text_dim),
                            )
                            .corner_radius(10)
                            .min_size(vec2(36.0, 22.0)),
                        )
                        .on_hover_text(format!("Switch to {}", next.app_name));
                    if response.clicked() {
                        ctx.act(Action::SelectMediaSession(next.id.clone()));
                    }
                }
            });
        });
    });

    if let Some(duration) = session.duration.filter(|d| !d.is_zero()) {
        let progress = session.progress(ctx.instant);
        if let Some(fraction) = widgets::seek_bar(ui, &palette, progress, session.can_seek) {
            ctx.act(Action::Media(MediaCommand::Seek {
                session: session.id.clone(),
                position: duration.mul_f32(fraction),
            }));
        }
        let position = session.live_position(ctx.instant).unwrap_or_default();
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 12.0), Sense::hover());
        let font = FontId::proportional(12.0);
        ui.painter().text(
            rect.left_center(),
            Align2::LEFT_CENTER,
            format_duration(position),
            font.clone(),
            palette.text_faint,
        );
        ui.painter().text(
            rect.right_center(),
            Align2::RIGHT_CENTER,
            format_duration(duration),
            font,
            palette.text_faint,
        );
    }

    let playing = session.status == PlaybackStatus::Playing;
    let total = 44.0 + 60.0 + 44.0 + 2.0 * 18.0;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 18.0;
        ui.add_space(((ui.available_width() - total) / 2.0).max(0.0));
        ui.add_enabled_ui(session.can_previous, |ui| {
            if icon_button(ui, &palette, Icon::Previous, 44.0, ButtonKind::Soft, false).clicked() {
                ctx.act(Action::Media(MediaCommand::Previous { session: session.id.clone() }));
            }
        });
        let icon = if playing { Icon::Pause } else { Icon::Play };
        if icon_button(ui, &palette, icon, 60.0, ButtonKind::Primary, false).clicked() {
            ctx.act(Action::Media(MediaCommand::PlayPause { session: session.id.clone() }));
        }
        ui.add_enabled_ui(session.can_next, |ui| {
            if icon_button(ui, &palette, Icon::Next, 44.0, ButtonKind::Soft, false).clicked() {
                ctx.act(Action::Media(MediaCommand::Next { session: session.id.clone() }));
            }
        });
    });
}

fn more_toggle(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let expanded = ctx.state.watch_expanded;
    let unread = ctx.state.notifications.unread();
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let painter = ui.painter();
    if response.hovered() {
        painter.rect_filled(rect, 12.0, palette.card_hover);
    }
    let label = if expanded { "Less" } else { "More" };
    let galley = painter.layout_no_wrap(label.into(), FontId::proportional(15.0), palette.text_dim);
    let total = galley.size().x + 6.0 + 18.0;
    let start = rect.center().x - total / 2.0;
    painter.galley(egui::pos2(start, rect.center().y - galley.size().y / 2.0), galley, palette.text_dim);
    let icon_rect = egui::Rect::from_center_size(egui::pos2(start + total - 9.0, rect.center().y), vec2(18.0, 18.0));
    icons::paint(painter, icon_rect, if expanded { Icon::ChevronUp } else { Icon::ChevronDown }, palette.text_dim);
    if unread > 0 && !expanded {
        painter.circle_filled(egui::pos2(icon_rect.max.x + 8.0, icon_rect.min.y + 2.0), 4.5, palette.bad);
    }
    if response.clicked() {
        ctx.act(Action::SetWatchExpanded(!expanded));
    }
}

fn menu(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    let spacing = 8.0;
    let tile_size = vec2((ui.available_width() - spacing * 2.0) / 3.0, 78.0);
    let hidden = state.panels_hidden;
    let tiles = [
        (Icon::Monitor, "Desktops", Some(PanelKind::Desktop), 0),
        (Icon::Users, "VRCX", Some(PanelKind::Vrcx), 0),
        (Icon::Music, "Media", Some(PanelKind::Media), 0),
        (Icon::Bell, "Alerts", Some(PanelKind::Notifications), state.notifications.unread()),
        (Icon::Gear, "Settings", Some(PanelKind::Settings), 0),
        (if hidden { Icon::Eye } else { Icon::EyeOff }, if hidden { "Show all" } else { "Hide all" }, None, 0),
    ];
    for row in tiles.chunks(3) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = spacing;
            for &(icon, label, kind, badge) in row {
                let active = kind.is_none() && hidden;
                if widgets::tile(ui, &palette, icon, label, tile_size, active, badge).clicked() {
                    match kind {
                        Some(kind) => ctx.act(Action::OpenPanel(kind)),
                        None => ctx.act(Action::SetPanelsHidden(!hidden)),
                    }
                }
            }
        });
    }

    theme::card_frame(&palette).show(ui, |ui| {
        ui.set_width(ui.available_width());
        let mut keep = ctx.config.watch.keep_on_wrist;
        if widgets::toggle_row(ui, &palette, "Stay on wrist", "Keep the watch visible without raising it", &mut keep) {
            ctx.act(Action::SetKeepOnWrist(keep));
        }
        ui.add_space(2.0);
        let mut dnd = ctx.config.notifications.do_not_disturb;
        if widgets::toggle_row(ui, &palette, "Do not disturb", "Collect notifications without popups", &mut dnd) {
            ctx.act(Action::SetDoNotDisturb(dnd));
        }
    });

    if state.any_click_through
        && ui
            .add_sized(
                vec2(ui.available_width(), 36.0),
                egui::Button::new(RichText::new("Make click-through panels clickable").color(palette.accent)),
            )
            .clicked()
    {
        ctx.act(Action::MakeAllPanelsInteractive);
    }

    for warning in &state.warnings {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
            icons::paint(ui.painter(), rect, Icon::Warning, palette.warn);
            ui.add(egui::Label::new(RichText::new(warning).size(12.5).color(palette.warn)).wrap());
        });
    }
}
