//! 矩形，由 (left, top, right, bottom) 定义。
//! Rectangle defined by (left, top, right, bottom) coordinates.

use crate::pathkit;

/// 矩形，由 (left, top, right, bottom) 定义。
/// Rectangle defined by (left, top, right, bottom) coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// 左边界 / Left edge
    pub left: f32,
    /// 上边界 / Top edge
    pub top: f32,
    /// 右边界 / Right edge
    pub right: f32,
    /// 下边界 / Bottom edge
    pub bottom: f32,
}

impl Rect {
    /// 创建矩形。Creates a rectangle from bounds.
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// 宽度。Returns the width.
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// 高度。Returns the height.
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    /// 是否为空矩形。Returns true if the rect has zero or negative width/height.
    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }
}

impl From<Rect> for pathkit::SkRect {
    fn from(r: Rect) -> Self {
        pathkit::SkRect {
            fLeft: r.left,
            fTop: r.top,
            fRight: r.right,
            fBottom: r.bottom,
        }
    }
}

impl From<pathkit::SkRect> for Rect {
    fn from(r: pathkit::SkRect) -> Self {
        Self {
            left: r.fLeft,
            top: r.fTop,
            right: r.fRight,
            bottom: r.fBottom,
        }
    }
}
