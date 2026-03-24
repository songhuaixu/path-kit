//! 路径操作构建器。Path operation builder for combining multiple paths.

use crate::bridge::ffi;
use crate::path::Path;
use crate::PathOp;
use cxx::UniquePtr;

/// 路径操作构建器，用于批量执行路径布尔运算。
/// Path operation builder, optimized for unioning/combining many paths.
///
/// 底层为 `pk::SkOpBuilder`，与 Skia PathKit 行为一致。
/// Backed by `pk::SkOpBuilder`, matching Skia PathKit behavior.
///
/// 用法：多次 add 后调用 resolve 得到最终结果。
/// Usage: call add multiple times, then resolve to get the final result.
pub struct OpBuilder {
    inner: UniquePtr<ffi::OpBuilderHolder>,
}

impl Default for OpBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OpBuilder {
    /// 创建空构建器。Creates an empty builder.
    pub fn new() -> Self {
        Self {
            inner: ffi::op_builder_new(),
        }
    }

    /// 添加路径及对应操作（消费 `Path`，C++ 侧会拷贝几何）。
    /// Adds a path with its boolean operation (consumes `Path`; native side copies geometry).
    ///
    /// 第一次 add 时相当于 (空路径 OP path)，后续每次为 (当前结果 OP path)。
    /// First add: (empty OP path). Each subsequent: (current_result OP path).
    pub fn add(&mut self, path: Path, op: PathOp) -> &mut Self {
        ffi::op_builder_add(self.inner.pin_mut(), path.as_cpp_ref(), op);
        self
    }

    /// 按引用添加路径（C++ 侧拷贝几何，Rust 侧不消费 `Path`）。
    /// Adds path by reference (native side copies geometry).
    ///
    /// 复用同一路径时比 `add(path.clone(), op)` 更简洁。
    /// Convenient when reusing the same path.
    pub fn add_ref(&mut self, path: &Path, op: PathOp) -> &mut Self {
        ffi::op_builder_add(self.inner.pin_mut(), path.as_cpp_ref(), op);
        self
    }

    /// 计算并返回所有路径运算的结果。Resolves all operations and returns the result.
    ///
    /// 调用后构建器会清空，可继续复用。
    /// Builder is reset after resolve; can be reused.
    pub fn resolve(&mut self) -> Option<Path> {
        let mut result = Path::new();
        let ok = ffi::op_builder_resolve(self.inner.pin_mut(), result.pin_cpp_mut());
        if ok {
            Some(result)
        } else {
            None
        }
    }
}
