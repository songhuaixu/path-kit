//! 圆角路径效果。Corner path effect.

use crate::bridge::ffi;
use crate::path::Path;
use crate::stroke_rec::StrokeRec;
use cxx::UniquePtr;

/// 圆角路径效果，将尖角变为圆角。
/// Corner path effect - rounds sharp corners.
pub struct CornerPathEffect {
    inner: UniquePtr<ffi::PathEffectHolder>,
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
        let inner = ffi::corner_effect_make(radius);
        if inner.is_null() {
            return None;
        }
        Some(Self { inner })
    }

    /// 应用到路径。Applies effect to path.
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
