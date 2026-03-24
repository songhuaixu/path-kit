//! 二维点 (x, y)。2D point with x and y coordinates.

use crate::bridge::ffi;

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

impl From<Point> for ffi::Point {
    fn from(p: Point) -> Self {
        ffi::Point {
            fX: p.x,
            fY: p.y,
        }
    }
}

impl From<ffi::Point> for Point {
    fn from(p: ffi::Point) -> Self {
        Self { x: p.fX, y: p.fY }
    }
}
