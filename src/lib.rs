//! # path-kit
//!
//! 基于 Skia PathOps 和 PathKit 的 Rust 路径运算库，提供 safe 的 API 封装。
//! A Rust path operations library based on Skia PathOps and PathKit with safe API wrappers.
//!
//! ## 功能 / Features
//!
//! - **路径构建**：[`Path`]、[`PathBuilder`]；线段、二次/三次贝塞尔、矩形、椭圆、圆、圆角矩形、RRect
//!   Path construction: [`Path`], [`PathBuilder`]; lines, quad/cubic bezier, rect, oval, circle, round rect, RRect
//! - **路径布尔运算**：并集、交集、差集、异或
//!   Path boolean operations: union, intersect, difference, xor
//! - **路径简化、包围盒**：`simplify`, `path.tight_bounds`, `pathops_tight_bounds`
//!   Path simplification and tight bounds computation
//! - **路径变换**：[`Path::transform`]、[`Path::transformed`]、[`Matrix`]（`SkMatrix`）
//!   Affine/perspective transform via [`Path::transform`], [`Path::transformed`], [`Matrix`]
//! - **路径迭代**：按动词遍历 Move/Line/Quad/Cubic/Close
//!   Path iteration over verbs and points
//! - **描边**：将路径转为描边轮廓
//!   Stroke: convert path to stroked outline
//!
//! ## 线程安全 / Thread safety
//!
//! 当前未保证 `Send` / `Sync`，请勿跨线程共享 `Path`、`Matrix`、`RRect`、`StrokeRec`、`Paint` 等封装 C++ 的类型。
//! `Send` / `Sync` are not guaranteed; do not share `Path`, `Matrix`, `RRect`, `StrokeRec`, `Paint`, or other C++-backed handles across threads.
//!
//! ## 文档 / Documentation
//!
//! - **[docs.rs/path-kit](https://docs.rs/path-kit)**：托管 API 参考与 rustdoc。 *Hosted API reference and rustdoc.*
//! - **[CHANGELOG.md](https://github.com/songhuaixu/path-kit/blob/master/CHANGELOG.md)**：版本变更记录。 *Version history in the repo.*
//! - 本地：`cargo doc --open`。 *Locally: run `cargo doc --open`.*
//!
//! ## 类型概览 / Types
//!
//! | 类型 | 说明 · Description |
//! |------|-------------------|
//! | [`Path`] | 路径 · Path |
//! | [`PathBuilder`] | 路径构建器（`SkPathBuilder`，`snapshot` / `detach`）· Path builder (`SkPathBuilder`, `snapshot` / `detach`) |
//! | [`Rect`] | 矩形 · Axis-aligned rectangle |
//! | [`RRect`] | 圆角矩形（四角独立半径）· Rounded rect (per-corner radii) |
//! | [`Point`] | 二维点 · 2D point |
//! | [`Matrix`] / [`ScaleToFit`] / [`matrix_type`] / [`coeff`] | 3×3 矩阵（`SkMatrix`）、缩放适配、类型位、系数下标 · 3×3 transform, scale-to-fit, type bits, coeff indices |
//! | [`Direction`] | 绘制方向 Cw/Ccw · Fill direction clockwise / counter-clockwise |
//! | [`RectCorner`] | 矩形起始角 · Starting corner when adding a rect |
//! | [`PathOp`] | 布尔运算类型 · Path boolean operation kind |
//! | [`PathFillType`] | 填充规则（winding / even-odd / inverse 等）· Fill rule variants |
//! | [`PathVerb`] / [`PathVerbItem`] | 路径动词枚举 / 迭代项 · Path verb enum / iterator items |
//! | [`PathMeasure`] | 长度、位置与切线、分段提取 · Length, pos/tan, segment extraction |
//! | [`OpBuilder`] | 批量布尔运算（`SkOpBuilder`）· Batch boolean ops (`SkOpBuilder`) |
//! | [`path_op`] / [`simplify`] / [`pathops_tight_bounds`] | 两路径布尔、简化、pathops 紧包围盒 · Binary op, simplify, pathops tight bounds |
//! | [`StrokeRec`] / [`StrokeStyle`] | 描边参数与样式快照 · Stroke parameters and style snapshot |
//! | [`StrokeCap`] | 线端 Butt/Round/Square · Stroke end cap |
//! | [`StrokeJoin`] | 转角 Miter/Round/Bevel · Stroke corner join |
//! | [`Paint`] | 绘图参数（Style、Stroke 等）· Paint / stroke parameters (`SkPaint`) |
//! | [`PaintStyle`] | 填充/描边样式 · Fill / stroke / stroke-and-fill |
//! | [`CornerPathEffect`] / [`DashPathEffect`] | 圆角 / 虚线效果 · Corner / dash path effects |
//! | [`RRectType`] | 圆角矩形类型（`SkRRect::Type`）· RRect classification |
//!
//! ## 示例 / Examples
//!
//! ### 路径布尔运算 / Path boolean ops
//!
//! ```rust
//! use path_kit::{Path, Rect, Direction, RectCorner, PathOp, path_op, OpBuilder};
//!
//! let mut path1 = Path::new();
//! path1.add_rect(&Rect::new(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
//!
//! let mut path2 = Path::new();
//! path2.add_rect(&Rect::new(50.0, 50.0, 150.0, 150.0), Direction::Cw, RectCorner::UpperLeft);
//!
//! let union = path_op(&path1, &path2, PathOp::Union).unwrap();
//!
//! // 批量运算 / Batch operations (use add_ref to avoid clone when reusing paths)
//! let result = OpBuilder::new()
//!     .add_ref(&path1, PathOp::Union)
//!     .add_ref(&path2, PathOp::Union)
//!     .resolve()
//!     .unwrap();
//! ```
//!
//! ### 矩阵与路径变换 / Matrix and path transform
//!
//! ```rust
//! use path_kit::{Matrix, Path};
//!
//! let mut path = Path::new();
//! path.move_to(0.0, 0.0).line_to(100.0, 0.0).line_to(100.0, 100.0).close();
//!
//! let mut m = Matrix::identity();
//! m.pre_translate(10.0, 5.0).pre_scale(2.0, 2.0);
//! path.transform(&m);
//!
//! // Or keep the original: `let out = path.transformed(&m);`
//! ```
//!
//! ### PathBuilder · 增量构建 / Incremental path build
//!
//! ```rust
//! use path_kit::PathBuilder;
//!
//! let mut b = PathBuilder::new();
//! b.move_to(0.0, 0.0).line_to(50.0, 40.0).line_to(0.0, 40.0).close();
//! let _copy = b.snapshot(); // builder unchanged
//! let _owned = b.detach(); // builder reset; geometry moved into `Path`
//! ```
//!
//! ### 圆角矩形 RRect / Rounded rect with per-corner radii
//!
//! ```rust
//! use path_kit::{Path, Rect, RRect, Radii, Direction, RectCorner};
//!
//! // 统一圆角 / Uniform radii
//! let rr = RRect::from_rect_xy(&Rect::new(0.0, 0.0, 100.0, 50.0), 10.0, 10.0);
//! let mut path = Path::new();
//! path.add_rrect(&rr, Direction::Cw);
//!
//! // 四角独立半径 / Per-corner radii
//! let radii = [
//!     Radii { x: 10.0, y: 10.0 },
//!     Radii { x: 20.0, y: 10.0 },
//!     Radii { x: 10.0, y: 20.0 },
//!     Radii { x: 5.0, y: 5.0 },
//! ];
//! let rr2 = RRect::from_rect_radii(&Rect::new(0.0, 0.0, 80.0, 60.0), &radii);
//! path.add_rrect(&rr2, Direction::Ccw);
//! ```
//!
//! ### 路径迭代 / Path iteration
//!
//! ```rust
//! use path_kit::{Path, PathVerbItem};
//!
//! let mut path = Path::new();
//! path.move_to(0.0, 0.0).line_to(100.0, 0.0).line_to(100.0, 100.0).close();
//!
//! for item in path.iter(false) {
//!     match item {
//!         PathVerbItem::Move(p) => println!("Move to {:?}", p),
//!         PathVerbItem::Line(from, to) => println!("Line {:?} -> {:?}", from, to),
//!         PathVerbItem::Quad(c, to) => println!("Quad {:?} -> {:?}", c, to),
//!         PathVerbItem::Cubic(c1, c2, to) => println!("Cubic -> {:?}", to),
//!         PathVerbItem::Close => println!("Close"),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! ### 描边 / Stroke
//!
//! ```rust
//! use path_kit::{Path, StrokeRec};
//!
//! let rec = StrokeRec::new_stroke(4.0, false);
//! let mut path = Path::new();
//! path.move_to(0.0, 0.0).line_to(100.0, 0.0);
//! let stroked = rec.apply_to_path(&path).unwrap();
//! ```
//!
//! ### Paint · 描边转填充轮廓 / Stroke to fill outline (`get_fill_path`)
//!
//! ```rust
//! use path_kit::{Paint, PaintStyle, Path};
//!
//! let mut paint = Paint::new();
//! paint.set_style(PaintStyle::Stroke);
//! paint.set_stroke_width(4.0);
//! let mut path = Path::new();
//! path.move_to(0.0, 0.0).line_to(100.0, 0.0);
//! let stroked_as_fill = paint.get_fill_path(&path).unwrap();
//! assert!(stroked_as_fill.count_verbs() >= path.count_verbs());
//! ```
//!
//! ### 路径测量 / Path measure
//!
//! ```rust
//! use path_kit::{Path, PathMeasure};
//!
//! let mut path = Path::new();
//! path.move_to(0.0, 0.0).line_to(100.0, 0.0);
//! let mut measure = PathMeasure::from_path(&path, false, 1.0);
//! let len = measure.length();           // ~100
//! let (pos, tan) = measure.pos_tan(50.0).unwrap();  // position & tangent at midpoint
//! let mut segment = Path::new();
//! measure.get_segment(25.0, 75.0, &mut segment, true);  // extract sub-path
//! ```
//!
//! ### 路径简化与包围盒 / Simplify and bounds
//!
//! ```rust
//! use path_kit::{Path, simplify, pathops_tight_bounds};
//!
//! let mut path = Path::new();
//! path.move_to(0.0, 0.0)
//!     .line_to(100.0, 0.0)
//!     .line_to(100.0, 100.0)
//!     .line_to(50.0, 50.0)
//!     .line_to(0.0, 100.0)
//!     .close();
//!
//! let simplified = simplify(&path).unwrap();
//! let bounds = pathops_tight_bounds(&path).unwrap();  // or path.tight_bounds() for infallible
//! ```

/// PathKit（Skia PathOps）与 C++ 的 cxx 桥接（仅内部使用）。
/// PathKit C++ interop via cxx (internal only).
#[doc(hidden)]
pub(crate) mod bridge;

mod corner_path_effect;
mod dash_path_effect;
mod op_builder;
mod ops;
mod path;
mod path_builder;
mod path_iter;
mod path_measure;
mod path_fill_type;
mod matrix;
mod point;
mod rect;
mod rrect;
mod paint;
mod stroke_rec;

pub use bridge::ffi::Direction;
pub use bridge::ffi::PathOp;
pub use bridge::ffi::RectCorner;
pub use bridge::ffi::RRectType;

impl Default for Direction {
    fn default() -> Self {
        Self::Cw
    }
}

impl Default for RectCorner {
    fn default() -> Self {
        Self::UpperLeft
    }
}

pub use corner_path_effect::CornerPathEffect;
pub use dash_path_effect::DashPathEffect;
pub use op_builder::OpBuilder;
pub use ops::{path_op, pathops_tight_bounds, simplify};
pub use matrix::{Matrix, ScaleToFit, coeff, matrix_type};
pub use path::Path;
pub use path_builder::PathBuilder;
pub use path_fill_type::PathFillType;
pub use path_iter::{PathIter, PathVerb, PathVerbItem};
pub use path_measure::PathMeasure;
pub use point::Point;
pub use rect::Rect;
pub use rrect::{Radii, RRect};
pub use paint::{Paint, PaintStyle};
pub use stroke_rec::{StrokeCap, StrokeJoin, StrokeRec, StrokeStyle};

#[cfg(test)]
mod tests;
