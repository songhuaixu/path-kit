//! 3×3 仿射/透视变换矩阵，与 pathkit 中 `pk::SkMatrix` 的**公开** API 语义一致。
//! 3×3 affine/perspective transform matrix; behavior matches public `pk::SkMatrix` in pathkit.
//!
//! ## 存储 / Storage
//!
//! 九个系数按 `SkMatrix::get9` / `set9` 顺序存放在 [`Matrix::mat`]，下标见 [`coeff`]；行优先、与 Skia 文档一致：
//! `[scaleX, skewX, transX, skewY, scaleY, transY, persp0, persp1, persp2]`。
//! Nine coeffs in `get9`/`set9` order in [`Matrix::mat`]; indices in [`coeff`]. Row-major layout matches Skia.
//!
//! ## 前乘与后乘 / Pre vs post
//!
//! 与 `SkMatrix` 文档一致：`pre*` 为 **`self = self * T`**（新变换在乘积**右侧**；点空间上可理解为「先 `T` 再原矩阵」），
//! `post*` 为 **`self = T * self`**（新变换在**左侧**；「先原矩阵再 `T`」）。
//! Same as Skia docs: `pre*` is `self = self * T` (new transform on the **right**), `post*` is `self = T * self` (on the **left**).
//!
//! - [`Matrix::concat`] / [`Mul`]：与 `SkMatrix::setConcat` / `Concat(a, b)` 相同，结果为 **`a * b`**（对列向量 `p' = a * b * p`）。
//!
//! ## 未覆盖项 / Not wired to C++
//!
//! - `postIDiv` 等在 Skia 中非 public，当前无 FFI。
//! - [`Matrix::map_homogeneous`] 仅在 Rust 侧做线性组合；完整 `mapHomogeneousPoints` / `SkPoint3` 批处理未暴露。
//! - [`Matrix::invalid`] 为占位实现（全 `f32::MAX`），未调用 `SkMatrix::InvalidMatrix()` 单例。
//!
//! ## 序列化 / Serialization
//!
//! [`Matrix::write_to_memory`] / [`Matrix::read_from_memory`] 为固定 **36 字节**（9×`f32`），与当前 `SkMatrix::writeToMemory` / `readFromMemory` 布局一致。

use crate::bridge::ffi;
use crate::point::Point;
use crate::rect::Rect;
use std::ops::{Index, IndexMut, Mul};

/// 源矩形映射到目标矩形时的缩放/对齐策略，对应 `SkMatrix::ScaleToFit`。
/// Scale/align policy when mapping a source rect to a destination rect (`SkMatrix::ScaleToFit`).
///
/// 用于 [`Matrix::set_rect_to_rect`]。
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScaleToFit {
    /// 独立缩放 sx/sy 以完全填满 `dst`（可能非等比）。
    /// Scale X and Y independently to fill `dst` (aspect ratio may change).
    Fill = 0,
    /// 等比缩放并靠 `dst` 的「起始」边对齐（与 Skia `Start` 一致）。
    /// Uniform scale; aligned to the start edge of `dst` per Skia.
    Start = 1,
    /// 等比缩放并在 `dst` 内居中。
    /// Uniform scale; centered within `dst`.
    Center = 2,
    /// 等比缩放并靠「结束」边对齐。
    /// Uniform scale; aligned to the end edge of `dst`.
    End = 3,
}

/// `SkMatrix::TypeMask` 中与类别相关的公开位，可与 `|` 组合；[`Matrix::get_type`] 的返回值可与此做位运算。
/// Public `SkMatrix::TypeMask` bits; combine with `|`; use with [`Matrix::get_type`].
pub mod matrix_type {
    #![allow(dead_code)]
    /// 单位矩阵（无数值分类）。Identity (no transform classification).
    pub const IDENTITY: u32 = 0;
    /// 位：含平移分量。Bit: translation present.
    pub const TRANSLATE: u32 = 0x01;
    /// 位：含缩放。Bit: scale present.
    pub const SCALE: u32 = 0x02;
    /// 位：含一般仿射（错切、旋转等）。Bit: general affine.
    pub const AFFINE: u32 = 0x04;
    /// 位：含透视（`persp0`/`persp1` 非零等）。Bit: perspective.
    pub const PERSPECTIVE: u32 = 0x08;
}

/// `SkMatrix` 九个系数在 [`Matrix::mat`] 中的下标（`kMScaleX` … `kMPersp2`）。
/// Indices of the nine coeffs in [`Matrix::mat`] (`kMScaleX` …).
pub mod coeff {
    /// `scaleX`（x 方向缩放）。`scaleX`.
    pub const M_SCALE_X: usize = 0;
    /// `skewX`（x 受 y 影响的错切）。`skewX`.
    pub const M_SKEW_X: usize = 1;
    /// `transX`（x 平移）。`transX`.
    pub const M_TRANS_X: usize = 2;
    /// `skewY`（y 受 x 影响的错切）。`skewY`.
    pub const M_SKEW_Y: usize = 3;
    /// `scaleY`（y 方向缩放）。`scaleY`.
    pub const M_SCALE_Y: usize = 4;
    /// `transY`（y 平移）。`transY`.
    pub const M_TRANS_Y: usize = 5;
    /// 透视行第一列系数。Perspective row, first coeff.
    pub const M_PERSP_0: usize = 6;
    /// 透视行第二列系数。Perspective row, second coeff.
    pub const M_PERSP_1: usize = 7;
    /// 透视行第三列系数（通常为 1）。Perspective row, third coeff (often 1).
    pub const M_PERSP_2: usize = 8;
}

/// 3×3 变换矩阵（列向量形式 `p' = M * p`），系数布局见模块说明。
/// 3×3 transform (column vectors `p' = M * p`); layout described at module level.
#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    /// 与 `SkMatrix::fMat` / `get9` / `set9` 顺序一致。
    /// Same order as `SkMatrix::fMat` / `get9` / `set9`.
    pub mat: [f32; 9],
}

impl Matrix {
    /// 单位矩阵常量。Identity matrix constant.
    pub const IDENTITY: Self = Self {
        mat: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    };

    /// 返回单位矩阵。Returns the identity matrix.
    #[inline]
    pub const fn identity() -> Self {
        Self::IDENTITY
    }

    /// 非法/哨兵矩阵占位，**非** Skia 单例；系数全为 `f32::MAX`，勿用于几何变换。
    /// Sentinel / invalid placeholder (not the Skia singleton); all coeffs `f32::MAX`; do not use for mapping.
    pub fn invalid() -> Self {
        let x = f32::MAX;
        Self { mat: [x; 9] }
    }

    // —— 静态工厂（与 SkMatrix 同名语义）——

    /// 缩放矩阵：内部 `setScale(sx, sy)`。Scale matrix via `setScale(sx, sy)`.
    #[inline]
    pub fn scale(sx: f32, sy: f32) -> Self {
        let mut m = Self::IDENTITY;
        m.set_scale(sx, sy);
        m
    }

    /// 平移矩阵：`setTranslate`。Translation matrix via `setTranslate`.
    #[inline]
    pub fn translate(dx: f32, dy: f32) -> Self {
        let mut m = Self::IDENTITY;
        m.set_translate(dx, dy);
        m
    }

    /// 绕原点旋转；**正角度顺时针**（Skia 约定）。Rotate about origin; **positive degrees = clockwise** (Skia).
    pub fn rotate_deg(degrees: f32) -> Self {
        let mut m = Self::IDENTITY;
        m.set_rotate(degrees);
        m
    }

    /// 由九个系数构造，等价 `SkMatrix::MakeAll` / `setAll`。Build from nine coeffs (`MakeAll` / `setAll`).
    #[inline]
    pub fn make_all(
        scale_x: f32,
        skew_x: f32,
        trans_x: f32,
        skew_y: f32,
        scale_y: f32,
        trans_y: f32,
        pers0: f32,
        pers1: f32,
        pers2: f32,
    ) -> Self {
        let mut m = Self::IDENTITY;
        m.set_all(
            scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, pers0, pers1, pers2,
        );
        m
    }

    /// 从已有 `get9` 顺序数组封装，**不**校验数值。Wrap raw `get9` array; **no** validation.
    #[inline]
    pub const fn from_row_major_9(mat: [f32; 9]) -> Self {
        Self { mat }
    }

    /// 矩阵乘积 `a * b`，对齐 `SkMatrix::Concat(a, b)`。Matrix product `a * b` (`SkMatrix::Concat`).
    #[inline]
    pub fn concat(a: &Matrix, b: &Matrix) -> Matrix {
        let mut out = Self::IDENTITY;
        ffi::matrix_set_concat(out.mat.as_mut_slice(), a.mat.as_slice(), b.mat.as_slice());
        out
    }

    // —— 替换型 set —— //

    /// 置为单位矩阵。Reset to identity (`SkMatrix::reset`).
    #[inline]
    pub fn reset(&mut self) -> &mut Self {
        ffi::matrix_reset(self.mat.as_mut_slice());
        self
    }

    /// 同 [`Self::reset`]。Alias for [`Self::reset`].
    #[inline]
    pub fn set_identity(&mut self) -> &mut Self {
        self.reset()
    }

    /// 一次性写入九个系数，见模块「存储」说明。Set all nine coeffs (`setAll`).
    pub fn set_all(
        &mut self,
        scale_x: f32,
        skew_x: f32,
        trans_x: f32,
        skew_y: f32,
        scale_y: f32,
        trans_y: f32,
        pers0: f32,
        pers1: f32,
        pers2: f32,
    ) -> &mut Self {
        ffi::matrix_set_all(
            self.mat.as_mut_slice(),
            scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, pers0, pers1, pers2,
        );
        self
    }

    /// 替换为纯平移。Replace with translation (`setTranslate`).
    #[inline]
    pub fn set_translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        ffi::matrix_set_translate(self.mat.as_mut_slice(), dx, dy);
        self
    }

    /// 替换为以原点为基准的缩放。Replace with scale about origin.
    #[inline]
    pub fn set_scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        ffi::matrix_set_scale(self.mat.as_mut_slice(), sx, sy);
        self
    }

    /// 替换为绕 `(px, py)` 的缩放。Replace with scale about pivot `(px, py)`.
    #[inline]
    pub fn set_scale_pivot(&mut self, sx: f32, sy: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_set_scale_pivot(self.mat.as_mut_slice(), sx, sy, px, py);
        self
    }

    /// 替换为绕原点旋转（正角度顺时针）。Replace with rotation about origin (CW positive).
    #[inline]
    pub fn set_rotate(&mut self, degrees: f32) -> &mut Self {
        ffi::matrix_set_rotate(self.mat.as_mut_slice(), degrees);
        self
    }

    /// 替换为绕 `(px, py)` 旋转。Replace with rotation about `(px, py)`.
    #[inline]
    pub fn set_rotate_pivot(&mut self, degrees: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_set_rotate_pivot(self.mat.as_mut_slice(), degrees, px, py);
        self
    }

    /// 替换为由 `sin`/`cos` 给出的旋转（绕原点）。Rotation from `sin`/`cos` about origin.
    #[inline]
    pub fn set_sin_cos(&mut self, sin_v: f32, cos_v: f32) -> &mut Self {
        ffi::matrix_set_sin_cos(self.mat.as_mut_slice(), sin_v, cos_v);
        self
    }

    /// 同上，支点为 `(px, py)`。Same with pivot `(px, py)`.
    #[inline]
    pub fn set_sin_cos_pivot(&mut self, sin_v: f32, cos_v: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_set_sin_cos_pivot(self.mat.as_mut_slice(), sin_v, cos_v, px, py);
        self
    }

    /// 替换为错切（原点）。Replace with skew about origin.
    #[inline]
    pub fn set_skew(&mut self, kx: f32, ky: f32) -> &mut Self {
        ffi::matrix_set_skew(self.mat.as_mut_slice(), kx, ky);
        self
    }

    /// 替换为绕 `(px, py)` 的错切。Replace with skew about `(px, py)`.
    #[inline]
    pub fn set_skew_pivot(&mut self, kx: f32, ky: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_set_skew_pivot(self.mat.as_mut_slice(), kx, ky, px, py);
        self
    }

    /// 替换为无错切、无透视的缩放+平移（常用优化形式）。Replace with scale+translate only.
    #[inline]
    pub fn set_scale_translate(&mut self, sx: f32, sy: f32, tx: f32, ty: f32) -> &mut Self {
        ffi::matrix_set_scale_translate(self.mat.as_mut_slice(), sx, sy, tx, ty);
        self
    }

    /// `self = a * b`（[`Self::concat`]）。`self = a * b` (`setConcat`).
    #[inline]
    pub fn set_concat(&mut self, a: &Matrix, b: &Matrix) -> &mut Self {
        ffi::matrix_set_concat(
            self.mat.as_mut_slice(),
            a.mat.as_slice(),
            b.mat.as_slice(),
        );
        self
    }

    /// 构造将 `src` 映射到 `dst` 的变换（按 [`ScaleToFit`]）；成功时覆盖 `self`，失败时行为与 Skia 一致。
    /// Map `src` onto `dst` per [`ScaleToFit`]; on failure matches Skia `setRectToRect`.
    pub fn set_rect_to_rect(&mut self, src: &Rect, dst: &Rect, stf: ScaleToFit) -> bool {
        let sr: ffi::Rect = (*src).into();
        let dr: ffi::Rect = (*dst).into();
        ffi::matrix_set_rect_to_rect(self.mat.as_mut_slice(), &sr, &dr, stf as i32)
    }

    // —— pre / post —— //

    /// `self = self * T(dx, dy)`。`preTranslate`.
    #[inline]
    pub fn pre_translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        ffi::matrix_pre_translate(self.mat.as_mut_slice(), dx, dy);
        self
    }
    /// `self = self * S(sx, sy)`。`preScale`.
    #[inline]
    pub fn pre_scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        ffi::matrix_pre_scale(self.mat.as_mut_slice(), sx, sy);
        self
    }
    /// 带支点的 `preScale`。`preScale` with pivot.
    #[inline]
    pub fn pre_scale_pivot(&mut self, sx: f32, sy: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_pre_scale_pivot(self.mat.as_mut_slice(), sx, sy, px, py);
        self
    }
    /// `preRotate`（正角度顺时针）。`preRotate` (CW positive).
    #[inline]
    pub fn pre_rotate(&mut self, degrees: f32) -> &mut Self {
        ffi::matrix_pre_rotate(self.mat.as_mut_slice(), degrees);
        self
    }
    /// 带支点的 `preRotate`。`preRotate` with pivot.
    #[inline]
    pub fn pre_rotate_pivot(&mut self, degrees: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_pre_rotate_pivot(self.mat.as_mut_slice(), degrees, px, py);
        self
    }
    /// `preSkew`。`preSkew`.
    #[inline]
    pub fn pre_skew(&mut self, kx: f32, ky: f32) -> &mut Self {
        ffi::matrix_pre_skew(self.mat.as_mut_slice(), kx, ky);
        self
    }
    /// 带支点的 `preSkew`。`preSkew` with pivot.
    #[inline]
    pub fn pre_skew_pivot(&mut self, kx: f32, ky: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_pre_skew_pivot(self.mat.as_mut_slice(), kx, ky, px, py);
        self
    }
    /// `self = self * other`。`preConcat`.
    #[inline]
    pub fn pre_concat(&mut self, other: &Matrix) -> &mut Self {
        ffi::matrix_pre_concat(self.mat.as_mut_slice(), other.mat.as_slice());
        self
    }

    /// `self = T(dx, dy) * self`。`postTranslate`.
    #[inline]
    pub fn post_translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        ffi::matrix_post_translate(self.mat.as_mut_slice(), dx, dy);
        self
    }
    /// `postScale`。`postScale`.
    #[inline]
    pub fn post_scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        ffi::matrix_post_scale(self.mat.as_mut_slice(), sx, sy);
        self
    }
    /// 带支点的 `postScale`。`postScale` with pivot.
    #[inline]
    pub fn post_scale_pivot(&mut self, sx: f32, sy: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_post_scale_pivot(self.mat.as_mut_slice(), sx, sy, px, py);
        self
    }
    /// `postRotate`（正角度顺时针）。`postRotate` (CW positive).
    #[inline]
    pub fn post_rotate(&mut self, degrees: f32) -> &mut Self {
        ffi::matrix_post_rotate(self.mat.as_mut_slice(), degrees);
        self
    }
    /// 带支点的 `postRotate`。`postRotate` with pivot.
    #[inline]
    pub fn post_rotate_pivot(&mut self, degrees: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_post_rotate_pivot(self.mat.as_mut_slice(), degrees, px, py);
        self
    }
    /// `postSkew`。`postSkew`.
    #[inline]
    pub fn post_skew(&mut self, kx: f32, ky: f32) -> &mut Self {
        ffi::matrix_post_skew(self.mat.as_mut_slice(), kx, ky);
        self
    }
    /// 带支点的 `postSkew`。`postSkew` with pivot.
    #[inline]
    pub fn post_skew_pivot(&mut self, kx: f32, ky: f32, px: f32, py: f32) -> &mut Self {
        ffi::matrix_post_skew_pivot(self.mat.as_mut_slice(), kx, ky, px, py);
        self
    }
    /// `self = other * self`。`postConcat`.
    #[inline]
    pub fn post_concat(&mut self, other: &Matrix) -> &mut Self {
        ffi::matrix_post_concat(self.mat.as_mut_slice(), other.mat.as_slice());
        self
    }

    // —— 查询 —— //

    /// 类型掩码（[`matrix_type`] 中的位）。Type mask (`getType`).
    #[inline]
    pub fn get_type(&self) -> u32 {
        ffi::matrix_get_type(self.mat.as_slice())
    }

    /// 是否单位矩阵（容差内）。`isIdentity`.
    #[inline]
    pub fn is_identity(&self) -> bool {
        ffi::matrix_is_identity(self.mat.as_slice())
    }

    /// 是否可表示为仅有缩放与平移（无错切/旋转/透视）。`isScaleTranslate`.
    #[inline]
    pub fn is_scale_translate(&self) -> bool {
        ffi::matrix_is_scale_translate_matrix(self.mat.as_slice())
    }

    /// 是否将任意轴对齐矩形仍映射为轴对齐矩形。`rectStaysRect`.
    #[inline]
    pub fn rect_stays_rect(&self) -> bool {
        ffi::matrix_rect_stays_rect(self.mat.as_slice())
    }

    /// 同 `SkMatrix::preservesAxisAlignment`（等同于 [`Self::rect_stays_rect`]。）。
    /// Same as `preservesAxisAlignment` (alias of [`Self::rect_stays_rect`]).
    #[inline]
    pub fn preserves_axis_alignment(&self) -> bool {
        self.rect_stays_rect()
    }

    /// 是否含透视分量。`hasPerspective`.
    #[inline]
    pub fn has_perspective(&self) -> bool {
        ffi::matrix_has_perspective(self.mat.as_slice())
    }

    /// 所有系数是否均为有限浮点。`SkMatrix` finite check (`isFinite`-class API).
    #[inline]
    pub fn is_finite(&self) -> bool {
        ffi::matrix_is_finite_matrix(self.mat.as_slice())
    }

    /// `mat[coeff::M_SCALE_X]`。`getScaleX`.
    #[inline]
    pub fn get_scale_x(&self) -> f32 {
        self.mat[coeff::M_SCALE_X]
    }
    /// `getSkewX`.
    #[inline]
    pub fn get_skew_x(&self) -> f32 {
        self.mat[coeff::M_SKEW_X]
    }
    /// `getTranslateX`.
    #[inline]
    pub fn get_translate_x(&self) -> f32 {
        self.mat[coeff::M_TRANS_X]
    }
    /// `getSkewY`.
    #[inline]
    pub fn get_skew_y(&self) -> f32 {
        self.mat[coeff::M_SKEW_Y]
    }
    /// `getScaleY`.
    #[inline]
    pub fn get_scale_y(&self) -> f32 {
        self.mat[coeff::M_SCALE_Y]
    }
    /// `getTranslateY`.
    #[inline]
    pub fn get_translate_y(&self) -> f32 {
        self.mat[coeff::M_TRANS_Y]
    }
    /// 透视行：`getPerspX` / `fMat[kMPersp0]`.
    #[inline]
    pub fn get_persp_x(&self) -> f32 {
        self.mat[coeff::M_PERSP_0]
    }
    /// `getPerspY`.
    #[inline]
    pub fn get_persp_y(&self) -> f32 {
        self.mat[coeff::M_PERSP_1]
    }
    /// `getPerspZ`（末元通常 1）。`getPerspZ` (often 1).
    #[inline]
    pub fn get_persp_z(&self) -> f32 {
        self.mat[coeff::M_PERSP_2]
    }

    /// 按线性下标 `0..9` 读系数。Raw index `0..9` into [`Self::mat`].
    #[inline]
    pub fn get(&self, index: usize) -> f32 {
        self.mat[index]
    }

    /// 按线性下标写系数（**不**更新 Skia 内部缓存分类；仅改数组）。Set coeff by index (raw write; no Skia type cache update).
    #[inline]
    pub fn set_coeff(&mut self, index: usize, value: f32) -> &mut Self {
        self.mat[index] = value;
        self
    }

    /// 求逆写入 `out_or_inverse`；不可逆返回 `false`。`SkMatrix::invert`.
    pub fn invert_to(&self, out_or_inverse: &mut Matrix) -> bool {
        ffi::matrix_invert(
            self.mat.as_slice(),
            out_or_inverse.mat.as_mut_slice(),
        )
    }

    /// 可逆时 `Some(逆矩阵)`，否则 `None`。`invert` wrapped in `Option`.
    pub fn try_inverse(&self) -> Option<Matrix> {
        let mut out = Self::IDENTITY;
        if self.invert_to(&mut out) {
            Some(out)
        } else {
            None
        }
    }

    /// 映射 `(x, y)`（透视下含 projective divide），与 `SkMatrix::mapXY` 一致。Map point (`mapXY`).
    #[inline]
    pub fn map_xy(&self, x: f32, y: f32) -> Point {
        let mut p = ffi::Point { fX: 0.0, fY: 0.0 };
        ffi::matrix_map_xy(self.mat.as_slice(), x, y, &mut p);
        p.into()
    }

    /// [`Self::map_xy`] 的便捷封装。Convenience over [`Self::map_xy`].
    #[inline]
    pub fn map_point(&self, p: Point) -> Point {
        self.map_xy(p.x, p.y)
    }

    /// 线性部分左乘齐次向量 `[x,y,z]^T`，**不**做透视除法；与 `SkMatrix::mapHomogeneousPoints` 的线性步骤一致。
    /// Homogeneous multiply **without** divide-by-w; matches linear step of `mapHomogeneousPoints`.
    pub fn map_homogeneous(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
        let m = &self.mat;
        [
            m[0] * x + m[1] * y + m[2] * z,
            m[3] * x + m[4] * y + m[5] * z,
            m[6] * x + m[7] * y + m[8] * z,
        ]
    }

    /// 将 `src` 前 `min(len)` 项映射写入 `dst`（与 `mapPoints(dst, src)` 批量语义一致）。Batch map like `mapPoints`.
    pub fn map_points(&self, dst: &mut [Point], src: &[Point]) {
        let n = src.len().min(dst.len());
        for i in 0..n {
            dst[i] = self.map_point(src[i]);
        }
    }

    /// 原地映射点集。In-place `mapPoints`.
    pub fn map_points_inplace(&self, pts: &mut [Point]) {
        for p in pts.iter_mut() {
            *p = self.map_point(*p);
        }
    }

    /// `mapXY(0, 0)`，即原点经本矩阵映射后的位置。`SkMatrix::mapOrigin`.
    #[inline]
    pub fn map_origin(&self) -> Point {
        let mut p = ffi::Point { fX: 0.0, fY: 0.0 };
        ffi::matrix_map_origin(self.mat.as_slice(), &mut p);
        p.into()
    }

    /// 将矩形 `src` 经本矩阵映射；返回 `(包围盒, 是否仍为轴对齐矩形)`。`SkMatrix::mapRect`.
    pub fn map_rect(&self, src: &Rect) -> (Rect, bool) {
        let mut d = ffi::Rect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        let s: ffi::Rect = (*src).into();
        let axis = ffi::matrix_map_rect(self.mat.as_slice(), &s, &mut d);
        (d.into(), axis)
    }

    /// 当 [`Self::is_scale_translate`] 为真时的快速矩形映射（行为 undefined 若矩阵不满足前提——与 Skia 一致）。Fast path `mapRectScaleTranslate`.
    pub fn map_rect_scale_translate(&self, src: &Rect) -> Rect {
        let mut d = ffi::Rect {
            fLeft: 0.0,
            fTop: 0.0,
            fRight: 0.0,
            fBottom: 0.0,
        };
        let s: ffi::Rect = (*src).into();
        ffi::matrix_map_rect_scale_translate(self.mat.as_slice(), &s, &mut d);
        d.into()
    }

    /// 局部最小缩放因子（Skia 定义）。`getMinScale`.
    #[inline]
    pub fn get_min_scale(&self) -> f32 {
        ffi::matrix_get_min_scale(self.mat.as_slice())
    }

    /// 局部最大缩放因子。`getMaxScale`.
    #[inline]
    pub fn get_max_scale(&self) -> f32 {
        ffi::matrix_get_max_scale(self.mat.as_slice())
    }

    /// 同时取得最小/最大缩放；若矩阵退化或无法计算则 `None`。`getMinMaxScales`.
    pub fn get_min_max_scales(&self) -> Option<(f32, f32)> {
        let mut a = 0.0f32;
        let mut b = 0.0f32;
        if ffi::matrix_get_min_max_scales(self.mat.as_slice(), &mut a, &mut b) {
            Some((a, b))
        } else {
            None
        }
    }

    /// 序列化 **9×`f32`** 到 `buf`；返回实际写入字节数，不足时写入 0。`writeToMemory`.
    pub fn write_to_memory(&self, buf: &mut [u8]) -> usize {
        ffi::matrix_write_to_memory(self.mat.as_slice(), buf)
    }

    /// 从 `buf` 反序列化；成功返回读取字节数。`readFromMemory`.
    pub fn read_from_memory(&mut self, buf: &[u8]) -> usize {
        ffi::matrix_read_from_memory(self.mat.as_mut_slice(), buf)
    }
}

impl Default for Matrix {
    /// 单位矩阵。Identity.
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for Matrix {
    /// Skia `SkMatrix` 相等语义（非简单逐位 `==`）。Skia matrix equality (not naive bit-compare).
    fn eq(&self, other: &Self) -> bool {
        ffi::matrix_equals(self.mat.as_slice(), other.mat.as_slice())
    }
}

impl Eq for Matrix {}

impl Index<usize> for Matrix {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.mat[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.mat[index]
    }
}

/// 矩阵乘法：`lhs * rhs` 与 [`Matrix::concat`] 语义相同。`Mul`: same as [`Matrix::concat`].
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        Matrix::concat(&self, &rhs)
    }
}

/// 同上。Same as [`Matrix::concat`].
impl Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        Matrix::concat(&self, rhs)
    }
}

/// 同上。Same as [`Matrix::concat`].
impl Mul<Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        Matrix::concat(self, &rhs)
    }
}

/// 同上。Same as [`Matrix::concat`].
impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        Matrix::concat(self, rhs)
    }
}
