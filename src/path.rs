//! 路径类型，由 move/line/quad/cubic/close 组成的 2D 轮廓。
//! Path type representing 2D contours built from move, line, quad, cubic, and close verbs.

use crate::bridge::ffi;
use crate::matrix::Matrix;
use crate::path_fill_type::PathFillType;
use crate::path_iter::PathIter;
use crate::point::Point;
use crate::rect::Rect;
use crate::{Direction, RectCorner};
use crate::rrect::RRect;
use cxx::UniquePtr;

/// 路径的 safe 封装，底层为 C++ `pk::SkPath`。
/// Safe wrapper around `pk::SkPath`.
pub struct Path {
    inner: UniquePtr<ffi::Path>,
}

impl Path {
    /// 创建空路径。Creates an empty path.
    pub fn new() -> Self {
        Self {
            inner: ffi::path_new(),
        }
    }

    /// 从已有路径复制。Creates a copy of the given path.
    pub fn from_path(other: &Path) -> Self {
        Self {
            inner: ffi::path_clone(other.as_cpp_ref()),
        }
    }

    pub(crate) fn from_unique_ptr(inner: UniquePtr<ffi::Path>) -> Self {
        Self { inner }
    }

    pub(crate) fn as_cpp_ref(&self) -> &ffi::Path {
        self.inner.as_ref().expect("Path")
    }

    pub(crate) fn pin_cpp_mut(&mut self) -> std::pin::Pin<&mut ffi::Path> {
        self.inner.pin_mut()
    }

    /// 清空路径。Empties the path.
    pub fn reset(&mut self) {
        ffi::path_reset(self.pin_cpp_mut());
    }

    /// 清空几何并复位内部字段，与 [`Self::reset`] 类似，但 **保留** 已分配的 `SkPath` 内部缓冲区（适合反复构建同规模路径）。
    /// Same as `SkPath::rewind()`: clears geometry and fields like `reset`, but retains storage for reuse.
    pub fn rewind(&mut self) {
        ffi::path_rewind(self.pin_cpp_mut());
    }

    /// 是否无动词数组（空路径）。与 `SkPath::isEmpty` 一致（`countVerbs() == 0`），注意与「无控制点」不同。
    /// True when the path has no verbs (`countVerbs() == 0`), matching `SkPath::isEmpty` (not the same as zero points).
    pub fn is_empty(&self) -> bool {
        ffi::path_count_verbs(self.as_cpp_ref()) == 0
    }

    /// 点的数量。Returns the number of points in the path.
    pub fn count_points(&self) -> i32 {
        ffi::path_count_points(self.as_cpp_ref())
    }

    /// 动词的数量。Returns the number of verbs (move, line, quad, cubic, close).
    pub fn count_verbs(&self) -> i32 {
        ffi::path_count_verbs(self.as_cpp_ref())
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
            let mut pt = ffi::Point { fX: 0.0, fY: 0.0 };
            ffi::path_get_point(self.as_cpp_ref(), index, &mut pt);
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
        let mut bounds = ffi::Rect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        ffi::path_compute_tight_bounds(self.as_cpp_ref(), &mut bounds);
        bounds.into()
    }

    /// 全部控制点的轴对齐包围盒（`SkPath::getBounds`），含 `move` 点；通常有缓存，直线轮廓下与 [`Self::tight_bounds`] 接近。
    /// AABB of all path points (including moves); `SkPath::getBounds()`. Often cached; for lines, close to [`Self::tight_bounds`].
    pub fn bounds(&self) -> Rect {
        let mut r = ffi::Rect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        ffi::path_get_bounds(self.as_cpp_ref(), &mut r);
        r.into()
    }

    /// 当且仅当所有控制点坐标均为有限值（无 `±∞`、无 NaN）时为真。`SkPath::isFinite`。
    /// True if every stored coordinate is finite (no infinities or NaNs). `SkPath::isFinite`.
    pub fn is_finite(&self) -> bool {
        ffi::path_is_finite(self.as_cpp_ref())
    }

    /// 填充时是否为凸区域（可能按需计算并缓存）。`SkPath::isConvex`。
    /// Whether the filled path is convex (computed lazily if needed). `SkPath::isConvex`.
    pub fn is_convex(&self) -> bool {
        ffi::path_is_convex(self.as_cpp_ref())
    }

    /// 若路径等价于标准椭圆/圆（四段圆锥闭合），返回 `Some` 外接矩形；否则 `None`。`SkPath::isOval`。
    /// `Some(bounds)` if the path is a circle/oval; `None` otherwise. `SkPath::isOval`.
    pub fn is_oval(&self) -> Option<Rect> {
        let mut r = ffi::Rect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        if ffi::path_is_oval(self.as_cpp_ref(), &mut r) {
            Some(r.into())
        } else {
            None
        }
    }

    /// 若路径仅为 `move + line`（一条开线段），返回 `(起点, 终点)`；否则 `None`。`SkPath::isLine`。
    /// If the path is one open line segment (`move` + `line`), returns endpoints; else `None`. `SkPath::isLine`.
    pub fn is_line(&self) -> Option<(Point, Point)> {
        let mut p0 = ffi::Point { fX: 0.0, fY: 0.0 };
        let mut p1 = ffi::Point { fX: 0.0, fY: 0.0 };
        if ffi::path_is_line(self.as_cpp_ref(), &mut p0, &mut p1) {
            Some((p0.into(), p1.into()))
        } else {
            None
        }
    }

    /// 按内部顺序拷贝所有控制点（与 [`Self::get_point`] 下标一致）。`SkPath::getPoints`。
    /// Copies all stored points in order (same indexing as [`Self::get_point`]). `SkPath::getPoints`.
    pub fn points(&self) -> Vec<Point> {
        let mut v = Vec::new();
        ffi::path_get_points_copy(self.as_cpp_ref(), &mut v);
        v.into_iter().map(Point::from).collect()
    }

    /// 拷贝动词序列，每字节为 `pk::SkPathVerb` 枚举底层值（与 [`crate::PathVerb`] 取值一致）。`SkPath::getVerbs`。
    /// Raw verb bytes matching `pk::SkPathVerb` / [`crate::PathVerb`]. `SkPath::getVerbs`.
    pub fn verbs(&self) -> Vec<u8> {
        let mut v = Vec::new();
        ffi::path_get_verbs_copy(self.as_cpp_ref(), &mut v);
        v
    }

    /// 为即将追加的点预留容量以减小程序分配（`extra_pt_count` ≤ 0 时忽略）。`SkPath::incReserve`。
    /// Hints extra point capacity before building; no-op if `extra_pt_count <= 0`. `SkPath::incReserve`.
    pub fn inc_reserve(&mut self, extra_pt_count: i32) {
        ffi::path_inc_reserve(self.pin_cpp_mut(), extra_pt_count);
    }

    /// 是否可与 `other` 做逐点插值（动词种类与点数一致，含圆锥权重）。应先调用再 [`Self::try_interpolate`]。`SkPath::isInterpolatable`。
    /// True if paths can be blended pointwise (verbs/point count/conic weights match). `SkPath::isInterpolatable`.
    pub fn is_interpolatable_with(&self, other: &Path) -> bool {
        ffi::path_is_interpolatable(self.as_cpp_ref(), other.as_cpp_ref())
    }

    /// 在 `self` 与 `ending` 之间插值：`weight` 为 1 偏向起点、0 偏向终点（可超出 \[0,1\]）。不兼容返回 `None`。`SkPath::interpolate`。
    /// Blends points between `self` (`weight` toward start) and `ending`; returns `None` if incompatible. `SkPath::interpolate`.
    pub fn try_interpolate(&self, ending: &Path, weight: f32) -> Option<Path> {
        let mut out = Path::new();
        let ok = ffi::path_interpolate(
            self.as_cpp_ref(),
            ending.as_cpp_ref(),
            weight,
            out.pin_cpp_mut(),
        );
        if ok { Some(out) } else { None }
    }

    /// 点列末尾坐标；无任何点时 `None`。`SkPath::getLastPt`。
    /// Last point in the point array, or `None` if empty. `SkPath::getLastPt`.
    pub fn last_pt(&self) -> Option<Point> {
        let mut p = ffi::Point { fX: 0.0, fY: 0.0 };
        if ffi::path_get_last_pt(self.as_cpp_ref(), &mut p) {
            Some(p.into())
        } else {
            None
        }
    }

    /// 修改最后一个点；若当前无点则等价于 `move_to(x, y)`。`SkPath::setLastPt`。
    /// Updates the last point, or inserts a `moveTo` if the path is empty. `SkPath::setLastPt`.
    pub fn set_last_pt(&mut self, x: f32, y: f32) {
        ffi::path_set_last_pt(self.pin_cpp_mut(), x, y);
    }

    /// 路径中出现过的段类型位或运算结果，与 `pk::SkPathSegmentMask` 一致：`1<<0` line、`1<<1` quad、`1<<2` conic、`1<<3` cubic。
    /// Bitmask of segment kinds present (line/quad/conic/cubic). Matches `pk::SkPathSegmentMask`.
    pub fn segment_masks(&self) -> u32 {
        ffi::path_segment_masks(self.as_cpp_ref())
    }

    /// 是否包含多于一条独立轮廓（多于一个起始 `move` 所隐含的轮廓）。`SkPath::hasMultipleContours`。
    /// True if the path has more than one contour. `SkPath::hasMultipleContours`.
    pub fn has_multiple_contours(&self) -> bool {
        ffi::path_has_multiple_contours(self.as_cpp_ref())
    }

    /// 追加 `src` 的几何，所有点加上 `(dx, dy)`；`extend == true` 时若当前轮廓未闭合会先接一条线再拼接（`kExtend_AddPathMode`）。`SkPath::addPath(src, dx, dy, mode)`。
    /// Appends `src` translated by `(dx, dy)`; `extend` selects append vs extend mode. `SkPath::addPath`.
    pub fn add_path_offset(&mut self, src: &Path, dx: f32, dy: f32, extend: bool) -> &mut Self {
        ffi::path_add_path_offset(self.pin_cpp_mut(), src.as_cpp_ref(), dx, dy, extend);
        self
    }

    /// 将 `src` 的 **第一段** 轮廓逆序追加到本路径（总是新开轮廓）。`SkPath::reverseAddPath`。
    /// Appends the first contour of `src` in reverse. Always starts a new contour. `SkPath::reverseAddPath`.
    pub fn reverse_add_path(&mut self, src: &Path) -> &mut Self {
        ffi::path_reverse_add_path(self.pin_cpp_mut(), src.as_cpp_ref());
        self
    }

    /// 以 O(1) 代价交换两条路径的内容（替换底层 `UniquePtr`，语义等价于 `SkPath::swap`）。
    /// Exchanges path contents by swapping owning pointers (like `SkPath::swap`).
    pub fn swap(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.inner, &mut other.inner);
    }

    /// 按 [`Matrix`] 就地变换所有点与权重（含透视矩阵行为，与 `SkPath::transform` 一致）。
    /// Transforms geometry in place; matches `SkPath::transform(const SkMatrix&, this)`.
    pub fn transform(&mut self, matrix: &Matrix) {
        ffi::path_transform(self.pin_cpp_mut(), matrix.mat.as_slice());
    }

    /// 返回变换后的新路径；`self` 不变。等价于 `SkPath::makeTransform`。
    /// Returns a transformed copy; leaves `self` unchanged. Like `SkPath::makeTransform`.
    pub fn transformed(&self, matrix: &Matrix) -> Path {
        let mut out = Path::new();
        ffi::path_transform_to(self.as_cpp_ref(), matrix.mat.as_slice(), out.pin_cpp_mut());
        out
    }

    /// 最后一段轮廓是否闭合。Returns true if the last contour ends with close().
    pub fn is_last_contour_closed(&self) -> bool {
        ffi::path_is_last_contour_closed(self.as_cpp_ref())
    }

    /// 保守判断是否包含矩形（可能将部分内含矩形判为 false）。
    /// Conservatively tests rect containment; may return false for some contained rects.
    ///
    /// 适用于单段凸轮廓路径。Works for single convex contour paths.
    pub fn conservatively_contains_rect(&self, rect: &Rect) -> bool {
        let r: ffi::Rect = (*rect).into();
        ffi::path_conservatively_contains_rect(self.as_cpp_ref(), &r)
    }

    /// 是否可表示为矩形。Returns Some((rect, is_closed)) if path is a rect, None otherwise.
    pub fn is_rect(&self) -> Option<(Rect, bool)> {
        let mut out_rect = ffi::Rect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        let mut is_closed = false;
        let mut direction = Direction::Cw;
        let ok = ffi::path_is_rect(
            self.as_cpp_ref(),
            &mut out_rect,
            &mut is_closed,
            &mut direction,
        );
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
        ffi::path_contains(self.as_cpp_ref(), x, y)
    }

    /// 当前填充规则。Current fill rule.
    pub fn fill_type(&self) -> PathFillType {
        ffi::path_fill_type_bits(self.as_cpp_ref())
    }

    /// 设置填充规则。Sets fill rule.
    pub fn set_fill_type(&mut self, ft: PathFillType) {
        ffi::path_set_fill_type_bits(self.pin_cpp_mut(), ft);
    }

    /// 是否为反色填充（`InverseWinding` / `InverseEvenOdd`）。
    /// True if fill type is inverse winding or inverse even-odd.
    pub fn is_inverse_fill_type(&self) -> bool {
        self.fill_type().is_inverse()
    }

    /// 在「普通 / 反色」之间切换（`Winding` ↔ `InverseWinding`，`EvenOdd` ↔ `InverseEvenOdd`）。
    /// Toggles between normal and inverse fill (winding/even-odd pairs).
    pub fn toggle_inverse_fill_type(&mut self) {
        ffi::path_toggle_inverse_fill_type(self.pin_cpp_mut());
    }

    // ---------- 构建方法 / Construction methods ----------

    /// 移动到 (x, y)，开始新轮廓。Moves to (x, y) and starts a new contour.
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        ffi::path_move_to(self.pin_cpp_mut(), x, y);
        self
    }

    /// 画线到 (x, y)。Adds a line from current point to (x, y).
    ///
    /// 需先调用 `move_to`；否则 Skia 以 (0, 0) 为隐式起点。
    /// Requires prior `move_to`; otherwise Skia uses (0, 0) as implicit start.
    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        ffi::path_line_to(self.pin_cpp_mut(), x, y);
        self
    }

    /// 二次贝塞尔曲线。Adds a quadratic bezier (control point, end point).
    pub fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        ffi::path_quad_to(self.pin_cpp_mut(), x1, y1, x2, y2);
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
        ffi::path_cubic_to(self.pin_cpp_mut(), x1, y1, x2, y2, x3, y3);
        self
    }

    /// 从当前点到 `(x2,y2)`，控制点为 `(x1,y1)`，圆锥权重 `w`（`w==1` 时可能降为二次曲线）。`SkPath::conicTo`。
    /// Conic from current point to `(x2,y2)` with control `(x1,y1)` and weight `w`. `SkPath::conicTo`.
    pub fn conic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, w: f32) -> &mut Self {
        ffi::path_conic_to(self.pin_cpp_mut(), x1, y1, x2, y2, w);
        self
    }

    /// 在 `(x1,y1)`、`(x2,y2)` 与 `radius` 约束下追加与当前点相切的圆弧（PostScript `arct`）。需已有轮廓起点。`SkPath::arcTo`。
    /// Tangent arc through the tangent at `(x1,y1)` toward `(x2,y2)` with `radius` (PostScript `arct`). `SkPath::arcTo`.
    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) -> &mut Self {
        ffi::path_arc_to(self.pin_cpp_mut(), x1, y1, x2, y2, radius);
        self
    }

    /// 以 `pts[0]` 为起点依次连线；`pts` 为空时行为与 Skia 一致（仅 `move` 等）；`close == true` 时闭合轮廓。`SkPath::addPoly`。
    /// Polyline from `pts[0]`; empty slice uses Skia `addPoly` semantics; optional close. `SkPath::addPoly`.
    pub fn add_poly(&mut self, pts: &[Point], close: bool) -> &mut Self {
        let ffi_pts: Vec<ffi::Point> = pts.iter().copied().map(Into::into).collect();
        ffi::path_add_poly(self.pin_cpp_mut(), ffi_pts.as_slice(), close);
        self
    }

    /// 闭合当前轮廓。Closes the current contour (line back to first point).
    pub fn close(&mut self) -> &mut Self {
        ffi::path_close(self.pin_cpp_mut());
        self
    }

    /// 添加矩形。Adds a rectangle as a closed contour.
    pub fn add_rect(&mut self, rect: &Rect, dir: Direction, start: RectCorner) -> &mut Self {
        let r: ffi::Rect = (*rect).into();
        ffi::path_add_rect(self.pin_cpp_mut(), &r, dir, start);
        self
    }

    /// 添加椭圆（由矩形包围）。Adds an oval (ellipse) bounded by the given rect.
    pub fn add_oval(&mut self, rect: &Rect, dir: Direction) -> &mut Self {
        let r: ffi::Rect = (*rect).into();
        ffi::path_add_oval(self.pin_cpp_mut(), &r, dir);
        self
    }

    /// 添加椭圆，起始点由 `start`（四角之一）与 `dir` 决定，与矩形 `addRect` 角语义一致。`SkPath::addOval(rect, dir, start)`。
    /// Adds an oval starting at corner `start`, wound per `dir`. `SkPath::addOval(rect, dir, start)`.
    pub fn add_oval_with_start(
        &mut self,
        rect: &Rect,
        dir: Direction,
        start: RectCorner,
    ) -> &mut Self {
        let r: ffi::Rect = (*rect).into();
        ffi::path_add_oval_start(self.pin_cpp_mut(), &r, dir, start);
        self
    }

    /// 添加圆。Adds a circle centered at (cx, cy) with given radius.
    ///
    /// `radius` 应 ≥ 0；负值时 Skia 行为未定义。
    /// `radius` should be ≥ 0; negative values have undefined Skia behavior.
    pub fn add_circle(&mut self, cx: f32, cy: f32, radius: f32, dir: Direction) -> &mut Self {
        ffi::path_add_circle(self.pin_cpp_mut(), cx, cy, radius, dir);
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
        let r: ffi::Rect = (*rect).into();
        ffi::path_add_round_rect(self.pin_cpp_mut(), &r, rx, ry, dir);
        self
    }

    /// 添加 RRect（支持四角独立半径）。Adds RRect with per-corner radii.
    pub fn add_rrect(&mut self, rrect: &RRect, dir: Direction) -> &mut Self {
        let rr = rrect.as_ffi();
        ffi::path_add_rrect(self.pin_cpp_mut(), &rr, dir);
        self
    }

    /// 添加 RRect 并指定起始角。Adds RRect with start corner.
    pub fn add_rrect_with_start(
        &mut self,
        rrect: &RRect,
        dir: Direction,
        start: RectCorner,
    ) -> &mut Self {
        let rr = rrect.as_ffi();
        ffi::path_add_rrect_start(self.pin_cpp_mut(), &rr, dir, start);
        self
    }

    /// 路径是否可表示为 RRect。Returns Some(rrect) if path is an RRect, None otherwise.
    pub fn is_rrect(&self) -> Option<RRect> {
        let mut out = ffi::RRect {
            fRect: ffi::Rect {
                fLeft: 0.0,
                fTop: 0.0,
                fRight: 0.0,
                fBottom: 0.0,
            },
            fRadii: [ffi::Point { fX: 0.0, fY: 0.0 }; 4],
            fType: ffi::RRectType::Empty,
        };
        ffi::rrect_new_empty(&mut out);
        let ok = ffi::path_is_rrect(self.as_cpp_ref(), &mut out);
        if ok {
            Some(RRect::from_ffi(out))
        } else {
            None
        }
    }

    /// 内部 `ffi::Path` 引用（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_raw(&self) -> &ffi::Path {
        self.as_cpp_ref()
    }

    /// 内部 `ffi::Path` 可变 Pin（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_raw_pin_mut(&mut self) -> std::pin::Pin<&mut ffi::Path> {
        self.pin_cpp_mut()
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

/// 路径相等性，与 `pk::SkPath::operator==` 一致（填充类型 + 动词/点/圆锥权重数据）。
/// Path equality per `pk::SkPath::operator==` (fill type and path ref data).
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        ffi::path_equals(self.as_cpp_ref(), other.as_cpp_ref())
    }
}

impl Eq for Path {}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Path")
            .field("fill_type", &self.fill_type())
            .field("points", &self.count_points())
            .field("verbs", &self.count_verbs())
            .field("bounds", &self.bounds())
            .finish()
    }
}
