//! 绘图参数。Paint parameters for path stroking/filling.

use crate::bridge::ffi;
use crate::path::Path;
use crate::stroke_rec::{StrokeCap, StrokeJoin};
use cxx::UniquePtr;

pub use crate::bridge::ffi::PaintStyle;

impl Default for PaintStyle {
    fn default() -> Self {
        PaintStyle::Fill
    }
}

/// 绘图参数，控制路径的填充/描边及描边样式。
/// Paint - controls fill/stroke style and stroke parameters for path rendering.
///
/// 用于 `get_fill_path` 将路径转换为描边后的填充路径（类似 StrokeRec，但包含 Style）。
/// Used with `get_fill_path` to convert path to stroked fill path (like StrokeRec, but includes Style).
pub struct Paint {
    inner: UniquePtr<ffi::PaintHolder>,
}

impl Paint {
    /// 创建默认绘图参数（填充样式）。Creates default paint (fill style).
    pub fn new() -> Self {
        Self {
            inner: ffi::paint_new(),
        }
    }

    /// 设置为填充样式。Sets to fill style.
    pub fn set_fill(&mut self) {
        ffi::paint_set_fill(self.inner.pin_mut());
    }

    /// 设置为描边样式。Sets to stroke style.
    pub fn set_stroke(&mut self, enable: bool) {
        ffi::paint_set_stroke(self.inner.pin_mut(), enable);
    }

    /// 设置绘图样式。Sets paint style.
    pub fn set_style(&mut self, style: PaintStyle) {
        ffi::paint_set_style(self.inner.pin_mut(), style);
    }

    /// 设置描边宽度。Sets stroke width (0 = hairline).
    pub fn set_stroke_width(&mut self, width: f32) {
        ffi::paint_set_stroke_width(self.inner.pin_mut(), width);
    }

    /// 设置 Miter 限制。Sets miter limit for sharp corners.
    pub fn set_stroke_miter(&mut self, miter: f32) {
        ffi::paint_set_stroke_miter(self.inner.pin_mut(), miter);
    }

    /// 设置线端样式。Sets line cap.
    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        ffi::paint_set_stroke_cap(self.inner.pin_mut(), cap);
    }

    /// 设置转角连接样式。Sets line join.
    pub fn set_stroke_join(&mut self, join: StrokeJoin) {
        ffi::paint_set_stroke_join(self.inner.pin_mut(), join);
    }

    /// 将路径转为填充等效路径。
    /// Converts path to filled equivalent (applies stroke/style).
    ///
    /// Fill 样式返回原路径副本；Stroke 样式返回描边后的填充路径。
    /// Fill style returns copy of path; stroke style returns stroked fill path.
    pub fn get_fill_path(&self, path: &Path) -> Option<Path> {
        let mut dst = Path::new();
        let ok = ffi::paint_get_fill_path(
            self.inner.as_ref().expect("Paint"),
            path.as_raw(),
            dst.as_raw_pin_mut(),
        );
        if ok {
            Some(dst)
        } else {
            None
        }
    }

    /// 内部引用（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_holder_ref(&self) -> &ffi::PaintHolder {
        self.inner.as_ref().expect("Paint")
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Paint {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::paint_clone(self.as_holder_ref()),
        }
    }
}
