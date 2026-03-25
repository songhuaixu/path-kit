# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- **Matrix（`SkMatrix`）**：3×3 变换矩阵的 Rust 封装，经 pathkit-bridge 调用 `pk::SkMatrix`，与公开 API 语义对齐——`set_*` / `pre_*` / `post_*`、`set_rect_to_rect`（`ScaleToFit`）、`get_type`、各类 `is_*`、系数读写与 `invert` / `map_xy` / `map_rect` / `map_rect_scale_translate` / `map_origin`、`get_min_max_scales` 等、`write_to_memory` / `read_from_memory`、`PartialEq`（Skia 相等语义）、`Mul` 与 `Matrix::concat`
- **Path**：`transform` / `transformed` 接受 `Matrix`，与 `SkPath::transform` / `makeTransform` 一致（依赖上述 FFI）

---

## [0.3.0] - 2026-03-25

### Added

- **PathBuilder**：基于 `pk::SkPathBuilder` 的 FFI 封装，`snapshot` / `detach`、`set_fill_type`、`add_path` 及与 `Path` 对齐的几何构建 API

### Changed

- **OpBuilder**：改为调用 `pk::SkOpBuilder`（FFI），不再在 Rust 内逐次 `path_op`


## [0.2.0] - 2026-03-24

### Added

- **PathMeasure**：SkPathMeasure safe 封装，`length`, `pos_tan`, `get_segment`, `set_path`, `is_closed`, `next_contour`
- **描边样式**：`StrokeCap`（Butt, Round, Square）, `StrokeJoin`（Miter, Round, Bevel）
- **描边参数**：`StrokeRec::set_cap`, `set_join`, `set_stroke_params`, `cap`, `join`, `miter_limit`
- **Paint**：SkPaint safe 封装，`PaintStyle`（Fill/Stroke/StrokeAndFill）
- **Paint 方法**：`set_style`, `set_stroke_width`, `set_stroke_cap`, `set_stroke_join`, `set_stroke_miter`, `get_fill_path`

---

## [0.1.0] - 2026-03-24

### Added

- **Path 构建**：`move_to`, `line_to`, `quad_to`, `cubic_to`, `close`
- **Path 形状**：`add_rect`, `add_oval`, `add_circle`, `add_round_rect`, `add_rrect`, `add_rrect_with_start`
- **Path 查询**：`is_empty`, `count_points`, `count_verbs`, `get_point`, `tight_bounds`, `contains`, `is_rect`, `is_rrect`, `conservatively_contains_rect`
- **路径布尔运算**：`path_op`, `OpBuilder`（`add` / `add_ref` 支持引用传入）, Union / Intersect / Difference / Xor / ReverseDifference
- **路径运算**：`simplify`, `pathops_tight_bounds`
- **路径迭代**：`PathIter`, `PathVerbItem`（Move, Line, Quad, Conic, Cubic, Close）
- **描边**：`StrokeRec`（fill, hairline, stroke）, `apply_to_path`
- **类型**：`Path`, `Rect`, `Point`, `RRect`, `Radii`, `Direction`, `RectCorner`, `PathOp`
- **构建**：`cargo:rerun-if-changed` 优化增量编译

### Dependencies

- **[pathkit](https://github.com/libpag/pathkit)** — 从 Skia 提取的 PathOps C++ 库，随本 crate 内嵌编译（BSD-3-Clause）
- PathOps C++ library extracted from Skia, vendored and compiled with this crate (BSD-3-Clause)
