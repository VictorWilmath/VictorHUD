//! Notification inbox and the popup toast.

use egui::{Align, CornerRadius, Layout, Margin, RichText, ScrollArea, Stroke, Ui};

use super::UiCtx;
use super::icons::Icon;
use super::theme::{self, Palette};
use super::widgets::{self, ButtonKind, icon_button};
use crate::state::{Action, Notification};
use crate::util::format_ago;

pub fn draw(ui: &mut Ui, ctx: &mut UiCtx) {
    let palette = ctx.palette;
    let state = ctx.state;
    let center = &state.notifications;
    ui.horizontal(|ui| {
        let mut dnd = ctx.config.notifications.do_not_disturb;
        if widgets::toggle(ui, &palette, &mut dnd).changed() {
            ctx.act(Action::SetDoNotDisturb(dnd));
        }
        widgets::icon(ui, Icon::Moon, 18.0, palette.text_dim);
        ui.label(RichText::new("Do not disturb").color(palette.text_dim));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_enabled_ui(!center.history.is_empty(), |ui| {
                if widgets::icon_button(ui, &palette, Icon::Trash, 32.0, ButtonKind::Ghost, false)
                    .on_hover_text("Clear all")
                    .clicked()
                {
                    ctx.act(Action::ClearNotifications);
                }
            });
            ui.add_enabled_ui(center.unread() > 0, |ui| {
                if ui.button("Mark read").clicked() {
                    ctx.act(Action::MarkNotificationsRead);
                }
            });
            if ui.button("Test").on_hover_text("Send yourself a test notification").clicked() {
                ctx.act(Action::SendTestNotification);
            }
        });
    });
    ui.add_space(4.0);

    if center.history.is_empty() {
        let detail = format!(
            "Apps that support XSOverlay notifications can send them here (UDP {} / WebSocket {}).",
            ctx.config.notifications.udp_port, ctx.config.notifications.websocket_port
        );
        widgets::empty_state(ui, &palette, Icon::Bell, "You're all caught up", &detail);
        return;
    }

    ScrollArea::vertical().id_salt("inbox").auto_shrink([false, false]).show(ui, |ui| {
        for notification in &center.history {
            let frame = theme::card_frame(&palette).inner_margin(Margin::symmetric(12, 10));
            let frame = if notification.read {
                frame
            } else {
                frame.stroke(Stroke::new(1.0, palette.accent.gamma_multiply(0.5)))
            };
            frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    notification_icon(ui, &palette, notification, 36.0);
                    ui.vertical(|ui| {
                        ui.set_width((ui.available_width() - 48.0).max(60.0));
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&notification.source).size(12.5).color(palette.accent));
                            let age = (ctx.now - notification.received).to_std().unwrap_or_default();
                            ui.label(RichText::new(format_ago(age)).size(12.0).color(palette.text_faint));
                        });
                        if !notification.title.is_empty() {
                            ui.add(egui::Label::new(RichText::new(&notification.title).strong()).wrap());
                        }
                        if !notification.body.is_empty() {
                            ui.add(
                                egui::Label::new(RichText::new(&notification.body).size(14.0).color(palette.text_dim))
                                    .wrap(),
                            );
                        }
                    });
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if icon_button(ui, &palette, Icon::Close, 30.0, ButtonKind::Ghost, false)
                            .on_hover_text("Dismiss")
                            .clicked()
                        {
                            ctx.act(Action::DismissNotification(notification.id));
                        }
                    });
                });
            });
        }
    });
}

fn notification_icon(ui: &mut Ui, palette: &Palette, notification: &Notification, size: f32) {
    match &notification.icon {
        Some(icon) => {
            let texture = widgets::artwork_texture(ui.ctx(), icon);
            let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(size), egui::Sense::hover());
            egui::Image::from_texture(&texture)
                .corner_radius(CornerRadius::same((size / 4.0) as u8))
                .paint_at(ui, rect);
        }
        None => {
            let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(size), egui::Sense::hover());
            ui.painter().rect_filled(rect, size / 4.0, palette.accent_soft);
            super::icons::paint(ui.painter(), rect.shrink(size * 0.25), Icon::Bell, palette.accent);
        }
    }
}

/// The popup shown when a notification arrives. Fills `ui` (sized [`super::TOAST_SIZE`]).
pub fn draw_toast(ui: &mut Ui, palette: &Palette, notification: &Notification) {
    let frame = egui::Frame::new()
        .fill(palette.panel)
        .stroke(Stroke::new(1.0, palette.accent.gamma_multiply(0.6)))
        .corner_radius(CornerRadius::same(22))
        .inner_margin(Margin::symmetric(16, 12));
    frame.show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.horizontal(|ui| {
            notification_icon(ui, palette, notification, 44.0);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 1.0;
                ui.label(RichText::new(&notification.source).size(12.5).color(palette.accent));
                let title = if notification.title.is_empty() { &notification.body } else { &notification.title };
                ui.add(egui::Label::new(RichText::new(title).size(16.0).strong()).truncate());
                if !notification.title.is_empty() && !notification.body.is_empty() {
                    ui.add(
                        egui::Label::new(RichText::new(&notification.body).size(13.5).color(palette.text_dim))
                            .truncate(),
                    );
                }
            });
        });
    });
}
