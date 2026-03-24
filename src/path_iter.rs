//! 路径迭代器。Path iterator over verbs and points.

use crate::bridge::ffi;
use crate::path::Path;
use crate::point::Point;
use cxx::UniquePtr;

/// 路径动词，与 `pk::SkPath::Verb` 一致。
/// Path verb, matches `pk::SkPath::Verb`.
pub use crate::bridge::ffi::PathVerb;

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
    inner: UniquePtr<ffi::PathIterInner>,
    p0: ffi::Point,
    p1: ffi::Point,
    p2: ffi::Point,
    p3: ffi::Point,
}

impl<'a> PathIter<'a> {
    pub(crate) fn new(path: &'a Path, force_close: bool) -> Self {
        let inner = ffi::path_iter_new(path.as_raw(), force_close);
        let z = ffi::Point { fX: 0.0, fY: 0.0 };
        Self {
            _path: path,
            inner,
            p0: z,
            p1: z,
            p2: z,
            p3: z,
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = PathVerbItem;

    fn next(&mut self) -> Option<Self::Item> {
        let v = ffi::path_iter_next(
            self.inner.pin_mut(),
            &mut self.p0,
            &mut self.p1,
            &mut self.p2,
            &mut self.p3,
        );
        let item = match v {
            PathVerb::Move => PathVerbItem::Move(self.p0.into()),
            PathVerb::Line => PathVerbItem::Line(self.p0.into(), self.p1.into()),
            PathVerb::Quad => PathVerbItem::Quad(self.p1.into(), self.p2.into()),
            PathVerb::Conic => PathVerbItem::Conic(self.p1.into(), self.p2.into(), 1.0),
            PathVerb::Cubic => PathVerbItem::Cubic(
                self.p1.into(),
                self.p2.into(),
                self.p3.into(),
            ),
            PathVerb::Close => PathVerbItem::Close,
            PathVerb::Done => return None,
            _ => return None,
        };
        Some(item)
    }
}
