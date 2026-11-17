//! All panel UIs. Everything here is platform independent: the VR host and the desktop preview
//! both render these into their own egui contexts.

pub mod chrome;
pub mod desktop;
pub mod icons;
pub mod media;
pub mod notifications;
pub mod settings;
pub mod theme;
pub mod vrcx;
pub mod watch;
pub mod widgets;

use std::time::{Instant, SystemTime};

use chrono::{DateTime, Local};
use egui::{Rect, Vec2, vec2};

use crate::config::Config;
use crate::services::desktop::CaptureTarget;
use crate::state::{Action, HudState, PanelId, PanelKind, Placement};
use theme::Palette;

/// What a panel needs to draw itself and report what the user did.
pub struct UiCtx<'a> {
    pub state: &'a HudState,
    pub config: &'a mut Config,
    /// Set when the UI edited `config` directly (the host saves it after a short delay).
    pub config_changed: bool,
    pub actions: Vec<Action>,
    pub palette: Palette,
    pub now: DateTime<Local>,
    pub system_now: SystemTime,
    pub instant: Instant,
    /// False in the desktop preview; hides controls that only make sense in a headset.
    pub in_vr: bool,
}

impl<'a> UiCtx<'a> {
    pub fn new(state: &'a HudState, config: &'a mut Config, in_vr: bool) -> Self {
        let palette = Palette::new(config.general.accent);
        Self {
            state,
            config,
            config_changed: false,
            actions: Vec::new(),
            palette,
            now: Local::now(),
            system_now: SystemTime::now(),
            instant: Instant::now(),
            in_vr,
        }
    }

    pub fn act(&mut self, action: Action) {
        self.actions.push(action);
    }
}

/// Per-panel window state the chrome displays.
#[derive(Clone, Debug, PartialEq)]
pub struct PanelInfo {
    pub id: PanelId,
    pub kind: PanelKind,
    pub placement: Placement,
    pub locked: bool,
    pub click_through: bool,
    pub opacity: f32,
    pub capture: Option<CaptureTarget>,
}

impl PanelInfo {
    pub fn new(id: PanelId, kind: PanelKind) -> Self {
        Self {
            id,
            kind,
            placement: if kind == PanelKind::Watch { Placement::Wrist } else { Placement::World },
            locked: false,
            click_through: false,
            opacity: 1.0,
            capture: None,
        }
    }
}

/// A captured desktop/window frame for the desktop panel.
#[derive(Clone, Debug, Default)]
pub struct DesktopView {
    pub texture: Option<egui::TextureId>,
    pub size: [u32; 2],
    /// OS cursor position inside the captured area, normalized to 0..1.
    pub cursor: Option<[f32; 2]>,
    /// Shown instead of the image while capture is starting or failed.
    pub status: Option<String>,
    pub virtual_desktops: bool,
}

#[derive(Clone, Debug, Default)]
pub struct PanelOutput {
    /// Where the desktop image was drawn (in points), for routing laser clicks to Windows.
    pub desktop_image_rect: Option<Rect>,
    /// Height the content actually needed, for panels that size to fit.
    pub content_height: f32,
}

pub const WATCH_WIDTH: f32 = 400.0;
pub const TOAST_SIZE: Vec2 = vec2(460.0, 96.0);
/// Vertical space the desktop panel uses around the captured image (title bar, toolbar, margins).
pub const DESKTOP_CHROME_HEIGHT: f32 = 134.0;

/// Logical size of a panel in egui points. The watch sizes to its content, so its height here
/// is only the initial guess.
pub fn panel_size(kind: PanelKind, desktop_aspect: Option<f32>) -> Vec2 {
    match kind {
        PanelKind::Watch => vec2(WATCH_WIDTH, 380.0),
        PanelKind::Media => vec2(600.0, 440.0),
        PanelKind::Desktop => {
            let aspect = desktop_aspect.unwrap_or(16.0 / 9.0).clamp(0.4, 4.0);
            let width = 960.0;
            vec2(width, (width - 32.0) / aspect + DESKTOP_CHROME_HEIGHT)
        }
        PanelKind::Vrcx => vec2(680.0, 580.0),
        PanelKind::Notifications => vec2(500.0, 540.0),
        PanelKind::Settings => vec2(780.0, 620.0),
    }
}

/// Draws one panel filling `ui`.
pub fn draw_panel(ui: &mut egui::Ui, ctx: &mut UiCtx, panel: &PanelInfo, desktop: Option<&DesktopView>) -> PanelOutput {
    let mut output = PanelOutput::default();
    let top = ui.min_rect().min.y;
    match panel.kind {
        PanelKind::Watch => watch::draw(ui, ctx),
        kind => {
            let frame = theme::panel_frame(&ctx.palette);
            frame.show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                chrome::header(ui, ctx, panel);
                ui.add_space(6.0);
                match kind {
                    PanelKind::Media => media::draw(ui, ctx),
                    PanelKind::Desktop => {
                        output.desktop_image_rect = desktop::draw(ui, ctx, panel, desktop);
                    }
                    PanelKind::Vrcx => vrcx::draw(ui, ctx, panel),
                    PanelKind::Notifications => notifications::draw(ui, ctx),
                    PanelKind::Settings => settings::draw(ui, ctx),
                    PanelKind::Watch => unreachable!(),
                }
            });
        }
    }
    output.content_height = ui.min_rect().max.y - top;
    if ctx.in_vr {
        widgets::pointer_marker(ui, &ctx.palette);
    }
    output
}
