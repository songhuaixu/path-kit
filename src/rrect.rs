//! 圆角矩形，支持四角独立半径。
//! Rounded rectangle with per-corner radii.

use crate::bridge::ffi;
use crate::rect::Rect;
use crate::RectCorner;

/// 圆角矩形，支持四角独立半径。
/// Rounded rectangle with per-corner radii (each corner can have different x/y).
pub struct RRect {
    inner: ffi::RRect,
}

/// 单角椭圆半径 (x, y)，与 `SkRRect` 每角 radii 一致。
/// Per-corner elliptical radii (x, y), matching `SkRRect` corner radii.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radii {
    /// 该角 x 方向半径 / X-axis corner radius
    pub x: f32,
    /// 该角 y 方向半径 / Y-axis corner radius
    pub y: f32,
}

impl RRect {
    /// 创建空圆角矩形。Creates an empty RRect.
    pub fn new() -> Self {
        let mut inner = ffi::RRect {
            fRect: ffi::Rect {
                fLeft: 0.0,
                fTop: 0.0,
                fRight: 0.0,
                fBottom: 0.0,
            },
            fRadii: [ffi::Point { fX: 0.0, fY: 0.0 }; 4],
            fType: ffi::RRectType::Empty,
        };
        ffi::rrect_new_empty(&mut inner);
        Self { inner }
    }

    /// 从矩形创建（直角）。Creates RRect from rect (sharp corners).
    pub fn from_rect(rect: &Rect) -> Self {
        let mut rr = Self::new();
        let r: ffi::Rect = (*rect).into();
        ffi::rrect_set_rect_xy(&mut rr.inner, &r, 0.0, 0.0);
        rr
    }

    /// 从椭圆边界创建。Creates oval (ellipse) bounded by rect.
    pub fn from_oval(rect: &Rect) -> Self {
        let mut rr = Self::new();
        let r: ffi::Rect = (*rect).into();
        ffi::rrect_set_oval(&mut rr.inner, &r);
        rr
    }

    /// 从矩形与统一圆角创建。Creates rounded rect with uniform radii.
    pub fn from_rect_xy(rect: &Rect, rx: f32, ry: f32) -> Self {
        let mut rr = Self::new();
        let r: ffi::Rect = (*rect).into();
        ffi::rrect_set_rect_xy(&mut rr.inner, &r, rx, ry);
        rr
    }

    /// 从矩形与四角半径创建。Creates RRect with per-corner radii.
    ///
    /// `radii` 顺序：UpperLeft, UpperRight, LowerRight, LowerLeft。
    /// Radii order: UpperLeft, UpperRight, LowerRight, LowerLeft.
    pub fn from_rect_radii(rect: &Rect, radii: &[Radii; 4]) -> Self {
        let mut rr = Self::new();
        let r: ffi::Rect = (*rect).into();
        let pts: [ffi::Point; 4] = [
            ffi::Point {
                fX: radii[0].x,
                fY: radii[0].y,
            },
            ffi::Point {
                fX: radii[1].x,
                fY: radii[1].y,
            },
            ffi::Point {
                fX: radii[2].x,
                fY: radii[2].y,
            },
            ffi::Point {
                fX: radii[3].x,
                fY: radii[3].y,
            },
        ];
        ffi::rrect_set_rect_radii(&mut rr.inner, &r, &pts);
        rr
    }

    /// 边界矩形。Returns the bounding rect.
    pub fn rect(&self) -> Rect {
        self.inner.fRect.into()
    }

    /// 宽度。Returns width.
    pub fn width(&self) -> f32 {
        self.inner.fRect.fRight - self.inner.fRect.fLeft
    }

    /// 高度。Returns height.
    pub fn height(&self) -> f32 {
        self.inner.fRect.fBottom - self.inner.fRect.fTop
    }

    /// 是否为空。Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.inner.fType == ffi::RRectType::Empty
    }

    /// 是否为普通矩形。Returns true if sharp corners (no rounding).
    pub fn is_rect(&self) -> bool {
        self.inner.fType == ffi::RRectType::Rect
    }

    /// 是否为椭圆。Returns true if oval.
    pub fn is_oval(&self) -> bool {
        self.inner.fType == ffi::RRectType::Oval
    }

    /// 是否所有角半径相同。Returns true if uniform radii (simple type).
    pub fn is_simple(&self) -> bool {
        self.inner.fType == ffi::RRectType::Simple
    }

    /// 获取某角半径。Returns radii for corner (UpperLeft, UpperRight, LowerRight, LowerLeft).
    pub fn radii(&self, corner: RectCorner) -> Radii {
        let idx = match corner {
            RectCorner::UpperLeft => 0usize,
            RectCorner::UpperRight => 1,
            RectCorner::LowerRight => 2,
            RectCorner::LowerLeft => 3,
            _ => 0,
        };
        Radii {
            x: self.inner.fRadii[idx].fX,
            y: self.inner.fRadii[idx].fY,
        }
    }

    /// 数据是否有效。Returns true if valid.
    pub fn is_valid(&self) -> bool {
        ffi::rrect_is_valid(&self.inner)
    }

    /// 从 cxx 桥接结构创建（仅 crate 内使用）。Internal use only.
    pub(crate) fn from_ffi(inner: ffi::RRect) -> Self {
        Self { inner }
    }

    /// 内部 FFI 副本（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_ffi(&self) -> ffi::RRect {
        self.inner
    }
}

impl Default for RRect {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RRect {
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}
