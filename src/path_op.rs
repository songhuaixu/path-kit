//! 路径布尔运算类型。Path boolean operation type.

use crate::pathkit;

/// 并集、交集、差集、异或等路径集合运算。Union, intersect, difference, xor, reverse difference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOp {
    /// 差集：path1 - path2 / Difference
    Difference,
    /// 交集：path1 ∩ path2 / Intersection
    Intersect,
    /// 并集：path1 ∪ path2 / Union
    Union,
    /// 异或：path1 ⊕ path2 / XOR
    Xor,
    /// 反向差集：path2 - path1 / Reverse difference
    ReverseDifference,
}

impl From<PathOp> for pathkit::SkPathOp::Type {
    fn from(op: PathOp) -> Self {
        match op {
            PathOp::Difference => pathkit::SkPathOp::kDifference_SkPathOp,
            PathOp::Intersect => pathkit::SkPathOp::kIntersect_SkPathOp,
            PathOp::Union => pathkit::SkPathOp::kUnion_SkPathOp,
            PathOp::Xor => pathkit::SkPathOp::kXOR_SkPathOp,
            PathOp::ReverseDifference => pathkit::SkPathOp::kReverseDifference_SkPathOp,
        }
    }
}
