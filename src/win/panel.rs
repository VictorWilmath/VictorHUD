//! One SteamVR overlay showing an egui UI.

use std::rc::Rc;
use std::time::{Duration, Instant};

use egui::{Pos2, Rect, Vec2, ViewportId};

use super::d3d::{Gpu, RenderTarget};
use super::desktop::Capture;
use super::painter::{Painter, Pipeline};
use crate::config::Config;
use crate::interaction::placement::Pose;
use crate::state::{Action, HudState};
use crate::ui::{PanelInfo, PanelOutput, UiCtx, theme};
use crate::vr::{OverlayHandle, Vr, sys};

/// Texture pixels per egui point. Panels are seen from ~0.5–1 m, where this keeps text crisp.
pub const PIXELS_PER_POINT: f32 = 1.5;
/// Panels showing a clock or progress bar repaint at least this often while visible.
const MAX_IDLE: Duration = Duration::from_millis(1000);

/// Where an overlay's transform was last set, so unchanged transforms aren't re-sent.
#[derive(Clone, Copy, PartialEq)]
enum AppliedTransform {
    Absolute([[f32; 4]; 3]),
    Device(u32, [[f32; 4]; 3]),
}

pub struct PaintResult {
    pub actions: Vec<Action>,
    pub config_changed: bool,
    pub output: PanelOutput,
}

pub struct VrPanel {
    pub info: PanelInfo,
    pub handle: OverlayHandle,
    ctx: egui::Context,
    painter: Painter,
    target: RenderTarget,
    /// Logical size in egui points.
    pub size: Vec2,
    pub width_m: f32,
    /// Pose relative to the panel's anchor (room, head or wrist, by `info.placement`).
    pub pose: Pose,
    pub output: PanelOutput,
    pub capture: Option<Capture>,
    registered_capture: Option<u64>,
    /// egui wants text input (a text field is focused).
    pub wants_keyboard: bool,
    events: Vec<egui::Event>,
    pointer: Option<Pos2>,
    dirty: bool,
    repaint_at: Option<Instant>,
    accent: Option<[u8; 3]>,
    started: Instant,
    shown: bool,
    alpha: f32,
    width_applied: f32,
    transform: Option<AppliedTransform>,
}

fn pixel_size(size: Vec2) -> [u32; 2] {
    [(size.x * PIXELS_PER_POINT).round().max(1.0) as u32, (size.y * PIXELS_PER_POINT).round().max(1.0) as u32]
}

impl VrPanel {
    pub fn create(
        vr: &Vr,
        gpu: &Gpu,
        pipeline: Rc<Pipeline>,
        key: &str,
        name: &str,
        info: PanelInfo,
        size: Vec2,
    ) -> anyhow::Result<Self> {
        let handle = vr.create_overlay(key, name)?;
        Self::with_handle(vr, gpu, pipeline, handle, info, size)
    }

    pub fn with_handle(
        vr: &Vr,
        gpu: &Gpu,
        pipeline: Rc<Pipeline>,
        handle: OverlayHandle,
        info: PanelInfo,
        size: Vec2,
    ) -> anyhow::Result<Self> {
        vr.set_overlay_flag(handle, sys::VROverlayFlags_IsPremultiplied, true);
        let [w, h] = pixel_size(size);
        let target = RenderTarget::new(gpu, w, h)?;
        Ok(Self {
            info,
            handle,
            ctx: egui::Context::default(),
            painter: Painter::new(pipeline),
            target,
            size,
            width_m: 0.4,
            pose: Pose::IDENTITY,
            output: PanelOutput::default(),
            capture: None,
            registered_capture: None,
            wants_keyboard: false,
            events: Vec::new(),
            pointer: None,
            dirty: true,
            repaint_at: None,
            accent: None,
            started: Instant::now(),
            shown: false,
            alpha: -1.0,
            width_applied: -1.0,
            transform: None,
        })
    }

    /// Physical height in meters (the overlay keeps the texture's aspect ratio).
    pub fn height_m(&self) -> f32 {
        self.width_m * self.size.y / self.size.x
    }

    pub fn resize(&mut self, gpu: &Gpu, size: Vec2) -> anyhow::Result<()> {
        if (size - self.size).length() < 0.5 {
            return Ok(());
        }
        let pixels = pixel_size(size);
        if pixels != self.target.size {
            self.target = RenderTarget::new(gpu, pixels[0], pixels[1])?;
        }
        self.size = size;
        self.dirty = true;
        Ok(())
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn push_event(&mut self, event: egui::Event) {
        self.events.push(event);
        self.dirty = true;
    }

    /// Moves the egui pointer to `uv` (0..1 across the panel).
    pub fn pointer_at_uv(&mut self, uv: glam::Vec2) -> Pos2 {
        let pos = Pos2::new(uv.x * self.size.x, uv.y * self.size.y);
        if self.pointer.is_none_or(|old| (old - pos).length() > 0.25) {
            self.pointer = Some(pos);
            self.push_event(egui::Event::PointerMoved(pos));
        }
        pos
    }

    pub fn pointer(&self) -> Option<Pos2> {
        self.pointer
    }

    pub fn pointer_gone(&mut self) {
        if self.pointer.take().is_some() {
            self.push_event(egui::Event::PointerGone);
        }
    }

    pub fn button(&mut self, button: egui::PointerButton, pressed: bool) {
        let pos = self.pointer.unwrap_or(Pos2::new(-10.0, -10.0));
        self.push_event(egui::Event::PointerButton { pos, button, pressed, modifiers: egui::Modifiers::NONE });
    }

    pub fn scroll(&mut self, delta: Vec2) {
        if delta != Vec2::ZERO {
            self.push_event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta,
                phase: egui::TouchPhase::Move,
                source: egui::MouseWheelSource::Unknown,
                modifiers: egui::Modifiers::NONE,
            });
        }
    }

    /// Feeds text from the SteamVR keyboard into the focused text field.
    pub fn type_text(&mut self, text: &str) {
        for c in text.chars() {
            let key = match c {
                '\u{8}' => Some(egui::Key::Backspace),
                '\n' | '\r' => Some(egui::Key::Enter),
                '\t' => Some(egui::Key::Tab),
                '\u{1b}' => Some(egui::Key::Escape),
                _ => None,
            };
            match key {
                Some(key) => {
                    for pressed in [true, false] {
                        self.push_event(egui::Event::Key {
                            key,
                            physical_key: None,
                            pressed,
                            repeat: false,
                            modifiers: egui::Modifiers::NONE,
                        });
                    }
                }
                None => self.push_event(egui::Event::Text(c.to_string())),
            }
        }
    }

    pub fn is_shown(&self) -> bool {
        self.shown && self.alpha > 0.01
    }

    /// Shows or hides the overlay and sets its opacity and width, skipping unchanged values.
    pub fn apply_visibility(&mut self, vr: &Vr, alpha: f32) {
        let visible = alpha > 0.01;
        if (alpha - self.alpha).abs() > 0.002 {
            vr.set_overlay_alpha(self.handle, alpha);
            self.alpha = alpha;
        }
        if (self.width_m - self.width_applied).abs() > 0.0005 {
            vr.set_overlay_width(self.handle, self.width_m);
            self.width_applied = self.width_m;
        }
        if visible != self.shown {
            if visible {
                vr.show_overlay(self.handle);
                self.dirty = true;
            } else {
                vr.hide_overlay(self.handle);
                self.pointer_gone();
            }
            self.shown = visible;
        }
    }

    pub fn place_absolute(&mut self, vr: &Vr, world: Pose) {
        let rows = world.to_rows();
        if self.transform != Some(AppliedTransform::Absolute(rows)) {
            vr.set_overlay_transform_absolute(self.handle, world);
            self.transform = Some(AppliedTransform::Absolute(rows));
        }
    }

    pub fn place_on_device(&mut self, vr: &Vr, device: u32, relative: Pose) {
        let rows = relative.to_rows();
        if self.transform != Some(AppliedTransform::Device(device, rows)) {
            vr.set_overlay_transform_device(self.handle, device, relative);
            self.transform = Some(AppliedTransform::Device(device, rows));
        }
    }

    pub fn needs_paint(&self, now: Instant) -> bool {
        self.dirty || self.repaint_at.is_some_and(|at| now >= at)
    }

    /// Shows the current capture frame as `TextureId::User(id)`, re-registering it when the
    /// capture replaced its texture. Returns the texture id if there is a frame.
    pub fn sync_capture_texture(&mut self) -> Option<egui::TextureId> {
        let id = self.info.id.0 as u64;
        let Some(capture) = &self.capture else {
            if self.registered_capture.take().is_some() {
                self.painter.remove_native_texture(id);
            }
            return None;
        };
        match capture.view() {
            Some((view, _)) => {
                if self.registered_capture != Some(capture.generation) {
                    self.painter.set_native_texture(id, view.clone(), true);
                    self.registered_capture = Some(capture.generation);
                }
                Some(egui::TextureId::User(id))
            }
            None => {
                if self.registered_capture.take().is_some() {
                    self.painter.remove_native_texture(id);
                }
                None
            }
        }
    }

    /// Runs the UI and hands the new image to SteamVR.
    pub fn paint<F>(
        &mut self,
        vr: &Vr,
        gpu: &Gpu,
        state: &HudState,
        config: &mut Config,
        mut draw: F,
    ) -> anyhow::Result<PaintResult>
    where
        F: FnMut(&mut egui::Ui, &mut UiCtx) -> PanelOutput,
    {
        if self.accent != Some(config.general.accent) {
            theme::apply(&self.ctx, &theme::Palette::new(config.general.accent));
            self.accent = Some(config.general.accent);
        }
        let mut raw = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, self.size)),
            max_texture_side: Some(8192),
            time: Some(self.started.elapsed().as_secs_f64()),
            focused: true,
            events: std::mem::take(&mut self.events),
            ..Default::default()
        };
        raw.viewports.entry(ViewportId::ROOT).or_default().native_pixels_per_point = Some(PIXELS_PER_POINT);

        let mut ui_ctx = UiCtx::new(state, config, true);
        let mut output = PanelOutput::default();
        let mut full = self.ctx.run_ui(raw, |ui| {
            // egui may run several passes per frame; only the last one's actions count.
            ui_ctx.actions.clear();
            output = draw(ui, &mut ui_ctx);
        });
        let actions = std::mem::take(&mut ui_ctx.actions);
        let config_changed = ui_ctx.config_changed;
        drop(ui_ctx);

        let primitives = self.ctx.tessellate(full.shapes, full.pixels_per_point);
        let painted = self
            .painter
            .update_textures(&mut full.textures_delta)
            .and_then(|()| self.painter.paint(&self.target, full.pixels_per_point, &primitives));
        self.painter.free_textures(&mut full.textures_delta);
        painted?;
        unsafe { gpu.context.Flush() };
        vr.set_overlay_texture(self.handle, self.target.raw())?;

        let now = Instant::now();
        let delay = full
            .viewport_output
            .get(&ViewportId::ROOT)
            .map(|viewport| viewport.repaint_delay)
            .unwrap_or(Duration::MAX)
            .min(MAX_IDLE);
        self.repaint_at = Some(now + delay);
        self.dirty = false;

        let platform = full.platform_output;
        self.wants_keyboard = platform.ime.is_some();
        for command in platform.commands {
            match command {
                egui::OutputCommand::CopyText(text) => {
                    if let Err(err) = super::set_clipboard(&text) {
                        log::warn!("copying to the clipboard: {err:#}");
                    }
                }
                egui::OutputCommand::OpenUrl(open) => crate::util::open_url(&open.url),
                _ => {}
            }
        }
        self.output = output.clone();
        Ok(PaintResult { actions, config_changed, output })
    }

    pub fn destroy(&self, vr: &Vr) {
        vr.destroy_overlay(self.handle);
    }
}
