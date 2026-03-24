//! 路径操作构建器。Path operation builder for combining multiple paths.

use crate::path::Path;
use crate::path_op::PathOp;

/// 路径操作构建器，用于批量执行路径布尔运算。
/// Path operation builder, optimized for unioning/combining many paths.
///
/// 用法：多次 add 后调用 resolve 得到最终结果。
/// Usage: call add multiple times, then resolve to get the final result.
#[derive(Default)]
pub struct OpBuilder {
    paths: Vec<(Path, PathOp)>,
}

impl OpBuilder {
    /// 创建空构建器。Creates an empty builder.
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
        }
    }

    /// 添加路径及对应操作。Add a path with its boolean operation.
    ///
    /// 第一次 add 时相当于 (空路径 OP path)，后续每次为 (当前结果 OP path)。
    /// First add: (empty OP path). Each subsequent: (current_result OP path).
    pub fn add(&mut self, path: Path, op: PathOp) -> &mut Self {
        self.paths.push((path, op));
        self
    }

    /// 按引用添加路径（内部 clone）。Add path by reference (clones internally).
    ///
    /// 复用同一路径时比 `add(path.clone(), op)` 更简洁。
    /// Convenient when reusing the same path.
    pub fn add_ref(&mut self, path: &Path, op: PathOp) -> &mut Self {
        self.paths.push((path.clone(), op));
        self
    }

    /// 计算并返回所有路径运算的结果。Resolves all operations and returns the result.
    ///
    /// 调用后构建器会清空，可继续复用。
    /// Builder is reset after resolve; can be reused.
    pub fn resolve(&mut self) -> Option<Path> {
        let paths = std::mem::take(&mut self.paths);
        if paths.is_empty() {
            return Some(Path::new());
        }
        let mut result = Path::new();
        for (path, op) in paths {
            result = crate::ops::path_op(&result, &path, op)?;
        }
        Some(result)
    }
}
