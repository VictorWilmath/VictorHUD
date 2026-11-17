//! "Check your watch" detection: the watch shows when the back of the wrist faces your eyes,
//! is roughly where you're looking, and is within reach.

use glam::Vec3;

use super::placement::Pose;
use crate::config::WatchConfig;

/// Extra degrees allowed once the watch is showing, so it doesn't flicker at the threshold.
const HYSTERESIS_DEG: f32 = 12.0;
const FADE_IN_SECONDS: f32 = 0.12;
const FADE_OUT_SECONDS: f32 = 0.2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureThresholds {
    pub facing_angle_deg: f32,
    pub view_angle_deg: f32,
    pub max_distance: f32,
}

impl From<&WatchConfig> for GestureThresholds {
    fn from(config: &WatchConfig) -> Self {
        Self {
            facing_angle_deg: config.facing_angle,
            view_angle_deg: config.view_angle,
            max_distance: config.max_distance,
        }
    }
}

/// Whether the watch is currently being "looked at", given world-space head and watch poses.
pub fn is_watch_raised(thresholds: GestureThresholds, head: Pose, watch: Pose, currently_shown: bool) -> bool {
    let slack = if currently_shown { HYSTERESIS_DEG } else { 0.0 };
    let to_eye = head.translation - watch.translation;
    let distance = to_eye.length();
    let max_distance = thresholds.max_distance + if currently_shown { 0.1 } else { 0.0 };
    if distance < 1e-3 || distance > max_distance {
        return false;
    }
    let to_eye = to_eye / distance;
    let facing_cos = watch.normal().dot(to_eye);
    let view_cos = head.forward().dot(-to_eye);
    facing_cos >= cos_deg(thresholds.facing_angle_deg + slack) && view_cos >= cos_deg(thresholds.view_angle_deg + slack)
}

fn cos_deg(deg: f32) -> f32 {
    deg.clamp(0.0, 179.0).to_radians().cos()
}

/// Debounces the raw condition and produces a fade alpha.
#[derive(Clone, Debug, Default)]
pub struct WatchGesture {
    shown: bool,
    condition_time: f32,
    alpha: f32,
}

impl WatchGesture {
    pub fn shown(&self) -> bool {
        self.shown
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Advances the state machine by `dt` seconds. `raised` is the raw per-frame condition and
    /// `forced` keeps the watch up regardless (e.g. "keep on wrist" or while it's being grabbed).
    pub fn update(&mut self, raised: bool, forced: bool, dt: f32, show_delay: f32, hide_delay: f32) -> f32 {
        let want_change = if forced { !self.shown } else { raised != self.shown };
        if want_change {
            self.condition_time += dt;
            let delay = if forced { 0.0 } else if self.shown { hide_delay } else { show_delay };
            if self.condition_time >= delay {
                self.shown = !self.shown;
                self.condition_time = 0.0;
            }
        } else {
            self.condition_time = 0.0;
        }

        let target = if self.shown { 1.0 } else { 0.0 };
        let rate = if self.shown { 1.0 / FADE_IN_SECONDS } else { 1.0 / FADE_OUT_SECONDS };
        if self.alpha < target {
            self.alpha = (self.alpha + rate * dt).min(target);
        } else {
            self.alpha = (self.alpha - rate * dt).max(target);
        }
        self.alpha
    }
}

/// World pose of the watch given the wrist hand's pointer pose and the configured offset.
pub fn watch_world_pose(tip: Pose, offset: Pose) -> Pose {
    tip.mul(offset)
}

/// Head pose looking from `eye` at `target` (used by tests and the preview simulator).
pub fn head_looking_at(eye: Vec3, target: Vec3) -> Pose {
    // A head looks down -Z, so its +Z faces away from the target.
    Pose::facing(eye, eye + (eye - target))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Hand;
    use glam::Quat;

    fn thresholds() -> GestureThresholds {
        GestureThresholds { facing_angle_deg: 40.0, view_angle_deg: 38.0, max_distance: 0.75 }
    }

    /// Left forearm held flat across the chest, back of the wrist pointing straight up.
    fn flat_left_tip() -> Pose {
        // Controller forward (-Z) points to the user's right (+X), the back of the left hand
        // (-X in controller space) points up, and the thumb (+Y) points at the body.
        let basis = glam::Mat3::from_cols(Vec3::NEG_Y, Vec3::Z, Vec3::NEG_X);
        Pose::new(Quat::from_mat3(&basis), Vec3::new(0.05, 1.25, -0.35))
    }

    /// The same pose with the wrist rolled toward the face, as people do when checking a watch.
    fn raised_left_tip() -> Pose {
        let flat = flat_left_tip();
        Pose::new(Quat::from_rotation_x(40f32.to_radians()) * flat.rotation, flat.translation)
    }

    #[test]
    fn default_offset_puts_face_on_back_of_wrist() {
        let watch = watch_world_pose(flat_left_tip(), Pose::default_watch_offset(Hand::Left));
        assert!(watch.normal().y > 0.99, "normal {:?}", watch.normal());
        // 12 o'clock points away from the body and the text reads left-to-right toward the hand.
        assert!(watch.up().z < -0.99, "up {:?}", watch.up());
    }

    #[test]
    fn raised_wrist_in_view_shows_watch() {
        let watch = watch_world_pose(raised_left_tip(), Pose::default_watch_offset(Hand::Left));
        let head = head_looking_at(Vec3::new(0.0, 1.65, 0.0), watch.translation);
        assert!(is_watch_raised(thresholds(), head, watch, false));
    }

    #[test]
    fn looking_away_or_hand_down_hides_watch() {
        let tip = flat_left_tip();
        let watch = watch_world_pose(tip, Pose::default_watch_offset(Hand::Left));
        let eye = Vec3::new(0.0, 1.65, 0.0);

        let looking_up = head_looking_at(eye, eye + Vec3::new(0.0, 1.0, -1.0));
        assert!(!is_watch_raised(thresholds(), looking_up, watch, false));

        // Arm hanging at the side, palm in: watch face points sideways, far below the eyes.
        let hanging_basis = glam::Mat3::from_cols(Vec3::Z, Vec3::Y, Vec3::NEG_X);
        let hanging = Pose::new(Quat::from_mat3(&hanging_basis), Vec3::new(-0.25, 0.9, -0.05));
        let watch = watch_world_pose(hanging, Pose::default_watch_offset(Hand::Left));
        let head = head_looking_at(eye, watch.translation);
        assert!(!is_watch_raised(thresholds(), head, watch, false));
    }

    #[test]
    fn hysteresis_keeps_watch_up_near_threshold() {
        let eye = Vec3::new(0.0, 1.65, 0.0);
        let watch_pos = Vec3::new(0.0, 1.25, -0.35);
        let to_eye = (eye - watch_pos).normalize();
        // Tilt the face 45 degrees away from the eyes: outside 40, inside 40 + hysteresis.
        let axis = to_eye.cross(Vec3::X).normalize();
        let normal = Quat::from_axis_angle(axis, 45f32.to_radians()) * to_eye;
        let watch = Pose::facing(watch_pos, watch_pos + normal);
        let head = head_looking_at(eye, watch_pos);
        assert!(!is_watch_raised(thresholds(), head, watch, false));
        assert!(is_watch_raised(thresholds(), head, watch, true));
    }

    #[test]
    fn too_far_away_does_not_count() {
        let eye = Vec3::new(0.0, 1.65, 0.0);
        let watch_pos = Vec3::new(0.0, 1.2, -1.2);
        let watch = Pose::facing(watch_pos, eye);
        let head = head_looking_at(eye, watch_pos);
        assert!(!is_watch_raised(thresholds(), head, watch, false));
    }

    #[test]
    fn debounce_and_fade() {
        let mut gesture = WatchGesture::default();
        // Brief raise shorter than the show delay does nothing.
        gesture.update(true, false, 0.05, 0.12, 0.35);
        gesture.update(false, false, 0.05, 0.12, 0.35);
        assert!(!gesture.shown());

        for _ in 0..3 {
            gesture.update(true, false, 0.05, 0.12, 0.35);
        }
        assert!(gesture.shown());
        for _ in 0..5 {
            gesture.update(true, false, 0.05, 0.12, 0.35);
        }
        assert_eq!(gesture.alpha(), 1.0);

        // Short dips below the threshold are ignored.
        gesture.update(false, false, 0.2, 0.12, 0.35);
        assert!(gesture.shown());
        gesture.update(false, false, 0.2, 0.12, 0.35);
        assert!(!gesture.shown());
        for _ in 0..10 {
            gesture.update(false, false, 0.05, 0.12, 0.35);
        }
        assert_eq!(gesture.alpha(), 0.0);
    }

    #[test]
    fn forced_shows_immediately() {
        let mut gesture = WatchGesture::default();
        gesture.update(false, true, 0.011, 0.12, 0.35);
        assert!(gesture.shown());
    }
}
