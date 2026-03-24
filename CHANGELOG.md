# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2025-03-24

### Added

- **Path 构建**：`move_to`, `line_to`, `quad_to`, `cubic_to`, `close`
- **Path 形状**：`add_rect`, `add_oval`, `add_circle`, `add_round_rect`, `add_rrect`, `add_rrect_with_start`
- **Path 查询**：`is_empty`, `count_points`, `count_verbs`, `get_point`, `tight_bounds`, `contains`, `is_rect`, `is_rrect`, `conservatively_contains_rect`
- **路径布尔运算**：`path_op`, `OpBuilder`（`add` / `add_ref` 支持引用传入）, Union / Intersect / Difference / Xor / ReverseDifference
- **路径运算**：`simplify`, `pathops_tight_bounds`
- **路径迭代**：`PathIter`, `PathVerbItem`（Move, Line, Quad, Conic, Cubic, Close）
- **描边**：`StrokeRec`（fill, hairline, stroke）, `apply_to_path`
- **类型**：`Path`, `Rect`, `Point`, `RRect`, `Radii`, `Direction`, `RectCorner`, `PathOp`
- **PathEffect（实验性）**：`DashPathEffect`, `CornerPathEffect`（sk_sp 生命周期未完善，使用需注意）
- **构建**：`cargo:rerun-if-changed` 优化增量编译

### Dependencies

- **[pathkit](https://github.com/libpag/pathkit)** — 从 Skia 提取的 PathOps C++ 库，随本 crate 内嵌编译（BSD-3-Clause）
- PathOps C++ library extracted from Skia, vendored and compiled with this crate (BSD-3-Clause)
