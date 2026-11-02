//! Screen geometry: which edge leads to the other computer, where the cursor
//! enters, and when it leaves again.
//!
//! Coordinates are whatever the platform uses for global cursor positions:
//! points with a top-left origin on macOS, physical pixels on Windows.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Side {
    Left,
    #[default]
    Right,
    Top,
    Bottom,
}

impl Side {
    pub fn opposite(self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
        }
    }

    pub fn is_horizontal(self) -> bool {
        matches!(self, Side::Left | Side::Right)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn right(&self) -> f64 {
        self.x + self.w
    }
    pub fn bottom(&self) -> f64 {
        self.y + self.h
    }
    /// Half-open containment: [x, right) × [y, bottom).
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px < self.right() && py >= self.y && py < self.bottom()
    }
    fn clamp_point(&self, px: f64, py: f64) -> (f64, f64) {
        (
            px.clamp(self.x, self.right() - 1.0),
            py.clamp(self.y, self.bottom() - 1.0),
        )
    }
    fn distance2(&self, px: f64, py: f64) -> f64 {
        let (cx, cy) = self.clamp_point(px, py);
        (cx - px).powi(2) + (cy - py).powi(2)
    }
}

/// Distance from the edge (in screen units) that still counts as "at the edge".
const EDGE_TOLERANCE: f64 = 1.0;
/// Positions this close to a desktop corner never cross, so hot corners and
/// the Windows Start button keep working.
const CORNER_DEAD_ZONE: f64 = 6.0;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Desktop {
    pub screens: Vec<Rect>,
}

impl Desktop {
    pub fn new(screens: Vec<Rect>) -> Desktop {
        let screens = if screens.is_empty() {
            vec![Rect::new(0.0, 0.0, 1920.0, 1080.0)]
        } else {
            screens
        };
        Desktop { screens }
    }

    pub fn bounds(&self) -> Rect {
        let x0 = self.screens.iter().map(|s| s.x).fold(f64::MAX, f64::min);
        let y0 = self.screens.iter().map(|s| s.y).fold(f64::MAX, f64::min);
        let x1 = self.screens.iter().map(|s| s.right()).fold(f64::MIN, f64::max);
        let y1 = self.screens.iter().map(|s| s.bottom()).fold(f64::MIN, f64::max);
        Rect::new(x0, y0, x1 - x0, y1 - y0)
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.screens.iter().any(|s| s.contains(x, y))
    }

    fn screen_at(&self, x: f64, y: f64) -> Option<&Rect> {
        self.screens.iter().find(|s| s.contains(x, y))
    }

    fn nearest_screen(&self, x: f64, y: f64) -> &Rect {
        self.screen_at(x, y).unwrap_or_else(|| {
            self.screens
                .iter()
                .min_by(|a, b| a.distance2(x, y).total_cmp(&b.distance2(x, y)))
                .expect("desktop has at least one screen")
        })
    }

    /// Clamps a point onto the desktop. `from` is the last valid position; the
    /// point is kept on the screen it was on, like the OS does.
    pub fn clamp(&self, from: (f64, f64), to: (f64, f64)) -> (f64, f64) {
        if self.contains(to.0, to.1) {
            return to;
        }
        let s = self.nearest_screen(from.0, from.1);
        s.clamp_point(to.0, to.1)
    }

    /// Center of the screen containing the point (or the nearest one).
    pub fn center_of_screen_at(&self, x: f64, y: f64) -> (f64, f64) {
        let s = self.nearest_screen(x, y);
        ((s.x + s.w / 2.0).floor(), (s.y + s.h / 2.0).floor())
    }

    /// Returns true when the point (already on the desktop) sits on an outer
    /// edge of the desktop on `side`: no other screen continues past it.
    fn on_outer_edge(&self, side: Side, x: f64, y: f64) -> bool {
        let s = match self.screen_at(x, y) {
            Some(s) => s,
            None => return false,
        };
        let at = match side {
            // The last pixel row/column is at right()-1 / bottom()-1.
            Side::Left => x < s.x + EDGE_TOLERANCE,
            Side::Right => x > s.right() - 1.0 - EDGE_TOLERANCE,
            Side::Top => y < s.y + EDGE_TOLERANCE,
            Side::Bottom => y > s.bottom() - 1.0 - EDGE_TOLERANCE,
        };
        if !at {
            return false;
        }
        // Is there another screen just beyond this point?
        let (px, py) = match side {
            Side::Left => (s.x - 2.0, y),
            Side::Right => (s.right() + 1.0, y),
            Side::Top => (x, s.y - 2.0),
            Side::Bottom => (x, s.bottom() + 1.0),
        };
        !self.contains(px, py)
    }

    /// Fraction 0..1 along the desktop's extent for a point on the `side` edge.
    fn fraction_along(&self, side: Side, x: f64, y: f64) -> f64 {
        let b = self.bounds();
        let f = if side.is_horizontal() {
            (y - b.y) / b.h.max(1.0)
        } else {
            (x - b.x) / b.w.max(1.0)
        };
        f.clamp(0.0, 1.0)
    }

    /// If the point is on the outer edge of `side` (and not in a corner),
    /// returns where along that edge it is, as a fraction 0..1.
    pub fn edge_hit(&self, side: Side, x: f64, y: f64) -> Option<f64> {
        if !self.on_outer_edge(side, x, y) {
            return None;
        }
        let s = self.screen_at(x, y)?;
        let near_corner = if side.is_horizontal() {
            y < s.y + CORNER_DEAD_ZONE || y > s.bottom() - 1.0 - CORNER_DEAD_ZONE
        } else {
            x < s.x + CORNER_DEAD_ZONE || x > s.right() - 1.0 - CORNER_DEAD_ZONE
        };
        if near_corner {
            return None;
        }
        Some(self.fraction_along(side, x, y))
    }

    /// Where the cursor should appear when it enters through `side` at
    /// fraction `pos`. The point is `inset` units inside the edge.
    pub fn entry_point(&self, side: Side, pos: f64, inset: f64) -> (f64, f64) {
        let b = self.bounds();
        let pos = pos.clamp(0.0, 1.0);
        // Coordinate along the edge.
        let along = if side.is_horizontal() {
            b.y + pos * b.h
        } else {
            b.x + pos * b.w
        };
        // Among the screens, find the one that is furthest toward `side` and
        // covers `along`; otherwise the closest one along the edge.
        let extreme = |s: &Rect| match side {
            Side::Left => -s.x,
            Side::Right => s.right(),
            Side::Top => -s.y,
            Side::Bottom => s.bottom(),
        };
        let covers = |s: &Rect| {
            if side.is_horizontal() {
                along >= s.y && along < s.bottom()
            } else {
                along >= s.x && along < s.right()
            }
        };
        let gap = |s: &Rect| {
            if covers(s) {
                0.0
            } else if side.is_horizontal() {
                (s.y - along).abs().min((s.bottom() - along).abs())
            } else {
                (s.x - along).abs().min((s.right() - along).abs())
            }
        };
        let s = self
            .screens
            .iter()
            .min_by(|a, b| {
                gap(a)
                    .total_cmp(&gap(b))
                    .then_with(|| extreme(b).total_cmp(&extreme(a)))
            })
            .expect("desktop has at least one screen");
        let inset = inset.min(s.w / 2.0).min(s.h / 2.0);
        let (x, y) = match side {
            Side::Left => (s.x + inset, along),
            Side::Right => (s.right() - 1.0 - inset, along),
            Side::Top => (along, s.y + inset),
            Side::Bottom => (along, s.bottom() - 1.0 - inset),
        };
        s.clamp_point(x.floor(), y.floor())
    }

    /// Moves the cursor from `from` by (dx, dy). Returns the new position and,
    /// if the movement pushed out through the outer edge on `exit_side`,
    /// the fraction along that edge.
    pub fn step(&self, from: (f64, f64), dx: f64, dy: f64, exit_side: Side) -> ((f64, f64), Option<f64>) {
        let cand = (from.0 + dx, from.1 + dy);
        let clamped = self.clamp(from, cand);
        if clamped == cand {
            return (clamped, None);
        }
        let pushed_out = match exit_side {
            Side::Left => cand.0 < clamped.0,
            Side::Right => cand.0 > clamped.0,
            Side::Top => cand.1 < clamped.1,
            Side::Bottom => cand.1 > clamped.1,
        };
        if pushed_out && self.on_outer_edge(exit_side, clamped.0, clamped.1) {
            let f = self.fraction_along(exit_side, clamped.0, clamped.1);
            return (clamped, Some(f));
        }
        (clamped, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_monitors() -> Desktop {
        // Main 1920x1080 at origin, second 1280x1024 to the right, lower.
        Desktop::new(vec![
            Rect::new(0.0, 0.0, 1920.0, 1080.0),
            Rect::new(1920.0, 200.0, 1280.0, 1024.0),
        ])
    }

    #[test]
    fn right_edge_only_on_outer_screen() {
        let d = two_monitors();
        // Right edge of the main screen continues onto the second screen.
        assert_eq!(d.edge_hit(Side::Right, 1919.0, 500.0), None);
        // ...but above the second screen it is an outer edge.
        assert!(d.edge_hit(Side::Right, 1919.0, 100.0).is_some());
        // Right edge of the second screen.
        let f = d.edge_hit(Side::Right, 3199.0, 700.0).unwrap();
        assert!((f - 700.0 / 1224.0).abs() < 1e-9);
        // Not at the edge.
        assert_eq!(d.edge_hit(Side::Right, 3000.0, 700.0), None);
        // Corner dead zone.
        assert_eq!(d.edge_hit(Side::Right, 3199.0, 201.0), None);
    }

    #[test]
    fn left_edge() {
        let d = two_monitors();
        assert!(d.edge_hit(Side::Left, 0.0, 500.0).is_some());
        assert_eq!(d.edge_hit(Side::Left, 1.0, 500.0), None);
        assert_eq!(d.edge_hit(Side::Left, 1920.0, 500.0), None);
    }

    #[test]
    fn entry_and_exit() {
        let d = Desktop::new(vec![Rect::new(0.0, 0.0, 1440.0, 900.0)]);
        let p = d.entry_point(Side::Left, 0.5, 1.0);
        assert_eq!(p, (1.0, 450.0));
        // Move right: stays inside.
        let (p2, exit) = d.step(p, 10.0, 0.0, Side::Left);
        assert_eq!(p2, (11.0, 450.0));
        assert_eq!(exit, None);
        // Push out through the left edge.
        let (p3, exit) = d.step(p2, -50.0, 0.0, Side::Left);
        assert_eq!(p3, (0.0, 450.0));
        assert!((exit.unwrap() - 0.5).abs() < 1e-9);
        // Pushing out through the top does not exit.
        let (_, exit) = d.step((100.0, 0.0), 0.0, -10.0, Side::Left);
        assert_eq!(exit, None);
    }

    #[test]
    fn entry_on_multi_monitor() {
        let d = two_monitors();
        // Entering from the right at the very top lands on the main screen
        // (the second screen doesn't cover y=0).
        let p = d.entry_point(Side::Right, 0.0, 1.0);
        assert_eq!(p, (1918.0, 0.0));
        // Lower down, lands on the second screen.
        let p = d.entry_point(Side::Right, 0.9, 1.0);
        assert_eq!(p.0, 3198.0);
    }

    #[test]
    fn clamp_keeps_screen() {
        let d = two_monitors();
        assert_eq!(d.clamp((100.0, 100.0), (100.0, -5.0)), (100.0, 0.0));
        assert_eq!(d.clamp((1919.0, 100.0), (1925.0, 100.0)), (1919.0, 100.0));
        assert_eq!(d.clamp((1919.0, 500.0), (1925.0, 500.0)), (1925.0, 500.0));
    }
}
