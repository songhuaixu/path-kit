//! 路径填充规则。Path fill rule (winding / even-odd / inverse).

use crate::pathkit;

/// 与 `SkPathFillType` 对应：非零环绕、奇偶规则及反色变体。
/// Corresponds to `SkPathFillType`: winding, even-odd, and inverse variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathFillType {
    /// 非零环绕（`SkPathFillType::kWinding`）。
    Winding,
    /// 奇偶规则（`SkPathFillType::kEvenOdd`）。
    EvenOdd,
    /// 非零环绕，填充路径外部（`SkPathFillType::kInverseWinding`）。
    InverseWinding,
    /// 奇偶规则，填充路径外部（`SkPathFillType::kInverseEvenOdd`）。
    InverseEvenOdd,
}

impl PathFillType {
    /// 自 bindgen `SkPathFillType::Type` 构造（只取低位 2 bit）。
    pub(crate) fn from_raw(raw: pathkit::SkPathFillType::Type) -> Self {
        match raw & 3 {
            0 => PathFillType::Winding,
            1 => PathFillType::EvenOdd,
            2 => PathFillType::InverseWinding,
            _ => PathFillType::InverseEvenOdd,
        }
    }

    /// 自 `SkPath::fFillType()` 的低位 2 bit 构造。
    pub fn from_sk_bits(raw: u8) -> Self {
        Self::from_raw((raw & 3) as pathkit::SkPathFillType::Type)
    }

    /// 转为 bindgen `SkPathFillType::Type`（0..=3）。
    pub(crate) fn as_raw(self) -> pathkit::SkPathFillType::Type {
        self.into()
    }

    /// 是否为奇偶类规则（`EvenOdd` / `InverseEvenOdd`）。
    pub fn is_even_odd(self) -> bool {
        self.as_raw() & 1 != 0
    }

    /// 是否为「反色」填充（填充几何外部）。
    pub fn is_inverse(self) -> bool {
        self.as_raw() & 2 != 0
    }

    /// 去掉 inverse 位，得到 `Winding` 或 `EvenOdd`。
    pub fn to_non_inverse(self) -> Self {
        Self::from_raw(self.as_raw() & 1)
    }
}

impl From<PathFillType> for pathkit::SkPathFillType::Type {
    fn from(ft: PathFillType) -> Self {
        match ft {
            PathFillType::Winding => pathkit::SkPathFillType::kWinding,
            PathFillType::EvenOdd => pathkit::SkPathFillType::kEvenOdd,
            PathFillType::InverseWinding => pathkit::SkPathFillType::kInverseWinding,
            PathFillType::InverseEvenOdd => pathkit::SkPathFillType::kInverseEvenOdd,
        }
    }
}

impl From<pathkit::SkPathFillType::Type> for PathFillType {
    fn from(raw: pathkit::SkPathFillType::Type) -> Self {
        Self::from_raw(raw)
    }
}
