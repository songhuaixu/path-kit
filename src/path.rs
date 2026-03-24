//! 路径类型，由 move/line/quad/cubic/close 组成的 2D 轮廓。
//! Path type representing 2D contours built from move, line, quad, cubic, and close verbs.

use crate::direction::Direction;
use crate::path_fill_type::PathFillType;
use crate::path_iter::PathIter;
use crate::pathkit;
use crate::point::Point;
use crate::rect::Rect;
use crate::rect_corner::RectCorner;
use crate::rrect::RRect;

/// 路径的 safe 封装，对应 SkPath。Safe wrapper for SkPath.
pub struct Path {
    inner: pathkit::SkPath,
}

impl Path {
    /// 创建空路径。Creates an empty path.
    pub fn new() -> Self {
        Self {
            inner: unsafe { pathkit::SkPath::new() },
        }
    }

    /// 从已有路径复制。Creates a copy of the given path.
    pub fn from_path(other: &Path) -> Self {
        Self {
            inner: unsafe { pathkit::SkPath::new1(&other.inner as *const _) },
        }
    }

    /// 清空路径。Empties the path.
    pub fn reset(&mut self) {
        unsafe {
            self.inner.reset();
        }
    }

    /// 是否为空。Returns true if the path has no points.
    pub fn is_empty(&self) -> bool {
        unsafe { self.inner.countPoints() == 0 }
    }

    /// 点的数量。Returns the number of points in the path.
    pub fn count_points(&self) -> i32 {
        unsafe { self.inner.countPoints() }
    }

    /// 动词的数量。Returns the number of verbs (move, line, quad, cubic, close).
    pub fn count_verbs(&self) -> i32 {
        unsafe { self.inner.countVerbs() }
    }

    /// 迭代路径中的动词与点。Iterates over path verbs and points.
    ///
    /// `force_close` 为 true 时，开放轮廓会生成隐式 close。
    /// When `force_close` is true, open contours generate implicit close.
    pub fn iter(&self, force_close: bool) -> PathIter<'_> {
        PathIter::new(self, force_close)
    }

    /// 获取第 i 个点。Returns the point at index, or None if out of range.
    pub fn get_point(&self, index: i32) -> Option<Point> {
        let n = self.count_points();
        if index >= 0 && index < n {
            let pt = unsafe { self.inner.getPoint(index) };
            Some(pt.into())
        } else {
            None
        }
    }

    /// 计算紧密包围盒。Returns the tight axis-aligned bounding box.
    ///
    /// 永不失败；空路径返回 (0,0,0,0)。对复杂曲线，`pathops_tight_bounds` 可能更精确但可能返回 None。
    /// Never fails; empty path returns (0,0,0,0). For complex curves, `pathops_tight_bounds` may be more accurate but can return None.
    pub fn tight_bounds(&self) -> Rect {
        let bounds = unsafe { self.inner.computeTightBounds() };
        bounds.into()
    }

    /// 最后一段轮廓是否闭合。Returns true if the last contour ends with close().
    pub fn is_last_contour_closed(&self) -> bool {
        unsafe { self.inner.isLastContourClosed() }
    }

    /// 保守判断是否包含矩形。May return false for some contained rects.
    /// Works for single convex contour paths.
    pub fn conservatively_contains_rect(&self, rect: &Rect) -> bool {
        let r: pathkit::SkRect = (*rect).into();
        unsafe { self.inner.conservativelyContainsRect(&r) }
    }

    /// 是否可表示为矩形。Returns Some((rect, is_closed)) if path is a rect, None otherwise.
    pub fn is_rect(&self) -> Option<(Rect, bool)> {
        let mut out_rect = pathkit::SkRect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        let mut is_closed = false;
        let mut direction = pathkit::SkPathDirection::kCW;
        let ok = unsafe {
            self.inner.isRect(
                &mut out_rect as *mut _,
                &mut is_closed as *mut _,
                &mut direction as *mut _,
            )
        };
        if ok {
            Some((out_rect.into(), is_closed))
        } else {
            None
        }
    }

    /// 是否包含点。Returns true if (x, y) is inside the filled path.
    ///
    /// 使用当前 [`fill_type`](Self::fill_type)（新建路径默认为 [`PathFillType::Winding`]）。
    /// Uses current [`fill_type`](Self::fill_type) (new paths default to [`PathFillType::Winding`]).
    pub fn contains(&self, x: f32, y: f32) -> bool {
        unsafe { self.inner.contains(x, y) }
    }

    /// 当前填充规则。Current fill rule.
    pub fn fill_type(&self) -> PathFillType {
        PathFillType::from_sk_bits(self.inner.fFillType())
    }

    /// 设置填充规则。Sets fill rule.
    pub fn set_fill_type(&mut self, ft: PathFillType) {
        self.inner.set_fFillType(pathkit::SkPathFillType::Type::from(ft) as u8);
    }

    /// 是否为反色填充（`InverseWinding` / `InverseEvenOdd`）。
    pub fn is_inverse_fill_type(&self) -> bool {
        self.fill_type().is_inverse()
    }

    /// 在「普通 / 反色」之间切换（`Winding` ↔ `InverseWinding`，`EvenOdd` ↔ `InverseEvenOdd`）。
    pub fn toggle_inverse_fill_type(&mut self) {
        let v = self.inner.fFillType();
        self.inner.set_fFillType((v ^ 2) & 3);
    }

    // ---------- 构建方法 / Construction methods ----------

    /// 移动到 (x, y)，开始新轮廓。Moves to (x, y) and starts a new contour.
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        unsafe {
            self.inner.moveTo(x, y);
        }
        self
    }

    /// 画线到 (x, y)。Adds a line from current point to (x, y).
    ///
    /// 需先调用 `move_to`；否则 Skia 以 (0, 0) 为隐式起点。
    /// Requires prior `move_to`; otherwise Skia uses (0, 0) as implicit start.
    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        unsafe {
            self.inner.lineTo(x, y);
        }
        self
    }

    /// 二次贝塞尔曲线。Adds a quadratic bezier (control point, end point).
    pub fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        unsafe {
            self.inner.quadTo(x1, y1, x2, y2);
        }
        self
    }

    /// 三次贝塞尔曲线。Adds a cubic bezier (ctrl1, ctrl2, end point).
    pub fn cubic_to(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> &mut Self {
        unsafe {
            self.inner.cubicTo(x1, y1, x2, y2, x3, y3);
        }
        self
    }

    /// 闭合当前轮廓。Closes the current contour (line back to first point).
    pub fn close(&mut self) -> &mut Self {
        unsafe {
            self.inner.close();
        }
        self
    }

    /// 添加矩形。Adds a rectangle as a closed contour.
    pub fn add_rect(&mut self, rect: &Rect, dir: Direction, start: RectCorner) -> &mut Self {
        let r: pathkit::SkRect = (*rect).into();
        unsafe {
            self.inner.addRect(&r, dir.into(), start.into());
        }
        self
    }

    /// 添加椭圆（由矩形包围）。Adds an oval (ellipse) bounded by the given rect.
    pub fn add_oval(&mut self, rect: &Rect, dir: Direction) -> &mut Self {
        let r: pathkit::SkRect = (*rect).into();
        unsafe {
            self.inner.addOval(&r, dir.into());
        }
        self
    }

    /// 添加圆。Adds a circle centered at (cx, cy) with given radius.
    ///
    /// `radius` 应 ≥ 0；负值时 Skia 行为未定义。
    /// `radius` should be ≥ 0; negative values have undefined Skia behavior.
    pub fn add_circle(&mut self, cx: f32, cy: f32, radius: f32, dir: Direction) -> &mut Self {
        unsafe {
            self.inner.addCircle(cx, cy, radius, dir.into());
        }
        self
    }

    /// 添加圆角矩形。Adds a rounded rectangle (rx, ry = corner radii).
    ///
    /// `rx`, `ry` 应 ≥ 0。Should be ≥ 0.
    pub fn add_round_rect(
        &mut self,
        rect: &Rect,
        rx: f32,
        ry: f32,
        dir: Direction,
    ) -> &mut Self {
        let r: pathkit::SkRect = (*rect).into();
        unsafe {
            self.inner.addRoundRect(&r, rx, ry, dir.into());
        }
        self
    }

    /// 添加 RRect（支持四角独立半径）。Adds RRect with per-corner radii.
    pub fn add_rrect(&mut self, rrect: &RRect, dir: Direction) -> &mut Self {
        unsafe {
            self.inner.addRRect(rrect.as_raw() as *const _, dir.into());
        }
        self
    }

    /// 添加 RRect 并指定起始角。Adds RRect with start corner.
    pub fn add_rrect_with_start(
        &mut self,
        rrect: &RRect,
        dir: Direction,
        start: RectCorner,
    ) -> &mut Self {
        unsafe {
            self.inner.addRRect1(rrect.as_raw() as *const _, dir.into(), start.into());
        }
        self
    }

    /// 路径是否可表示为 RRect。Returns Some(rrect) if path is an RRect, None otherwise.
    pub fn is_rrect(&self) -> Option<RRect> {
        let mut out = pathkit::SkRRect {
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
            fType: 0,
        };
        let ok = unsafe { self.inner.isRRect(&mut out as *mut _) };
        if ok {
            Some(RRect::from_raw(out))
        } else {
            None
        }
    }

    /// 内部 SkPath 引用（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_raw(&self) -> &pathkit::SkPath {
        &self.inner
    }

    /// 内部 SkPath 可变引用（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_raw_mut(&mut self) -> &mut pathkit::SkPath {
        &mut self.inner
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Self::from_path(self)
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Path")
            .field("fill_type", &self.fill_type())
            .field("points", &self.count_points())
            .field("verbs", &self.count_verbs())
            .field("bounds", &self.tight_bounds())
            .finish()
    }
}
