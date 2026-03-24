//! 矩形起始角，指定 add_rect 时从哪个角开始绘制。
//! Rectangle start corner for add_rect (which corner to begin drawing from).

/// 矩形起始角，指定 add_rect 时从哪个角开始绘制。
/// Rectangle start corner for add_rect (which corner to begin drawing from).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RectCorner {
    /// 左上 / Upper-left
    UpperLeft = 0,
    /// 右上 / Upper-right
    UpperRight = 1,
    /// 右下 / Lower-right
    LowerRight = 2,
    /// 左下 / Lower-left
    LowerLeft = 3,
}

impl Default for RectCorner {
    fn default() -> Self {
        Self::UpperLeft
    }
}

impl From<RectCorner> for u32 {
    fn from(c: RectCorner) -> Self {
        c as u32
    }
}
