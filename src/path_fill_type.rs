//! 路径填充规则。Path fill rule (winding / even-odd / inverse).
//!
//! 枚举变体与 `pk::SkPathFillType` 一致，由 cxx 桥接共享；本模块 re-export 并扩展便捷方法。
//! Variants match `pk::SkPathFillType` (cxx shared type); re-exported here with helpers.

pub use crate::bridge::ffi::PathFillType;

impl PathFillType {
    /// 是否为奇偶类规则（`EvenOdd` / `InverseEvenOdd`）。
    /// True for even-odd style fill rules.
    pub fn is_even_odd(self) -> bool {
        matches!(self, Self::EvenOdd | Self::InverseEvenOdd)
    }

    /// 是否为「反色」填充（填充几何外部）。
    /// True for inverse fill (paint outside the geometry).
    pub fn is_inverse(self) -> bool {
        matches!(self, Self::InverseWinding | Self::InverseEvenOdd)
    }

    /// 去掉 inverse 位，得到 `Winding` 或 `EvenOdd`。
    /// Strips the inverse bit, yielding `Winding` or `EvenOdd`.
    pub fn to_non_inverse(self) -> Self {
        match self {
            Self::Winding | Self::InverseWinding => Self::Winding,
            Self::EvenOdd | Self::InverseEvenOdd => Self::EvenOdd,
            _ => Self::Winding,
        }
    }
}
