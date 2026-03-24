//! 绘图参数。Paint parameters for path stroking/filling.

use crate::path::Path;
use crate::stroke_rec::{StrokeCap, StrokeJoin};
use crate::pathkit;

/// 绘图样式。Paint style - fill, stroke, or both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PaintStyle {
    /// 填充 / Fill
    #[default]
    Fill = 0,
    /// 描边 / Stroke
    Stroke = 1,
    /// 描边并填充 / Stroke and fill
    StrokeAndFill = 2,
}

impl From<PaintStyle> for pathkit::SkPaint_Style::Type {
    fn from(s: PaintStyle) -> Self {
        match s {
            PaintStyle::Fill => pathkit::SkPaint_Style::kFill_Style,
            PaintStyle::Stroke => pathkit::SkPaint_Style::kStroke_Style,
            PaintStyle::StrokeAndFill => pathkit::SkPaint_Style::kStrokeAndFill_Style,
        }
    }
}

/// 绘图参数，控制路径的填充/描边及描边样式。
/// Paint - controls fill/stroke style and stroke parameters for path rendering.
///
/// 用于 `get_fill_path` 将路径转换为描边后的填充路径（类似 StrokeRec，但包含 Style）。
/// Used with `get_fill_path` to convert path to stroked fill path (like StrokeRec, but includes Style).
pub struct Paint {
    inner: pathkit::SkPaint,
}

impl Paint {
    /// 创建默认绘图参数（填充样式）。Creates default paint (fill style).
    pub fn new() -> Self {
        Self {
            inner: unsafe { pathkit::SkPaint::new() },
        }
    }

    /// 设置为填充样式。Sets to fill style.
    pub fn set_fill(&mut self) {
        unsafe {
            self.inner.setStyle(pathkit::SkPaint_Style::kFill_Style);
        }
    }

    /// 设置为描边样式。Sets to stroke style.
    pub fn set_stroke(&mut self, enable: bool) {
        unsafe {
            self.inner.setStroke(enable);
        }
    }

    /// 设置绘图样式。Sets paint style.
    pub fn set_style(&mut self, style: PaintStyle) {
        unsafe {
            self.inner.setStyle(style.into());
        }
    }

    /// 设置描边宽度。Sets stroke width (0 = hairline).
    pub fn set_stroke_width(&mut self, width: f32) {
        unsafe {
            self.inner.setStrokeWidth(width);
        }
    }

    /// 设置 Miter 限制。Sets miter limit for sharp corners.
    pub fn set_stroke_miter(&mut self, miter: f32) {
        unsafe {
            self.inner.setStrokeMiter(miter);
        }
    }

    /// 设置线端样式。Sets line cap.
    pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
        unsafe {
            self.inner.setStrokeCap(cap.into());
        }
    }

    /// 设置转角连接样式。Sets line join.
    pub fn set_stroke_join(&mut self, join: StrokeJoin) {
        unsafe {
            self.inner.setStrokeJoin(join.into());
        }
    }

    /// 将路径转为填充等效路径。
    /// Converts path to filled equivalent (applies stroke/style).
    ///
    /// Fill 样式返回原路径副本；Stroke 样式返回描边后的填充路径。
    /// Fill style returns copy of path; stroke style returns stroked fill path.
    pub fn get_fill_path(&self, path: &Path) -> Option<Path> {
        let mut dst = Path::new();
        let ok = unsafe {
            self.inner.getFillPath(
                path.as_raw() as *const _,
                dst.as_raw_mut() as *mut _,
                std::ptr::null(),
                1.0,
            )
        };
        if ok {
            Some(dst)
        } else {
            None
        }
    }

    /// 内部 SkPaint 引用（仅 crate 内使用）。Internal use only.
    pub(crate) fn as_raw(&self) -> &pathkit::SkPaint {
        &self.inner
    }

    /// 内部 SkPaint 可变引用（仅 crate 内使用）。Internal use only.
    #[allow(dead_code)]
    pub(crate) fn as_raw_mut(&mut self) -> &mut pathkit::SkPaint {
        &mut self.inner
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
            inner: unsafe { pathkit::SkPaint::new1(self.as_raw() as *const _) },
        }
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe {
            self.inner.destruct();
        }
    }
}
