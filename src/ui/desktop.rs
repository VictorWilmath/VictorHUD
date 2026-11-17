//! Desktop panel: source picker toolbar plus the captured image. Laser input over the image is
//! routed to Windows by the host, not to egui.

use egui::{Color32, Pos2, Rect, RichText, Sense, Shape, Stroke, Ui, pos2, vec2};

use super::icons::Icon;
use super::widgets::{self, ButtonKind, icon_button};
use super::{DesktopView, PanelInfo, UiCtx};
use crate::services::desktop::{CaptureTarget, target_label};
use crate::state::{Action, PanelAction};

pub fn draw(ui: &mut Ui, ctx: &mut UiCtx, panel: &PanelInfo, view: Option<&DesktopView>) -> Option<Rect> {
    let palette = ctx.palette;
    let state = ctx.state;
    let target = panel.capture.unwrap_or(CaptureTarget::Monitor(ctx.config.desktop.default_monitor));
    let id = panel.id;

    ui.horizontal(|ui| {
        ui.set_min_height(36.0);
        for monitor in &state.monitors {
            let selected = target == CaptureTarget::Monitor(monitor.index);
            if widgets::chip(ui, &palette, &monitor.label, selected).clicked() && !selected {
                ctx.act(Action::Panel(id, PanelAction::SetCaptureTarget(CaptureTarget::Monitor(monitor.index))));
            }
        }
        let window_selected = matches!(target, CaptureTarget::Window(_));
        let combo_text = if window_selected { target_label(&target, &state.monitors, &state.windows) } else { "Window…".into() };
        let combo = egui::ComboBox::from_id_salt(("window-picker", id.0))
            .selected_text(RichText::new(combo_text).color(if window_selected { palette.accent } else { palette.text_dim }))
            .width(240.0)
            .height(420.0)
            .show_ui(ui, |ui| {
                if state.windows.is_empty() {
                    ui.label(RichText::new("No capturable windows").color(palette.text_faint));
                }
                for window in &state.windows {
                    let selected = target == CaptureTarget::Window(window.hwnd);
                    let label = format!("{}  —  {}", window.title, window.process);
                    if ui.selectable_label(selected, label).clicked() && !selected {
                        ctx.act(Action::Panel(id, PanelAction::SetCaptureTarget(CaptureTarget::Window(window.hwnd))));
                    }
                }
            });
        if combo.response.clicked() {
            ctx.act(Action::RefreshWindows);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            if ctx.in_vr
                && icon_button(ui, &palette, Icon::Keyboard, 36.0, ButtonKind::Soft, false)
                    .on_hover_text("Type with the SteamVR keyboard")
                    .clicked()
            {
                ctx.act(Action::ShowKeyboard(id));
            }
            if view.is_some_and(|v| v.virtual_desktops) {
                ui.add_space(6.0);
                if icon_button(ui, &palette, Icon::ArrowRight, 36.0, ButtonKind::Soft, false)
                    .on_hover_text("Next virtual desktop")
                    .clicked()
                {
                    ctx.act(Action::SwitchVirtualDesktop { forward: true });
                }
                if icon_button(ui, &palette, Icon::ArrowLeft, 36.0, ButtonKind::Soft, false)
                    .on_hover_text("Previous virtual desktop")
                    .clicked()
                {
                    ctx.act(Action::SwitchVirtualDesktop { forward: false });
                }
                ui.label(RichText::new("Virtual desktop").size(13.0).color(palette.text_faint));
            }
        });
    });
    ui.add_space(8.0);

    let area = ui.available_rect_before_wrap();
    let (texture, size) = match view {
        Some(DesktopView { texture: Some(texture), size, .. }) if size[0] > 0 && size[1] > 0 => (Some(*texture), *size),
        _ => (None, [16, 9]),
    };
    let aspect = size[0] as f32 / size[1] as f32;
    let mut image_size = vec2(area.width(), area.width() / aspect);
    if image_size.y > area.height() {
        image_size = vec2(area.height() * aspect, area.height());
    }
    let image_rect = Rect::from_center_size(pos2(area.center().x, area.min.y + image_size.y / 2.0), image_size);
    ui.allocate_rect(Rect::from_min_max(area.min, pos2(area.max.x, image_rect.max.y)), Sense::hover());

    let painter = ui.painter();
    match texture {
        Some(texture) => {
            painter.add(
                egui::epaint::RectShape::filled(image_rect, 10.0, Color32::WHITE)
                    .with_texture(texture, Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0))),
            );
            if ctx.config.desktop.show_cursor
                && let Some([x, y]) = view.and_then(|v| v.cursor)
            {
                let tip = image_rect.min + vec2(x * image_rect.width(), y * image_rect.height());
                draw_cursor(painter, tip, image_rect.width() / 960.0 * 22.0);
            }
        }
        None => {
            painter.rect_filled(image_rect, 10.0, Color32::from_black_alpha(120));
            let status = view.and_then(|v| v.status.as_deref()).unwrap_or("Starting capture…");
            painter.text(
                image_rect.center(),
                egui::Align2::CENTER_CENTER,
                status,
                egui::FontId::proportional(18.0),
                palette.text_dim,
            );
        }
    }
    texture.map(|_| image_rect)
}

fn draw_cursor(painter: &egui::Painter, tip: Pos2, size: f32) {
    let points: Vec<Pos2> = [(0.0, 0.0), (0.0, 1.0), (0.27, 0.77), (0.45, 1.1), (0.58, 1.04), (0.41, 0.72), (0.72, 0.72)]
        .iter()
        .map(|&(x, y)| tip + vec2(x * size, y * size))
        .collect();
    painter.add(Shape::Path(egui::epaint::PathShape::closed_line(points.clone(), Stroke::new(1.5, Color32::BLACK))));
    let body = [points[0], points[1], points[6]];
    painter.add(Shape::convex_polygon(body.to_vec(), Color32::WHITE, Stroke::new(1.2, Color32::BLACK)));
    let tail = [points[2], points[3], points[4], points[5]];
    painter.add(Shape::convex_polygon(tail.to_vec(), Color32::WHITE, Stroke::new(1.2, Color32::BLACK)));
}
