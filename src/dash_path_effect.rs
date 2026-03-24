//! 虚线路径效果。Dash path effect.

use crate::bridge::ffi;
use crate::path::Path;
use crate::stroke_rec::StrokeRec;
use cxx::UniquePtr;

/// 虚线路径效果，将路径描边转为虚线。
/// Dash path effect for stroked paths (on/off intervals).
pub struct DashPathEffect {
    inner: UniquePtr<ffi::PathEffectHolder>,
}

impl DashPathEffect {
    /// 创建虚线效果。Creates a dash effect.
    ///
    /// `intervals`: 交替的 on/off 长度，如 [10.0, 20.0] 表示 10 像素 on、20 像素 off
    /// `intervals`: on/off lengths, e.g. [10.0, 20.0] = 10px on, 20px off
    /// `phase`: 相位偏移
    /// `phase`: phase offset into the pattern
    pub fn new(intervals: &[f32], phase: f32) -> Option<Self> {
        if intervals.is_empty() || !intervals.len().is_multiple_of(2) {
            return None;
        }
        let inner = ffi::dash_effect_make(intervals, phase);
        if inner.is_null() {
            return None;
        }
        Some(Self { inner })
    }

    /// 应用到路径。Applies effect to path (requires stroke width).
    ///
    /// `path`: 输入路径
    /// `stroke_width`: 描边宽度（虚线仅对描边路径有效）
    /// `path`: input path
    /// `stroke_width`: stroke width (dash only affects stroked paths)
    ///
    /// 失败时返回 `None`。Returns `None` on failure.
    pub fn filter_path(&self, path: &Path, stroke_width: f32) -> Option<Path> {
        let mut dst = Path::new();
        let mut rec = StrokeRec::new_stroke(stroke_width, false);
        let bounds = path.tight_bounds();
        let cull: ffi::Rect = bounds.into();
        let ok = ffi::path_effect_filter(
            self.inner.as_ref().expect("PathEffect"),
            dst.as_raw_pin_mut(),
            path.as_raw(),
            rec.pin_holder_mut(),
            &cull,
        );
        if ok {
            Some(dst)
        } else {
            None
        }
    }
}
