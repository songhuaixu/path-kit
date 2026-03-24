#[cxx::bridge]
pub(crate) mod ffi {
    /// 路径布尔运算，与 `pk::SkPathOp` 取值一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum PathOp {
        Difference = 0,
        Intersect = 1,
        Union = 2,
        Xor = 3,
        ReverseDifference = 4,
    }

    /// 与 `pk::SkPathDirection` 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Direction {
        Cw = 0,
        Ccw = 1,
    }

    /// 与 `pk::SkPathFillType` 低位取值一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum PathFillType {
        Winding = 0,
        EvenOdd = 1,
        InverseWinding = 2,
        InverseEvenOdd = 3,
    }

    /// 矩形起始角，与 `SkPath::Corner` / `addRect` 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum RectCorner {
        UpperLeft = 0,
        UpperRight = 1,
        LowerRight = 2,
        LowerLeft = 3,
    }

    /// 与 `pk::SkRRect::Type` 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i32)]
    pub enum RRectType {
        Empty = 0,
        Rect = 1,
        Oval = 2,
        Simple = 3,
        NinePatch = 4,
        Complex = 5,
    }

    /// 与 `pk::SkPath::Verb` 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum PathVerb {
        Move = 0,
        Line = 1,
        Quad = 2,
        Conic = 3,
        Cubic = 4,
        Close = 5,
        Done = 6,
    }

    /// 与 `pk::SkPaint::Style` 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum PaintStyle {
        Fill = 0,
        Stroke = 1,
        StrokeAndFill = 2,
    }

    /// 与 `pk::SkPaint::Cap` / `SkStrokeRec` cap 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum StrokeCap {
        Butt = 0,
        Round = 1,
        Square = 2,
    }

    /// 与 `pk::SkPaint::Join` / `SkStrokeRec` join 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum StrokeJoin {
        Miter = 0,
        Round = 1,
        Bevel = 2,
    }

    /// `pk::SkStrokeRec::InitStyle`（C++ 侧仍为 `pk_stroke_rec_new`）。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum StrokeRecInit {
        Hairline = 0,
        Fill = 1,
    }

    /// `pk::SkStrokeRec::Style`（`SkStrokeRec::getStyle` 返回值）。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum StrokeRecStyleTag {
        Hairline = 0,
        Fill = 1,
        Stroke = 2,
        StrokeAndFill = 3,
    }

    #[derive(Clone, Copy, Debug)]
    #[cxx_name = "SkPoint"]
    struct Point {
        fX: f32,
        fY: f32,
    }

    #[derive(Clone, Copy, Debug)]
    #[cxx_name = "SkRect"]
    struct Rect {
        fLeft: f32,
        fTop: f32,
        fRight: f32,
        fBottom: f32,
    }

    #[derive(Clone, Copy, Debug)]
    #[cxx_name = "SkRRect"]
    struct RRect {
        fRect: Rect,
        fRadii: [Point; 4],
        fType: RRectType,
    }

    unsafe extern "C++" {
        include!("include/pathkit_cxx_decl.h");

        #[cxx_name = "SkPath"]
        #[namespace = "pk"]
        type Path;

        type PathIterInner;
        type PathMeasureHolder;
        type PaintHolder;
        type StrokeRecHolder;
        type PathEffectHolder;
        type OpBuilderHolder;
        type PathBuilderHolder;

        #[cxx_name = "pk_path_new"]
        fn path_new() -> UniquePtr<Path>;
        #[cxx_name = "pk_path_clone"]
        fn path_clone(p: &Path) -> UniquePtr<Path>;
        #[cxx_name = "pk_path_reset"]
        fn path_reset(p: Pin<&mut Path>);

        #[cxx_name = "pk_path_count_points"]
        fn path_count_points(p: &Path) -> i32;
        #[cxx_name = "pk_path_count_verbs"]
        fn path_count_verbs(p: &Path) -> i32;
        #[cxx_name = "pk_path_get_point"]
        fn path_get_point(p: &Path, index: i32, out: &mut Point);
        #[cxx_name = "pk_path_compute_tight_bounds"]
        fn path_compute_tight_bounds(p: &Path, out: &mut Rect);
        #[cxx_name = "pk_path_is_last_contour_closed"]
        fn path_is_last_contour_closed(p: &Path) -> bool;
        #[cxx_name = "pk_path_conservatively_contains_rect"]
        fn path_conservatively_contains_rect(p: &Path, r: &Rect) -> bool;
        #[cxx_name = "pk_path_is_rect"]
        fn path_is_rect(
            p: &Path,
            rect: &mut Rect,
            is_closed: &mut bool,
            direction: &mut Direction,
        ) -> bool;
        #[cxx_name = "pk_path_contains"]
        fn path_contains(p: &Path, x: f32, y: f32) -> bool;
        #[cxx_name = "pk_path_fill_type_bits"]
        fn path_fill_type_bits(p: &Path) -> PathFillType;
        #[cxx_name = "pk_path_set_fill_type_bits"]
        fn path_set_fill_type_bits(p: Pin<&mut Path>, v: PathFillType);
        #[cxx_name = "pk_path_toggle_inverse_fill_type"]
        fn path_toggle_inverse_fill_type(p: Pin<&mut Path>);

        #[cxx_name = "pk_path_move_to"]
        fn path_move_to(p: Pin<&mut Path>, x: f32, y: f32);
        #[cxx_name = "pk_path_line_to"]
        fn path_line_to(p: Pin<&mut Path>, x: f32, y: f32);
        #[cxx_name = "pk_path_quad_to"]
        fn path_quad_to(p: Pin<&mut Path>, x1: f32, y1: f32, x2: f32, y2: f32);
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
        #[cxx_name = "pk_path_close"]
        fn path_close(p: Pin<&mut Path>);

        #[cxx_name = "pk_path_add_rect"]
        fn path_add_rect(p: Pin<&mut Path>, rect: &Rect, dir: Direction, start: RectCorner);
        #[cxx_name = "pk_path_add_oval"]
        fn path_add_oval(p: Pin<&mut Path>, rect: &Rect, dir: Direction);
        #[cxx_name = "pk_path_add_circle"]
        fn path_add_circle(p: Pin<&mut Path>, cx: f32, cy: f32, radius: f32, dir: Direction);
        #[cxx_name = "pk_path_add_round_rect"]
        fn path_add_round_rect(
            p: Pin<&mut Path>,
            rect: &Rect,
            rx: f32,
            ry: f32,
            dir: Direction,
        );
        #[cxx_name = "pk_path_add_rrect"]
        fn path_add_rrect(p: Pin<&mut Path>, rr: &RRect, dir: Direction);
        #[cxx_name = "pk_path_add_rrect_start"]
        fn path_add_rrect_start(
            p: Pin<&mut Path>,
            rr: &RRect,
            dir: Direction,
            start: RectCorner,
        );
        #[cxx_name = "pk_path_is_rrect"]
        fn path_is_rrect(p: &Path, out: &mut RRect) -> bool;

        #[cxx_name = "pk_path_builder_new"]
        fn path_builder_new() -> UniquePtr<PathBuilderHolder>;
        #[cxx_name = "pk_path_builder_reset"]
        fn path_builder_reset(b: Pin<&mut PathBuilderHolder>);
        #[cxx_name = "pk_path_builder_fill_type"]
        fn path_builder_fill_type(b: &PathBuilderHolder) -> PathFillType;
        #[cxx_name = "pk_path_builder_set_fill_type"]
        fn path_builder_set_fill_type(b: Pin<&mut PathBuilderHolder>, ft: PathFillType);
        #[cxx_name = "pk_path_builder_toggle_inverse_fill_type"]
        fn path_builder_toggle_inverse_fill_type(b: Pin<&mut PathBuilderHolder>);
        #[cxx_name = "pk_path_builder_snapshot"]
        fn path_builder_snapshot(b: &PathBuilderHolder) -> UniquePtr<Path>;
        #[cxx_name = "pk_path_builder_detach"]
        fn path_builder_detach(b: Pin<&mut PathBuilderHolder>) -> UniquePtr<Path>;
        #[cxx_name = "pk_path_builder_move_to"]
        fn path_builder_move_to(b: Pin<&mut PathBuilderHolder>, x: f32, y: f32);
        #[cxx_name = "pk_path_builder_line_to"]
        fn path_builder_line_to(b: Pin<&mut PathBuilderHolder>, x: f32, y: f32);
        #[cxx_name = "pk_path_builder_quad_to"]
        fn path_builder_quad_to(b: Pin<&mut PathBuilderHolder>, x1: f32, y1: f32, x2: f32, y2: f32);
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
        #[cxx_name = "pk_path_builder_close"]
        fn path_builder_close(b: Pin<&mut PathBuilderHolder>);
        #[cxx_name = "pk_path_builder_add_rect"]
        fn path_builder_add_rect(
            b: Pin<&mut PathBuilderHolder>,
            rect: &Rect,
            dir: Direction,
            start: RectCorner,
        );
        #[cxx_name = "pk_path_builder_add_oval"]
        fn path_builder_add_oval(b: Pin<&mut PathBuilderHolder>, rect: &Rect, dir: Direction);
        #[cxx_name = "pk_path_builder_add_circle"]
        fn path_builder_add_circle(
            b: Pin<&mut PathBuilderHolder>,
            cx: f32,
            cy: f32,
            radius: f32,
            dir: Direction,
        );
        #[cxx_name = "pk_path_builder_add_round_rect"]
        fn path_builder_add_round_rect(
            b: Pin<&mut PathBuilderHolder>,
            rect: &Rect,
            rx: f32,
            ry: f32,
            dir: Direction,
        );
        #[cxx_name = "pk_path_builder_add_rrect"]
        fn path_builder_add_rrect(b: Pin<&mut PathBuilderHolder>, rr: &RRect, dir: Direction);
        #[cxx_name = "pk_path_builder_add_rrect_start"]
        fn path_builder_add_rrect_start(
            b: Pin<&mut PathBuilderHolder>,
            rr: &RRect,
            dir: Direction,
            start: RectCorner,
        );
        #[cxx_name = "pk_path_builder_add_path"]
        fn path_builder_add_path(b: Pin<&mut PathBuilderHolder>, src: &Path);

        #[cxx_name = "pk_path_iter_new"]
        fn path_iter_new(path: &Path, force_close: bool) -> UniquePtr<PathIterInner>;
        #[cxx_name = "pk_path_iter_next"]
        fn path_iter_next(
            it: Pin<&mut PathIterInner>,
            p0: &mut Point,
            p1: &mut Point,
            p2: &mut Point,
            p3: &mut Point,
        ) -> PathVerb;

        #[cxx_name = "pk_measure_new"]
        fn measure_new() -> UniquePtr<PathMeasureHolder>;
        #[cxx_name = "pk_measure_from_path"]
        fn measure_from_path(path: &Path, force_closed: bool, res_scale: f32)
            -> UniquePtr<PathMeasureHolder>;
        #[cxx_name = "pk_measure_set_path"]
        fn measure_set_path(m: Pin<&mut PathMeasureHolder>, path: &Path, force_closed: bool);
        #[cxx_name = "pk_measure_length"]
        fn measure_length(m: Pin<&mut PathMeasureHolder>) -> f32;
        #[cxx_name = "pk_measure_get_pos_tan"]
        fn measure_get_pos_tan(
            m: Pin<&mut PathMeasureHolder>,
            distance: f32,
            position: &mut Point,
            tangent: &mut Point,
        ) -> bool;
        #[cxx_name = "pk_measure_get_segment"]
        fn measure_get_segment(
            m: Pin<&mut PathMeasureHolder>,
            start_d: f32,
            stop_d: f32,
            dst: Pin<&mut Path>,
            start_with_move_to: bool,
        ) -> bool;
        #[cxx_name = "pk_measure_is_closed"]
        fn measure_is_closed(m: Pin<&mut PathMeasureHolder>) -> bool;
        #[cxx_name = "pk_measure_next_contour"]
        fn measure_next_contour(m: Pin<&mut PathMeasureHolder>) -> bool;

        #[cxx_name = "pk_paint_new"]
        fn paint_new() -> UniquePtr<PaintHolder>;
        #[cxx_name = "pk_paint_clone"]
        fn paint_clone(p: &PaintHolder) -> UniquePtr<PaintHolder>;
        #[cxx_name = "pk_paint_set_fill"]
        fn paint_set_fill(p: Pin<&mut PaintHolder>);
        #[cxx_name = "pk_paint_set_stroke"]
        fn paint_set_stroke(p: Pin<&mut PaintHolder>, enable: bool);
        #[cxx_name = "pk_paint_set_style"]
        fn paint_set_style(p: Pin<&mut PaintHolder>, style: PaintStyle);
        #[cxx_name = "pk_paint_set_stroke_width"]
        fn paint_set_stroke_width(p: Pin<&mut PaintHolder>, width: f32);
        #[cxx_name = "pk_paint_set_stroke_miter"]
        fn paint_set_stroke_miter(p: Pin<&mut PaintHolder>, miter: f32);
        #[cxx_name = "pk_paint_set_stroke_cap"]
        fn paint_set_stroke_cap(p: Pin<&mut PaintHolder>, cap: StrokeCap);
        #[cxx_name = "pk_paint_set_stroke_join"]
        fn paint_set_stroke_join(p: Pin<&mut PaintHolder>, join: StrokeJoin);
        #[cxx_name = "pk_paint_get_fill_path"]
        fn paint_get_fill_path(
            p: &PaintHolder,
            src: &Path,
            dst: Pin<&mut Path>,
        ) -> bool;

        #[cxx_name = "pk_stroke_rec_new"]
        fn stroke_rec_new(init: StrokeRecInit) -> UniquePtr<StrokeRecHolder>;
        #[cxx_name = "pk_stroke_rec_set_fill"]
        fn stroke_rec_set_fill(p: Pin<&mut StrokeRecHolder>);
        #[cxx_name = "pk_stroke_rec_set_hairline"]
        fn stroke_rec_set_hairline(p: Pin<&mut StrokeRecHolder>);
        #[cxx_name = "pk_stroke_rec_set_stroke_style"]
        fn stroke_rec_set_stroke_style(p: Pin<&mut StrokeRecHolder>, width: f32, saf: bool);
        #[cxx_name = "pk_stroke_rec_get_style"]
        fn stroke_rec_get_style(p: &StrokeRecHolder) -> StrokeRecStyleTag;
        #[cxx_name = "pk_stroke_rec_width"]
        fn stroke_rec_width(p: &StrokeRecHolder) -> f32;
        #[cxx_name = "pk_stroke_rec_cap"]
        fn stroke_rec_cap(p: &StrokeRecHolder) -> StrokeCap;
        #[cxx_name = "pk_stroke_rec_set_cap"]
        fn stroke_rec_set_cap(p: Pin<&mut StrokeRecHolder>, cap: StrokeCap);
        #[cxx_name = "pk_stroke_rec_join"]
        fn stroke_rec_join(p: &StrokeRecHolder) -> StrokeJoin;
        #[cxx_name = "pk_stroke_rec_set_join"]
        fn stroke_rec_set_join(p: Pin<&mut StrokeRecHolder>, join: StrokeJoin);
        #[cxx_name = "pk_stroke_rec_miter_limit"]
        fn stroke_rec_miter_limit(p: &StrokeRecHolder) -> f32;
        #[cxx_name = "pk_stroke_rec_set_stroke_params"]
        fn stroke_rec_set_stroke_params(
            p: Pin<&mut StrokeRecHolder>,
            cap: StrokeCap,
            join: StrokeJoin,
            miter_limit: f32,
        );
        #[cxx_name = "pk_stroke_rec_inflation_radius"]
        fn stroke_rec_inflation_radius(p: &StrokeRecHolder) -> f32;
        #[cxx_name = "pk_stroke_rec_apply_to_path"]
        fn stroke_rec_apply_to_path(
            p: &StrokeRecHolder,
            dst: Pin<&mut Path>,
            src: &Path,
        ) -> bool;

        #[cxx_name = "pk_op"]
        fn boolean_path_op(one: &Path, two: &Path, op: PathOp, result: Pin<&mut Path>) -> bool;

        #[cxx_name = "pk_op_builder_new"]
        fn op_builder_new() -> UniquePtr<OpBuilderHolder>;
        #[cxx_name = "pk_op_builder_add"]
        fn op_builder_add(h: Pin<&mut OpBuilderHolder>, path: &Path, op: PathOp);
        #[cxx_name = "pk_op_builder_resolve"]
        fn op_builder_resolve(h: Pin<&mut OpBuilderHolder>, result: Pin<&mut Path>) -> bool;

        #[cxx_name = "pk_simplify"]
        fn simplify_path(path: &Path, result: Pin<&mut Path>) -> bool;
        #[cxx_name = "pk_tight_bounds"]
        fn pathops_tight_bounds(path: &Path, out: &mut Rect) -> bool;

        #[cxx_name = "pk_rrect_new_empty"]
        fn rrect_new_empty(out: &mut RRect);
        #[cxx_name = "pk_rrect_set_rect_xy"]
        fn rrect_set_rect_xy(rr: &mut RRect, rect: &Rect, rx: f32, ry: f32);
        #[cxx_name = "pk_rrect_set_oval"]
        fn rrect_set_oval(rr: &mut RRect, rect: &Rect);
        #[cxx_name = "pk_rrect_set_rect_radii"]
        fn rrect_set_rect_radii(rr: &mut RRect, rect: &Rect, radii: &[Point]);
        #[cxx_name = "pk_rrect_is_valid"]
        fn rrect_is_valid(rr: &RRect) -> bool;

        #[cxx_name = "pk_corner_effect_make"]
        fn corner_effect_make(radius: f32) -> UniquePtr<PathEffectHolder>;
        #[cxx_name = "pk_dash_effect_make"]
        fn dash_effect_make(intervals: &[f32], phase: f32) -> UniquePtr<PathEffectHolder>;
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
