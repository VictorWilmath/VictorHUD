//! The SteamVR host: overlays, controller input and the main loop.

use std::rc::Rc;
use std::time::{Duration, Instant};

use anyhow::Context as _;
use egui::{Pos2, vec2};
use glam::{Affine3A, Mat3, Quat, Vec2, Vec3};

use super::d3d::Gpu;
use super::desktop::{self, Capture};
use super::input::{self, MouseButton};
use super::manifest;
use super::painter::Pipeline;
use super::panel::{PaintResult, VrPanel};
use super::session::SessionTracker;
use crate::config::{Config, SavedLayout, ToastPlacement};
use crate::hud::Hud;
use crate::interaction::gesture::{GestureThresholds, WatchGesture, is_watch_raised};
use crate::interaction::placement::{Pose, ray_plane_hit, ray_quad_hit};
use crate::interaction::pointer::{ScrollAccumulator, panel_point_to_screen};
use crate::services::DataMode;
use crate::services::desktop::CaptureTarget;
use crate::state::{Action, Hand, Notification, PanelAction, PanelId, PanelKind, Placement};
use crate::ui::{self, DesktopView, PanelInfo, PanelOutput};
use crate::vr::{self, DeviceIndex, OverlayHandle, Vr, sys};

const ACTIVE_FRAME: Duration = Duration::from_micros(11_111);
const IDLE_FRAME: Duration = Duration::from_micros(22_222);
const LASER_THICKNESS: f32 = 0.004;
const TOAST_WIDTH_M: f32 = 0.3;
const DASHBOARD_KEY: &str = "victorwilmath.victorhud.dashboard";
const DASHBOARD_SIZE: egui::Vec2 = vec2(780.0, 620.0);
const MIN_PANEL_WIDTH: f32 = 0.12;
const MAX_PANEL_WIDTH: f32 = 5.0;

pub fn run() -> anyhow::Result<()> {
    let vr = Vr::init().context("connecting to SteamVR")?;
    let mut app = App::new(vr)?;
    log::info!("overlay running");
    while !app.quit {
        let started = Instant::now();
        app.frame();
        let budget = if app.active { ACTIVE_FRAME } else { IDLE_FRAME };
        if let Some(rest) = budget.checked_sub(started.elapsed()) {
            std::thread::sleep(rest);
        }
    }
    app.shutdown();
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Target {
    Watch,
    Toast,
    Panel(PanelId),
}

#[derive(Clone, Copy, Debug)]
struct Hover {
    target: Target,
    uv: Vec2,
    distance: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Button {
    Primary,
    Secondary,
}

impl Button {
    fn egui(self) -> egui::PointerButton {
        match self {
            Button::Primary => egui::PointerButton::Primary,
            Button::Secondary => egui::PointerButton::Secondary,
        }
    }

    fn mouse(self) -> MouseButton {
        match self {
            Button::Primary => MouseButton::Left,
            Button::Secondary => MouseButton::Right,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Press {
    target: Target,
    button: Button,
    /// Pressed on a desktop image: the button is held down in Windows, at this screen point.
    desktop: Option<(i32, i32)>,
}

#[derive(Clone, Copy, Debug)]
struct Grab {
    target: Target,
    /// The grabbed target's pose in this hand's pointer space.
    offset: Pose,
    /// Set on the first hand; the second hand on the same panel only resizes it.
    primary: bool,
    scale: Option<(f32, f32)>,
}

#[derive(Default)]
struct HandState {
    device: Option<DeviceIndex>,
    raw: Option<Pose>,
    tip_in_raw: Option<Pose>,
    hover: Option<Hover>,
    press: Option<Press>,
    grab: Option<Grab>,
    scroll: ScrollAccumulator,
    cursor: Option<(i32, i32)>,
}

impl HandState {
    fn tip(&self) -> Option<Pose> {
        Some(self.raw?.mul(self.tip_in_raw?))
    }
}

struct Laser {
    overlays: [OverlayHandle; 2],
    shown: bool,
    style: Option<([u8; 3], bool)>,
}

struct Actions {
    set: sys::VRActionSetHandle_t,
    pointer: sys::VRActionHandle_t,
    click: sys::VRActionHandle_t,
    right_click: sys::VRActionHandle_t,
    grab: sys::VRActionHandle_t,
    scroll: sys::VRActionHandle_t,
    toggle_watch: sys::VRActionHandle_t,
    haptic: sys::VRActionHandle_t,
    hands: [sys::VRInputValueHandle_t; 2],
}

impl Actions {
    fn load(vr: &Vr) -> anyhow::Result<Self> {
        Ok(Self {
            set: vr.action_set("/actions/main")?,
            pointer: vr.action("/actions/main/in/pointer")?,
            click: vr.action("/actions/main/in/click")?,
            right_click: vr.action("/actions/main/in/right_click")?,
            grab: vr.action("/actions/main/in/grab")?,
            scroll: vr.action("/actions/main/in/scroll")?,
            toggle_watch: vr.action("/actions/main/in/toggle_watch")?,
            haptic: vr.action("/actions/main/out/haptic")?,
            hands: [vr.input_source("/user/hand/left")?, vr.input_source("/user/hand/right")?],
        })
    }
}

/// One frame's controller input for a hand.
#[derive(Default, Clone, Copy)]
struct HandInput {
    click: Edge,
    right: Edge,
    grab: Edge,
    toggle: Edge,
    stick: Vec2,
}

#[derive(Default, Clone, Copy)]
struct Edge {
    down: bool,
    pressed: bool,
    released: bool,
}

impl Edge {
    fn from(data: sys::InputDigitalActionData_t) -> Self {
        let down = data.bActive && data.bState;
        let changed = data.bActive && data.bChanged;
        Self { down, pressed: changed && down, released: changed && !down }
    }
}

struct Dashboard {
    panel: VrPanel,
    thumbnail: OverlayHandle,
    visible: bool,
}

struct App {
    vr: Vr,
    gpu: Gpu,
    pipeline: Rc<Pipeline>,
    hud: Hud,
    actions: Option<Actions>,
    watch: VrPanel,
    toast: VrPanel,
    toast_note: Option<Notification>,
    toast_world: Option<Pose>,
    dashboard: Option<Dashboard>,
    panels: Vec<VrPanel>,
    next_id: u32,
    hands: [HandState; 2],
    lasers: [Laser; 2],
    head: Option<Pose>,
    gesture: WatchGesture,
    /// Wrist visibility (0..1) shared by the watch and panels placed on the wrist.
    wrist_alpha: f32,
    watch_toggled: bool,
    session: SessionTracker,
    keyboard: Option<Target>,
    manifest: Option<manifest::Paths>,
    applied_auto_launch: Option<bool>,
    next_refresh: Instant,
    last_frame: Instant,
    active: bool,
    quit: bool,
}

fn device_pose(poses: &[sys::TrackedDevicePose_t], device: DeviceIndex) -> Option<Pose> {
    let pose = poses.get(device as usize)?;
    (pose.bPoseIsValid && pose.bDeviceIsConnected).then(|| Pose::from_rows(pose.mDeviceToAbsoluteTracking.m))
}

fn index_of(hand: Hand) -> usize {
    match hand {
        Hand::Left => 0,
        Hand::Right => 1,
    }
}

impl App {
    fn new(vr: Vr) -> anyhow::Result<Self> {
        let gpu = Gpu::new(vr.dxgi_adapter_index())?;
        let pipeline = Pipeline::new(&gpu)?;
        let mut hud = Hud::new(Config::path(), DataMode::Live);

        let manifest = match manifest::write() {
            Ok(paths) => Some(paths),
            Err(err) => {
                log::warn!("writing SteamVR manifests: {err:#}");
                hud.state.add_warning("Couldn't write the SteamVR manifests; controller bindings may be missing");
                None
            }
        };
        let mut applied_auto_launch = None;
        let mut actions = None;
        if let Some(paths) = &manifest {
            match manifest::register(&vr.apps, paths, hud.config.general.start_with_steamvr) {
                Ok(()) => applied_auto_launch = Some(hud.config.general.start_with_steamvr),
                Err(err) => log::warn!("registering with SteamVR: {err:#}"),
            }
            if let Err(err) = vr.apps.identify_app(manifest::APP_KEY) {
                log::debug!("{err:#}");
            }
            match vr.set_action_manifest(&paths.actions).and_then(|()| Actions::load(&vr)) {
                Ok(loaded) => actions = Some(loaded),
                Err(err) => {
                    log::error!("controller input unavailable: {err:#}");
                    hud.state.add_warning("Controller input isn't available; the watch can't be clicked");
                }
            }
        }

        let watch = VrPanel::create(
            &vr,
            &gpu,
            pipeline.clone(),
            "victorwilmath.victorhud.watch",
            "VictorHUD watch",
            PanelInfo::new(PanelId::WATCH, PanelKind::Watch),
            ui::panel_size(PanelKind::Watch, None),
        )?;
        vr.set_overlay_sort_order(watch.handle, 10);
        let mut toast = VrPanel::create(
            &vr,
            &gpu,
            pipeline.clone(),
            "victorwilmath.victorhud.toast",
            "VictorHUD notification",
            PanelInfo::new(PanelId(u32::MAX), PanelKind::Notifications),
            ui::TOAST_SIZE,
        )?;
        toast.width_m = TOAST_WIDTH_M;
        vr.set_overlay_sort_order(toast.handle, 15);

        let lasers = [Hand::Left, Hand::Right].map(|hand| {
            let make = |n: usize| {
                let key = format!("victorwilmath.victorhud.laser.{}.{n}", hand.label().to_lowercase());
                vr.create_overlay(&key, "VictorHUD laser").unwrap_or(0)
            };
            let overlays = [make(0), make(1)];
            for overlay in overlays.into_iter().filter(|&o| o != 0) {
                vr.set_overlay_width(overlay, LASER_THICKNESS);
                vr.set_overlay_sort_order(overlay, 20);
            }
            Laser { overlays, shown: false, style: None }
        });

        let dashboard = match Self::create_dashboard(&vr, &gpu, &pipeline, hud.config.general.accent) {
            Ok(dashboard) => Some(dashboard),
            Err(err) => {
                log::warn!("SteamVR dashboard tab: {err:#}");
                None
            }
        };

        hud.state.monitors = desktop::monitors();
        hud.state.windows = desktop::windows();

        Ok(Self {
            vr,
            gpu,
            pipeline,
            hud,
            actions,
            watch,
            toast,
            toast_note: None,
            toast_world: None,
            dashboard,
            panels: Vec::new(),
            next_id: 1,
            hands: Default::default(),
            lasers,
            head: None,
            gesture: WatchGesture::default(),
            wrist_alpha: 0.0,
            watch_toggled: false,
            session: SessionTracker::new(),
            keyboard: None,
            manifest,
            applied_auto_launch,
            next_refresh: Instant::now() + Duration::from_secs(10),
            last_frame: Instant::now(),
            active: false,
            quit: false,
        })
    }

    fn create_dashboard(vr: &Vr, gpu: &Gpu, pipeline: &Rc<Pipeline>, accent: [u8; 3]) -> anyhow::Result<Dashboard> {
        let (main, thumbnail) = vr.create_dashboard_overlay(DASHBOARD_KEY, "VictorHUD")?;
        let mut panel = VrPanel::with_handle(
            vr,
            gpu,
            pipeline.clone(),
            main,
            PanelInfo::new(PanelId(u32::MAX - 1), PanelKind::Settings),
            DASHBOARD_SIZE,
        )?;
        panel.width_m = 2.4;
        vr.set_overlay_width(main, 2.4);
        vr.enable_mouse_input(main, DASHBOARD_SIZE.x, DASHBOARD_SIZE.y);
        let mut icon = icon_pixels(accent, 128);
        vr.set_overlay_raw(thumbnail, &mut icon, 128, 128)?;
        Ok(Dashboard { panel, thumbnail, visible: false })
    }

    fn shutdown(&mut self) {
        for panel in &self.panels {
            panel.destroy(&self.vr);
        }
        self.watch.destroy(&self.vr);
        self.toast.destroy(&self.vr);
        for laser in &self.lasers {
            for &overlay in &laser.overlays {
                if overlay != 0 {
                    self.vr.destroy_overlay(overlay);
                }
            }
        }
        if let Some(dashboard) = &self.dashboard {
            self.vr.destroy_overlay(dashboard.thumbnail);
            dashboard.panel.destroy(&self.vr);
        }
        self.hud.save_now();
    }

    fn frame(&mut self) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32().min(0.1);
        self.last_frame = now;

        self.poll_events();
        if self.hud.update() {
            self.mark_all_dirty();
        }
        self.apply_settings();
        self.refresh_system_info(now);
        self.update_tracking();
        self.update_watch(dt);
        self.update_hands(dt);
        self.update_panels();
        self.update_toast();
        self.update_lasers();
        self.render(now);
        self.active = self.hands.iter().any(|h| h.hover.is_some() || h.press.is_some() || h.grab.is_some());
    }

    fn mark_all_dirty(&mut self) {
        self.watch.mark_dirty();
        for panel in &mut self.panels {
            panel.mark_dirty();
        }
        if let Some(dashboard) = &mut self.dashboard {
            dashboard.panel.mark_dirty();
        }
    }

    // ---- events ----

    fn poll_events(&mut self) {
        while let Some(event) = self.vr.poll_event() {
            match event.eventType {
                t if t == sys::EVREventType_VREvent_Quit => {
                    log::info!("SteamVR is shutting down");
                    self.quit = true;
                }
                t if t == sys::EVREventType_VREvent_SceneApplicationChanged => self.session.refresh_soon(),
                t if t == sys::EVREventType_VREvent_TrackedDeviceRoleChanged
                    || t == sys::EVREventType_VREvent_TrackedDeviceActivated =>
                {
                    for hand in &mut self.hands {
                        hand.tip_in_raw = None;
                    }
                }
                _ => {}
            }
        }

        let mut targets = vec![(Target::Watch, self.watch.handle)];
        targets.extend(self.panels.iter().map(|p| (Target::Panel(p.info.id), p.handle)));
        for (target, handle) in targets {
            while let Some(event) = self.vr.poll_overlay_event(handle) {
                self.keyboard_event(target, &event);
            }
        }
        self.poll_dashboard_events();
    }

    fn keyboard_event(&mut self, target: Target, event: &sys::VREvent_t) {
        let kind = event.eventType;
        if kind == sys::EVREventType_VREvent_KeyboardCharInput {
            let text = vr::keyboard_text(event);
            self.type_into(target, &text);
        } else if kind == sys::EVREventType_VREvent_KeyboardClosed
            || kind == sys::EVREventType_VREvent_KeyboardClosed_Global
            || kind == sys::EVREventType_VREvent_KeyboardDone
        {
            self.keyboard = None;
        }
    }

    fn type_into(&mut self, target: Target, text: &str) {
        match target {
            Target::Panel(id) => {
                let Some(panel) = self.panels.iter_mut().find(|p| p.info.id == id) else { return };
                if panel.info.kind == PanelKind::Desktop {
                    if let Some(CaptureTarget::Window(hwnd)) = panel.info.capture {
                        desktop::focus_window(hwnd);
                    }
                    input::type_text(text);
                } else {
                    panel.type_text(text);
                }
            }
            Target::Watch => self.watch.type_text(text),
            Target::Toast => {}
        }
    }

    fn poll_dashboard_events(&mut self) {
        let Some(dashboard) = &mut self.dashboard else { return };
        while let Some(event) = self.vr.poll_overlay_event(dashboard.panel.handle) {
            let kind = event.eventType;
            let panel = &mut dashboard.panel;
            if kind == sys::EVREventType_VREvent_OverlayShown {
                dashboard.visible = true;
                panel.mark_dirty();
            } else if kind == sys::EVREventType_VREvent_OverlayHidden {
                dashboard.visible = false;
                panel.pointer_gone();
            } else if kind == sys::EVREventType_VREvent_MouseMove {
                let mouse = unsafe { event.data.mouse };
                let uv = Vec2::new(mouse.x / DASHBOARD_SIZE.x, 1.0 - mouse.y / DASHBOARD_SIZE.y);
                panel.pointer_at_uv(uv);
            } else if kind == sys::EVREventType_VREvent_MouseButtonDown
                || kind == sys::EVREventType_VREvent_MouseButtonUp
            {
                let mouse = unsafe { event.data.mouse };
                let uv = Vec2::new(mouse.x / DASHBOARD_SIZE.x, 1.0 - mouse.y / DASHBOARD_SIZE.y);
                panel.pointer_at_uv(uv);
                let button = match mouse.button {
                    b if b == sys::EVRMouseButton_VRMouseButton_Right => egui::PointerButton::Secondary,
                    b if b == sys::EVRMouseButton_VRMouseButton_Middle => egui::PointerButton::Middle,
                    _ => egui::PointerButton::Primary,
                };
                panel.button(button, kind == sys::EVREventType_VREvent_MouseButtonDown);
            } else if kind == sys::EVREventType_VREvent_ScrollSmooth || kind == sys::EVREventType_VREvent_ScrollDiscrete
            {
                let scroll = unsafe { event.data.scroll };
                panel.scroll(vec2(scroll.xdelta * 60.0, scroll.ydelta * 60.0));
            } else if kind == sys::EVREventType_VREvent_KeyboardCharInput {
                panel.type_text(&vr::keyboard_text(&event));
            }
        }
    }

    // ---- settings and system info ----

    fn apply_settings(&mut self) {
        let auto_launch = self.hud.config.general.start_with_steamvr;
        if self.applied_auto_launch != Some(auto_launch)
            && let Some(paths) = &self.manifest
        {
            match manifest::register(&self.vr.apps, paths, auto_launch) {
                Ok(()) => log::info!("start with SteamVR: {auto_launch}"),
                Err(err) => log::warn!("changing auto-launch: {err:#}"),
            }
            self.applied_auto_launch = Some(auto_launch);
        }
    }

    fn refresh_system_info(&mut self, now: Instant) {
        let (session, batteries) = self.session.update(&self.vr);
        if let Some(session) = session {
            self.hud.state.session = session;
            self.watch.mark_dirty();
        }
        if let Some(batteries) = batteries
            && batteries != self.hud.state.batteries
        {
            self.hud.state.batteries = batteries;
            self.watch.mark_dirty();
        }
        if now >= self.next_refresh {
            self.next_refresh = now + Duration::from_secs(10);
            let monitors = desktop::monitors();
            if monitors != self.hud.state.monitors {
                self.hud.state.monitors = monitors;
                self.mark_all_dirty();
            }
        }
    }

    // ---- tracking ----

    fn update_tracking(&mut self) {
        let poses = self.vr.device_poses(0.0);
        self.head = device_pose(&poses, vr::HMD);
        for index in 0..2 {
            let role = if index == 0 {
                sys::ETrackedControllerRole_TrackedControllerRole_LeftHand
            } else {
                sys::ETrackedControllerRole_TrackedControllerRole_RightHand
            };
            let device = self.vr.controller_for_role(role);
            let raw = device.and_then(|device| device_pose(&poses, device));
            let tip = match (&self.actions, raw) {
                (Some(actions), Some(_)) => self.vr.pose_at(actions.pointer, actions.hands[index], 0.0),
                _ => None,
            };
            let hand = &mut self.hands[index];
            hand.device = device;
            hand.raw = raw;
            if let (Some(raw), Some(tip)) = (raw, tip) {
                hand.tip_in_raw = Some(raw.relative(tip));
            } else if hand.tip_in_raw.is_none() && raw.is_some() && self.actions.is_none() {
                hand.tip_in_raw = Some(Pose::IDENTITY);
            }
        }
    }

    fn wrist_index(&self) -> usize {
        index_of(self.hud.config.watch.hand)
    }

    fn wrist_tip(&self) -> Option<Pose> {
        self.hands[self.wrist_index()].tip()
    }

    fn anchor(&self, placement: Placement) -> Option<Pose> {
        match placement {
            Placement::World => Some(Pose::IDENTITY),
            Placement::Head => self.head,
            Placement::Wrist => self.wrist_tip(),
        }
    }

    /// The hand moving `target`, if any, and where it currently holds it.
    fn primary_grab(&self, target: Target) -> Option<(usize, Pose)> {
        self.hands.iter().enumerate().find_map(|(index, hand)| {
            let grab = hand.grab.filter(|g| g.primary && g.target == target)?;
            Some((index, hand.tip()?.mul(grab.offset)))
        })
    }

    fn panel(&self, id: PanelId) -> Option<&VrPanel> {
        self.panels.iter().find(|p| p.info.id == id)
    }

    fn panel_mut(&mut self, id: PanelId) -> Option<&mut VrPanel> {
        self.panels.iter_mut().find(|p| p.info.id == id)
    }

    fn vr_panel_mut(&mut self, target: Target) -> Option<&mut VrPanel> {
        match target {
            Target::Watch => Some(&mut self.watch),
            Target::Toast => Some(&mut self.toast),
            Target::Panel(id) => self.panel_mut(id),
        }
    }

    fn watch_world(&self) -> Option<Pose> {
        if let Some((_, world)) = self.primary_grab(Target::Watch) {
            return Some(world);
        }
        Some(self.wrist_tip()?.mul(self.hud.config.watch.offset(self.hud.config.watch.hand)))
    }

    fn target_world(&self, target: Target) -> Option<Pose> {
        if let Some((_, world)) = self.primary_grab(target) {
            return Some(world);
        }
        match target {
            Target::Watch => self.watch_world(),
            Target::Toast => self.toast_world,
            Target::Panel(id) => {
                let panel = self.panel(id)?;
                Some(self.anchor(panel.info.placement)?.mul(panel.pose))
            }
        }
    }

    /// Everything a laser can hit this frame: (target, world pose, width, height).
    fn hit_quads(&self) -> Vec<(Target, Pose, f32, f32)> {
        let mut quads = Vec::new();
        if self.watch.is_shown()
            && self.wrist_alpha > 0.5
            && let Some(world) = self.watch_world()
        {
            quads.push((Target::Watch, world, self.watch.width_m, self.watch.height_m()));
        }
        if self.toast.is_shown()
            && let Some(world) = self.toast_world
        {
            quads.push((Target::Toast, world, self.toast.width_m, self.toast.height_m()));
        }
        for panel in &self.panels {
            if !panel.is_shown() || panel.info.click_through {
                continue;
            }
            let target = Target::Panel(panel.info.id);
            if let Some(world) = self.target_world(target) {
                quads.push((target, world, panel.width_m, panel.height_m()));
            }
        }
        quads
    }

    /// Whether `target` rides on `hand`'s wrist (so that hand's own laser shouldn't hit it).
    fn attached_to(&self, target: Target, hand: usize) -> bool {
        if hand != self.wrist_index() {
            return false;
        }
        match target {
            Target::Watch => self.primary_grab(Target::Watch).is_none(),
            Target::Toast => self.hud.config.notifications.placement == ToastPlacement::Wrist,
            Target::Panel(id) => self.panel(id).is_some_and(|p| p.info.placement == Placement::Wrist),
        }
    }

    // ---- watch ----

    fn update_watch(&mut self, dt: f32) {
        let config = &self.hud.config;
        let wrist = self.wrist_index();
        let offset = config.watch.offset(config.watch.hand);
        let wrist_tip = self.wrist_tip();
        let raised = match (self.head, wrist_tip) {
            (Some(head), Some(tip)) => {
                is_watch_raised(GestureThresholds::from(&config.watch), head, tip.mul(offset), self.gesture.shown())
            }
            _ => false,
        };
        let interacting = self.hands.iter().any(|h| {
            h.hover.is_some_and(|hover| hover.target == Target::Watch)
                || h.press.is_some_and(|p| p.target == Target::Watch)
                || h.grab.is_some_and(|g| g.target == Target::Watch)
        });
        let forced =
            config.watch.keep_on_wrist || self.watch_toggled || interacting || self.keyboard == Some(Target::Watch);
        let alpha = self.gesture.update(raised, forced, dt, config.watch.show_delay, config.watch.hide_delay);
        self.wrist_alpha = if wrist_tip.is_some() { alpha } else { 0.0 };
        if alpha == 0.0 && self.hud.state.watch_expanded {
            self.hud.state.watch_expanded = false;
            self.watch.mark_dirty();
        }

        self.watch.width_m = config.watch.width_m * config.general.ui_scale;
        let opacity = config.watch.opacity;
        if let Some((index, _)) = self.primary_grab(Target::Watch) {
            let hand = &self.hands[index];
            let grab = hand.grab.expect("primary_grab found a grab");
            if let (Some(device), Some(tip_in_raw)) = (hand.device, hand.tip_in_raw) {
                self.watch.place_on_device(&self.vr, device, tip_in_raw.mul(grab.offset));
            }
        } else if let (Some(device), Some(tip_in_raw)) = (self.hands[wrist].device, self.hands[wrist].tip_in_raw) {
            self.watch.place_on_device(&self.vr, device, tip_in_raw.mul(offset));
        }
        self.watch.apply_visibility(&self.vr, self.wrist_alpha * opacity);
    }

    // ---- hands ----

    fn read_input(&self, index: usize) -> HandInput {
        let Some(actions) = &self.actions else { return HandInput::default() };
        let source = actions.hands[index];
        let stick = self.vr.analog(actions.scroll, source);
        HandInput {
            click: Edge::from(self.vr.digital(actions.click, source)),
            right: Edge::from(self.vr.digital(actions.right_click, source)),
            grab: Edge::from(self.vr.digital(actions.grab, source)),
            toggle: Edge::from(self.vr.digital(actions.toggle_watch, source)),
            stick: if stick.bActive { Vec2::new(stick.x, stick.y) } else { Vec2::ZERO },
        }
    }

    fn haptic(&self, index: usize, strength: f32) {
        let config = &self.hud.config.interaction;
        if !config.haptics {
            return;
        }
        if let Some(actions) = &self.actions {
            self.vr.haptic(actions.haptic, actions.hands[index], 0.012, 160.0, config.haptic_strength * strength);
        }
    }

    fn update_hands(&mut self, dt: f32) {
        if let Some(actions) = &self.actions {
            let exclusive: Vec<_> = (0..2)
                .map(|i| {
                    let hand = &self.hands[i];
                    (actions.hands[i], hand.hover.is_some() || hand.press.is_some() || hand.grab.is_some())
                })
                .collect();
            self.vr.update_actions(actions.set, &exclusive);
        }
        let inputs = [self.read_input(0), self.read_input(1)];
        let quads = self.hit_quads();
        let max_length = self.hud.config.interaction.laser_max_length;

        // Hover: nearest panel along each laser.
        for index in 0..2 {
            if inputs[index].toggle.pressed {
                self.watch_toggled = !self.watch_toggled;
            }
            let Some(tip) = self.hands[index].tip() else {
                self.hands[index].hover = None;
                continue;
            };
            let grabbing = self.hands[index].grab.map(|g| g.target);
            let hover = quads
                .iter()
                .filter(|(target, ..)| Some(*target) != grabbing && !self.attached_to(*target, index))
                .filter_map(|&(target, pose, w, h)| {
                    let hit = ray_quad_hit(tip.translation, tip.forward(), pose, w, h)?;
                    (hit.distance <= max_length).then_some(Hover { target, uv: hit.uv, distance: hit.distance })
                })
                .min_by(|a, b| a.distance.total_cmp(&b.distance));
            let previous = self.hands[index].hover.map(|h| h.target);
            self.hands[index].hover =
                if grabbing.is_some() && self.hands[index].grab.is_some_and(|g| g.primary) { None } else { hover };
            let now = self.hands[index].hover.map(|h| h.target);
            if now.is_some() && now != previous {
                self.haptic(index, 0.4);
            }
        }

        self.update_grabs(&inputs, dt);
        self.update_pointers(&inputs);
        for (index, input) in inputs.iter().enumerate() {
            self.update_buttons(index, input);
            self.update_scroll(index, input, dt);
        }
    }

    fn update_grabs(&mut self, inputs: &[HandInput; 2], dt: f32) {
        for index in 0..2 {
            let other = 1 - index;
            let input = inputs[index];
            let Some(tip) = self.hands[index].tip() else { continue };

            if input.grab.pressed
                && self.hands[index].grab.is_none()
                && self.hands[index].press.is_none()
                && let Some(hover) = self.hands[index].hover
            {
                let movable = match hover.target {
                    Target::Watch => true,
                    Target::Toast => false,
                    Target::Panel(id) => self.panel(id).is_some_and(|p| !p.info.locked),
                };
                if movable && let Some(world) = self.target_world(hover.target) {
                    let other_holds = self.hands[other].grab.is_some_and(|g| g.primary && g.target == hover.target);
                    if other_holds {
                        // Second hand on the same panel: resize by moving the hands apart.
                        if let (Some(other_tip), Some(panel)) =
                            (self.hands[other].tip(), self.vr_panel_mut(hover.target))
                        {
                            let start = (tip.translation - other_tip.translation).length().max(0.05);
                            let width = panel.width_m;
                            if let Some(grab) = &mut self.hands[other].grab {
                                grab.scale = Some((start, width));
                            }
                        }
                        self.hands[index].grab = Some(Grab {
                            target: hover.target,
                            offset: tip.relative(world),
                            primary: false,
                            scale: None,
                        });
                    } else {
                        self.hands[index].grab = Some(Grab {
                            target: hover.target,
                            offset: tip.relative(world),
                            primary: true,
                            scale: None,
                        });
                    }
                    self.hands[index].hover = None;
                    self.haptic(index, 0.8);
                }
            }

            if input.grab.released
                && let Some(grab) = self.hands[index].grab.take()
            {
                if grab.primary {
                    let world = tip.mul(grab.offset);
                    // If the other hand is still holding on, it takes over.
                    let other_tip = self.hands[other].tip();
                    if let Some(other_grab) = &mut self.hands[other].grab
                        && other_grab.target == grab.target
                        && let Some(other_tip) = other_tip
                    {
                        *other_grab =
                            Grab { target: grab.target, offset: other_tip.relative(world), primary: true, scale: None };
                    } else {
                        self.drop_target(grab.target, world);
                    }
                } else if let Some(other_grab) = &mut self.hands[other].grab {
                    other_grab.scale = None;
                }
            }

            // Push and pull with the stick while holding a panel.
            if let Some(grab) = &mut self.hands[index].grab
                && grab.primary
                && grab.target != Target::Watch
            {
                let push = crate::interaction::pointer::shape_stick(input.stick.y);
                if push != 0.0 {
                    let distance = (-grab.offset.translation.z + push * dt * 1.5).clamp(0.08, 6.0);
                    grab.offset.translation.z = -distance;
                }
            }
        }

        // Two-handed resize.
        for index in 0..2 {
            let Some(grab) = self.hands[index].grab else { continue };
            let Some((start, width)) = grab.scale else { continue };
            let (Some(a), Some(b)) = (self.hands[0].tip(), self.hands[1].tip()) else { continue };
            let distance = (a.translation - b.translation).length();
            let new_width = (width * distance / start).clamp(MIN_PANEL_WIDTH, MAX_PANEL_WIDTH);
            if let Some(panel) = self.vr_panel_mut(grab.target) {
                panel.width_m = new_width;
            }
        }
    }

    /// Finishes moving `target` to `world`.
    fn drop_target(&mut self, target: Target, world: Pose) {
        match target {
            Target::Watch => {
                if let Some(tip) = self.wrist_tip() {
                    let hand = self.hud.config.watch.hand;
                    self.hud.config.watch.set_offset(hand, Some(tip.relative(world)));
                    self.hud.mark_config_dirty();
                }
            }
            Target::Toast => {}
            Target::Panel(id) => {
                let head = self.head;
                let wrist = self.wrist_tip();
                let Some(panel) = self.panel_mut(id) else { return };
                panel.pose = match panel.info.placement {
                    Placement::World => world,
                    Placement::Head => head.map(|h| h.relative(world)).unwrap_or(panel.pose),
                    Placement::Wrist => wrist.map(|w| w.relative(world)).unwrap_or(panel.pose),
                };
                self.remember(id);
            }
        }
    }

    /// Gives each panel's egui pointer to one hand: the one pressing on it, else the one that
    /// just pressed, else whichever points at it.
    fn update_pointers(&mut self, inputs: &[HandInput; 2]) {
        let mut targets = vec![Target::Watch, Target::Toast];
        targets.extend(self.panels.iter().map(|p| Target::Panel(p.info.id)));
        let wrist = self.wrist_index();
        for target in targets {
            let mut owner: Option<(usize, Vec2, u8)> = None;
            for index in 0..2 {
                let hand = &self.hands[index];
                let rank_uv = if hand.press.is_some_and(|p| p.target == target) {
                    let Some(tip) = hand.tip() else { continue };
                    let Some(world) = self.target_world(target) else { continue };
                    let Some((w, h)) = self.target_size(target) else { continue };
                    ray_plane_hit(tip.translation, tip.forward(), world, w, h).map(|hit| (3, hit.uv))
                } else if let Some(hover) = hand.hover.filter(|h| h.target == target) {
                    let pressing = inputs[index].click.pressed || inputs[index].right.pressed;
                    Some((
                        if pressing {
                            2
                        } else if index != wrist {
                            1
                        } else {
                            0
                        },
                        hover.uv,
                    ))
                } else {
                    None
                };
                if let Some((rank, uv)) = rank_uv
                    && owner.is_none_or(|(_, _, best)| rank > best)
                {
                    owner = Some((index, uv, rank));
                }
            }
            let Some(panel) = self.vr_panel_mut(target) else { continue };
            match owner {
                Some((_, uv, _)) => {
                    panel.pointer_at_uv(uv);
                }
                None => panel.pointer_gone(),
            }
        }
    }

    fn target_size(&self, target: Target) -> Option<(f32, f32)> {
        let panel = match target {
            Target::Watch => &self.watch,
            Target::Toast => &self.toast,
            Target::Panel(id) => self.panel(id)?,
        };
        Some((panel.width_m, panel.height_m()))
    }

    /// Screen pixel under `pos` if it's on a desktop panel's captured image.
    fn desktop_point(&self, target: Target, pos: Pos2, clamp: bool) -> Option<(i32, i32)> {
        let Target::Panel(id) = target else { return None };
        let panel = self.panel(id)?;
        if panel.info.kind != PanelKind::Desktop {
            return None;
        }
        let rect = panel.output.desktop_image_rect?;
        let screen = desktop::target_rect(panel.info.capture?, &self.hud.state.monitors)?;
        let pos = if clamp { rect.shrink(0.01).clamp(pos) } else { pos };
        panel_point_to_screen(pos, rect, screen)
    }

    fn update_buttons(&mut self, index: usize, input: &HandInput) {
        // Ongoing press: drag.
        if let Some(press) = self.hands[index].press {
            let pos = self.vr_panel_mut(press.target).and_then(|p| p.pointer());
            if let (Some(_), Some(pos)) = (press.desktop, pos)
                && let Some(point) = self.desktop_point(press.target, pos, true)
                && Some(point) != press.desktop
            {
                input::move_to(point.0, point.1);
                if let Some(p) = &mut self.hands[index].press {
                    p.desktop = Some(point);
                }
            }
            let released = match press.button {
                Button::Primary => input.click.released || !input.click.down,
                Button::Secondary => input.right.released || !input.right.down,
            };
            if released {
                let press = self.hands[index].press.take().expect("checked above");
                match press.desktop {
                    Some((x, y)) => input::button(x, y, press.button.mouse(), false),
                    None => {
                        if let Some(panel) = self.vr_panel_mut(press.target) {
                            panel.button(press.button.egui(), false);
                        }
                    }
                }
            }
            return;
        }

        if self.hands[index].grab.is_some() {
            return;
        }
        let Some(hover) = self.hands[index].hover else {
            self.hands[index].cursor = None;
            return;
        };
        let pos = self.vr_panel_mut(hover.target).and_then(|p| p.pointer());
        let button = if input.click.pressed {
            Some(Button::Primary)
        } else if input.right.pressed {
            Some(Button::Secondary)
        } else {
            None
        };

        if let Some(button) = button {
            self.haptic(index, 1.0);
            if hover.target == Target::Toast {
                self.handle_action(Action::OpenPanel(PanelKind::Notifications));
                return;
            }
            if let Target::Panel(id) = hover.target
                && let Some(panel) = self.panel(id)
                && let Some(CaptureTarget::Window(hwnd)) = panel.info.capture
                && desktop::is_minimized(hwnd)
            {
                desktop::focus_window(hwnd);
            }
            let desktop = pos.and_then(|pos| self.desktop_point(hover.target, pos, false));
            if let Some((x, y)) = desktop {
                if let Target::Panel(id) = hover.target
                    && let Some(CaptureTarget::Window(hwnd)) = self.panel(id).and_then(|p| p.info.capture)
                {
                    desktop::focus_window(hwnd);
                }
                input::button(x, y, button.mouse(), true);
            } else if let Some(panel) = self.vr_panel_mut(hover.target) {
                panel.button(button.egui(), true);
            }
            self.hands[index].press = Some(Press { target: hover.target, button, desktop });
            return;
        }

        // Hovering a desktop moves the Windows cursor along, if enabled.
        if self.hud.config.interaction.hover_moves_cursor
            && let Some(pos) = pos
            && let Some(point) = self.desktop_point(hover.target, pos, false)
        {
            let moved = self.hands[index].cursor.is_none_or(|(x, y)| (x - point.0).abs() + (y - point.1).abs() >= 2);
            if moved {
                input::move_to(point.0, point.1);
                self.hands[index].cursor = Some(point);
            }
        }
    }

    fn update_scroll(&mut self, index: usize, input: &HandInput, dt: f32) {
        if self.hands[index].grab.is_some() {
            return;
        }
        let Some(hover) = self.hands[index].hover else {
            self.hands[index].scroll = ScrollAccumulator::default();
            return;
        };
        let config = self.hud.config.interaction.clone();
        let pos = self.vr_panel_mut(hover.target).and_then(|p| p.pointer());
        let on_desktop = pos.and_then(|pos| self.desktop_point(hover.target, pos, false)).is_some();
        let stick = egui::vec2(input.stick.x, input.stick.y);
        if on_desktop {
            let (vertical, horizontal) =
                self.hands[index].scroll.update(stick, dt, config.scroll_speed, config.invert_scroll);
            input::wheel(vertical, horizontal);
        } else {
            let delta = ScrollAccumulator::ui_delta(stick, dt, config.scroll_speed, config.invert_scroll);
            if let Some(panel) = self.vr_panel_mut(hover.target) {
                panel.scroll(egui::vec2(delta.x, delta.y));
            }
        }
    }

    // ---- panels ----

    fn update_panels(&mut self) {
        let hidden = self.hud.state.panels_hidden;
        let wrist = self.wrist_index();
        let wrist_hand = (self.hands[wrist].device, self.hands[wrist].tip_in_raw);
        for i in 0..self.panels.len() {
            let target = Target::Panel(self.panels[i].info.id);
            let grabbed = self.primary_grab(target).map(|(index, _)| index);
            let panel = &mut self.panels[i];
            let mut alpha = if hidden { 0.0 } else { panel.info.opacity };
            if let Some(index) = grabbed {
                let hand = &self.hands[index];
                if let (Some(device), Some(tip_in_raw), Some(grab)) = (hand.device, hand.tip_in_raw, hand.grab) {
                    panel.place_on_device(&self.vr, device, tip_in_raw.mul(grab.offset));
                }
            } else {
                match panel.info.placement {
                    Placement::World => panel.place_absolute(&self.vr, panel.pose),
                    Placement::Head => panel.place_on_device(&self.vr, vr::HMD, panel.pose),
                    Placement::Wrist => match wrist_hand {
                        (Some(device), Some(tip_in_raw)) => {
                            panel.place_on_device(&self.vr, device, tip_in_raw.mul(panel.pose));
                            alpha *= self.wrist_alpha;
                        }
                        _ => alpha = 0.0,
                    },
                }
            }
            panel.apply_visibility(&self.vr, alpha);
        }
    }

    fn update_toast(&mut self) {
        let current = self.hud.current_toast().map(|(n, alpha)| (n.clone(), alpha));
        let Some((note, alpha)) = current else {
            self.toast.apply_visibility(&self.vr, 0.0);
            self.toast_world = None;
            return;
        };
        if self.toast_note.as_ref().map(|n| n.id) != Some(note.id) {
            self.toast_note = Some(note);
            self.toast.mark_dirty();
        }
        let scale = self.hud.config.general.ui_scale;
        let wrist = self.wrist_index();
        let on_wrist = self.hud.config.notifications.placement == ToastPlacement::Wrist
            && self.hands[wrist].device.is_some()
            && self.hands[wrist].tip_in_raw.is_some();
        if on_wrist {
            self.toast.width_m = self.watch.width_m * 1.15;
            let offset = self.hud.config.watch.offset(self.hud.config.watch.hand);
            let lift = self.watch.height_m() / 2.0 + self.toast.height_m() / 2.0 + 0.012;
            let relative = offset.mul(Pose::new(Quat::IDENTITY, Vec3::new(0.0, lift, 0.004)));
            let hand = &self.hands[wrist];
            if let (Some(device), Some(tip_in_raw)) = (hand.device, hand.tip_in_raw) {
                self.toast.place_on_device(&self.vr, device, tip_in_raw.mul(relative));
            }
            self.toast_world = self.wrist_tip().map(|tip| tip.mul(relative));
        } else {
            self.toast.width_m = TOAST_WIDTH_M * scale;
            let relative = Pose::new(Quat::from_rotation_x(-0.12), Vec3::new(0.0, 0.11, -0.7));
            self.toast.place_on_device(&self.vr, vr::HMD, relative);
            self.toast_world = self.head.map(|head| head.mul(relative));
        }
        self.toast.apply_visibility(&self.vr, alpha);
    }

    fn update_lasers(&mut self) {
        let color = self.hud.config.interaction.laser_color;
        for index in 0..2 {
            let hand = &self.hands[index];
            let pressing = hand.press.is_some() || hand.grab.is_some();
            let length = match (hand.press, hand.grab, hand.hover) {
                (_, Some(_), _) => None,
                (Some(press), ..) => {
                    self.target_world(press.target).zip(self.target_size(press.target)).and_then(|(world, (w, h))| {
                        let tip = hand.tip()?;
                        ray_plane_hit(tip.translation, tip.forward(), world, w, h).map(|hit| hit.distance)
                    })
                }
                (None, None, Some(hover)) => Some(hover.distance),
                _ => None,
            };
            let laser = &mut self.lasers[index];
            let placement = match (length, hand.device, hand.tip_in_raw) {
                (Some(length), Some(device), Some(tip_in_raw)) if length > 0.01 => Some((length, device, tip_in_raw)),
                _ => None,
            };
            match placement {
                Some((length, device, tip_in_raw)) => {
                    let style = (color, pressing);
                    for (n, &overlay) in laser.overlays.iter().enumerate() {
                        if overlay == 0 {
                            continue;
                        }
                        if laser.style != Some(style) {
                            let mut pixels = laser_pixels(color, pressing);
                            let _ = self.vr.set_overlay_raw(overlay, &mut pixels, 4, 4);
                        }
                        self.vr.set_overlay_transform_device_rows(
                            overlay,
                            device,
                            laser_rows(tip_in_raw, length - 0.004, n == 1),
                        );
                        if !laser.shown {
                            self.vr.show_overlay(overlay);
                        }
                    }
                    laser.style = Some(style);
                    laser.shown = true;
                }
                None => {
                    if laser.shown {
                        for &overlay in laser.overlays.iter().filter(|&&o| o != 0) {
                            self.vr.hide_overlay(overlay);
                        }
                        laser.shown = false;
                    }
                }
            }
        }
    }

    // ---- rendering ----

    fn render(&mut self, now: Instant) {
        let mut results: Vec<(Target, PaintResult)> = Vec::new();

        if self.watch.is_shown() && self.watch.needs_paint(now) {
            let info = self.watch.info.clone();
            match self.watch.paint(&self.vr, &self.gpu, &self.hud.state, &mut self.hud.config, |ui, ctx| {
                ui::draw_panel(ui, ctx, &info, None)
            }) {
                Ok(result) => {
                    let height = result.output.content_height.ceil().max(120.0);
                    if let Err(err) = self.watch.resize(&self.gpu, vec2(ui::WATCH_WIDTH, height)) {
                        log::warn!("resizing the watch: {err:#}");
                    }
                    results.push((Target::Watch, result));
                }
                Err(err) => log::warn!("drawing the watch: {err:#}"),
            }
        }

        let max_fps = self.hud.config.desktop.max_fps;
        let performance = self.hud.config.general.performance_mode;
        for i in 0..self.panels.len() {
            let panel = &mut self.panels[i];
            let shown = panel.is_shown();
            if performance && !shown {
                continue;
            }
            if let Some(capture) = &mut panel.capture {
                let status = capture.status.clone();
                if capture.poll(&self.gpu, max_fps) || capture.status != status {
                    panel.mark_dirty();
                }
            }
            if !panel.needs_paint(now) {
                continue;
            }
            let view = (panel.info.kind == PanelKind::Desktop).then(|| desktop_view(panel, &self.hud.state.monitors));
            if let Some(view) = &view {
                let aspect = (view.size[0] > 0 && view.size[1] > 0).then(|| view.size[0] as f32 / view.size[1] as f32);
                let size = ui::panel_size(PanelKind::Desktop, aspect);
                if let Err(err) = panel.resize(&self.gpu, size) {
                    log::warn!("resizing a desktop panel: {err:#}");
                }
            }
            let info = panel.info.clone();
            match panel.paint(&self.vr, &self.gpu, &self.hud.state, &mut self.hud.config, |ui, ctx| {
                ui::draw_panel(ui, ctx, &info, view.as_ref())
            }) {
                Ok(result) => results.push((Target::Panel(info.id), result)),
                Err(err) => log::warn!("drawing {:?}: {err:#}", info.kind),
            }
        }

        if self.toast.is_shown()
            && self.toast.needs_paint(now)
            && let Some(note) = self.toast_note.clone()
        {
            match self.toast.paint(&self.vr, &self.gpu, &self.hud.state, &mut self.hud.config, |ui, ctx| {
                ui::notifications::draw_toast(ui, &ctx.palette, &note);
                PanelOutput::default()
            }) {
                Ok(result) => results.push((Target::Toast, result)),
                Err(err) => log::warn!("drawing a notification: {err:#}"),
            }
        }

        if let Some(dashboard) = &mut self.dashboard
            && dashboard.visible
            && dashboard.panel.needs_paint(now)
        {
            match dashboard.panel.paint(&self.vr, &self.gpu, &self.hud.state, &mut self.hud.config, |ui, ctx| {
                let frame = ui::theme::panel_frame(&ctx.palette);
                frame.show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    ui::settings::draw(ui, ctx);
                });
                PanelOutput::default()
            }) {
                Ok(result) => {
                    if result.config_changed {
                        self.hud.mark_config_dirty();
                    }
                    for action in result.actions {
                        self.handle_action(action);
                    }
                }
                Err(err) => log::warn!("drawing the dashboard tab: {err:#}"),
            }
        }

        for (target, result) in results {
            if result.config_changed {
                self.hud.mark_config_dirty();
                self.mark_all_dirty();
            }
            self.sync_keyboard(target);
            for action in result.actions {
                self.handle_action(action);
            }
        }
    }

    /// Opens the SteamVR keyboard when a text field gets focus, and closes it when it loses it.
    fn sync_keyboard(&mut self, target: Target) {
        let wants = match target {
            Target::Watch => self.watch.wants_keyboard,
            Target::Toast => false,
            Target::Panel(id) => self.panel(id).is_some_and(|p| p.wants_keyboard),
        };
        if wants && self.keyboard.is_none() {
            self.show_keyboard(target);
        } else if !wants && self.keyboard == Some(target) && !self.is_desktop(target) {
            self.vr.hide_keyboard();
            self.keyboard = None;
        }
    }

    fn is_desktop(&self, target: Target) -> bool {
        matches!(target, Target::Panel(id) if self.panel(id).is_some_and(|p| p.info.kind == PanelKind::Desktop))
    }

    fn show_keyboard(&mut self, target: Target) {
        let handle = match target {
            Target::Watch => self.watch.handle,
            Target::Toast => return,
            Target::Panel(id) => match self.panel(id) {
                Some(panel) => panel.handle,
                None => return,
            },
        };
        let description = if self.is_desktop(target) { "Type into Windows" } else { "VictorHUD" };
        match self.vr.show_keyboard(handle, description, 0) {
            Ok(()) => self.keyboard = Some(target),
            Err(err) => log::warn!("{err:#}"),
        }
    }

    // ---- actions ----

    fn handle_action(&mut self, action: Action) {
        let Some(action) = self.hud.handle(action) else {
            self.mark_all_dirty();
            return;
        };
        match action {
            Action::OpenPanel(kind) => self.open_panel(kind),
            Action::Panel(id, action) => self.panel_action(id, action),
            Action::MakeAllPanelsInteractive => {
                for panel in &mut self.panels {
                    panel.info.click_through = false;
                    panel.mark_dirty();
                }
                self.hud.state.any_click_through = false;
            }
            Action::ResetWatchPosition => {
                let hand = self.hud.config.watch.hand;
                self.hud.config.watch.set_offset(hand, None);
                self.hud.mark_config_dirty();
            }
            Action::SwitchVirtualDesktop { forward } => input::switch_virtual_desktop(forward),
            Action::ShowKeyboard(id) => self.show_keyboard(Target::Panel(id)),
            Action::RefreshWindows => self.hud.state.windows = desktop::windows(),
            Action::CopyToClipboard(text) => {
                if let Err(err) = super::set_clipboard(&text) {
                    log::warn!("copying to the clipboard: {err:#}");
                }
            }
            Action::Quit => self.quit = true,
            other => log::debug!("unhandled action {other:?}"),
        }
        self.mark_all_dirty();
    }

    fn default_width(&self, kind: PanelKind) -> f32 {
        let base = match kind {
            PanelKind::Watch => self.hud.config.watch.width_m,
            PanelKind::Media => 0.42,
            PanelKind::Desktop => self.hud.config.desktop.default_width_m,
            PanelKind::Vrcx => 0.5,
            PanelKind::Notifications => 0.36,
            PanelKind::Settings => 0.6,
        };
        base * self.hud.config.general.ui_scale
    }

    /// A spot in front of you for a new panel, nudged sideways when another panel is there.
    fn spawn_pose(&self) -> Pose {
        let head = self.head.unwrap_or(Pose::new(Quat::IDENTITY, Vec3::new(0.0, 1.6, 0.0)));
        let mut pose = Pose::in_front_of(head, 0.75, 0.12);
        for _ in 0..4 {
            let crowded = self.panels.iter().any(|p| {
                p.info.placement == Placement::World && (p.pose.translation - pose.translation).length() < 0.15
            });
            if !crowded {
                break;
            }
            let right = pose.rotation * Vec3::X;
            pose = Pose::facing(pose.translation + right * 0.25, head.translation);
        }
        pose
    }

    fn in_view(&self, world: Pose) -> bool {
        let Some(head) = self.head else { return true };
        let to_panel = world.translation - head.translation;
        to_panel.length() < 3.0 && head.forward().dot(to_panel.normalize_or_zero()) > 0.6
    }

    fn open_panel(&mut self, kind: PanelKind) {
        self.hud.state.panels_hidden = false;
        if kind != PanelKind::Desktop
            && let Some(existing) = self.panels.iter().position(|p| p.info.kind == kind)
        {
            let id = self.panels[existing].info.id;
            let world = self.target_world(Target::Panel(id));
            if self.panels[existing].info.placement == Placement::World && !world.is_some_and(|w| self.in_view(w)) {
                let pose = self.spawn_pose();
                self.panels[existing].pose = pose;
            }
            return;
        }

        let id = PanelId(self.next_id);
        self.next_id += 1;
        let mut info = PanelInfo::new(id, kind);
        let mut width = self.default_width(kind);
        let mut pose = self.spawn_pose();
        if let Some(layout) = self.hud.config.layout_for(kind).cloned() {
            info.placement = layout.placement;
            info.opacity = layout.opacity.clamp(0.2, 1.0);
            width = layout.width_m.clamp(MIN_PANEL_WIDTH, MAX_PANEL_WIDTH);
            pose = layout.pose;
            let restored = self.anchor(layout.placement).map(|anchor| anchor.mul(layout.pose));
            let reachable = restored.is_some_and(|world| {
                self.head.is_none_or(|head| (world.translation - head.translation).length() < 4.0)
            });
            let duplicate = kind == PanelKind::Desktop && self.panels.iter().any(|p| p.info.kind == kind);
            if !reachable || duplicate {
                info.placement = Placement::World;
                pose = self.spawn_pose();
            }
        }
        if kind == PanelKind::Desktop {
            let monitors = &self.hud.state.monitors;
            let wanted = self.hud.config.desktop.default_monitor;
            let index = monitors
                .iter()
                .find(|m| m.index == wanted)
                .or_else(|| monitors.iter().find(|m| m.primary))
                .map(|m| m.index)
                .unwrap_or(0);
            info.capture = Some(CaptureTarget::Monitor(index));
        }
        let size = ui::panel_size(kind, None);
        let key = format!("victorwilmath.victorhud.panel.{}", id.0);
        match VrPanel::create(&self.vr, &self.gpu, self.pipeline.clone(), &key, kind.title(), info, size) {
            Ok(mut panel) => {
                panel.width_m = width;
                panel.pose = pose;
                if let Some(target) = panel.info.capture {
                    panel.capture = Some(Capture::new(target));
                }
                self.vr.set_overlay_sort_order(panel.handle, 1);
                self.panels.push(panel);
            }
            Err(err) => {
                log::warn!("opening {kind:?}: {err:#}");
                self.hud.state.add_warning(format!("Couldn't open the {} panel: {err}", kind.title()));
            }
        }
    }

    fn remember(&mut self, id: PanelId) {
        let Some(panel) = self.panel(id) else { return };
        let layout = SavedLayout {
            kind: panel.info.kind,
            placement: panel.info.placement,
            pose: panel.pose,
            width_m: panel.width_m,
            opacity: panel.info.opacity,
        };
        self.hud.config.remember_layout(layout);
        self.hud.mark_config_dirty();
    }

    fn panel_action(&mut self, id: PanelId, action: PanelAction) {
        match action {
            PanelAction::Close => {
                self.remember(id);
                if let Some(index) = self.panels.iter().position(|p| p.info.id == id) {
                    let panel = self.panels.remove(index);
                    panel.destroy(&self.vr);
                    if self.keyboard == Some(Target::Panel(id)) {
                        self.vr.hide_keyboard();
                        self.keyboard = None;
                    }
                    for hand in &mut self.hands {
                        if hand.hover.is_some_and(|h| h.target == Target::Panel(id)) {
                            hand.hover = None;
                        }
                        if hand.press.is_some_and(|p| p.target == Target::Panel(id)) {
                            hand.press = None;
                        }
                        if hand.grab.is_some_and(|g| g.target == Target::Panel(id)) {
                            hand.grab = None;
                        }
                    }
                }
            }
            PanelAction::SetPlacement(placement) => {
                let world = self.target_world(Target::Panel(id));
                let head = self.head;
                let wrist = self.wrist_tip();
                let watch_offset = self.hud.config.watch.offset(self.hud.config.watch.hand);
                let watch_height = self.watch.height_m();
                let Some(panel) = self.panel_mut(id) else { return };
                let Some(world) = world else { return };
                match placement {
                    Placement::World => panel.pose = world,
                    Placement::Head => {
                        let Some(head) = head else { return };
                        let mut relative = head.relative(world);
                        let distance = -relative.translation.z;
                        if !(0.25..=1.5).contains(&distance) || relative.translation.length() > 1.6 {
                            relative = Pose::new(Quat::IDENTITY, Vec3::new(0.0, -0.05, -0.7));
                        }
                        panel.pose = relative;
                    }
                    Placement::Wrist => {
                        if wrist.is_none() {
                            return;
                        }
                        panel.width_m = panel.width_m.min(0.32);
                        let lift = watch_height / 2.0 + panel.height_m() / 2.0 + 0.015;
                        panel.pose = watch_offset.mul(Pose::new(Quat::IDENTITY, Vec3::new(0.0, lift, 0.003)));
                    }
                }
                panel.info.placement = placement;
                panel.mark_dirty();
                self.remember(id);
            }
            PanelAction::ToggleLock => {
                if let Some(panel) = self.panel_mut(id) {
                    panel.info.locked = !panel.info.locked;
                    panel.mark_dirty();
                }
            }
            PanelAction::ToggleClickThrough => {
                if let Some(panel) = self.panel_mut(id) {
                    panel.info.click_through = !panel.info.click_through;
                    panel.mark_dirty();
                }
                self.hud.state.any_click_through = self.panels.iter().any(|p| p.info.click_through);
            }
            PanelAction::AdjustOpacity(delta) => {
                if let Some(panel) = self.panel_mut(id) {
                    let next = panel.info.opacity + delta;
                    panel.info.opacity = if next < 0.35 { 1.0 } else { next.clamp(0.2, 1.0) };
                    panel.mark_dirty();
                }
                self.remember(id);
            }
            PanelAction::AdjustSize(delta) => {
                if let Some(panel) = self.panel_mut(id) {
                    panel.width_m = (panel.width_m * (1.0 + delta)).clamp(MIN_PANEL_WIDTH, MAX_PANEL_WIDTH);
                }
                self.remember(id);
            }
            PanelAction::SetCaptureTarget(target) => {
                if let Some(panel) = self.panel_mut(id) {
                    panel.info.capture = Some(target);
                    panel.capture = Some(Capture::new(target));
                    panel.mark_dirty();
                }
            }
        }
    }
}

fn desktop_view(panel: &mut VrPanel, monitors: &[crate::services::desktop::MonitorInfo]) -> DesktopView {
    let texture = panel.sync_capture_texture();
    let (size, status) = match &panel.capture {
        Some(capture) => (capture.view().map(|(_, size)| size).unwrap_or([0, 0]), capture.status.clone()),
        None => ([0, 0], Some("No source selected".into())),
    };
    let cursor = panel.info.capture.and_then(|target| {
        let [x, y, w, h] = desktop::target_rect(target, monitors)?;
        let (cx, cy) = input::cursor_pos()?;
        let (u, v) = ((cx - x) as f32 / w as f32, (cy - y) as f32 / h as f32);
        ((0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v)).then_some([u, v])
    });
    DesktopView {
        texture,
        size,
        cursor,
        status: if texture.is_some() { None } else { status },
        virtual_desktops: matches!(panel.info.capture, Some(CaptureTarget::Monitor(_))),
    }
}

/// A laser beam quad in the controller's raw space: it starts at the pointer tip and runs
/// `length` meters forward. Two crossed quads make it visible from any angle.
fn laser_rows(tip_in_raw: Pose, length: f32, crossed: bool) -> [[f32; 4]; 3] {
    let stretch = length.max(0.001) / LASER_THICKNESS;
    let (x, z) = if crossed { (Vec3::Y, Vec3::NEG_X) } else { (Vec3::X, Vec3::Y) };
    let local = Affine3A::from_mat3_translation(
        Mat3::from_cols(x, Vec3::NEG_Z * stretch, z),
        Vec3::new(0.0, 0.0, -length / 2.0),
    );
    affine_rows(tip_in_raw.to_affine() * local)
}

fn affine_rows(affine: Affine3A) -> [[f32; 4]; 3] {
    let m = affine.matrix3;
    let t = affine.translation;
    [
        [m.x_axis.x, m.y_axis.x, m.z_axis.x, t.x],
        [m.x_axis.y, m.y_axis.y, m.z_axis.y, t.y],
        [m.x_axis.z, m.y_axis.z, m.z_axis.z, t.z],
    ]
}

fn laser_pixels(color: [u8; 3], pressed: bool) -> Vec<u8> {
    let (rgb, alpha) = if pressed { (color.map(|c| c.saturating_add(70)), 250) } else { (color, 200) };
    (0..16).flat_map(|_| [rgb[0], rgb[1], rgb[2], alpha]).collect()
}

/// The SteamVR dashboard button icon: a watch face in the accent color.
fn icon_pixels(accent: [u8; 3], size: usize) -> Vec<u8> {
    let mut pixels = vec![0u8; size * size * 4];
    let center = size as f32 / 2.0;
    let radius = size as f32 * 0.46;
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 + 0.5 - center;
            let dy = y as f32 + 0.5 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            let coverage = (radius - distance + 0.5).clamp(0.0, 1.0);
            if coverage <= 0.0 {
                continue;
            }
            let ring = (distance - radius * 0.72).abs() < radius * 0.07;
            let hour = dx.abs() < radius * 0.06 && dy < 0.0 && -dy < radius * 0.45;
            let minute = dy.abs() < radius * 0.06 && dx > 0.0 && dx < radius * 0.55;
            let white = ring || hour || minute;
            let rgb = if white { [255, 255, 255] } else { accent };
            let i = (y * size + x) * 4;
            pixels[i..i + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], (coverage * 255.0) as u8]);
        }
    }
    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laser_starts_at_the_tip_and_points_forward() {
        let rows = laser_rows(Pose::IDENTITY, 2.0, false);
        // Center halfway down -Z.
        assert!((rows[2][3] + 1.0).abs() < 1e-5);
        // The quad's Y axis (its length) runs along -Z, stretched so a 4 mm quad spans 2 m.
        let y_axis = Vec3::new(rows[0][1], rows[1][1], rows[2][1]);
        assert!((y_axis.normalize() - Vec3::NEG_Z).length() < 1e-5);
        assert!((y_axis.length() * LASER_THICKNESS - 2.0).abs() < 1e-4);
        // Both quads are proper (right-handed) frames.
        for crossed in [false, true] {
            let rows = laser_rows(Pose::IDENTITY, 1.0, crossed);
            let x = Vec3::new(rows[0][0], rows[1][0], rows[2][0]);
            let y = Vec3::new(rows[0][1], rows[1][1], rows[2][1]).normalize();
            let z = Vec3::new(rows[0][2], rows[1][2], rows[2][2]);
            assert!((x.cross(y) - z).length() < 1e-5);
        }
    }

    #[test]
    fn icon_is_round_and_opaque_in_the_middle() {
        let pixels = icon_pixels([124, 92, 255], 64);
        assert_eq!(pixels.len(), 64 * 64 * 4);
        assert_eq!(pixels[3], 0, "corners are transparent");
        let center = (32 * 64 + 20) * 4;
        assert_eq!(pixels[center + 3], 255);
    }
}
