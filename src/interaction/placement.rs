//! Rigid-transform math shared by overlay placement, grabbing and laser hit-testing.
//!
//! Conventions follow OpenVR: right-handed, meters, +Y up, -Z forward. An overlay quad is
//! centered on its pose origin, spans X (width) and Y (height), and is seen from +Z.

use glam::{Affine3A, Mat3, Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::state::Hand;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub rotation: Quat,
    pub translation: Vec3,
}

impl Default for Pose {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Pose {
    pub const IDENTITY: Pose = Pose { rotation: Quat::IDENTITY, translation: Vec3::ZERO };

    pub fn new(rotation: Quat, translation: Vec3) -> Self {
        Self { rotation: rotation.normalize(), translation }
    }

    pub fn from_affine(affine: Affine3A) -> Self {
        let (_, rotation, translation) = affine.to_scale_rotation_translation();
        Self::new(rotation, translation)
    }

    pub fn to_affine(self) -> Affine3A {
        Affine3A::from_rotation_translation(self.rotation, self.translation)
    }

    /// Reads an OpenVR `HmdMatrix34_t` (row-major 3x4).
    pub fn from_rows(m: [[f32; 4]; 3]) -> Self {
        let basis = Mat3::from_cols(
            Vec3::new(m[0][0], m[1][0], m[2][0]),
            Vec3::new(m[0][1], m[1][1], m[2][1]),
            Vec3::new(m[0][2], m[1][2], m[2][2]),
        );
        Self::new(Quat::from_mat3(&basis), Vec3::new(m[0][3], m[1][3], m[2][3]))
    }

    pub fn to_rows(self) -> [[f32; 4]; 3] {
        let basis = Mat3::from_quat(self.rotation);
        let t = self.translation;
        [
            [basis.x_axis.x, basis.y_axis.x, basis.z_axis.x, t.x],
            [basis.x_axis.y, basis.y_axis.y, basis.z_axis.y, t.y],
            [basis.x_axis.z, basis.y_axis.z, basis.z_axis.z, t.z],
        ]
    }

    pub fn inverse(self) -> Self {
        let rotation = self.rotation.inverse();
        Self { rotation, translation: -(rotation * self.translation) }
    }

    /// `self * other`: applies `other` first, then `self`.
    pub fn mul(self, other: Pose) -> Self {
        Self::new(self.rotation * other.rotation, self.translation + self.rotation * other.translation)
    }

    pub fn transform_point(self, point: Vec3) -> Vec3 {
        self.translation + self.rotation * point
    }

    pub fn transform_vector(self, vector: Vec3) -> Vec3 {
        self.rotation * vector
    }

    pub fn forward(self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    pub fn up(self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// The direction an overlay at this pose is visible from.
    pub fn normal(self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    /// Expresses `world` relative to `self` (so that `self.mul(result) == world`).
    pub fn relative(self, world: Pose) -> Pose {
        self.inverse().mul(world)
    }

    /// Default watch placement on the back of the wrist, relative to the pointer ("tip") pose.
    ///
    /// The face points out of the back of the hand, 12 o'clock points away from the body when the
    /// forearm is raised across the chest, and the quad sits ~16 cm behind the controller tip.
    pub fn default_watch_offset(hand: Hand) -> Pose {
        let (right, normal, x_offset) = match hand {
            Hand::Left => (Vec3::NEG_Z, Vec3::NEG_X, -0.03),
            Hand::Right => (Vec3::Z, Vec3::X, 0.03),
        };
        let up = Vec3::NEG_Y;
        let basis = Mat3::from_cols(right, up, normal);
        Pose::new(Quat::from_mat3(&basis), Vec3::new(x_offset, -0.025, 0.16))
    }

    /// A pose at `position` whose visible side (+Z) faces `target`, keeping the panel upright.
    pub fn facing(position: Vec3, target: Vec3) -> Pose {
        let to_target = (target - position).normalize_or(Vec3::Z);
        let mut right = Vec3::Y.cross(to_target);
        if right.length_squared() < 1e-6 {
            right = Vec3::X;
        }
        let right = right.normalize();
        let up = to_target.cross(right).normalize();
        Pose::new(Quat::from_mat3(&Mat3::from_cols(right, up, to_target)), position)
    }

    /// Where a newly opened panel goes: in front of the head at `distance`, `drop` below eye height,
    /// facing the user and level with the horizon.
    pub fn in_front_of(head: Pose, distance: f32, drop: f32) -> Pose {
        let mut forward = head.forward();
        forward.y = 0.0;
        let forward = forward.normalize_or(Vec3::NEG_Z);
        let position = head.translation + forward * distance - Vec3::Y * drop;
        Pose::facing(position, head.translation - Vec3::Y * drop)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadHit {
    pub distance: f32,
    pub point: Vec3,
    /// Normalized coordinates with (0, 0) at the top-left of the texture.
    pub uv: Vec2,
}

/// Intersects a ray with a flat overlay quad. Only hits on the visible side count.
pub fn ray_quad_hit(origin: Vec3, direction: Vec3, quad: Pose, width: f32, height: f32) -> Option<QuadHit> {
    let direction = direction.normalize_or_zero();
    let normal = quad.normal();
    let denom = direction.dot(normal);
    if denom > -1e-4 {
        return None;
    }
    let distance = (quad.translation - origin).dot(normal) / denom;
    if distance <= 0.0 {
        return None;
    }
    let point = origin + direction * distance;
    let local = quad.inverse().transform_point(point);
    let u = local.x / width + 0.5;
    let v = 0.5 - local.y / height;
    if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
        return None;
    }
    Some(QuadHit { distance, point, uv: Vec2::new(u, v) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-4
    }

    #[test]
    fn rows_round_trip() {
        let pose = Pose::new(Quat::from_euler(glam::EulerRot::YXZ, 0.4, -0.3, 0.2), Vec3::new(1.0, 2.0, -3.0));
        let back = Pose::from_rows(pose.to_rows());
        assert!(approx(back.translation, pose.translation));
        assert!(back.rotation.dot(pose.rotation).abs() > 0.9999);
    }

    #[test]
    fn inverse_and_relative_compose_to_identity() {
        let a = Pose::new(Quat::from_rotation_y(0.7), Vec3::new(0.3, 1.2, -0.4));
        let b = Pose::new(Quat::from_rotation_x(-0.2), Vec3::new(-1.0, 0.5, 2.0));
        let rel = a.relative(b);
        let back = a.mul(rel);
        assert!(approx(back.translation, b.translation));
        assert!(back.rotation.dot(b.rotation).abs() > 0.9999);
        let identity = a.mul(a.inverse());
        assert!(approx(identity.translation, Vec3::ZERO));
    }

    #[test]
    fn default_watch_offsets_are_proper_rotations() {
        for hand in [Hand::Left, Hand::Right] {
            let pose = Pose::default_watch_offset(hand);
            let basis = Mat3::from_quat(pose.rotation);
            assert!((basis.determinant() - 1.0).abs() < 1e-4);
        }
        // Left watch faces out of the back of the left hand (-X in controller space).
        assert!(approx(Pose::default_watch_offset(Hand::Left).normal(), Vec3::NEG_X));
        assert!(approx(Pose::default_watch_offset(Hand::Right).normal(), Vec3::X));
    }

    #[test]
    fn facing_points_visible_side_at_target() {
        let pose = Pose::facing(Vec3::new(0.0, 1.5, -1.0), Vec3::new(0.0, 1.5, 0.0));
        assert!(approx(pose.normal(), Vec3::Z));
        assert!(approx(pose.up(), Vec3::Y));
    }

    #[test]
    fn spawned_panels_sit_in_front_and_face_the_head() {
        let head = Pose::new(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), Vec3::new(0.0, 1.6, 0.0));
        let panel = Pose::in_front_of(head, 0.6, 0.1);
        // Head yawed 90 degrees left looks down -X.
        assert!(approx(panel.translation, Vec3::new(-0.6, 1.5, 0.0)));
        assert!(approx(panel.normal(), Vec3::X));
    }

    #[test]
    fn ray_hits_center_and_corners() {
        let quad = Pose::facing(Vec3::new(0.0, 1.0, -1.0), Vec3::new(0.0, 1.0, 0.0));
        let hit = ray_quad_hit(Vec3::new(0.0, 1.0, 0.0), Vec3::NEG_Z, quad, 1.0, 0.5).unwrap();
        assert!((hit.distance - 1.0).abs() < 1e-4);
        assert!((hit.uv - Vec2::new(0.5, 0.5)).length() < 1e-4);

        // Upper-left corner region maps near uv (0, 0).
        let hit = ray_quad_hit(Vec3::new(-0.49, 1.24, 0.0), Vec3::NEG_Z, quad, 1.0, 0.5).unwrap();
        assert!(hit.uv.x < 0.02 && hit.uv.y < 0.05);

        // Misses outside the quad and from behind.
        assert!(ray_quad_hit(Vec3::new(0.6, 1.0, 0.0), Vec3::NEG_Z, quad, 1.0, 0.5).is_none());
        assert!(ray_quad_hit(Vec3::new(0.0, 1.0, -2.0), Vec3::Z, quad, 1.0, 0.5).is_none());
        // Pointing away.
        assert!(ray_quad_hit(Vec3::new(0.0, 1.0, 0.0), Vec3::Z, quad, 1.0, 0.5).is_none());
    }
}
