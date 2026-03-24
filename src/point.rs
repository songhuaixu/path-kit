//! 二维点 (x, y)。2D point with x and y coordinates.

use crate::pathkit;

/// 二维点。A 2D point with x and y coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// X 坐标 / X coordinate
    pub x: f32,
    /// Y 坐标 / Y coordinate
    pub y: f32,
}

impl Point {
    /// 创建点。Creates a point at (x, y).
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for pathkit::SkPoint {
    fn from(p: Point) -> Self {
        pathkit::SkPoint { fX: p.x, fY: p.y }
    }
}

impl From<pathkit::SkPoint> for Point {
    fn from(p: pathkit::SkPoint) -> Self {
        Self { x: p.fX, y: p.fY }
    }
}
