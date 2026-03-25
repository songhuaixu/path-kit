#[cxx::bridge]
pub(crate) mod ffi {
    //! cxx 与 PathKit C++ 的共享类型及 `pk_*` 声明（仅 crate 内使用）。
    //! Shared cxx types and `pk_*` bindings (crate-internal only).

    /// 路径布尔运算，与 `pk::SkPathOp` 取值一致。
    /// Path boolean op; matches `pk::SkPathOp`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum PathOp {
        /// 差集 (one \\ two)。Difference.
        Difference = 0,
        /// 交集。Intersection.
        Intersect = 1,
        /// 并集。Union.
        Union = 2,
        /// 异或。XOR.
        Xor = 3,
        /// 反色差集。Reverse difference.
        ReverseDifference = 4,
    }

    /// 路径方向，与 `pk::SkPathDirection` 一致。
    /// Path contour direction; matches `pk::SkPathDirection`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Direction {
        /// 顺时针。Clockwise.
        Cw = 0,
        /// 逆时针。Counter-clockwise.
        Ccw = 1,
    }

    /// 填充规则低位，与 `pk::SkPathFillType` 一致。
    /// Fill rule; matches `pk::SkPathFillType`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum PathFillType {
        /// 环绕规则。Non-zero winding.
        Winding = 0,
        /// 奇偶规则。Even-odd.
        EvenOdd = 1,
        /// 反色环绕。Inverse winding.
        InverseWinding = 2,
        /// 反色奇偶。Inverse even-odd.
        InverseEvenOdd = 3,
    }

    /// 矩形起始角，与 `SkPath::addRect` 角索引一致。
    /// Rect start corner index for `addRect`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum RectCorner {
        /// 左上。Upper-left.
        UpperLeft = 0,
        /// 右上。Upper-right.
        UpperRight = 1,
        /// 右下。Lower-right.
        LowerRight = 2,
        /// 左下。Lower-left.
        LowerLeft = 3,
    }

    /// 圆角矩形分类，与 `pk::SkRRect::Type` 一致。
    /// RRect specialization; matches `pk::SkRRect::Type`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i32)]
    pub enum RRectType {
        /// 空或无面积。Empty.
        Empty = 0,
        /// 直角矩形。Axis rect, zero radii.
        Rect = 1,
        /// 椭圆。Oval.
        Oval = 2,
        /// 四角等径圆角。Simple (uniform radii).
        Simple = 3,
        /// 轴对齐不等径。Nine-patch-style radii.
        NinePatch = 4,
        /// 一般圆角矩形。Complex.
        Complex = 5,
    }

    /// 路径动词，与 `pk::SkPath::Verb` 一致。
    /// Path verb; matches `pk::SkPath::Verb`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum PathVerb {
        /// Move。Move.
        Move = 0,
        /// Line。Line.
        Line = 1,
        /// 二次曲线。Quadratic.
        Quad = 2,
        /// 圆锥曲线。Conic.
        Conic = 3,
        /// 三次曲线。Cubic.
        Cubic = 4,
        /// 闭合。Close.
        Close = 5,
        /// 迭代结束。Iterator done.
        Done = 6,
    }

    /// 绘图样式，与 `pk::SkPaint::Style` 一致。
    /// Paint style; matches `pk::SkPaint::Style`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PaintStyle {
        /// 仅填充。Fill.
        Fill = 0,
        /// 仅描边。Stroke.
        Stroke = 1,
        /// 描边+填充。Stroke and fill.
        StrokeAndFill = 2,
    }

    /// 线端样式，与 `pk::SkPaint::Cap` / `SkStrokeRec` 一致。
    /// Stroke cap; matches `pk::SkPaint::Cap`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum StrokeCap {
        /// 平头。Butt.
        Butt = 0,
        /// 圆头。Round.
        Round = 1,
        /// 方头。Square.
        Square = 2,
    }

    /// 转角连接，与 `pk::SkPaint::Join` / `SkStrokeRec` 一致。
    /// Stroke join; matches `pk::SkPaint::Join`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum StrokeJoin {
        /// 尖角（斜接）。Miter.
        Miter = 0,
        /// 圆角。Round.
        Round = 1,
        /// 斜切。Bevel.
        Bevel = 2,
    }

    /// `SkStrokeRec` 初始样式（`pk::SkStrokeRec::InitStyle`）。
    /// Initial style for `stroke_rec_new`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum StrokeRecInit {
        /// 极细线。Hairline.
        Hairline = 0,
        /// 填充。Fill.
        Fill = 1,
    }

    /// `SkStrokeRec::getStyle` 返回值。
    /// Result of `SkStrokeRec::getStyle`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum StrokeRecStyleTag {
        /// 极细线。Hairline.
        Hairline = 0,
        /// 填充。Fill.
        Fill = 1,
        /// 描边。Stroke.
        Stroke = 2,
        /// 描边+填充。Stroke and fill.
        StrokeAndFill = 3,
    }

    /// cxx 共享 `SkPoint` 布局。
    /// Shared `SkPoint` layout for FFI.
    #[derive(Clone, Copy, Debug)]
    #[cxx_name = "SkPoint"]
    struct Point {
        /// X 坐标。X coordinate.
        fX: f32,
        /// Y 坐标。Y coordinate.
        fY: f32,
    }

    /// cxx 共享 `SkRect` 布局。
    /// Shared `SkRect` layout for FFI.
    #[derive(Clone, Copy, Debug)]
    #[cxx_name = "SkRect"]
    struct Rect {
        /// 左。Left.
        fLeft: f32,
        /// 上。Top.
        fTop: f32,
        /// 右。Right.
        fRight: f32,
        /// 下。Bottom.
        fBottom: f32,
    }

    /// cxx 共享 `SkRRect` 布局（外接矩形 + 四角半径 + 类型）。
    /// Shared `SkRRect` layout (bounds, corner radii, type).
    #[derive(Clone, Copy, Debug)]
    #[cxx_name = "SkRRect"]
    struct RRect {
        /// 外接矩形。Bounds.
        fRect: Rect,
        /// 四角 (UL, UR, LR, LL) 半径点。Corner radii (upper-left … lower-left).
        fRadii: [Point; 4],
        /// 分类枚举。`SkRRect::Type`.
        fType: RRectType,
    }

    unsafe extern "C++" {
        include!("include/pathkit_cxx_decl.h");

        /// `pk::SkPath` 不透明句柄。Opaque `pk::SkPath` handle.
        #[cxx_name = "SkPath"]
        #[namespace = "pk"]
        type Path;

        /// `SkPath::Iter` 包装。Wraps `SkPath::Iter`.
        type PathIterInner;
        /// `SkPathMeasure` 包装。Wraps `SkPathMeasure`.
        type PathMeasureHolder;
        /// `SkPaint` 包装。Wraps `SkPaint`.
        type PaintHolder;
        /// `SkStrokeRec` 包装。Wraps `SkStrokeRec`.
        type StrokeRecHolder;
        /// `SkPathEffect` 的 `sk_sp` 包装。Wraps `sk_sp<SkPathEffect>`.
        type PathEffectHolder;
        /// `SkOpBuilder` 包装。Wraps `SkOpBuilder`.
        type OpBuilderHolder;
        /// `SkPathBuilder` 包装。Wraps `SkPathBuilder`.
        type PathBuilderHolder;

        /// 新建空路径。Create empty path.
        #[cxx_name = "pk_path_new"]
        fn path_new() -> UniquePtr<Path>;
        /// 克隆路径。Clone path.
        #[cxx_name = "pk_path_clone"]
        fn path_clone(p: &Path) -> UniquePtr<Path>;
        /// 重置为空。Reset path to empty.
        #[cxx_name = "pk_path_reset"]
        fn path_reset(p: Pin<&mut Path>);

        /// 点数。Point count.
        #[cxx_name = "pk_path_count_points"]
        fn path_count_points(p: &Path) -> i32;
        /// 动词数。Verb count.
        #[cxx_name = "pk_path_count_verbs"]
        fn path_count_verbs(p: &Path) -> i32;
        /// 写入第 `index` 个点。Write point at index.
        #[cxx_name = "pk_path_get_point"]
        fn path_get_point(p: &Path, index: i32, out: &mut Point);
        /// 紧密轴对齐包围盒。Tight bounds (control points).
        #[cxx_name = "pk_path_compute_tight_bounds"]
        fn path_compute_tight_bounds(p: &Path, out: &mut Rect);
        /// 末轮廓是否闭合。Last contour closed.
        #[cxx_name = "pk_path_is_last_contour_closed"]
        fn path_is_last_contour_closed(p: &Path) -> bool;
        /// 保守包含判断。Conservative rect containment.
        #[cxx_name = "pk_path_conservatively_contains_rect"]
        fn path_conservatively_contains_rect(p: &Path, r: &Rect) -> bool;
        /// 若路径为矩形则写出边界与方向。Detect axis rect.
        #[cxx_name = "pk_path_is_rect"]
        fn path_is_rect(
            p: &Path,
            rect: &mut Rect,
            is_closed: &mut bool,
            direction: &mut Direction,
        ) -> bool;
        /// 填充内点判断（当前 fill type）。Contains (x,y) in fill.
        #[cxx_name = "pk_path_contains"]
        fn path_contains(p: &Path, x: f32, y: f32) -> bool;
        /// 当前填充类型。Current fill type.
        #[cxx_name = "pk_path_fill_type_bits"]
        fn path_fill_type_bits(p: &Path) -> PathFillType;
        /// 设置填充类型。Set fill type.
        #[cxx_name = "pk_path_set_fill_type_bits"]
        fn path_set_fill_type_bits(p: Pin<&mut Path>, v: PathFillType);
        /// 切换反色填充位。Toggle inverse fill.
        #[cxx_name = "pk_path_toggle_inverse_fill_type"]
        fn path_toggle_inverse_fill_type(p: Pin<&mut Path>);

        /// moveTo。moveTo.
        #[cxx_name = "pk_path_move_to"]
        fn path_move_to(p: Pin<&mut Path>, x: f32, y: f32);
        /// lineTo。lineTo.
        #[cxx_name = "pk_path_line_to"]
        fn path_line_to(p: Pin<&mut Path>, x: f32, y: f32);
        /// quadTo。quadTo.
        #[cxx_name = "pk_path_quad_to"]
        fn path_quad_to(p: Pin<&mut Path>, x1: f32, y1: f32, x2: f32, y2: f32);
        /// cubicTo。cubicTo.
        #[cxx_name = "pk_path_cubic_to"]
        fn path_cubic_to(
            p: Pin<&mut Path>,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            x3: f32,
            y3: f32,
        );
        /// close。close.
        #[cxx_name = "pk_path_close"]
        fn path_close(p: Pin<&mut Path>);

        /// addRect。addRect.
        #[cxx_name = "pk_path_add_rect"]
        fn path_add_rect(p: Pin<&mut Path>, rect: &Rect, dir: Direction, start: RectCorner);
        /// addOval。addOval.
        #[cxx_name = "pk_path_add_oval"]
        fn path_add_oval(p: Pin<&mut Path>, rect: &Rect, dir: Direction);
        /// addCircle。addCircle.
        #[cxx_name = "pk_path_add_circle"]
        fn path_add_circle(p: Pin<&mut Path>, cx: f32, cy: f32, radius: f32, dir: Direction);
        /// addRoundRect。addRoundRect.
        #[cxx_name = "pk_path_add_round_rect"]
        fn path_add_round_rect(
            p: Pin<&mut Path>,
            rect: &Rect,
            rx: f32,
            ry: f32,
            dir: Direction,
        );
        /// addRRect。addRRect.
        #[cxx_name = "pk_path_add_rrect"]
        fn path_add_rrect(p: Pin<&mut Path>, rr: &RRect, dir: Direction);
        /// addRRect 指定起点角。addRRect with start index.
        #[cxx_name = "pk_path_add_rrect_start"]
        fn path_add_rrect_start(
            p: Pin<&mut Path>,
            rr: &RRect,
            dir: Direction,
            start: RectCorner,
        );
        /// 若路径为 RRect 则写出。isRRect.
        #[cxx_name = "pk_path_is_rrect"]
        fn path_is_rrect(p: &Path, out: &mut RRect) -> bool;

        /// 新建 `SkPathBuilder`。New path builder.
        #[cxx_name = "pk_path_builder_new"]
        fn path_builder_new() -> UniquePtr<PathBuilderHolder>;
        /// reset。reset.
        #[cxx_name = "pk_path_builder_reset"]
        fn path_builder_reset(b: Pin<&mut PathBuilderHolder>);
        /// 当前填充类型。Builder fill type.
        #[cxx_name = "pk_path_builder_fill_type"]
        fn path_builder_fill_type(b: &PathBuilderHolder) -> PathFillType;
        /// 设置填充类型。Set builder fill type.
        #[cxx_name = "pk_path_builder_set_fill_type"]
        fn path_builder_set_fill_type(b: Pin<&mut PathBuilderHolder>, ft: PathFillType);
        /// 切换反色填充。Toggle inverse fill on builder.
        #[cxx_name = "pk_path_builder_toggle_inverse_fill_type"]
        fn path_builder_toggle_inverse_fill_type(b: Pin<&mut PathBuilderHolder>);
        /// snapshot → 路径副本。Snapshot (builder unchanged).
        #[cxx_name = "pk_path_builder_snapshot"]
        fn path_builder_snapshot(b: &PathBuilderHolder) -> UniquePtr<Path>;
        /// detach → 取路径并清空 builder。Detach (resets builder).
        #[cxx_name = "pk_path_builder_detach"]
        fn path_builder_detach(b: Pin<&mut PathBuilderHolder>) -> UniquePtr<Path>;
        /// Builder moveTo。Builder moveTo.
        #[cxx_name = "pk_path_builder_move_to"]
        fn path_builder_move_to(b: Pin<&mut PathBuilderHolder>, x: f32, y: f32);
        /// Builder lineTo。Builder lineTo.
        #[cxx_name = "pk_path_builder_line_to"]
        fn path_builder_line_to(b: Pin<&mut PathBuilderHolder>, x: f32, y: f32);
        /// Builder quadTo。Builder quadTo.
        #[cxx_name = "pk_path_builder_quad_to"]
        fn path_builder_quad_to(b: Pin<&mut PathBuilderHolder>, x1: f32, y1: f32, x2: f32, y2: f32);
        /// Builder cubicTo。Builder cubicTo.
        #[cxx_name = "pk_path_builder_cubic_to"]
        fn path_builder_cubic_to(
            b: Pin<&mut PathBuilderHolder>,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            x3: f32,
            y3: f32,
        );
        /// Builder close。Builder close.
        #[cxx_name = "pk_path_builder_close"]
        fn path_builder_close(b: Pin<&mut PathBuilderHolder>);
        /// Builder addRect。Builder addRect.
        #[cxx_name = "pk_path_builder_add_rect"]
        fn path_builder_add_rect(
            b: Pin<&mut PathBuilderHolder>,
            rect: &Rect,
            dir: Direction,
            start: RectCorner,
        );
        /// Builder addOval。Builder addOval.
        #[cxx_name = "pk_path_builder_add_oval"]
        fn path_builder_add_oval(b: Pin<&mut PathBuilderHolder>, rect: &Rect, dir: Direction);
        /// Builder addCircle。Builder addCircle.
        #[cxx_name = "pk_path_builder_add_circle"]
        fn path_builder_add_circle(
            b: Pin<&mut PathBuilderHolder>,
            cx: f32,
            cy: f32,
            radius: f32,
            dir: Direction,
        );
        /// Builder addRoundRect。Builder addRoundRect.
        #[cxx_name = "pk_path_builder_add_round_rect"]
        fn path_builder_add_round_rect(
            b: Pin<&mut PathBuilderHolder>,
            rect: &Rect,
            rx: f32,
            ry: f32,
            dir: Direction,
        );
        /// Builder addRRect。Builder addRRect.
        #[cxx_name = "pk_path_builder_add_rrect"]
        fn path_builder_add_rrect(b: Pin<&mut PathBuilderHolder>, rr: &RRect, dir: Direction);
        /// Builder addRRect 带起点。Builder addRRect with start.
        #[cxx_name = "pk_path_builder_add_rrect_start"]
        fn path_builder_add_rrect_start(
            b: Pin<&mut PathBuilderHolder>,
            rr: &RRect,
            dir: Direction,
            start: RectCorner,
        );
        /// Builder 追加路径几何。Builder addPath.
        #[cxx_name = "pk_path_builder_add_path"]
        fn path_builder_add_path(b: Pin<&mut PathBuilderHolder>, src: &Path);

        /// 新建路径迭代器。New path iterator.
        #[cxx_name = "pk_path_iter_new"]
        fn path_iter_new(path: &Path, force_close: bool) -> UniquePtr<PathIterInner>;
        /// 下一步动词与点。Next verb (fills `p0`…`p3`).
        #[cxx_name = "pk_path_iter_next"]
        fn path_iter_next(
            it: Pin<&mut PathIterInner>,
            p0: &mut Point,
            p1: &mut Point,
            p2: &mut Point,
            p3: &mut Point,
        ) -> PathVerb;

        /// 空 `PathMeasure`。Empty measure.
        #[cxx_name = "pk_measure_new"]
        fn measure_new() -> UniquePtr<PathMeasureHolder>;
        /// 绑定路径的测量器。Measure from path.
        #[cxx_name = "pk_measure_from_path"]
        fn measure_from_path(path: &Path, force_closed: bool, res_scale: f32)
            -> UniquePtr<PathMeasureHolder>;
        /// 重新绑定路径。Reset measure path.
        #[cxx_name = "pk_measure_set_path"]
        fn measure_set_path(m: Pin<&mut PathMeasureHolder>, path: &Path, force_closed: bool);
        /// 当前轮廓长度。Contour length.
        #[cxx_name = "pk_measure_length"]
        fn measure_length(m: Pin<&mut PathMeasureHolder>) -> f32;
        /// 距离处位姿与切线。getPosTan.
        #[cxx_name = "pk_measure_get_pos_tan"]
        fn measure_get_pos_tan(
            m: Pin<&mut PathMeasureHolder>,
            distance: f32,
            position: &mut Point,
            tangent: &mut Point,
        ) -> bool;
        /// 提取子段到 `dst`。getSegment.
        #[cxx_name = "pk_measure_get_segment"]
        fn measure_get_segment(
            m: Pin<&mut PathMeasureHolder>,
            start_d: f32,
            stop_d: f32,
            dst: Pin<&mut Path>,
            start_with_move_to: bool,
        ) -> bool;
        /// 当前轮廓是否闭合。isClosed.
        #[cxx_name = "pk_measure_is_closed"]
        fn measure_is_closed(m: Pin<&mut PathMeasureHolder>) -> bool;
        /// 下一轮廓。nextContour.
        #[cxx_name = "pk_measure_next_contour"]
        fn measure_next_contour(m: Pin<&mut PathMeasureHolder>) -> bool;

        /// 新建 `SkPaint`。New paint.
        #[cxx_name = "pk_paint_new"]
        fn paint_new() -> UniquePtr<PaintHolder>;
        /// 克隆 `SkPaint`。Clone paint.
        #[cxx_name = "pk_paint_clone"]
        fn paint_clone(p: &PaintHolder) -> UniquePtr<PaintHolder>;
        /// setStyle(Fill)。Set fill style.
        #[cxx_name = "pk_paint_set_fill"]
        fn paint_set_fill(p: Pin<&mut PaintHolder>);
        /// 开启/关闭描边。Enable stroke.
        #[cxx_name = "pk_paint_set_stroke"]
        fn paint_set_stroke(p: Pin<&mut PaintHolder>, enable: bool);
        /// 设置 `PaintStyle`。Set paint style.
        #[cxx_name = "pk_paint_set_style"]
        fn paint_set_style(p: Pin<&mut PaintHolder>, style: PaintStyle);
        /// 描边宽度。Stroke width.
        #[cxx_name = "pk_paint_set_stroke_width"]
        fn paint_set_stroke_width(p: Pin<&mut PaintHolder>, width: f32);
        /// Miter 限制。Miter limit.
        #[cxx_name = "pk_paint_set_stroke_miter"]
        fn paint_set_stroke_miter(p: Pin<&mut PaintHolder>, miter: f32);
        /// 线端。Stroke cap.
        #[cxx_name = "pk_paint_set_stroke_cap"]
        fn paint_set_stroke_cap(p: Pin<&mut PaintHolder>, cap: StrokeCap);
        /// 转角。Stroke join.
        #[cxx_name = "pk_paint_set_stroke_join"]
        fn paint_set_stroke_join(p: Pin<&mut PaintHolder>, join: StrokeJoin);
        /// 描边/填充后的填充路径。getFillPath.
        #[cxx_name = "pk_paint_get_fill_path"]
        fn paint_get_fill_path(
            p: &PaintHolder,
            src: &Path,
            dst: Pin<&mut Path>,
        ) -> bool;

        /// 新建 `SkStrokeRec`。New stroke rec.
        #[cxx_name = "pk_stroke_rec_new"]
        fn stroke_rec_new(init: StrokeRecInit) -> UniquePtr<StrokeRecHolder>;
        /// setFillStyle。setFillStyle.
        #[cxx_name = "pk_stroke_rec_set_fill"]
        fn stroke_rec_set_fill(p: Pin<&mut StrokeRecHolder>);
        /// setHairlineStyle。setHairlineStyle.
        #[cxx_name = "pk_stroke_rec_set_hairline"]
        fn stroke_rec_set_hairline(p: Pin<&mut StrokeRecHolder>);
        /// setStrokeStyle。setStrokeStyle.
        #[cxx_name = "pk_stroke_rec_set_stroke_style"]
        fn stroke_rec_set_stroke_style(p: Pin<&mut StrokeRecHolder>, width: f32, saf: bool);
        /// getStyle。getStyle.
        #[cxx_name = "pk_stroke_rec_get_style"]
        fn stroke_rec_get_style(p: &StrokeRecHolder) -> StrokeRecStyleTag;
        /// getWidth。getWidth.
        #[cxx_name = "pk_stroke_rec_width"]
        fn stroke_rec_width(p: &StrokeRecHolder) -> f32;
        /// getCap。getCap.
        #[cxx_name = "pk_stroke_rec_cap"]
        fn stroke_rec_cap(p: &StrokeRecHolder) -> StrokeCap;
        /// setCap（保留 join/miter）。set cap.
        #[cxx_name = "pk_stroke_rec_set_cap"]
        fn stroke_rec_set_cap(p: Pin<&mut StrokeRecHolder>, cap: StrokeCap);
        /// getJoin。getJoin.
        #[cxx_name = "pk_stroke_rec_join"]
        fn stroke_rec_join(p: &StrokeRecHolder) -> StrokeJoin;
        /// setJoin。setJoin.
        #[cxx_name = "pk_stroke_rec_set_join"]
        fn stroke_rec_set_join(p: Pin<&mut StrokeRecHolder>, join: StrokeJoin);
        /// getMiter。getMiter.
        #[cxx_name = "pk_stroke_rec_miter_limit"]
        fn stroke_rec_miter_limit(p: &StrokeRecHolder) -> f32;
        /// setStrokeParams。setStrokeParams.
        #[cxx_name = "pk_stroke_rec_set_stroke_params"]
        fn stroke_rec_set_stroke_params(
            p: Pin<&mut StrokeRecHolder>,
            cap: StrokeCap,
            join: StrokeJoin,
            miter_limit: f32,
        );
        /// 描边向外膨胀半径。Inflation radius.
        #[cxx_name = "pk_stroke_rec_inflation_radius"]
        fn stroke_rec_inflation_radius(p: &StrokeRecHolder) -> f32;
        /// 描边到填充路径。applyToPath.
        #[cxx_name = "pk_stroke_rec_apply_to_path"]
        fn stroke_rec_apply_to_path(
            p: &StrokeRecHolder,
            dst: Pin<&mut Path>,
            src: &Path,
        ) -> bool;

        /// 两路径布尔运算写入 `result`。PathOp on two paths.
        #[cxx_name = "pk_op"]
        fn boolean_path_op(one: &Path, two: &Path, op: PathOp, result: Pin<&mut Path>) -> bool;

        /// 新建 `SkOpBuilder`。New op builder.
        #[cxx_name = "pk_op_builder_new"]
        fn op_builder_new() -> UniquePtr<OpBuilderHolder>;
        /// add(path, op)。Add path to op builder.
        #[cxx_name = "pk_op_builder_add"]
        fn op_builder_add(h: Pin<&mut OpBuilderHolder>, path: &Path, op: PathOp);
        /// resolve → `result`。Resolve op chain.
        #[cxx_name = "pk_op_builder_resolve"]
        fn op_builder_resolve(h: Pin<&mut OpBuilderHolder>, result: Pin<&mut Path>) -> bool;

        /// PathOps 简化。Simplify path.
        #[cxx_name = "pk_simplify"]
        fn simplify_path(path: &Path, result: Pin<&mut Path>) -> bool;
        /// PathOps 紧密包围盒。Tight bounds (pathops).
        #[cxx_name = "pk_tight_bounds"]
        fn pathops_tight_bounds(path: &Path, out: &mut Rect) -> bool;

        /// 空 RRect。Empty RRect.
        #[cxx_name = "pk_rrect_new_empty"]
        fn rrect_new_empty(out: &mut RRect);
        /// setRectXY。setRectXY.
        #[cxx_name = "pk_rrect_set_rect_xy"]
        fn rrect_set_rect_xy(rr: &mut RRect, rect: &Rect, rx: f32, ry: f32);
        /// setOval。setOval.
        #[cxx_name = "pk_rrect_set_oval"]
        fn rrect_set_oval(rr: &mut RRect, rect: &Rect);
        /// setRectRadii。setRectRadii.
        #[cxx_name = "pk_rrect_set_rect_radii"]
        fn rrect_set_rect_radii(rr: &mut RRect, rect: &Rect, radii: &[Point]);
        /// isValid。isValid.
        #[cxx_name = "pk_rrect_is_valid"]
        fn rrect_is_valid(rr: &RRect) -> bool;

        /// `SkCornerPathEffect`。Corner path effect.
        #[cxx_name = "pk_corner_effect_make"]
        fn corner_effect_make(radius: f32) -> UniquePtr<PathEffectHolder>;
        /// `SkDashPathEffect`。Dash path effect.
        #[cxx_name = "pk_dash_effect_make"]
        fn dash_effect_make(intervals: &[f32], phase: f32) -> UniquePtr<PathEffectHolder>;
        /// PathEffect::filterPath + stroke rec + cull rect。Filter path effect.
        #[cxx_name = "pk_path_effect_filter"]
        fn path_effect_filter(
            e: &PathEffectHolder,
            dst: Pin<&mut Path>,
            src: &Path,
            rec: Pin<&mut StrokeRecHolder>,
            cull: &Rect,
        ) -> bool;
    }
}
