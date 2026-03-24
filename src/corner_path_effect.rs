//! 圆角路径效果。Corner path effect.
//!
//! ⚠️ **实验性 / Experimental**: sk_sp 生命周期与 Rust 集成尚不完善，使用时需注意。
//! ⚠️ **Experimental**: sk_sp lifecycle integration with Rust is incomplete; use with caution.

use crate::path::Path;
use crate::pathkit;

/// 圆角路径效果，将尖角变为圆角。
/// Corner path effect - rounds sharp corners.
pub struct CornerPathEffect {
    inner: pathkit::sk_sp<pathkit::SkPathEffect>,
}

impl CornerPathEffect {
    /// 创建圆角效果。Creates a corner effect.
    ///
    /// `radius`: 圆角半径，必须 > 0
    /// `radius`: corner radius, must be > 0
    pub fn new(radius: f32) -> Option<Self> {
        if radius <= 0.0 {
            return None;
        }
        let inner = unsafe { pathkit::SkCornerPathEffect_Make(radius) };
        if inner.fPtr.is_null() {
            return None;
        }
        Some(Self { inner })
    }

    /// 应用到路径。Applies effect to path.
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
