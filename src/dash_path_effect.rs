//! 虚线路径效果。Dash path effect.
//!
//! ⚠️ **实验性 / Experimental**: sk_sp 生命周期与 Rust 集成尚不完善，使用时需注意。
//! ⚠️ **Experimental**: sk_sp lifecycle integration with Rust is incomplete; use with caution.

use crate::path::Path;
use crate::pathkit;

/// 虚线路径效果，将路径描边转为虚线。
/// Dash path effect for stroked paths (on/off intervals).
pub struct DashPathEffect {
    inner: pathkit::sk_sp<pathkit::SkPathEffect>,
}

impl DashPathEffect {
    /// 创建虚线效果。Creates a dash effect.
    ///
    /// `intervals`: 交替的 on/off 长度，如 [10.0, 20.0] 表示 10 像素 on、20 像素 off
    /// `intervals`: on/off lengths, e.g. [10.0, 20.0] = 10px on, 20px off
    /// `phase`: 相位偏移
    /// `phase`: phase offset into the pattern
    pub fn new(intervals: &[f32], phase: f32) -> Option<Self> {
        if intervals.is_empty() || intervals.len() % 2 != 0 {
            return None;
        }
        let inner = unsafe {
            pathkit::SkDashPathEffect_Make(
                intervals.as_ptr(),
                intervals.len() as i32,
                phase,
            )
        };
        if inner.fPtr.is_null() {
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
    pub fn filter_path(&self, path: &Path, stroke_width: f32) -> Option<Path> {
        let mut dst = Path::new();
        let mut rec = unsafe {
            pathkit::SkStrokeRec::new(pathkit::SkStrokeRec_InitStyle::kHairline_InitStyle)
        };
        unsafe {
            rec.setStrokeStyle(stroke_width, false);
        }
        let bounds = path.tight_bounds();
        let cull: pathkit::SkRect = bounds.into();
        let ok = unsafe {
            pathkit::SkPathEffect_filterPath(
                self.inner.fPtr,
                dst.as_raw_mut() as *mut _,
                path.as_raw() as *const _,
                &mut rec as *mut _,
                &cull as *const _,
            )
        };
        if ok {
            Some(dst)
        } else {
            None
        }
    }
}
