#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>

#include "pathkit.h"
#include "include/core/SkPathBuilder.h"
#include "rust/cxx.h"

struct SkPoint;
struct SkRect;
struct SkRRect;

// 完整定义由 cxx 在 bridge.rs.h 中生成；此处仅前向声明。实现符号仍为 pk_*（Rust 侧已用 cxx_name 映射）。
enum class PathOp : std::uint32_t;
enum class Direction : std::uint32_t;
enum class PathFillType : std::uint8_t;
enum class RectCorner : std::uint32_t;
enum class PathVerb : std::uint32_t;
enum class PaintStyle : std::uint8_t;
enum class StrokeCap : std::uint32_t;
enum class StrokeJoin : std::uint8_t;
enum class StrokeRecInit : std::uint32_t;
enum class StrokeRecStyleTag : std::uint32_t;

struct PathIterInner {
    pk::SkPath::Iter iter;
    PathIterInner(const pk::SkPath& p, bool fc) : iter(p, fc) {}
};

struct PathMeasureHolder {
    std::unique_ptr<pk::SkPathMeasure> m;
    PathMeasureHolder() : m(std::make_unique<pk::SkPathMeasure>()) {}
};

struct PaintHolder {
    pk::SkPaint p;
};

struct StrokeRecHolder {
    pk::SkStrokeRec rec;
    explicit StrokeRecHolder(pk::SkStrokeRec::InitStyle s) : rec(s) {}
};

struct PathEffectHolder {
    pk::sk_sp<pk::SkPathEffect> effect;
};

struct OpBuilderHolder {
    pk::SkOpBuilder b;
};

struct PathBuilderHolder {
    pk::SkPathBuilder b;
};

std::unique_ptr<pk::SkPath> pk_path_new();
std::unique_ptr<pk::SkPath> pk_path_clone(const pk::SkPath& p);
void pk_path_reset(pk::SkPath& p);
void pk_path_rewind(pk::SkPath& p);

void pk_path_get_bounds(const pk::SkPath& p, SkRect& out);
bool pk_path_is_finite(const pk::SkPath& p);
bool pk_path_is_convex(const pk::SkPath& p);
bool pk_path_is_oval(const pk::SkPath& p, SkRect& bounds);
bool pk_path_is_line(const pk::SkPath& p, SkPoint& p0, SkPoint& p1);
void pk_path_get_points_copy(const pk::SkPath& p, rust::Vec<SkPoint>& out);
void pk_path_get_verbs_copy(const pk::SkPath& p, rust::Vec<std::uint8_t>& out);
void pk_path_inc_reserve(pk::SkPath& p, std::int32_t extra_pt_count);
bool pk_path_is_interpolatable(const pk::SkPath& a, const pk::SkPath& b);
bool pk_path_interpolate(const pk::SkPath& start, const pk::SkPath& end, float weight, pk::SkPath& out);
bool pk_path_get_last_pt(const pk::SkPath& p, SkPoint& out);
void pk_path_set_last_pt(pk::SkPath& p, float x, float y);
std::uint32_t pk_path_segment_masks(const pk::SkPath& p);
bool pk_path_has_multiple_contours(const pk::SkPath& p);
void pk_path_add_path_offset(pk::SkPath& dst, const pk::SkPath& src, float dx, float dy, bool extend);
void pk_path_reverse_add_path(pk::SkPath& dst, const pk::SkPath& src);

void pk_path_transform(pk::SkPath& p, rust::Slice<const float> mat9);
void pk_path_transform_to(const pk::SkPath& src, rust::Slice<const float> mat9, pk::SkPath& dst);

// SkMatrix 桥接：9 个 float 为 SkMatrix::get9 顺序。
void pk_matrix_reset(rust::Slice<float> m9);
void pk_matrix_set_all(rust::Slice<float> m9, float v0, float v1, float v2, float v3, float v4, float v5,
                       float v6, float v7, float v8);
void pk_matrix_set_translate(rust::Slice<float> m9, float dx, float dy);
void pk_matrix_set_scale(rust::Slice<float> m9, float sx, float sy);
void pk_matrix_set_scale_pivot(rust::Slice<float> m9, float sx, float sy, float px, float py);
void pk_matrix_set_rotate(rust::Slice<float> m9, float degrees);
void pk_matrix_set_rotate_pivot(rust::Slice<float> m9, float degrees, float px, float py);
void pk_matrix_set_sin_cos(rust::Slice<float> m9, float sin_v, float cos_v);
void pk_matrix_set_sin_cos_pivot(rust::Slice<float> m9, float sin_v, float cos_v, float px, float py);
void pk_matrix_set_skew(rust::Slice<float> m9, float kx, float ky);
void pk_matrix_set_skew_pivot(rust::Slice<float> m9, float kx, float ky, float px, float py);
void pk_matrix_set_scale_translate(rust::Slice<float> m9, float sx, float sy, float tx, float ty);
void pk_matrix_set_concat(rust::Slice<float> out9, rust::Slice<const float> a9, rust::Slice<const float> b9);
void pk_matrix_pre_translate(rust::Slice<float> m9, float dx, float dy);
void pk_matrix_pre_scale(rust::Slice<float> m9, float sx, float sy);
void pk_matrix_pre_scale_pivot(rust::Slice<float> m9, float sx, float sy, float px, float py);
void pk_matrix_pre_rotate(rust::Slice<float> m9, float degrees);
void pk_matrix_pre_rotate_pivot(rust::Slice<float> m9, float degrees, float px, float py);
void pk_matrix_pre_skew(rust::Slice<float> m9, float kx, float ky);
void pk_matrix_pre_skew_pivot(rust::Slice<float> m9, float kx, float ky, float px, float py);
void pk_matrix_pre_concat(rust::Slice<float> m9, rust::Slice<const float> o9);
void pk_matrix_post_translate(rust::Slice<float> m9, float dx, float dy);
void pk_matrix_post_scale(rust::Slice<float> m9, float sx, float sy);
void pk_matrix_post_scale_pivot(rust::Slice<float> m9, float sx, float sy, float px, float py);
void pk_matrix_post_rotate(rust::Slice<float> m9, float degrees);
void pk_matrix_post_rotate_pivot(rust::Slice<float> m9, float degrees, float px, float py);
void pk_matrix_post_skew(rust::Slice<float> m9, float kx, float ky);
void pk_matrix_post_skew_pivot(rust::Slice<float> m9, float kx, float ky, float px, float py);
void pk_matrix_post_concat(rust::Slice<float> m9, rust::Slice<const float> o9);
bool pk_matrix_set_rect_to_rect(rust::Slice<float> m9, const SkRect& src, const SkRect& dst,
                                std::int32_t scale_to_fit);

std::uint32_t pk_matrix_get_type(rust::Slice<const float> m9);
bool pk_matrix_is_identity(rust::Slice<const float> m9);
bool pk_matrix_is_scale_translate(rust::Slice<const float> m9);
bool pk_matrix_rect_stays_rect(rust::Slice<const float> m9);
bool pk_matrix_has_perspective(rust::Slice<const float> m9);
bool pk_matrix_is_finite_matrix(rust::Slice<const float> m9);
bool pk_matrix_invert(rust::Slice<const float> src9, rust::Slice<float> dst9);
void pk_matrix_map_xy(rust::Slice<const float> m9, float x, float y, SkPoint& out);
bool pk_matrix_map_rect(rust::Slice<const float> m9, const SkRect& src, SkRect& dst);
void pk_matrix_map_rect_scale_translate(rust::Slice<const float> m9, const SkRect& src, SkRect& dst);
void pk_matrix_map_origin(rust::Slice<const float> m9, SkPoint& out);
float pk_matrix_get_min_scale(rust::Slice<const float> m9);
float pk_matrix_get_max_scale(rust::Slice<const float> m9);
bool pk_matrix_get_min_max_scales(rust::Slice<const float> m9, float& min_s, float& max_s);
bool pk_matrix_equals(rust::Slice<const float> a9, rust::Slice<const float> b9);
std::size_t pk_matrix_write_to_memory(rust::Slice<const float> m9, rust::Slice<std::uint8_t> buf);
std::size_t pk_matrix_read_from_memory(rust::Slice<float> m9, rust::Slice<const std::uint8_t> buf);

std::int32_t pk_path_count_points(const pk::SkPath& p);
std::int32_t pk_path_count_verbs(const pk::SkPath& p);
void pk_path_get_point(const pk::SkPath& p, std::int32_t index, SkPoint& out);
void pk_path_compute_tight_bounds(const pk::SkPath& p, SkRect& out);
bool pk_path_is_last_contour_closed(const pk::SkPath& p);
bool pk_path_conservatively_contains_rect(const pk::SkPath& p, const SkRect& r);
bool pk_path_is_rect(const pk::SkPath& p, SkRect& rect, bool& is_closed, Direction& direction);
bool pk_path_contains(const pk::SkPath& p, float x, float y);
PathFillType pk_path_fill_type_bits(const pk::SkPath& p);
void pk_path_set_fill_type_bits(pk::SkPath& p, PathFillType v);
void pk_path_toggle_inverse_fill_type(pk::SkPath& p);

void pk_path_move_to(pk::SkPath& p, float x, float y);
void pk_path_line_to(pk::SkPath& p, float x, float y);
void pk_path_quad_to(pk::SkPath& p, float x1, float y1, float x2, float y2);
void pk_path_cubic_to(pk::SkPath& p, float x1, float y1, float x2, float y2, float x3, float y3);
void pk_path_conic_to(pk::SkPath& p, float x1, float y1, float x2, float y2, float w);
void pk_path_arc_to(pk::SkPath& p, float x1, float y1, float x2, float y2, float radius);
void pk_path_close(pk::SkPath& p);
void pk_path_add_poly(pk::SkPath& p, rust::Slice<const SkPoint> pts, bool close);

void pk_path_add_rect(pk::SkPath& p, const SkRect& rect, Direction dir, RectCorner start);
void pk_path_add_oval(pk::SkPath& p, const SkRect& rect, Direction dir);
void pk_path_add_oval_start(pk::SkPath& p, const SkRect& rect, Direction dir, RectCorner start);
void pk_path_add_circle(pk::SkPath& p, float cx, float cy, float radius, Direction dir);
void pk_path_add_round_rect(pk::SkPath& p, const SkRect& rect, float rx, float ry, Direction dir);
void pk_path_add_rrect(pk::SkPath& p, const SkRRect& rr, Direction dir);
void pk_path_add_rrect_start(pk::SkPath& p, const SkRRect& rr, Direction dir, RectCorner start);
bool pk_path_is_rrect(const pk::SkPath& p, SkRRect& out);
bool pk_path_equals(const pk::SkPath& a, const pk::SkPath& b);

std::unique_ptr<PathBuilderHolder> pk_path_builder_new();
void pk_path_builder_reset(PathBuilderHolder& h);
PathFillType pk_path_builder_fill_type(const PathBuilderHolder& h);
void pk_path_builder_set_fill_type(PathBuilderHolder& h, PathFillType ft);
void pk_path_builder_toggle_inverse_fill_type(PathBuilderHolder& h);
std::unique_ptr<pk::SkPath> pk_path_builder_snapshot(const PathBuilderHolder& h);
std::unique_ptr<pk::SkPath> pk_path_builder_detach(PathBuilderHolder& h);
void pk_path_builder_move_to(PathBuilderHolder& h, float x, float y);
void pk_path_builder_line_to(PathBuilderHolder& h, float x, float y);
void pk_path_builder_quad_to(PathBuilderHolder& h, float x1, float y1, float x2, float y2);
void pk_path_builder_cubic_to(PathBuilderHolder& h, float x1, float y1, float x2, float y2, float x3,
                              float y3);
void pk_path_builder_close(PathBuilderHolder& h);
void pk_path_builder_add_rect(PathBuilderHolder& h, const SkRect& rect, Direction dir,
                              RectCorner start);
void pk_path_builder_add_oval(PathBuilderHolder& h, const SkRect& rect, Direction dir);
void pk_path_builder_add_circle(PathBuilderHolder& h, float cx, float cy, float radius,
                                Direction dir);
void pk_path_builder_add_round_rect(PathBuilderHolder& h, const SkRect& rect, float rx, float ry,
                                    Direction dir);
void pk_path_builder_add_rrect(PathBuilderHolder& h, const SkRRect& rr, Direction dir);
void pk_path_builder_add_rrect_start(PathBuilderHolder& h, const SkRRect& rr, Direction dir,
                                     RectCorner start);
void pk_path_builder_add_path(PathBuilderHolder& h, const pk::SkPath& src);

std::unique_ptr<PathIterInner> pk_path_iter_new(const pk::SkPath& path, bool force_close);
PathVerb pk_path_iter_next(PathIterInner& it, SkPoint& p0, SkPoint& p1, SkPoint& p2, SkPoint& p3);

std::unique_ptr<PathMeasureHolder> pk_measure_new();
std::unique_ptr<PathMeasureHolder> pk_measure_from_path(const pk::SkPath& path, bool force_closed,
                                                        float res_scale);
void pk_measure_set_path(PathMeasureHolder& m, const pk::SkPath& path, bool force_closed);
float pk_measure_length(PathMeasureHolder& m);
bool pk_measure_get_pos_tan(PathMeasureHolder& m, float distance, SkPoint& position, SkPoint& tangent);
bool pk_measure_get_segment(PathMeasureHolder& m, float start_d, float stop_d, pk::SkPath& dst,
                            bool start_with_move_to);
bool pk_measure_is_closed(PathMeasureHolder& m);
bool pk_measure_next_contour(PathMeasureHolder& m);

std::unique_ptr<PaintHolder> pk_paint_new();
std::unique_ptr<PaintHolder> pk_paint_clone(const PaintHolder& ph);
void pk_paint_set_fill(PaintHolder& ph);
void pk_paint_set_stroke(PaintHolder& ph, bool enable);
void pk_paint_set_style(PaintHolder& ph, PaintStyle style);
void pk_paint_set_stroke_width(PaintHolder& ph, float width);
void pk_paint_set_stroke_miter(PaintHolder& ph, float miter);
void pk_paint_set_stroke_cap(PaintHolder& ph, StrokeCap cap);
void pk_paint_set_stroke_join(PaintHolder& ph, StrokeJoin join);
bool pk_paint_get_fill_path(const PaintHolder& ph, const pk::SkPath& src, pk::SkPath& dst);

std::unique_ptr<StrokeRecHolder> pk_stroke_rec_new(StrokeRecInit init);
void pk_stroke_rec_set_fill(StrokeRecHolder& h);
void pk_stroke_rec_set_hairline(StrokeRecHolder& h);
void pk_stroke_rec_set_stroke_style(StrokeRecHolder& h, float width, bool saf);
StrokeRecStyleTag pk_stroke_rec_get_style(const StrokeRecHolder& h);
float pk_stroke_rec_width(const StrokeRecHolder& h);
StrokeCap pk_stroke_rec_cap(const StrokeRecHolder& h);
void pk_stroke_rec_set_cap(StrokeRecHolder& h, StrokeCap cap);
StrokeJoin pk_stroke_rec_join(const StrokeRecHolder& h);
void pk_stroke_rec_set_join(StrokeRecHolder& h, StrokeJoin join);
float pk_stroke_rec_miter_limit(const StrokeRecHolder& h);
void pk_stroke_rec_set_stroke_params(StrokeRecHolder& h, StrokeCap cap, StrokeJoin join,
                                     float miter_limit);
float pk_stroke_rec_inflation_radius(const StrokeRecHolder& h);
bool pk_stroke_rec_apply_to_path(const StrokeRecHolder& h, pk::SkPath& dst, const pk::SkPath& src);

bool pk_op(const pk::SkPath& one, const pk::SkPath& two, PathOp op, pk::SkPath& result);

std::unique_ptr<OpBuilderHolder> pk_op_builder_new();
void pk_op_builder_add(OpBuilderHolder& h, const pk::SkPath& path, PathOp op);
bool pk_op_builder_resolve(OpBuilderHolder& h, pk::SkPath& result);

bool pk_simplify(const pk::SkPath& path, pk::SkPath& result);
bool pk_tight_bounds(const pk::SkPath& path, SkRect& out);

void pk_rrect_new_empty(SkRRect& out);
void pk_rrect_set_rect_xy(SkRRect& rr, const SkRect& rect, float rx, float ry);
void pk_rrect_set_oval(SkRRect& rr, const SkRect& rect);
void pk_rrect_set_rect_radii(SkRRect& rr, const SkRect& rect, rust::Slice<const SkPoint> radii);
bool pk_rrect_is_valid(const SkRRect& rr);

std::unique_ptr<PathEffectHolder> pk_corner_effect_make(float radius);
std::unique_ptr<PathEffectHolder> pk_dash_effect_make(rust::Slice<const float> intervals, float phase);
bool pk_path_effect_filter(const PathEffectHolder& h, pk::SkPath& dst, const pk::SkPath& src,
                           StrokeRecHolder& rec, const SkRect& cull);
