//! 描边参数。Stroke parameters for path outlining.

use crate::bridge::ffi;
use crate::path::Path;
use cxx::UniquePtr;

pub use crate::bridge::ffi::{StrokeCap, StrokeJoin};

impl Default for StrokeCap {
    fn default() -> Self {
        StrokeCap::Butt
    }
}

impl Default for StrokeJoin {
    fn default() -> Self {
        StrokeJoin::Miter
    }
}

/// 描边样式快照，由 `SkStrokeRec::getStyle` 与宽度组合而成。
/// Stroke style snapshot from `SkStrokeRec::getStyle` plus width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeStyle {
    /// 极细线。Hairline stroke.
    Hairline,
    /// 填充（无描边扩展）。Fill only (no stroke expansion).
    Fill,
    /// 描边宽度；`stroke_and_fill` 与原生 `kStroke_Style` 一致时为 false。
    /// Stroke width; `stroke_and_fill` is false for native `kStroke_Style`.
    Stroke {
        /// 描边宽度 / Stroke width
        width: f32,
        /// 是否与 fill 组合（原生 `StrokeAndFill` 为单独枚举变体）。
        /// Combined stroke+fill flag (native uses a separate style for stroke-and-fill).
        stroke_and_fill: bool,
    },
    /// 描边并填充。Stroke and fill.
    StrokeAndFill {
        /// 描边宽度 / Stroke width
        width: f32,
    },
}

/// 描边参数，描述如何将路径转为描边轮廓。
/// Stroke parameters - describes how to expand a path to a stroke outline.
///
/// 用于 `apply_to_path` 将路径转换为描边后的填充路径。
/// Used with `apply_to_path` to convert a path to a stroked fill equivalent.
pub struct StrokeRec {
    inner: UniquePtr<ffi::StrokeRecHolder>,
}

impl StrokeRec {
    /// 创建填充样式（无描边）。Creates fill style (no stroke).
    pub fn new_fill() -> Self {
        Self {
            inner: ffi::stroke_rec_new(ffi::StrokeRecInit::Fill),
        }
    }

    /// 创建极细线样式。Creates hairline style (1-pixel stroke).
    pub fn new_hairline() -> Self {
        Self {
            inner: ffi::stroke_rec_new(ffi::StrokeRecInit::Hairline),
        }
    }

    /// 创建描边样式。Creates stroke style with given width.
    pub fn new_stroke(width: f32, stroke_and_fill: bool) -> Self {
        let mut rec = Self::new_hairline();
        ffi::stroke_rec_set_stroke_style(rec.inner.pin_mut(), width, stroke_and_fill);
        rec
    }

    /// 设置为填充样式。Sets to fill style.
    pub fn set_fill(&mut self) {
        ffi::stroke_rec_set_fill(self.inner.pin_mut());
    }

    /// 设置为极细线。Sets to hairline style.
    pub fn set_hairline(&mut self) {
        ffi::stroke_rec_set_hairline(self.inner.pin_mut());
    }

    /// 设置描边宽度。Sets stroke width (width=0 may switch to fill/hairline).
    pub fn set_stroke_style(&mut self, width: f32, stroke_and_fill: bool) {
        ffi::stroke_rec_set_stroke_style(self.inner.pin_mut(), width, stroke_and_fill);
    }

    /// 设置线端样式。Sets line cap (Butt, Round, Square).
    pub fn set_cap(&mut self, cap: StrokeCap) {
        ffi::stroke_rec_set_cap(self.inner.pin_mut(), cap);
    }

    /// 设置转角连接样式。Sets line join (Miter, Round, Bevel).
    pub fn set_join(&mut self, join: StrokeJoin) {
        ffi::stroke_rec_set_join(self.inner.pin_mut(), join);
    }

    /// 设置描边端点与转角参数。Sets cap, join, and miter limit.
    pub fn set_stroke_params(&mut self, cap: StrokeCap, join: StrokeJoin, miter_limit: f32) {
        ffi::stroke_rec_set_stroke_params(self.inner.pin_mut(), cap, join, miter_limit);
    }

    /// 线端样式。Returns line cap.
    pub fn cap(&self) -> StrokeCap {
        ffi::stroke_rec_cap(self.as_holder_ref())
    }

    /// 转角连接样式。Returns line join.
    pub fn join(&self) -> StrokeJoin {
        ffi::stroke_rec_join(self.as_holder_ref())
    }

    /// Miter 限制（尖角最长延伸比）。Returns miter limit.
    pub fn miter_limit(&self) -> f32 {
        ffi::stroke_rec_miter_limit(self.as_holder_ref())
    }

    /// 当前样式。Returns current style (hairline/fill/stroke).
    pub fn style(&self) -> StrokeStyle {
        match ffi::stroke_rec_get_style(self.as_holder_ref()) {
            ffi::StrokeRecStyleTag::Hairline => StrokeStyle::Hairline,
            ffi::StrokeRecStyleTag::Fill => StrokeStyle::Fill,
            ffi::StrokeRecStyleTag::Stroke => StrokeStyle::Stroke {
                width: ffi::stroke_rec_width(self.as_holder_ref()),
                stroke_and_fill: false,
            },
            ffi::StrokeRecStyleTag::StrokeAndFill => StrokeStyle::StrokeAndFill {
                width: ffi::stroke_rec_width(self.as_holder_ref()),
            },
            _ => StrokeStyle::Fill,
        }
    }

    /// 描边宽度。Returns stroke width.
    pub fn width(&self) -> f32 {
        ffi::stroke_rec_width(self.as_holder_ref())
    }

    /// 膨胀半径（用于边界计算）。Inflation radius for bounds.
    pub fn inflation_radius(&self) -> f32 {
        ffi::stroke_rec_inflation_radius(self.as_holder_ref())
    }

    /// 将描边参数应用到路径，返回描边后的填充路径。
    /// Applies stroke to path, returning the stroked outline as a fill path.
    ///
    /// 若样式为 fill/hairline 则返回 None。
    /// Returns None if style is fill or hairline (no expansion).
    pub fn apply_to_path(&self, path: &Path) -> Option<Path> {
        let mut dst = Path::new();
        let ok = ffi::stroke_rec_apply_to_path(
            self.as_holder_ref(),
            dst.as_raw_pin_mut(),
            path.as_raw(),
        );
        if ok {
            Some(dst)
        } else {
            None
        }
    }

    pub(crate) fn as_holder_ref(&self) -> &ffi::StrokeRecHolder {
        self.inner.as_ref().expect("StrokeRec")
    }

    pub(crate) fn pin_holder_mut(&mut self) -> std::pin::Pin<&mut ffi::StrokeRecHolder> {
        self.inner.pin_mut()
    }
}

impl Default for StrokeRec {
    fn default() -> Self {
        Self::new_fill()
    }
}
