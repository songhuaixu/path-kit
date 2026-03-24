//! 路径测量。PathMeasure - measure path length and sample position/tangent.

use crate::path::Path;
use crate::point::Point;
use crate::pathkit;

/// 路径测量器，用于获取路径长度、沿路径的位置和切线。
/// PathMeasure - measures path length and samples position/tangent along path.
///
/// 支持多轮廓路径，通过 `next_contour` 遍历。
/// Supports multi-contour paths; use `next_contour` to iterate.
pub struct PathMeasure {
    inner: pathkit::SkPathMeasure,
}

impl PathMeasure {
    /// 创建空测量器。Creates empty path measure (no path).
    pub fn new() -> Self {
        Self {
            inner: unsafe { pathkit::SkPathMeasure::new() },
        }
    }

    /// 从路径创建测量器。
    /// Creates path measure from path.
    ///
    /// `force_closed` 为 true 时，开放轮廓按闭合计算长度。
    /// `res_scale` 控制精度，> 1 提高精度（可能变慢）。
    pub fn from_path(path: &Path, force_closed: bool, res_scale: f32) -> Self {
        let inner = unsafe {
            pathkit::SkPathMeasure::new1(path.as_raw() as *const _, force_closed, res_scale)
        };
        Self { inner }
    }

    /// 重置为指定路径。Resets with given path.
    pub fn set_path(&mut self, path: &Path, force_closed: bool) {
        unsafe {
            self.inner.setPath(path.as_raw() as *const _, force_closed);
        }
    }

    /// 当前轮廓的总长度，无路径时返回 0。
    /// Total length of current contour, or 0 if no path.
    pub fn length(&mut self) -> f32 {
        unsafe { self.inner.getLength() }
    }

    /// 在指定距离处的位置和切线。
    /// Position and tangent at given distance along path.
    ///
    /// 距离会被限制在 [0, length] 内。
    pub fn pos_tan(&mut self, distance: f32) -> Option<(Point, Point)> {
        let mut position = pathkit::SkPoint { fX: 0.0, fY: 0.0 };
        let mut tangent = pathkit::SkPoint { fX: 0.0, fY: 0.0 };
        let ok = unsafe {
            self.inner.getPosTan(
                distance,
                &mut position as *mut _ as *mut pathkit::SkPoint,
                &mut tangent as *mut _ as *mut pathkit::SkVector,
            )
        };
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
    pub fn get_segment(&mut self, start_d: f32, stop_d: f32, dst: &mut Path, start_with_move_to: bool) -> bool {
        unsafe {
            self.inner.getSegment(
                start_d,
                stop_d,
                dst.as_raw_mut() as *mut _,
                start_with_move_to,
            )
        }
    }

    /// 当前轮廓是否闭合。Whether current contour is closed.
    pub fn is_closed(&mut self) -> bool {
        unsafe { self.inner.isClosed() }
    }

    /// 移动到下一个轮廓，返回是否还有更多。
    /// Move to next contour; returns true if more exist.
    pub fn next_contour(&mut self) -> bool {
        unsafe { self.inner.nextContour() }
    }
}

impl Default for PathMeasure {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PathMeasure {
    fn drop(&mut self) {
        unsafe {
            self.inner.destruct();
        }
    }
}
