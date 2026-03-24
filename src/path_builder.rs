//! 路径构建器，基于 `pk::SkPathBuilder`（`snapshot` / `detach` 语义与 Skia 一致）。
//! Path builder backed by `pk::SkPathBuilder` (`snapshot` leaves builder unchanged; `detach` resets it).

use crate::bridge::ffi;
use crate::path::Path;
use crate::path_fill_type::PathFillType;
use crate::rect::Rect;
use crate::rrect::RRect;
use crate::{Direction, RectCorner};
use cxx::UniquePtr;

/// 可增量构建路径；完成后用 [`Self::snapshot`] 或 [`Self::detach`] 得到 [`Path`]。
/// Incremental path construction; finish with [`Self::snapshot`] or [`Self::detach`] into a [`Path`].
pub struct PathBuilder {
    inner: UniquePtr<ffi::PathBuilderHolder>,
}

impl PathBuilder {
    /// 创建空构建器。Creates an empty builder.
    pub fn new() -> Self {
        Self {
            inner: ffi::path_builder_new(),
        }
    }

    /// 清空内容（与 SkPathBuilder::reset 一致）。Clears the builder.
    pub fn reset(&mut self) -> &mut Self {
        ffi::path_builder_reset(self.inner.pin_mut());
        self
    }

    /// 当前填充规则。Fill rule used when producing a path.
    pub fn fill_type(&self) -> PathFillType {
        ffi::path_builder_fill_type(self.as_holder())
    }

    /// 设置填充规则。Sets fill rule for the built path.
    pub fn set_fill_type(&mut self, ft: PathFillType) -> &mut Self {
        ffi::path_builder_set_fill_type(self.inner.pin_mut(), ft);
        self
    }

    /// 在普通 / 反色填充之间切换。Toggles inverse fill (same as SkPathBuilder).
    pub fn toggle_inverse_fill_type(&mut self) -> &mut Self {
        ffi::path_builder_toggle_inverse_fill_type(self.inner.pin_mut());
        self
    }

    /// 生成路径副本，构建器保持不变。Builds a copy; builder is unchanged.
    pub fn snapshot(&self) -> Path {
        Path::from_unique_ptr(ffi::path_builder_snapshot(self.as_holder()))
    }

    /// 取出路径并将构建器重置为空。Takes the path and resets the builder to empty.
    pub fn detach(&mut self) -> Path {
        Path::from_unique_ptr(ffi::path_builder_detach(self.inner.pin_mut()))
    }

    /// 移动到 (x, y)。Moves to (x, y).
    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        ffi::path_builder_move_to(self.inner.pin_mut(), x, y);
        self
    }

    /// 直线到 (x, y)。Line to (x, y).
    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        ffi::path_builder_line_to(self.inner.pin_mut(), x, y);
        self
    }

    /// 二次贝塞尔。Quadratic bezier.
    pub fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> &mut Self {
        ffi::path_builder_quad_to(self.inner.pin_mut(), x1, y1, x2, y2);
        self
    }

    /// 三次贝塞尔。Cubic bezier.
    pub fn cubic_to(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> &mut Self {
        ffi::path_builder_cubic_to(self.inner.pin_mut(), x1, y1, x2, y2, x3, y3);
        self
    }

    /// 闭合当前轮廓。Closes the current contour.
    pub fn close(&mut self) -> &mut Self {
        ffi::path_builder_close(self.inner.pin_mut());
        self
    }

    /// 添加矩形。Adds a rectangle contour.
    pub fn add_rect(&mut self, rect: &Rect, dir: Direction, start: RectCorner) -> &mut Self {
        let r: ffi::Rect = (*rect).into();
        ffi::path_builder_add_rect(self.inner.pin_mut(), &r, dir, start);
        self
    }

    /// 添加椭圆。Adds an oval.
    pub fn add_oval(&mut self, rect: &Rect, dir: Direction) -> &mut Self {
        let r: ffi::Rect = (*rect).into();
        ffi::path_builder_add_oval(self.inner.pin_mut(), &r, dir);
        self
    }

    /// 添加圆。Adds a circle.
    pub fn add_circle(&mut self, cx: f32, cy: f32, radius: f32, dir: Direction) -> &mut Self {
        ffi::path_builder_add_circle(self.inner.pin_mut(), cx, cy, radius, dir);
        self
    }

    /// 添加圆角矩形（统一 rx/ry）。Adds round rect.
    pub fn add_round_rect(
        &mut self,
        rect: &Rect,
        rx: f32,
        ry: f32,
        dir: Direction,
    ) -> &mut Self {
        let r: ffi::Rect = (*rect).into();
        ffi::path_builder_add_round_rect(self.inner.pin_mut(), &r, rx, ry, dir);
        self
    }

    /// 添加 RRect。Adds RRect.
    pub fn add_rrect(&mut self, rrect: &RRect, dir: Direction) -> &mut Self {
        let rr = rrect.as_ffi();
        ffi::path_builder_add_rrect(self.inner.pin_mut(), &rr, dir);
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
        ffi::path_builder_add_rrect_start(self.inner.pin_mut(), &rr, dir, start);
        self
    }

    /// 追加已有路径的几何。Appends another path's geometry.
    pub fn add_path(&mut self, path: &Path) -> &mut Self {
        ffi::path_builder_add_path(self.inner.pin_mut(), path.as_raw());
        self
    }

    fn as_holder(&self) -> &ffi::PathBuilderHolder {
        self.inner.as_ref().expect("PathBuilder")
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}
