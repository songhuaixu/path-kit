//! 描边参数。Stroke parameters for path outlining.

use crate::path::Path;
use crate::pathkit;

/// 线端样式。Line cap style - how stroke ends are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrokeCap {
    /// 平头 / Butt (no cap)
    #[default]
    Butt = 0,
    /// 圆头 / Round cap
    Round = 1,
    /// 方头 / Square cap
    Square = 2,
}

/// 转角连接样式。Line join style - how stroke corners are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrokeJoin {
    /// 尖角 / Miter join (sharp)
    #[default]
    Miter = 0,
    /// 圆角 / Round join
    Round = 1,
    /// 斜切 / Bevel join
    Bevel = 2,
}

impl From<StrokeCap> for pathkit::SkPaint_Cap::Type {
    fn from(c: StrokeCap) -> Self {
        match c {
            StrokeCap::Butt => pathkit::SkPaint_Cap::kButt_Cap,
            StrokeCap::Round => pathkit::SkPaint_Cap::kRound_Cap,
            StrokeCap::Square => pathkit::SkPaint_Cap::kSquare_Cap,
        }
    }
}

impl From<u32> for StrokeCap {
    fn from(v: u32) -> Self {
        match v {
            1 => StrokeCap::Round,
            2 => StrokeCap::Square,
            _ => StrokeCap::Butt,
        }
    }
}

impl From<StrokeJoin> for pathkit::SkPaint_Join::Type {
    fn from(j: StrokeJoin) -> Self {
        match j {
            StrokeJoin::Miter => pathkit::SkPaint_Join::kMiter_Join,
            StrokeJoin::Round => pathkit::SkPaint_Join::kRound_Join,
            StrokeJoin::Bevel => pathkit::SkPaint_Join::kBevel_Join,
        }
    }
}

impl From<u32> for StrokeJoin {
    fn from(v: u32) -> Self {
        match v {
            1 => StrokeJoin::Round,
            2 => StrokeJoin::Bevel,
            _ => StrokeJoin::Miter,
        }
    }
}

/// 描边样式。Stroke style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeStyle {
    /// 极细线 / Hairline (1 pixel)
    Hairline,
    /// 填充 / Fill (no stroke)
    Fill,
    /// 描边 / Stroke with width
    Stroke { width: f32, stroke_and_fill: bool },
    /// 描边+填充 / Stroke and fill
    StrokeAndFill { width: f32 },
}

/// 描边参数，描述如何将路径转为描边轮廓。
/// Stroke parameters - describes how to expand a path to a stroke outline.
///
/// 用于 `apply_to_path` 将路径转换为描边后的填充路径。
/// Used with `apply_to_path` to convert a path to its stroked fill equivalent.
pub struct StrokeRec {
    inner: pathkit::SkStrokeRec,
}

impl StrokeRec {
    /// 创建填充样式（无描边）。Creates fill style (no stroke).
    pub fn new_fill() -> Self {
        Self {
            inner: unsafe {
                pathkit::SkStrokeRec::new(pathkit::SkStrokeRec_InitStyle::kFill_InitStyle)
            },
        }
    }

    /// 创建极细线样式。Creates hairline style (1-pixel stroke).
    pub fn new_hairline() -> Self {
        Self {
            inner: unsafe {
                pathkit::SkStrokeRec::new(pathkit::SkStrokeRec_InitStyle::kHairline_InitStyle)
            },
        }
    }

    /// 创建描边样式。Creates stroke style with given width.
    pub fn new_stroke(width: f32, stroke_and_fill: bool) -> Self {
        let mut rec = Self::new_hairline();
        unsafe {
            rec.inner.setStrokeStyle(width, stroke_and_fill);
        }
        rec
    }

    /// 设置为填充样式。Sets to fill style.
    pub fn set_fill(&mut self) {
        unsafe {
            self.inner.setFillStyle();
        }
    }

    /// 设置为极细线。Sets to hairline style.
    pub fn set_hairline(&mut self) {
        unsafe {
            self.inner.setHairlineStyle();
        }
    }

    /// 设置描边宽度。Sets stroke width (width=0 may switch to fill/hairline).
    pub fn set_stroke_style(&mut self, width: f32, stroke_and_fill: bool) {
        unsafe {
            self.inner.setStrokeStyle(width, stroke_and_fill);
        }
    }

    /// 设置线端样式。Sets line cap (Butt, Round, Square).
    pub fn set_cap(&mut self, cap: StrokeCap) {
        self.inner.set_fCap(cap as u32);
    }

    /// 设置转角连接样式。Sets line join (Miter, Round, Bevel).
    pub fn set_join(&mut self, join: StrokeJoin) {
        self.inner.set_fJoin(join as u32);
    }

    /// 设置描边端点与转角参数。Sets cap, join, and miter limit.
    pub fn set_stroke_params(&mut self, cap: StrokeCap, join: StrokeJoin, miter_limit: f32) {
        self.inner.set_fCap(cap as u32);
        self.inner.set_fJoin(join as u32);
        self.inner.fMiterLimit = miter_limit;
    }

    /// 线端样式。Returns line cap.
    pub fn cap(&self) -> StrokeCap {
        StrokeCap::from(self.inner.fCap())
    }

    /// 转角连接样式。Returns line join.
    pub fn join(&self) -> StrokeJoin {
        StrokeJoin::from(self.inner.fJoin())
    }

    /// Miter 限制（尖角最长延伸比）。Returns miter limit.
    pub fn miter_limit(&self) -> f32 {
        self.inner.fMiterLimit
    }

    /// 当前样式。Returns current style (hairline/fill/stroke).
    pub fn style(&self) -> StrokeStyle {
        let raw = unsafe { self.inner.getStyle() };
        match raw {
            pathkit::SkStrokeRec_Style::kHairline_Style => StrokeStyle::Hairline,
            pathkit::SkStrokeRec_Style::kFill_Style => StrokeStyle::Fill,
            pathkit::SkStrokeRec_Style::kStroke_Style => StrokeStyle::Stroke {
                width: self.inner.fWidth,
                stroke_and_fill: false,
            },
            pathkit::SkStrokeRec_Style::kStrokeAndFill_Style => StrokeStyle::StrokeAndFill {
                width: self.inner.fWidth,
            },
            _ => StrokeStyle::Fill,
        }
    }

    /// 描边宽度。Returns stroke width.
    pub fn width(&self) -> f32 {
        self.inner.fWidth
    }

    /// 膨胀半径（用于边界计算）。Inflation radius for bounds.
    pub fn inflation_radius(&self) -> f32 {
        unsafe { self.inner.getInflationRadius() }
    }

    /// 将描边参数应用到路径，返回描边后的填充路径。
    /// Applies stroke to path, returning the stroked outline as a fill path.
    ///
    /// 若样式为 fill/hairline 则返回 None。
    /// Returns None if style is fill or hairline (no expansion).
    pub fn apply_to_path(&self, path: &Path) -> Option<Path> {
        let mut dst = Path::new();
        let ok = unsafe {
            pathkit::SkStrokeRec_applyToPath(
                &self.inner as *const _,
                dst.as_raw_mut() as *mut _,
                path.as_raw() as *const _,
            )
        };
        if ok {
            Some(dst)
        } else {
            None
        }
    }

    /// 内部 SkStrokeRec 引用（仅 crate 内使用，如 PathEffect filterPath）。Internal use only.
    #[allow(dead_code)]
    pub(crate) fn as_raw(&self) -> &pathkit::SkStrokeRec {
        &self.inner
    }

    /// 内部 SkStrokeRec 可变引用（仅 crate 内使用，如 PathEffect filterPath）。Internal use only.
    #[allow(dead_code)]
    pub(crate) fn as_raw_mut(&mut self) -> &mut pathkit::SkStrokeRec {
        &mut self.inner
    }
}

impl Default for StrokeRec {
    fn default() -> Self {
        Self::new_fill()
    }
}
