//! Small vector icons drawn with the egui painter, so they stay crisp at any overlay resolution
//! and don't depend on which glyphs a font happens to include.

use std::f32::consts::{PI, TAU};

use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, pos2, vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Play,
    Pause,
    Next,
    Previous,
    ChevronDown,
    ChevronUp,
    Close,
    Pin,
    Eye,
    EyeOff,
    Watch,
    Lock,
    Unlock,
    Gear,
    Monitor,
    Users,
    Music,
    Bell,
    Keyboard,
    Moon,
    Plus,
    Minus,
    Shuffle,
    Repeat,
    RepeatOne,
    Volume,
    Mute,
    ArrowLeft,
    ArrowRight,
    Globe,
    Pointer,
    Refresh,
    Trash,
    Check,
    Clock,
    Warning,
}

struct Pen<'a> {
    painter: &'a Painter,
    rect: Rect,
    stroke: Stroke,
    color: Color32,
}

impl Pen<'_> {
    /// Maps a coordinate on a 24x24 design grid into the target rect.
    fn p(&self, x: f32, y: f32) -> Pos2 {
        pos2(self.rect.min.x + x / 24.0 * self.rect.width(), self.rect.min.y + y / 24.0 * self.rect.height())
    }

    fn s(&self, v: f32) -> f32 {
        v / 24.0 * self.rect.width()
    }

    fn line(&self, points: &[(f32, f32)]) {
        let points: Vec<Pos2> = points.iter().map(|&(x, y)| self.p(x, y)).collect();
        self.painter.add(Shape::line(points, self.stroke));
    }

    fn closed(&self, points: &[(f32, f32)]) {
        let points: Vec<Pos2> = points.iter().map(|&(x, y)| self.p(x, y)).collect();
        self.painter.add(Shape::closed_line(points, self.stroke));
    }

    fn fill(&self, points: &[(f32, f32)]) {
        let points: Vec<Pos2> = points.iter().map(|&(x, y)| self.p(x, y)).collect();
        self.painter.add(Shape::convex_polygon(points, self.color, Stroke::NONE));
    }

    fn circle(&self, x: f32, y: f32, r: f32) {
        self.painter.circle_stroke(self.p(x, y), self.s(r), self.stroke);
    }

    fn dot(&self, x: f32, y: f32, r: f32) {
        self.painter.circle_filled(self.p(x, y), self.s(r), self.color);
    }

    fn rect(&self, x0: f32, y0: f32, x1: f32, y1: f32, radius: f32) {
        let rect = Rect::from_two_pos(self.p(x0, y0), self.p(x1, y1));
        self.painter.rect_stroke(rect, self.s(radius), self.stroke, egui::StrokeKind::Middle);
    }

    fn fill_rect(&self, x0: f32, y0: f32, x1: f32, y1: f32, radius: f32) {
        let rect = Rect::from_two_pos(self.p(x0, y0), self.p(x1, y1));
        self.painter.rect_filled(rect, self.s(radius), self.color);
    }

    fn arc(&self, cx: f32, cy: f32, r: f32, from: f32, to: f32) -> Vec<(f32, f32)> {
        let steps = 20;
        (0..=steps)
            .map(|i| {
                let a = from + (to - from) * i as f32 / steps as f32;
                (cx + r * a.cos(), cy + r * a.sin())
            })
            .collect()
    }

    fn arrow_head(&self, tip: (f32, f32), dir: (f32, f32), size: f32) {
        let len = (dir.0 * dir.0 + dir.1 * dir.1).sqrt().max(1e-3);
        let (dx, dy) = (dir.0 / len, dir.1 / len);
        let back = (tip.0 - dx * size, tip.1 - dy * size);
        let side = (-dy * size * 0.7, dx * size * 0.7);
        self.line(&[(back.0 + side.0, back.1 + side.1), tip, (back.0 - side.0, back.1 - side.1)]);
    }
}

pub fn paint(painter: &Painter, rect: Rect, icon: Icon, color: Color32) {
    let pen = Pen { painter, rect, stroke: Stroke::new((rect.width() * 0.085).max(1.2), color), color };
    match icon {
        Icon::Play => pen.fill(&[(8.0, 5.0), (19.5, 12.0), (8.0, 19.0)]),
        Icon::Pause => {
            pen.fill_rect(6.5, 5.0, 10.5, 19.0, 1.2);
            pen.fill_rect(13.5, 5.0, 17.5, 19.0, 1.2);
        }
        Icon::Next => {
            pen.fill(&[(5.0, 5.5), (15.0, 12.0), (5.0, 18.5)]);
            pen.fill_rect(16.0, 5.5, 19.0, 18.5, 1.0);
        }
        Icon::Previous => {
            pen.fill(&[(19.0, 5.5), (19.0, 18.5), (9.0, 12.0)]);
            pen.fill_rect(5.0, 5.5, 8.0, 18.5, 1.0);
        }
        Icon::ChevronDown => pen.line(&[(6.0, 9.0), (12.0, 15.0), (18.0, 9.0)]),
        Icon::ChevronUp => pen.line(&[(6.0, 15.0), (12.0, 9.0), (18.0, 15.0)]),
        Icon::Close => {
            pen.line(&[(6.5, 6.5), (17.5, 17.5)]);
            pen.line(&[(17.5, 6.5), (6.5, 17.5)]);
        }
        Icon::Pin => {
            pen.circle(12.0, 8.5, 4.5);
            pen.line(&[(12.0, 13.0), (12.0, 21.0)]);
            pen.dot(12.0, 8.5, 1.5);
        }
        Icon::Eye | Icon::EyeOff => {
            // Two arcs that meet at (2.5, 12) and (21.5, 12).
            let mut lens = pen.arc(12.0, 16.5, 10.5, -PI * 0.859, -PI * 0.141);
            lens.extend(pen.arc(12.0, 7.5, 10.5, PI * 0.141, PI * 0.859));
            pen.closed(&lens);
            pen.circle(12.0, 12.0, 3.0);
            if icon == Icon::EyeOff {
                pen.line(&[(4.0, 4.0), (20.0, 20.0)]);
            }
        }
        Icon::Watch => {
            pen.rect(6.5, 6.0, 17.5, 18.0, 3.5);
            pen.line(&[(9.0, 6.0), (9.5, 2.5), (14.5, 2.5), (15.0, 6.0)]);
            pen.line(&[(9.0, 18.0), (9.5, 21.5), (14.5, 21.5), (15.0, 18.0)]);
            pen.line(&[(12.0, 9.0), (12.0, 12.0), (14.5, 13.5)]);
        }
        Icon::Lock | Icon::Unlock => {
            pen.fill_rect(5.5, 11.0, 18.5, 20.5, 2.5);
            let shackle = if icon == Icon::Lock {
                let mut points = vec![(8.5, 11.0)];
                points.extend(pen.arc(12.0, 8.0, 3.5, PI, TAU));
                points.push((15.5, 11.0));
                points
            } else {
                let mut points = vec![(8.5, 11.0)];
                points.extend(pen.arc(12.0, 6.5, 3.5, PI, TAU));
                points.push((15.5, 8.0));
                points
            };
            pen.line(&shackle);
        }
        Icon::Gear => {
            pen.circle(12.0, 12.0, 3.0);
            pen.circle(12.0, 12.0, 6.5);
            for i in 0..8 {
                let a = i as f32 * TAU / 8.0;
                pen.line(&[(12.0 + 6.5 * a.cos(), 12.0 + 6.5 * a.sin()), (12.0 + 9.5 * a.cos(), 12.0 + 9.5 * a.sin())]);
            }
        }
        Icon::Monitor => {
            pen.rect(3.0, 4.5, 21.0, 16.0, 2.0);
            pen.line(&[(12.0, 16.0), (12.0, 19.5)]);
            pen.line(&[(8.0, 20.0), (16.0, 20.0)]);
        }
        Icon::Users => {
            pen.circle(9.5, 8.5, 3.5);
            pen.line(&pen.arc(9.5, 20.5, 6.5, PI, TAU));
            pen.circle(17.0, 9.5, 2.5);
            pen.line(&pen.arc(17.5, 20.0, 4.5, PI * 1.25, TAU));
        }
        Icon::Music => {
            pen.line(&[(9.5, 17.0), (9.5, 5.5), (19.0, 3.5), (19.0, 15.0)]);
            pen.dot(7.0, 17.5, 2.6);
            pen.dot(16.5, 15.5, 2.6);
        }
        Icon::Bell => {
            let mut bell = vec![(4.5, 17.0)];
            bell.extend(pen.arc(12.0, 10.0, 6.0, PI, TAU));
            bell.push((18.0, 13.0));
            bell.push((19.5, 17.0));
            bell.insert(1, (6.0, 13.0));
            pen.closed(&bell);
            pen.line(&[(10.0, 20.0), (14.0, 20.0)]);
        }
        Icon::Keyboard => {
            pen.rect(2.5, 6.0, 21.5, 18.0, 2.0);
            for row in 0..2 {
                for col in 0..5 {
                    pen.dot(6.0 + col as f32 * 3.0, 9.5 + row as f32 * 3.0, 0.75);
                }
            }
            pen.line(&[(8.0, 15.0), (16.0, 15.0)]);
        }
        Icon::Moon => {
            // A disc with a bite taken out; the angles are where the two circles intersect.
            let mut shape = pen.arc(12.0, 12.0, 8.5, 0.155, 4.557);
            shape.extend(pen.arc(16.5, 7.5, 7.0, 3.732, 0.980));
            pen.closed(&shape);
        }
        Icon::Plus => {
            pen.line(&[(12.0, 5.0), (12.0, 19.0)]);
            pen.line(&[(5.0, 12.0), (19.0, 12.0)]);
        }
        Icon::Minus => pen.line(&[(5.0, 12.0), (19.0, 12.0)]),
        Icon::Shuffle => {
            pen.line(&[(3.5, 7.0), (8.0, 7.0), (15.0, 17.0), (20.0, 17.0)]);
            pen.line(&[(3.5, 17.0), (8.0, 17.0), (15.0, 7.0), (20.0, 7.0)]);
            pen.arrow_head((20.5, 7.0), (1.0, 0.0), 3.0);
            pen.arrow_head((20.5, 17.0), (1.0, 0.0), 3.0);
        }
        Icon::Repeat | Icon::RepeatOne => {
            pen.line(&[(4.5, 12.0), (4.5, 7.5), (19.0, 7.5)]);
            pen.arrow_head((19.5, 7.5), (1.0, 0.0), 3.0);
            pen.line(&[(19.5, 12.0), (19.5, 16.5), (5.0, 16.5)]);
            pen.arrow_head((4.5, 16.5), (-1.0, 0.0), 3.0);
            if icon == Icon::RepeatOne {
                pen.line(&[(11.0, 10.8), (12.3, 10.0), (12.3, 14.2)]);
            }
        }
        Icon::Volume | Icon::Mute => {
            pen.fill(&[(3.5, 9.0), (8.0, 9.0), (13.0, 4.5), (13.0, 19.5), (8.0, 15.0), (3.5, 15.0)]);
            if icon == Icon::Volume {
                pen.line(&pen.arc(13.0, 12.0, 4.0, -PI * 0.3, PI * 0.3));
                pen.line(&pen.arc(13.0, 12.0, 7.5, -PI * 0.3, PI * 0.3));
            } else {
                pen.line(&[(16.0, 9.0), (21.0, 15.0)]);
                pen.line(&[(21.0, 9.0), (16.0, 15.0)]);
            }
        }
        Icon::ArrowLeft => {
            pen.line(&[(19.0, 12.0), (5.5, 12.0)]);
            pen.arrow_head((5.0, 12.0), (-1.0, 0.0), 5.0);
        }
        Icon::ArrowRight => {
            pen.line(&[(5.0, 12.0), (18.5, 12.0)]);
            pen.arrow_head((19.0, 12.0), (1.0, 0.0), 5.0);
        }
        Icon::Globe => {
            pen.circle(12.0, 12.0, 8.5);
            let mut meridian = pen.arc(12.0, 12.0, 8.5, -PI / 2.0, PI / 2.0);
            for point in &mut meridian {
                point.0 = 12.0 + (point.0 - 12.0) * 0.45;
            }
            pen.line(&meridian);
            let mirrored: Vec<_> = meridian.iter().map(|&(x, y)| (24.0 - x, y)).collect();
            pen.line(&mirrored);
            pen.line(&[(3.5, 12.0), (20.5, 12.0)]);
        }
        Icon::Pointer => {
            pen.closed(&[
                (6.0, 3.5),
                (18.0, 14.5),
                (12.5, 15.0),
                (15.5, 20.5),
                (13.0, 21.5),
                (10.0, 16.0),
                (6.0, 19.5),
            ]);
        }
        Icon::Refresh => {
            pen.line(&pen.arc(12.0, 12.0, 7.5, -PI * 0.35, PI * 1.45));
            let tip = (12.0 + 7.5 * (-PI * 0.35).cos(), 12.0 + 7.5 * (-PI * 0.35).sin());
            pen.arrow_head((tip.0 + 0.6, tip.1 + 0.6), (0.4, 1.0), 3.5);
        }
        Icon::Trash => {
            pen.line(&[(4.5, 7.0), (19.5, 7.0)]);
            pen.line(&[(9.5, 7.0), (10.0, 4.5), (14.0, 4.5), (14.5, 7.0)]);
            pen.closed(&[(6.5, 7.0), (7.5, 20.0), (16.5, 20.0), (17.5, 7.0)]);
        }
        Icon::Check => pen.line(&[(5.0, 12.5), (10.0, 17.5), (19.5, 7.0)]),
        Icon::Clock => {
            pen.circle(12.0, 12.0, 8.5);
            pen.line(&[(12.0, 7.0), (12.0, 12.0), (15.5, 14.0)]);
        }
        Icon::Warning => {
            pen.closed(&[(12.0, 3.5), (21.0, 19.5), (3.0, 19.5)]);
            pen.line(&[(12.0, 9.5), (12.0, 14.0)]);
            pen.dot(12.0, 16.8, 1.0);
        }
    }
}

/// A battery glyph filled to `percent` (0..1).
pub fn battery(painter: &Painter, rect: Rect, percent: f32, color: Color32, outline: Color32) {
    let body = Rect::from_min_max(rect.min, pos2(rect.max.x - rect.width() * 0.1, rect.max.y));
    let nub = Rect::from_min_max(
        pos2(body.max.x, rect.center().y - rect.height() * 0.2),
        pos2(rect.max.x, rect.center().y + rect.height() * 0.2),
    );
    painter.rect_stroke(body, 3.0, Stroke::new(1.3, outline), egui::StrokeKind::Inside);
    painter.rect_filled(nub, 1.0, outline);
    let inner = body.shrink(2.5);
    let fill = Rect::from_min_size(inner.min, vec2(inner.width() * percent.clamp(0.0, 1.0), inner.height()));
    painter.rect_filled(fill, 1.5, color);
}
