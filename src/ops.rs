//! 路径运算函数。Path operation functions.

use crate::bridge::ffi;
use crate::path::Path;
use crate::PathOp;
use crate::rect::Rect;

/// 对两个路径执行布尔运算。Performs boolean operation on two paths.
///
/// 失败时返回 `None`（例如 pathops 无法解析几何）。Returns `None` if the operation fails.
pub fn path_op(path1: &Path, path2: &Path, op: PathOp) -> Option<Path> {
    let mut result = Path::new();
    let ok = ffi::boolean_path_op(
        path1.as_raw(),
        path2.as_raw(),
        op,
        result.as_raw_pin_mut(),
    );
    if ok {
        Some(result)
    } else {
        None
    }
}

/// 简化路径，处理自相交等。Simplifies path (resolves self-intersections, etc.).
///
/// 失败时返回 `None`。Returns `None` if simplification fails.
pub fn simplify(path: &Path) -> Option<Path> {
    let mut result = Path::new();
    let ok = ffi::simplify_path(path.as_raw(), result.as_raw_pin_mut());
    if ok {
        Some(result)
    } else {
        None
    }
}

/// 计算路径的紧密包围盒（pathops 实现）。Computes tight bounds using pathops algorithm.
///
/// 对含曲线路径可能比 `Path::tight_bounds` 更精确，但在解析失败时返回 None。
/// More accurate for curved paths than `Path::tight_bounds`, but returns None on parse failure.
pub fn pathops_tight_bounds(path: &Path) -> Option<Rect> {
    let mut result = ffi::Rect {
        fLeft: 0.0,
        fTop: 0.0,
        fRight: 0.0,
        fBottom: 0.0,
    };
    let ok = ffi::pathops_tight_bounds(path.as_raw(), &mut result);
    if ok {
        Some(result.into())
    } else {
        None
    }
}
