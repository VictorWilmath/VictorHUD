//! The title bar shared by every floating panel: placement, lock, click-through, size, close.

use egui::{Align, Layout, RichText, Sense, Ui, Vec2};

use super::icons::{self, Icon};
use super::widgets::{ButtonKind, icon_button};
use super::{PanelInfo, UiCtx};
use crate::state::{Action, PanelAction, PanelKind, Placement};

pub fn kind_icon(kind: PanelKind) -> Icon {
    match kind {
        PanelKind::Watch => Icon::Watch,
        PanelKind::Media => Icon::Music,
        PanelKind::Desktop => Icon::Monitor,
        PanelKind::Vrcx => Icon::Users,
        PanelKind::Notifications => Icon::Bell,
        PanelKind::Settings => Icon::Gear,
    }
}

pub fn header(ui: &mut Ui, ctx: &mut UiCtx, panel: &PanelInfo) {
    let palette = ctx.palette;
    let id = panel.id;
    ui.horizontal(|ui| {
        ui.set_min_height(36.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::hover());
        icons::paint(ui.painter(), rect, kind_icon(panel.kind), palette.accent);
        ui.label(RichText::new(panel.kind.title()).size(18.0).strong());

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            if icon_button(ui, &palette, Icon::Close, 34.0, ButtonKind::Ghost, false)
                .on_hover_text("Close")
                .clicked()
            {
                ctx.act(Action::Panel(id, PanelAction::Close));
            }
            separator(ui);
            if icon_button(ui, &palette, Icon::Plus, 34.0, ButtonKind::Ghost, false)
                .on_hover_text("Larger")
                .clicked()
            {
                ctx.act(Action::Panel(id, PanelAction::AdjustSize(1.15)));
            }
            if icon_button(ui, &palette, Icon::Minus, 34.0, ButtonKind::Ghost, false)
                .on_hover_text("Smaller")
                .clicked()
            {
                ctx.act(Action::Panel(id, PanelAction::AdjustSize(1.0 / 1.15)));
            }
            let opacity = format!("{:.0}%", panel.opacity * 100.0);
            if ui
                .add(egui::Button::new(RichText::new(opacity).size(13.0)).corner_radius(17).min_size(egui::vec2(52.0, 34.0)))
                .on_hover_text("Opacity")
                .clicked()
            {
                ctx.act(Action::Panel(id, PanelAction::AdjustOpacity(-0.2)));
            }
            separator(ui);
            if icon_button(ui, &palette, Icon::Pointer, 34.0, ButtonKind::Ghost, panel.click_through)
                .on_hover_text("Click-through (restore from the watch menu)")
                .clicked()
            {
                ctx.act(Action::Panel(id, PanelAction::ToggleClickThrough));
            }
            let lock = if panel.locked { Icon::Lock } else { Icon::Unlock };
            if icon_button(ui, &palette, lock, 34.0, ButtonKind::Ghost, panel.locked)
                .on_hover_text(if panel.locked { "Unlock position" } else { "Lock position" })
                .clicked()
            {
                ctx.act(Action::Panel(id, PanelAction::ToggleLock));
            }
            separator(ui);
            for (placement, icon, tip) in [
                (Placement::Wrist, Icon::Watch, "Attach to wrist"),
                (Placement::Head, Icon::Eye, "Pin to view (follows your head)"),
                (Placement::World, Icon::Pin, "Pin in place"),
            ] {
                let active = panel.placement == placement;
                if icon_button(ui, &palette, icon, 34.0, ButtonKind::Ghost, active).on_hover_text(tip).clicked() && !active
                {
                    ctx.act(Action::Panel(id, PanelAction::SetPlacement(placement)));
                }
            }
        });
    });
}

fn separator(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(9.0, 22.0), Sense::hover());
    ui.painter().vline(rect.center().x, rect.y_range(), egui::Stroke::new(1.0, egui::Color32::from_white_alpha(30)));
}
