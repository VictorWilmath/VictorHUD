use egui::{Color32, CornerRadius, FontFamily, FontId, Margin, Stroke, TextStyle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub panel: Color32,
    pub card: Color32,
    pub card_hover: Color32,
    pub outline: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub bad: Color32,
}

impl Palette {
    pub fn new(accent: [u8; 3]) -> Self {
        let accent = Color32::from_rgb(accent[0], accent[1], accent[2]);
        Self {
            panel: Color32::from_rgba_unmultiplied(14, 15, 22, 242),
            card: Color32::from_rgba_unmultiplied(255, 255, 255, 12),
            card_hover: Color32::from_rgba_unmultiplied(255, 255, 255, 24),
            outline: Color32::from_rgba_unmultiplied(255, 255, 255, 22),
            text: Color32::from_rgb(238, 240, 248),
            text_dim: Color32::from_rgb(160, 166, 186),
            text_faint: Color32::from_rgb(108, 114, 134),
            accent,
            accent_soft: accent.gamma_multiply(0.22),
            good: Color32::from_rgb(86, 222, 150),
            warn: Color32::from_rgb(255, 196, 92),
            bad: Color32::from_rgb(255, 98, 116),
        }
    }
}

pub const PANEL_RADIUS: u8 = 26;
pub const CARD_RADIUS: u8 = 16;

pub fn card_frame(palette: &Palette) -> egui::Frame {
    egui::Frame::new().fill(palette.card).corner_radius(CornerRadius::same(CARD_RADIUS)).inner_margin(Margin::same(12))
}

pub fn panel_frame(palette: &Palette) -> egui::Frame {
    egui::Frame::new()
        .fill(palette.panel)
        .stroke(Stroke::new(1.0, palette.outline))
        .corner_radius(CornerRadius::same(PANEL_RADIUS))
        .inner_margin(Margin::same(16))
}

/// Applies the VictorHUD look to an egui context.
pub fn apply(ctx: &egui::Context, palette: &Palette) {
    ctx.set_theme(egui::Theme::Dark);
    let palette = *palette;
    ctx.all_styles_mut(move |style| {
        style.text_styles = [
            (TextStyle::Small, FontId::new(12.5, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.5, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(15.5, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(22.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
        ]
        .into();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(12.0, 7.0);
        style.spacing.interact_size.y = 30.0;
        style.spacing.slider_width = 220.0;
        style.spacing.scroll.bar_width = 6.0;
        style.spacing.scroll.floating = true;
        style.interaction.tooltip_delay = 0.6;

        let visuals = &mut style.visuals;
        visuals.override_text_color = Some(palette.text);
        visuals.panel_fill = Color32::TRANSPARENT;
        visuals.window_fill = palette.panel;
        visuals.window_stroke = Stroke::new(1.0, palette.outline);
        visuals.window_corner_radius = CornerRadius::same(18);
        visuals.extreme_bg_color = Color32::from_rgba_unmultiplied(0, 0, 0, 90);
        visuals.faint_bg_color = palette.card;
        visuals.hyperlink_color = palette.accent;
        visuals.selection.bg_fill = palette.accent.gamma_multiply(0.55);
        visuals.selection.stroke = Stroke::new(1.5, palette.accent);
        visuals.slider_trailing_fill = true;
        visuals.warn_fg_color = palette.warn;
        visuals.error_fg_color = palette.bad;

        let radius = CornerRadius::same(12);
        let widgets = &mut visuals.widgets;
        widgets.noninteractive.bg_fill = palette.card;
        widgets.noninteractive.weak_bg_fill = palette.card;
        widgets.noninteractive.bg_stroke = Stroke::new(1.0, palette.outline);
        widgets.noninteractive.fg_stroke = Stroke::new(1.0, palette.text_dim);
        widgets.noninteractive.corner_radius = radius;
        for (visual, fill, stroke) in [
            (&mut widgets.inactive, palette.card, palette.text),
            (&mut widgets.hovered, palette.card_hover, palette.text),
            (&mut widgets.active, palette.accent.gamma_multiply(0.45), palette.text),
            (&mut widgets.open, palette.card_hover, palette.text),
        ] {
            visual.bg_fill = fill;
            visual.weak_bg_fill = fill;
            visual.bg_stroke = Stroke::NONE;
            visual.fg_stroke = Stroke::new(1.5, stroke);
            visual.corner_radius = radius;
            visual.expansion = 0.0;
        }
        widgets.hovered.bg_stroke = Stroke::new(1.0, palette.accent.gamma_multiply(0.6));
    });
}
