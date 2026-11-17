//! Full media panel: artwork, seek, shuffle/repeat, per-app and master volume, source picker.

use egui::{Align, Layout, RichText, Sense, Ui, vec2};

use super::UiCtx;
use super::icons::{self, Icon};
use super::theme;
use super::widgets::{self, ButtonKind, icon_button};
use crate::services::media::{MediaCommand, MediaSession, PlaybackStatus, RepeatMode};
use crate::state::Action;
use crate::util::format_duration;

pub fn draw(ui: &mut Ui, ctx: &mut UiCtx) {
    egui::ScrollArea::vertical().id_salt("media").auto_shrink([false, false]).show(ui, |ui| body(ui, ctx));
}

fn body(ui: &mut Ui, ctx: &mut UiCtx) {
    let state = ctx.state;
    let palette = ctx.palette;
    let Some(session) = state.media.active(state.preferred_media_session.as_deref()) else {
        let detail =
            state.media.error.as_deref().unwrap_or("Start music or a video on your PC and it will show up here.");
        widgets::empty_state(ui, &palette, Icon::Music, "Nothing is playing", detail);
        volume_section(ui, ctx, None);
        return;
    };

    ui.horizontal(|ui| {
        let texture = session.artwork.as_ref().map(|art| widgets::artwork_texture(ui.ctx(), art));
        widgets::artwork(ui, &palette, texture.as_ref(), 168.0);
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            let title = if session.title.is_empty() { "Unknown title" } else { &session.title };
            ui.add(egui::Label::new(RichText::new(title).size(22.0).strong()).truncate());
            if !session.artist.is_empty() {
                ui.add(egui::Label::new(RichText::new(&session.artist).size(16.0).color(palette.text_dim)).truncate());
            }
            if !session.album.is_empty() {
                ui.add(egui::Label::new(RichText::new(&session.album).size(13.5).color(palette.text_faint)).truncate());
            }
            ui.horizontal(|ui| {
                widgets::pill(ui, &format!("Playing from {}", session.app_name), palette.accent);
            });
            ui.add_space(6.0);
            timeline(ui, ctx, session);
            controls(ui, ctx, session);
        });
    });

    if state.media.sessions.len() > 1 {
        ui.add_space(4.0);
        widgets::section_label(ui, &palette, "Sources");
        ui.horizontal_wrapped(|ui| {
            for other in &state.media.sessions {
                let label = match other.status {
                    PlaybackStatus::Playing => format!("{}  ·  playing", other.app_name),
                    _ => other.app_name.clone(),
                };
                if widgets::chip(ui, &palette, &label, other.id == session.id).clicked() {
                    ctx.act(Action::SelectMediaSession(other.id.clone()));
                }
            }
        });
    }
    ui.add_space(4.0);
    volume_section(ui, ctx, Some(session));
}

fn timeline(ui: &mut Ui, ctx: &mut UiCtx, session: &MediaSession) {
    let palette = ctx.palette;
    let Some(duration) = session.duration.filter(|d| !d.is_zero()) else {
        ui.label(RichText::new("Live or unknown length").size(12.5).color(palette.text_faint));
        return;
    };
    if let Some(fraction) = widgets::seek_bar(ui, &palette, session.progress(ctx.instant), session.can_seek) {
        ctx.act(Action::Media(MediaCommand::Seek {
            session: session.id.clone(),
            position: duration.mul_f32(fraction),
        }));
    }
    ui.horizontal(|ui| {
        let position = session.live_position(ctx.instant).unwrap_or_default();
        ui.label(RichText::new(format_duration(position)).size(12.5).color(palette.text_faint));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(format_duration(duration)).size(12.5).color(palette.text_faint));
        });
    });
}

fn controls(ui: &mut Ui, ctx: &mut UiCtx, session: &MediaSession) {
    let palette = ctx.palette;
    let id = session.id.clone();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        let shuffle = session.shuffle.unwrap_or(false);
        ui.add_enabled_ui(session.shuffle.is_some(), |ui| {
            if icon_button(ui, &palette, Icon::Shuffle, 40.0, ButtonKind::Ghost, shuffle)
                .on_hover_text("Shuffle")
                .clicked()
            {
                ctx.act(Action::Media(MediaCommand::SetShuffle { session: id.clone(), shuffle: !shuffle }));
            }
        });
        ui.add_enabled_ui(session.can_previous, |ui| {
            if icon_button(ui, &palette, Icon::Previous, 46.0, ButtonKind::Soft, false).clicked() {
                ctx.act(Action::Media(MediaCommand::Previous { session: id.clone() }));
            }
        });
        let icon = if session.status == PlaybackStatus::Playing { Icon::Pause } else { Icon::Play };
        if icon_button(ui, &palette, icon, 58.0, ButtonKind::Primary, false).clicked() {
            ctx.act(Action::Media(MediaCommand::PlayPause { session: id.clone() }));
        }
        ui.add_enabled_ui(session.can_next, |ui| {
            if icon_button(ui, &palette, Icon::Next, 46.0, ButtonKind::Soft, false).clicked() {
                ctx.act(Action::Media(MediaCommand::Next { session: id.clone() }));
            }
        });
        let repeat = session.repeat.unwrap_or(RepeatMode::Off);
        let icon = if repeat == RepeatMode::Track { Icon::RepeatOne } else { Icon::Repeat };
        ui.add_enabled_ui(session.repeat.is_some(), |ui| {
            let tip = match repeat {
                RepeatMode::Off => "Repeat: off",
                RepeatMode::List => "Repeat: all",
                RepeatMode::Track => "Repeat: one",
            };
            if icon_button(ui, &palette, icon, 40.0, ButtonKind::Ghost, repeat != RepeatMode::Off)
                .on_hover_text(tip)
                .clicked()
            {
                ctx.act(Action::Media(MediaCommand::SetRepeat { session: id.clone(), repeat: repeat.next() }));
            }
        });
    });
}

fn volume_section(ui: &mut Ui, ctx: &mut UiCtx, session: Option<&MediaSession>) {
    let palette = ctx.palette;
    let state = ctx.state;
    let media = &state.media;
    if media.master_volume.is_none() && session.and_then(|s| s.app_volume).is_none() {
        return;
    }
    theme::card_frame(&palette).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.spacing_mut().slider_width = (ui.available_width() - 170.0).max(120.0);
        if let Some(master) = media.master_volume {
            let muted = media.muted.unwrap_or(false);
            ui.horizontal(|ui| {
                let icon = if muted { Icon::Mute } else { Icon::Volume };
                if icon_button(ui, &palette, icon, 34.0, ButtonKind::Ghost, muted).on_hover_text("Mute").clicked() {
                    ctx.act(Action::Media(MediaCommand::SetMuted(!muted)));
                }
                ui.add_sized(vec2(96.0, 20.0), egui::Label::new(RichText::new("Master").color(palette.text_dim)));
                let mut value = master;
                if ui.add(egui::Slider::new(&mut value, 0.0..=1.0).show_value(false)).changed() {
                    ctx.act(Action::Media(MediaCommand::SetMasterVolume(value)));
                }
                ui.label(RichText::new(format!("{:.0}%", value * 100.0)).size(13.0).color(palette.text_dim));
            });
        }
        if let Some((session, volume)) = session.and_then(|s| s.app_volume.map(|v| (s, v))) {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
                icons::paint(ui.painter(), rect.shrink(8.0), Icon::Music, palette.text_dim);
                ui.add_sized(
                    vec2(96.0, 20.0),
                    egui::Label::new(RichText::new(&session.app_name).color(palette.text_dim)).truncate(),
                );
                let mut value = volume;
                if ui.add(egui::Slider::new(&mut value, 0.0..=1.0).show_value(false)).changed() {
                    ctx.act(Action::Media(MediaCommand::SetAppVolume { session: session.id.clone(), volume: value }));
                }
                ui.label(RichText::new(format!("{:.0}%", value * 100.0)).size(13.0).color(palette.text_dim));
            });
        }
    });
}
