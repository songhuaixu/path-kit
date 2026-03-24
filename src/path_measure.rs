//! 路径测量。PathMeasure - measure path length and sample position/tangent.

use crate::bridge::ffi;
use crate::path::Path;
use crate::point::Point;
use cxx::UniquePtr;

/// 路径测量器，用于获取路径长度、沿路径的位置和切线。
/// PathMeasure - measures path length and samples position/tangent along path.
///
/// 支持多轮廓路径，通过 `next_contour` 遍历。
/// Supports multi-contour paths; use `next_contour` to iterate.
pub struct PathMeasure {
    inner: UniquePtr<ffi::PathMeasureHolder>,
}

impl PathMeasure {
    /// 创建空测量器。Creates empty path measure (no path).
    pub fn new() -> Self {
        Self {
            inner: ffi::measure_new(),
        }
    }

    /// 从路径创建测量器。
    /// Creates path measure from path.
    ///
    /// `force_closed` 为 true 时，开放轮廓按闭合计算长度。
    /// `res_scale` 控制精度，> 1 提高精度（可能变慢）。
    pub fn from_path(path: &Path, force_closed: bool, res_scale: f32) -> Self {
        Self {
            inner: ffi::measure_from_path(path.as_raw(), force_closed, res_scale),
        }
    }

    /// 重置为指定路径。Resets with given path.
    pub fn set_path(&mut self, path: &Path, force_closed: bool) {
        ffi::measure_set_path(self.inner.pin_mut(), path.as_raw(), force_closed);
    }

    /// 当前轮廓的总长度，无路径时返回 0。
    /// Total length of current contour, or 0 if no path.
    pub fn length(&mut self) -> f32 {
        ffi::measure_length(self.inner.pin_mut())
    }

    /// 在指定距离处的位置和切线。
    /// Position and tangent at given distance along path.
    ///
    /// 距离会被限制在 [0, length] 内。
    /// Distance is clamped to [0, length].
    pub fn pos_tan(&mut self, distance: f32) -> Option<(Point, Point)> {
        let mut position = ffi::Point { fX: 0.0, fY: 0.0 };
        let mut tangent = ffi::Point { fX: 0.0, fY: 0.0 };
        let ok = ffi::measure_get_pos_tan(
            self.inner.pin_mut(),
            distance,
            &mut position,
            &mut tangent,
        );
        if ok {
            Some((position.into(), tangent.into()))
        } else {
            None
        }
    }

    /// 提取路径的 [start_d, stop_d] 段。
    /// Extracts path segment between start_d and stop_d.
    ///
    /// `start_with_move_to` 为 true 时，段以 moveTo 开始。
    pub fn get_segment(
        &mut self,
        start_d: f32,
        stop_d: f32,
        dst: &mut Path,
        start_with_move_to: bool,
    ) -> bool {
        ffi::measure_get_segment(
            self.inner.pin_mut(),
            start_d,
            stop_d,
            dst.as_raw_pin_mut(),
            start_with_move_to,
        )
    }

    /// 当前轮廓是否闭合。Whether current contour is closed.
    pub fn is_closed(&mut self) -> bool {
        ffi::measure_is_closed(self.inner.pin_mut())
    }

    /// 移动到下一个轮廓，返回是否还有更多。
    /// Move to next contour; returns true if more exist.
    pub fn next_contour(&mut self) -> bool {
        ffi::measure_next_contour(self.inner.pin_mut())
    }
}

impl Default for PathMeasure {
    fn default() -> Self {
        Self::new()
    }
}
