//! Pure helpers for turning laser hits and stick input into clicks and scrolls.

use egui::{Pos2, Rect, Vec2};

const STICK_DEADZONE: f32 = 0.18;

/// Maps a point on a panel (in egui points) to a pixel on a captured screen or window, if the
/// point falls on the captured image. `image_rect` is where the image was drawn on the panel,
/// `target` is the screen-space rectangle `[x, y, width, height]` of the captured source.
pub fn panel_point_to_screen(point: Pos2, image_rect: Rect, target: [i32; 4]) -> Option<(i32, i32)> {
    if !image_rect.contains(point) || image_rect.width() <= 0.0 || image_rect.height() <= 0.0 {
        return None;
    }
    let u = ((point.x - image_rect.min.x) / image_rect.width()).clamp(0.0, 0.9999);
    let v = ((point.y - image_rect.min.y) / image_rect.height()).clamp(0.0, 0.9999);
    let [x, y, w, h] = target;
    Some((x + (u * w as f32) as i32, y + (v * h as f32) as i32))
}

/// Applies a deadzone and a gentle curve so small stick movements give fine control.
pub fn shape_stick(value: f32) -> f32 {
    let magnitude = value.abs();
    if magnitude < STICK_DEADZONE {
        return 0.0;
    }
    let normalized = ((magnitude - STICK_DEADZONE) / (1.0 - STICK_DEADZONE)).min(1.0);
    normalized * normalized * value.signum()
}

/// Turns continuous stick deflection into whole wheel notches over time.
#[derive(Clone, Debug, Default)]
pub struct ScrollAccumulator {
    vertical: f32,
    horizontal: f32,
}

impl ScrollAccumulator {
    /// Returns `(vertical, horizontal)` notches to send this frame. Positive vertical scrolls up.
    pub fn update(&mut self, stick: Vec2, dt: f32, notches_per_second: f32, invert: bool) -> (i32, i32) {
        let sign = if invert { -1.0 } else { 1.0 };
        self.vertical += shape_stick(stick.y) * notches_per_second * dt * sign;
        self.horizontal += shape_stick(stick.x) * notches_per_second * dt;
        if shape_stick(stick.y) == 0.0 {
            self.vertical = 0.0;
        }
        if shape_stick(stick.x) == 0.0 {
            self.horizontal = 0.0;
        }
        let vertical = self.vertical.trunc();
        let horizontal = self.horizontal.trunc();
        self.vertical -= vertical;
        self.horizontal -= horizontal;
        (vertical as i32, horizontal as i32)
    }

    /// Smooth scrolling in egui points for UI panels.
    pub fn ui_delta(stick: Vec2, dt: f32, notches_per_second: f32, invert: bool) -> Vec2 {
        let sign = if invert { -1.0 } else { 1.0 };
        let points_per_notch = 40.0;
        Vec2::new(
            -(shape_stick(stick.x) * notches_per_second * dt * points_per_notch),
            shape_stick(stick.y) * notches_per_second * dt * points_per_notch * sign,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_panel_points_onto_monitor_pixels() {
        let image = Rect::from_min_size(Pos2::new(0.0, 40.0), Vec2::new(960.0, 540.0));
        let monitor = [1920, 0, 2560, 1440];
        assert_eq!(panel_point_to_screen(Pos2::new(0.0, 40.0), image, monitor), Some((1920, 0)));
        assert_eq!(panel_point_to_screen(Pos2::new(480.0, 310.0), image, monitor), Some((1920 + 1280, 720)));
        let bottom_right = panel_point_to_screen(Pos2::new(960.0, 580.0), image, monitor).unwrap();
        assert!(bottom_right.0 < 1920 + 2560 && bottom_right.1 < 1440);
        // The toolbar above the image isn't part of the desktop.
        assert_eq!(panel_point_to_screen(Pos2::new(100.0, 10.0), image, monitor), None);
    }

    #[test]
    fn negative_virtual_desktop_coordinates_work() {
        let image = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
        assert_eq!(panel_point_to_screen(Pos2::new(50.0, 50.0), image, [-1920, -200, 1920, 1080]), Some((-960, 340)));
    }

    #[test]
    fn deadzone_and_curve() {
        assert_eq!(shape_stick(0.1), 0.0);
        assert_eq!(shape_stick(-0.1), 0.0);
        assert_eq!(shape_stick(1.0), 1.0);
        assert_eq!(shape_stick(-1.0), -1.0);
        assert!(shape_stick(0.5) > 0.0 && shape_stick(0.5) < 0.5);
    }

    #[test]
    fn scroll_accumulates_whole_notches() {
        let mut acc = ScrollAccumulator::default();
        let mut total = 0;
        for _ in 0..90 {
            total += acc.update(Vec2::new(0.0, 1.0), 1.0 / 90.0, 12.0, false).0;
        }
        assert!((11..=12).contains(&total), "{total}");

        let mut acc = ScrollAccumulator::default();
        let mut total = 0;
        for _ in 0..90 {
            total += acc.update(Vec2::new(0.0, 1.0), 1.0 / 90.0, 12.0, true).0;
        }
        assert!((-12..=-11).contains(&total), "{total}");

        // Releasing the stick discards leftover fractions.
        acc.update(Vec2::new(0.0, 0.6), 0.05, 12.0, false);
        assert_eq!(acc.update(Vec2::ZERO, 0.05, 12.0, false), (0, 0));
    }
}
