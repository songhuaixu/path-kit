//! # path-kit
//!
//! 基于 Skia PathKit 的 Rust 路径运算库，提供 safe 的 API 封装。
//! A Rust path operations library based on Skia PathKit with safe API wrappers.
//!
//! ## 功能 / Features
//!
//! - **路径构建**：线段、二次/三次贝塞尔、矩形、椭圆、圆、圆角矩形、RRect（四角独立半径）
//!   Path construction: lines, quadratic/cubic bezier, rect, oval, circle, round rect, RRect (per-corner radii)
//! - **路径布尔运算**：并集、交集、差集、异或
//!   Path boolean operations: union, intersect, difference, xor
//! - **路径简化、包围盒**：`simplify`, `path.tight_bounds`, `pathops_tight_bounds`
//!   Path simplification and tight bounds computation
//! - **路径迭代**：按动词遍历 Move/Line/Quad/Cubic/Close
//!   Path iteration over verbs and points
//! - **描边**：将路径转为描边轮廓
//!   Stroke: convert path to stroked outline
//!
//! ## 线程安全 / Thread safety
//!
//! 当前未保证 `Send` / `Sync`，请勿跨线程共享 `Path`、`RRect`、`StrokeRec` 等类型。
//! `Send` / `Sync` are not guaranteed; do not share `Path`, `RRect`, `StrokeRec`, etc. across threads.
//!
//! ## 类型概览 / Types
//!
//! | 类型 | 说明 |
//! |------|------|
//! | [`Path`] | 路径 |
//! | [`Rect`] | 矩形 |
//! | [`RRect`] | 圆角矩形（支持四角独立半径） |
//! | [`Point`] | 二维点 |
//! | [`Direction`] | 绘制方向 Cw/Ccw |
//! | [`RectCorner`] | 矩形起始角 |
//! | [`PathOp`] | 布尔运算类型 |
//! | [`PathVerbItem`] | 路径迭代项 |
//! | [`StrokeRec`] | 描边参数 |
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

/// PathKit FFI 绑定（仅内部使用，不对外暴露）。
/// PathKit FFI bindings (internal only, not exposed to external users).
#[doc(hidden)]
#[allow(warnings)]
mod pathkit {
    pub use root::pk::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod corner_path_effect;
mod dash_path_effect;
mod direction;
mod op_builder;
mod ops;
mod path;
mod path_iter;
mod path_op;
mod point;
mod rect;
mod rect_corner;
mod rrect;
mod stroke_rec;

pub use corner_path_effect::CornerPathEffect;
pub use dash_path_effect::DashPathEffect;
pub use direction::Direction;
pub use op_builder::OpBuilder;
pub use ops::{path_op, pathops_tight_bounds, simplify};
pub use path::Path;
pub use path_iter::{PathIter, PathVerb, PathVerbItem};
pub use path_op::PathOp;
pub use point::Point;
pub use rect::Rect;
pub use rect_corner::RectCorner;
pub use rrect::{Radii, RRect};
pub use stroke_rec::{StrokeRec, StrokeStyle};

#[cfg(test)]
mod tests;
