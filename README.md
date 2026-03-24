# path-kit

基于 Skia PathOps 和 PathKit 的 Rust 路径运算库，提供 safe 的 API 封装。  
A Rust path operations library based on Skia PathOps and PathKit with safe API wrappers.

## 功能 / Features

- **路径构建**：线段、二次/三次贝塞尔、矩形、椭圆、圆、圆角矩形、RRect（四角独立半径）
- **路径布尔运算**：并集、交集、差集、异或
- **路径简化、包围盒**：`simplify`, `path.tight_bounds`, `pathops_tight_bounds`
- **路径迭代**：按动词遍历 Move/Line/Quad/Cubic/Close
- **描边**：将路径转为描边轮廓

## 线程安全 / Thread safety

当前未保证 `Send` / `Sync`，请勿跨线程共享 `Path`、`RRect`、`StrokeRec` 等类型。  
`Send` / `Sync` are not guaranteed; do not share `Path`, `RRect`, `StrokeRec`, etc. across threads.

## 安装 / Installation

```toml
[dependencies]
path-kit = "0.1"
```

## 示例 / Examples

### 路径布尔运算

```rust
use path_kit::{Path, Rect, Direction, RectCorner, PathOp, path_op, OpBuilder};

let mut path1 = Path::new();
path1.add_rect(&Rect::new(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);

let mut path2 = Path::new();
path2.add_rect(&Rect::new(50.0, 50.0, 150.0, 150.0), Direction::Cw, RectCorner::UpperLeft);

let union = path_op(&path1, &path2, PathOp::Union).unwrap();

// 批量运算
let result = OpBuilder::new()
    .add(path1.clone(), PathOp::Union)
    .add(path2.clone(), PathOp::Union)
    .resolve()
    .unwrap();
```

### 圆角矩形 RRect

```rust
use path_kit::{Path, Rect, RRect, Radii, Direction};

// 统一圆角
let rr = RRect::from_rect_xy(&Rect::new(0.0, 0.0, 100.0, 50.0), 10.0, 10.0);
let mut path = Path::new();
path.add_rrect(&rr, Direction::Cw);

// 四角独立半径
let radii = [
    Radii { x: 10.0, y: 10.0 },
    Radii { x: 20.0, y: 10.0 },
    Radii { x: 10.0, y: 20.0 },
    Radii { x: 5.0, y: 5.0 },
];
let rr2 = RRect::from_rect_radii(&Rect::new(0.0, 0.0, 80.0, 60.0), &radii);
path.add_rrect(&rr2, Direction::Ccw);
```

### 路径迭代

```rust
use path_kit::{Path, PathVerbItem};

let mut path = Path::new();
path.move_to(0.0, 0.0).line_to(100.0, 0.0).line_to(100.0, 100.0).close();

for item in path.iter(false) {
    match item {
        PathVerbItem::Move(p) => println!("Move to {:?}", p),
        PathVerbItem::Line(from, to) => println!("Line {:?} -> {:?}", from, to),
        PathVerbItem::Close => println!("Close"),
        _ => {}
    }
}
```

### 描边

```rust
use path_kit::{Path, StrokeRec};

let rec = StrokeRec::new_stroke(4.0, false);
let mut path = Path::new();
path.move_to(0.0, 0.0).line_to(100.0, 0.0);
let stroked = rec.apply_to_path(&path).unwrap();
```

## 致谢 / Acknowledgments

感谢以下开源项目：

- **[Skia](https://skia.org/)** — 2D 图形库，本库的 PathOps 能力源自 Skia
- **[libpag/pathkit](https://github.com/libpag/pathkit)** — 从 Skia 提取的 PathOps 库，提供轻量级 PathOps API

Thanks to:
- **[Skia](https://skia.org/)** — 2D graphics library, PathOps capabilities originate from Skia
- **[libpag/pathkit](https://github.com/libpag/pathkit)** — PathOps library extracted from Skia, providing a lightweight PathOps API

## License

BSD-3-Clause （与 Skia 一致 / Same as Skia）
