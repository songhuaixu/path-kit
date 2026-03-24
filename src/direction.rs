//! 路径绘制方向 (顺时针/逆时针)。Path winding direction (CW/CCW).

use crate::pathkit;

/// 路径绘制方向。Direction for path winding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    /// 顺时针 / Clockwise
    #[default]
    Cw,
    /// 逆时针 / Counter-clockwise
    Ccw,
}

impl From<Direction> for pathkit::SkPathDirection::Type {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Cw => pathkit::SkPathDirection::kCW,
            Direction::Ccw => pathkit::SkPathDirection::kCCW,
        }
    }
}
