use egui::{
    Align2, Color32, CornerRadius, FontId, Rect, Response, RichText, Sense, Stroke, StrokeKind, TextureHandle, Ui,
    Vec2, pos2, vec2,
};

use super::icons::{self, Icon};
use super::theme::Palette;
use crate::state::Artwork;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// Transparent until hovered.
    Ghost,
    /// Subtle card-coloured fill.
    Soft,
    /// Accent-coloured fill (primary action).
    Primary,
}

pub fn icon_button(ui: &mut Ui, palette: &Palette, icon: Icon, size: f32, kind: ButtonKind, active: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();
        let fill = match kind {
            ButtonKind::Primary => {
                if pressed {
                    palette.accent.gamma_multiply(0.8)
                } else {
                    palette.accent
                }
            }
            _ if active => palette.accent_soft,
            ButtonKind::Soft if hovered => palette.card_hover,
            ButtonKind::Soft => palette.card,
            ButtonKind::Ghost if hovered => palette.card_hover,
            ButtonKind::Ghost => Color32::TRANSPARENT,
        };
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same((size / 2.0) as u8), fill);
        if hovered && kind == ButtonKind::Primary {
            painter.rect_stroke(rect, size / 2.0, Stroke::new(2.0, Color32::from_white_alpha(70)), StrokeKind::Inside);
        }
        let color = match kind {
            ButtonKind::Primary => Color32::from_rgb(12, 12, 18),
            _ if active => palette.accent,
            _ => palette.text,
        };
        let inset = size * 0.24;
        icons::paint(painter, rect.shrink(inset), icon, color);
    }
    response
}

/// A large rounded tile with an icon and caption, used by the watch menu.
pub fn tile(ui: &mut Ui, palette: &Palette, icon: Icon, label: &str, size: Vec2, active: bool, badge: usize) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let fill = if active {
            palette.accent_soft
        } else if response.hovered() {
            palette.card_hover
        } else {
            palette.card
        };
        painter.rect_filled(rect, CornerRadius::same(16), fill);
        if response.hovered() {
            painter.rect_stroke(rect, 16.0, Stroke::new(1.0, palette.accent.gamma_multiply(0.7)), StrokeKind::Inside);
        }
        let icon_size = (size.y * 0.36).min(30.0);
        let icon_rect = Rect::from_center_size(pos2(rect.center().x, rect.min.y + size.y * 0.38), Vec2::splat(icon_size));
        icons::paint(painter, icon_rect, icon, if active { palette.accent } else { palette.text });
        painter.text(
            pos2(rect.center().x, rect.max.y - size.y * 0.2),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(13.5),
            if active { palette.accent } else { palette.text_dim },
        );
        if badge > 0 {
            let center = pos2(icon_rect.max.x + 2.0, icon_rect.min.y + 2.0);
            painter.circle_filled(center, 9.0, palette.bad);
            painter.text(
                center,
                Align2::CENTER_CENTER,
                if badge > 9 { "9+".to_string() } else { badge.to_string() },
                FontId::proportional(11.0),
                Color32::WHITE,
            );
        }
    }
    response
}

/// A horizontal progress/seek bar. Returns the fraction the user clicked or dragged to.
pub fn seek_bar(ui: &mut Ui, palette: &Palette, fraction: Option<f32>, interactive: bool) -> Option<f32> {
    let height = 18.0;
    let width = ui.available_width();
    let sense = if interactive { Sense::click_and_drag() } else { Sense::hover() };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);
    let track = Rect::from_center_size(rect.center(), vec2(width, 5.0));
    let painter = ui.painter();
    painter.rect_filled(track, 3.0, Color32::from_white_alpha(28));
    let active = interactive && (response.hovered() || response.dragged());
    let mut result = None;
    let shown = if let (true, Some(pos)) = (response.dragged() || response.clicked(), response.interact_pointer_pos()) {
        let f = ((pos.x - track.min.x) / track.width()).clamp(0.0, 1.0);
        result = Some(f);
        Some(f)
    } else {
        fraction
    };
    if let Some(f) = shown {
        let filled = Rect::from_min_size(track.min, vec2(track.width() * f, track.height()));
        painter.rect_filled(filled, 3.0, palette.accent);
        if active {
            painter.circle_filled(pos2(filled.max.x, track.center().y), 7.0, Color32::WHITE);
        }
    }
    result
}

/// An on/off switch.
pub fn toggle(ui: &mut Ui, palette: &Palette, on: &mut bool) -> Response {
    let size = vec2(46.0, 26.0);
    let (rect, mut response) = ui.allocate_exact_size(size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    let t = ui.ctx().animate_bool_responsive(response.id, *on);
    let painter = ui.painter();
    let bg = lerp_color(Color32::from_white_alpha(40), palette.accent, t);
    painter.rect_filled(rect, 13.0, bg);
    let x = egui::lerp(rect.min.x + 13.0..=rect.max.x - 13.0, t);
    painter.circle_filled(pos2(x, rect.center().y), 10.0, Color32::WHITE);
    response
}

/// A labelled settings row with a switch on the right.
pub fn toggle_row(ui: &mut Ui, palette: &Palette, label: &str, hint: &str, on: &mut bool) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.set_width((ui.available_width() - 64.0).max(80.0));
            ui.label(RichText::new(label).color(palette.text));
            if !hint.is_empty() {
                ui.label(RichText::new(hint).small().color(palette.text_faint));
            }
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            changed = toggle(ui, palette, on).changed();
        });
    });
    changed
}

pub fn pill(ui: &mut Ui, text: &str, color: Color32) -> Response {
    let galley = ui.painter().layout_no_wrap(text.to_string(), FontId::proportional(12.0), color);
    let size = galley.size() + vec2(16.0, 6.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(255), color.gamma_multiply(0.16));
    painter.galley(rect.center() - galley.size() / 2.0, galley, color);
    response
}

pub fn chip(ui: &mut Ui, palette: &Palette, text: &str, selected: bool) -> Response {
    let color = if selected { palette.accent } else { palette.text_dim };
    let galley = ui.painter().layout_no_wrap(text.to_string(), FontId::proportional(14.0), color);
    let size = galley.size() + vec2(24.0, 14.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let painter = ui.painter();
    let fill = if selected {
        palette.accent_soft
    } else if response.hovered() {
        palette.card_hover
    } else {
        palette.card
    };
    painter.rect_filled(rect, CornerRadius::same(255), fill);
    painter.galley(rect.center() - galley.size() / 2.0, galley, color);
    response
}

pub fn section_label(ui: &mut Ui, palette: &Palette, text: &str) {
    ui.label(RichText::new(text.to_uppercase()).size(11.5).strong().color(palette.text_faint));
}

/// A coloured circle with someone's initials, used in player and friend lists.
pub fn avatar(ui: &mut Ui, name: &str, size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let hue = (crate::state::hash_bytes(name.as_bytes()) % 360) as f32 / 360.0;
    let color: Color32 = egui::ecolor::Hsva::new(hue, 0.55, 0.75, 1.0).into();
    let painter = ui.painter();
    painter.circle_filled(rect.center(), size / 2.0, color);
    let initials: String = name
        .split_whitespace()
        .filter_map(|word| word.chars().find(|c| c.is_alphanumeric()))
        .take(2)
        .collect::<String>()
        .to_uppercase();
    let initials = if initials.is_empty() { "?".to_string() } else { initials };
    painter.text(rect.center(), Align2::CENTER_CENTER, initials, FontId::proportional(size * 0.4), Color32::WHITE);
    response
}

pub fn status_dot(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(10.0, 10.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 4.5, color);
}

/// Album art or a placeholder.
pub fn artwork(ui: &mut Ui, palette: &Palette, texture: Option<&TextureHandle>, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let radius = CornerRadius::same((size * 0.14) as u8);
    match texture {
        Some(texture) => {
            egui::Image::from_texture(texture).corner_radius(radius).paint_at(ui, rect);
        }
        None => {
            let painter = ui.painter();
            painter.rect_filled(rect, radius, palette.accent_soft);
            icons::paint(painter, rect.shrink(size * 0.3), Icon::Music, palette.accent);
        }
    }
}

/// Keeps one GPU texture per artwork id alive in the egui context.
pub fn artwork_texture(ctx: &egui::Context, artwork: &Artwork) -> TextureHandle {
    let id = egui::Id::unique(("artwork", artwork.id));
    if let Some(texture) = ctx.data(|data| data.get_temp::<TextureHandle>(id)) {
        return texture;
    }
    let image = egui::ColorImage::from_rgba_unmultiplied(artwork.size, &artwork.rgba);
    let texture = ctx.load_texture(format!("artwork-{}", artwork.id), image, egui::TextureOptions::LINEAR);
    ctx.data_mut(|data| data.insert_temp(id, texture.clone()));
    texture
}

pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let lerp = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_premultiplied(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()), lerp(a.a(), b.a()))
}

/// A muted one-line message used for empty states.
pub fn empty_state(ui: &mut Ui, palette: &Palette, icon: Icon, title: &str, detail: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover());
        icons::paint(ui.painter(), rect, icon, palette.text_faint);
        ui.add_space(4.0);
        ui.label(RichText::new(title).size(16.0).color(palette.text_dim));
        if !detail.is_empty() {
            ui.label(RichText::new(detail).small().color(palette.text_faint));
        }
        ui.add_space(24.0);
    });
}

/// Draws a laser hit marker where the VR pointer hovers.
pub fn pointer_marker(ui: &Ui, palette: &Palette) {
    if let Some(pos) = ui.input(|input| input.pointer.hover_pos()) {
        let painter = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::unique("laser-dot")));
        painter.circle_filled(pos, 7.0, palette.accent.gamma_multiply(0.35));
        painter.circle_stroke(pos, 7.0, Stroke::new(2.0, Color32::WHITE));
        painter.circle_filled(pos, 2.5, Color32::WHITE);
    }
}
