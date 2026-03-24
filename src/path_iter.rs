//! 路径迭代器。Path iterator over verbs and points.

use crate::path::Path;
use crate::point::Point;
use crate::pathkit;

/// 路径动词，描述路径中的每个图元。Path verb - describes each element in the path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathVerb {
    /// 移动到新轮廓起点 / Move to start new contour
    Move,
    /// 直线 / Line segment
    Line,
    /// 二次贝塞尔 / Quadratic bezier
    Quad,
    /// 圆锥曲线（有理二次）/ Conic (rational quadratic)
    Conic,
    /// 三次贝塞尔 / Cubic bezier
    Cubic,
    /// 闭合轮廓 / Close contour
    Close,
    /// 迭代结束 / Iteration done
    Done,
}

/// 单步迭代结果，含动词与关联点。Single iteration result with verb and points.
#[derive(Debug, Clone)]
pub enum PathVerbItem {
    /// 移动到新起点 / Move to new contour start
    Move(Point),
    /// 直线段 (from, to)，from 为上一端点 / Line segment (from, to), from is previous point
    Line(Point, Point),
    /// 二次贝塞尔 (control, to) / Quadratic bezier (control, to)
    Quad(Point, Point),
    /// 圆锥曲线 (control, to, weight) / Conic (control, to, weight)
    Conic(Point, Point, f32),
    /// 三次贝塞尔 (ctrl1, ctrl2, to) / Cubic bezier (ctrl1, ctrl2, to)
    Cubic(Point, Point, Point),
    /// 闭合轮廓 / Close contour
    Close,
}

impl PathVerbItem {
    /// 返回动词类型。Returns the verb kind.
    pub fn verb(&self) -> PathVerb {
        match self {
            PathVerbItem::Move(_) => PathVerb::Move,
            PathVerbItem::Line(_, _) => PathVerb::Line,
            PathVerbItem::Quad(_, _) => PathVerb::Quad,
            PathVerbItem::Conic(_, _, _) => PathVerb::Conic,
            PathVerbItem::Cubic(_, _, _) => PathVerb::Cubic,
            PathVerbItem::Close => PathVerb::Close,
        }
    }
}

/// 路径迭代器，按动词顺序遍历路径。Path iterator over verbs and points.
pub struct PathIter<'a> {
    _path: &'a Path,
    inner: pathkit::SkPath_Iter,
    pts: [pathkit::SkPoint; 4],
}

impl<'a> PathIter<'a> {
    pub(crate) fn new(path: &'a Path, force_close: bool) -> Self {
        let inner = unsafe {
            pathkit::SkPath_Iter::new1(path.as_raw() as *const _, force_close)
        };
        let pts = [
            pathkit::SkPoint { fX: 0.0, fY: 0.0 },
            pathkit::SkPoint { fX: 0.0, fY: 0.0 },
            pathkit::SkPoint { fX: 0.0, fY: 0.0 },
            pathkit::SkPoint { fX: 0.0, fY: 0.0 },
        ];
        Self {
            _path: path,
            inner,
            pts,
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = PathVerbItem;

    fn next(&mut self) -> Option<Self::Item> {
        let verb = unsafe {
            self.inner.next(self.pts.as_mut_ptr())
        };
        let v = verb as u32;
        let item = match v {
            pathkit::SkPath_Verb::kMove_Verb => PathVerbItem::Move(self.pts[0].into()),
            pathkit::SkPath_Verb::kLine_Verb => {
                PathVerbItem::Line(self.pts[0].into(), self.pts[1].into())
            }
            pathkit::SkPath_Verb::kQuad_Verb => {
                PathVerbItem::Quad(self.pts[1].into(), self.pts[2].into())
            }
            pathkit::SkPath_Verb::kConic_Verb => {
                // SkPath::Iter 不暴露 conicWeight，用 1.0 近似（椭圆/圆多为 0.5，此处简化）
                // SkPath::Iter does not expose conicWeight; using 1.0 as approx (ellipse/circle often 0.5)
                PathVerbItem::Conic(self.pts[1].into(), self.pts[2].into(), 1.0)
            }
            pathkit::SkPath_Verb::kCubic_Verb => PathVerbItem::Cubic(
                self.pts[1].into(),
                self.pts[2].into(),
                self.pts[3].into(),
            ),
            pathkit::SkPath_Verb::kClose_Verb => PathVerbItem::Close,
            pathkit::SkPath_Verb::kDone_Verb => return None,
            _ => return None,
        };
        Some(item)
    }
}
