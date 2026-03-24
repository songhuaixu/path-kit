//! 圆角矩形，支持四角独立半径。
//! Rounded rectangle with per-corner radii.

use crate::pathkit;
use crate::rect::Rect;
use crate::rect_corner::RectCorner;

/// 圆角矩形，支持四角独立半径。
/// Rounded rectangle with per-corner radii (each corner can have different x/y).
pub struct RRect {
    inner: pathkit::SkRRect,
}

/// 单角半径 (x, y)。Corner radii (x, y).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radii {
    pub x: f32,
    pub y: f32,
}

impl RRect {
    /// 创建空圆角矩形。Creates an empty RRect.
    pub fn new() -> Self {
        Self {
            inner: pathkit::SkRRect {
                fRect: pathkit::SkRect {
                    fLeft: 0.0,
                    fTop: 0.0,
                    fRight: 0.0,
                    fBottom: 0.0,
                },
                fRadii: [
                    pathkit::SkPoint { fX: 0.0, fY: 0.0 },
                    pathkit::SkPoint { fX: 0.0, fY: 0.0 },
                    pathkit::SkPoint { fX: 0.0, fY: 0.0 },
                    pathkit::SkPoint { fX: 0.0, fY: 0.0 },
                ],
                fType: pathkit::SkRRect_Type::kEmpty_Type as i32,
            },
        }
    }

    /// 从矩形创建（直角）。Creates RRect from rect (sharp corners).
    pub fn from_rect(rect: &Rect) -> Self {
        let mut rr = Self::new();
        let r: pathkit::SkRect = (*rect).into();
        unsafe {
            rr.inner.setRectXY(&r as *const _, 0.0, 0.0);
        }
        rr
    }

    /// 从椭圆边界创建。Creates oval (ellipse) bounded by rect.
    pub fn from_oval(rect: &Rect) -> Self {
        let mut rr = Self::new();
        let r: pathkit::SkRect = (*rect).into();
        unsafe {
            rr.inner.setOval(&r as *const _);
        }
        rr
    }

    /// 从矩形与统一圆角创建。Creates rounded rect with uniform radii.
    pub fn from_rect_xy(rect: &Rect, rx: f32, ry: f32) -> Self {
        let mut rr = Self::new();
        let r: pathkit::SkRect = (*rect).into();
        unsafe {
            rr.inner.setRectXY(&r as *const _, rx, ry);
        }
        rr
    }

    /// 从矩形与四角半径创建。Creates RRect with per-corner radii.
    ///
    /// `radii` 顺序：UpperLeft, UpperRight, LowerRight, LowerLeft。
    /// Radii order: UpperLeft, UpperRight, LowerRight, LowerLeft.
    pub fn from_rect_radii(rect: &Rect, radii: &[Radii; 4]) -> Self {
        let mut rr = Self::new();
        let r: pathkit::SkRect = (*rect).into();
        let r4: [pathkit::SkPoint; 4] = [
            pathkit::SkPoint {
                fX: radii[0].x,
                fY: radii[0].y,
            },
            pathkit::SkPoint {
                fX: radii[1].x,
                fY: radii[1].y,
            },
            pathkit::SkPoint {
                fX: radii[2].x,
                fY: radii[2].y,
            },
            pathkit::SkPoint {
                fX: radii[3].x,
                fY: radii[3].y,
            },
        ];
        unsafe {
            rr.inner.setRectRadii(&r as *const _, r4.as_ptr());
        }
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
        self.inner.fType == pathkit::SkRRect_Type::kEmpty_Type as i32
    }

    /// 是否为普通矩形。Returns true if sharp corners (no rounding).
    pub fn is_rect(&self) -> bool {
        self.inner.fType == pathkit::SkRRect_Type::kRect_Type as i32
    }

    /// 是否为椭圆。Returns true if oval.
    pub fn is_oval(&self) -> bool {
        self.inner.fType == pathkit::SkRRect_Type::kOval_Type as i32
    }

    /// 是否所有角半径相同。Returns true if uniform radii (simple type).
    pub fn is_simple(&self) -> bool {
        self.inner.fType == pathkit::SkRRect_Type::kSimple_Type as i32
    }

    /// 获取某角半径。Returns radii for corner (UpperLeft, UpperRight, LowerRight, LowerLeft).
    pub fn radii(&self, corner: RectCorner) -> Radii {
        let idx = corner as usize;
        Radii {
            x: self.inner.fRadii[idx].fX,
            y: self.inner.fRadii[idx].fY,
        }
    }

    /// 数据是否有效。Returns true if valid.
    pub fn is_valid(&self) -> bool {
        unsafe { self.inner.isValid() }
    }

    /// 从 SkRRect 创建（仅 crate 内使用）。Internal use only.
    pub(crate) fn from_raw(inner: pathkit::SkRRect) -> Self {
        Self { inner }
    }

    /// 内部 SkRRect 引用（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_raw(&self) -> &pathkit::SkRRect {
        &self.inner
    }
}

impl Default for RRect {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RRect {
    fn clone(&self) -> Self {
        Self {
            inner: pathkit::SkRRect {
                fRect: self.inner.fRect,
                fRadii: self.inner.fRadii,
                fType: self.inner.fType,
            },
        }
    }
}
