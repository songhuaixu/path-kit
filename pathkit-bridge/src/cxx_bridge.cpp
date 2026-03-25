#include "path-kit/src/bridge.rs.h"

#include "include/pathkit_cxx_decl.h"

#include <cstdint>
#include <cstring>
#include <functional>
#include <memory>
#include <utility>
#include <vector>

#include "pathkit.h"
#include "rust/cxx.h"

namespace {

pk::SkRect to_native_rect(const SkRect& r) {
    pk::SkRect o;
    o.fLeft = r.fLeft;
    o.fTop = r.fTop;
    o.fRight = r.fRight;
    o.fBottom = r.fBottom;
    return o;
}

SkRect from_native_rect(const pk::SkRect& r) {
    SkRect o;
    o.fLeft = r.fLeft;
    o.fTop = r.fTop;
    o.fRight = r.fRight;
    o.fBottom = r.fBottom;
    return o;
}

pk::SkPoint to_native_point(const SkPoint& p) {
    pk::SkPoint o;
    o.fX = p.fX;
    o.fY = p.fY;
    return o;
}

pk::SkRRect to_native_rrect(const SkRRect& rr) {
    pk::SkPoint pr[4];
    for (int i = 0; i < 4; ++i) {
        pr[i] = to_native_point(rr.fRadii[static_cast<size_t>(i)]);
    }
    pk::SkRRect n{};
    pk::SkRect r = to_native_rect(rr.fRect);
    n.setRectRadii(r, pr);
    return n;
}

SkRRect from_native_rrect(const pk::SkRRect& rr) {
    SkRRect o;
    o.fRect = from_native_rect(rr.rect());
    for (int i = 0; i < 4; ++i) {
        pk::SkVector rv = rr.radii(static_cast<pk::SkRRect::Corner>(i));
        o.fRadii[static_cast<size_t>(i)] = SkPoint{rv.fX, rv.fY};
    }
    o.fType = static_cast<RRectType>(static_cast<std::int32_t>(rr.getType()));
    return o;
}

}  // namespace

std::unique_ptr<pk::SkPath> pk_path_new() {
    return std::make_unique<pk::SkPath>();
}

std::unique_ptr<pk::SkPath> pk_path_clone(const pk::SkPath& p) {
    return std::make_unique<pk::SkPath>(p);
}

void pk_path_reset(pk::SkPath& p) {
    p.reset();
}

void pk_path_rewind(pk::SkPath& p) {
    p.rewind();
}

void pk_path_get_bounds(const pk::SkPath& p, SkRect& out) {
    out = from_native_rect(p.getBounds());
}

bool pk_path_is_finite(const pk::SkPath& p) {
    return p.isFinite();
}

bool pk_path_is_convex(const pk::SkPath& p) {
    return p.isConvex();
}

bool pk_path_is_oval(const pk::SkPath& p, SkRect& bounds) {
    pk::SkRect nb{};
    bool ok = p.isOval(&nb);
    if (ok) {
        bounds = from_native_rect(nb);
    }
    return ok;
}

bool pk_path_is_line(const pk::SkPath& p, SkPoint& p0, SkPoint& p1) {
    pk::SkPoint line[2]{};
    bool ok = p.isLine(line);
    if (ok) {
        p0.fX = line[0].fX;
        p0.fY = line[0].fY;
        p1.fX = line[1].fX;
        p1.fY = line[1].fY;
    }
    return ok;
}

void pk_path_get_points_copy(const pk::SkPath& p, rust::Vec<SkPoint>& out) {
    int n = p.countPoints();
    out.clear();
    if (n <= 0) {
        return;
    }
    std::vector<pk::SkPoint> tmp(static_cast<size_t>(n));
    int got = p.getPoints(tmp.data(), n);
    out.reserve(static_cast<size_t>(got));
    for (int i = 0; i < got; ++i) {
        const auto& q = tmp[static_cast<size_t>(i)];
        out.push_back(SkPoint{q.fX, q.fY});
    }
}

void pk_path_get_verbs_copy(const pk::SkPath& p, rust::Vec<std::uint8_t>& out) {
    int n = p.countVerbs();
    out.clear();
    if (n <= 0) {
        return;
    }
    std::vector<std::uint8_t> tmp(static_cast<size_t>(n));
    int got = p.getVerbs(tmp.data(), n);
    out.reserve(static_cast<size_t>(got));
    for (int i = 0; i < got; ++i) {
        out.push_back(tmp[static_cast<size_t>(i)]);
    }
}

void pk_path_inc_reserve(pk::SkPath& p, int32_t extra_pt_count) {
    if (extra_pt_count > 0) {
        p.incReserve(extra_pt_count);
    }
}

bool pk_path_is_interpolatable(const pk::SkPath& a, const pk::SkPath& b) {
    return a.isInterpolatable(b);
}

bool pk_path_interpolate(const pk::SkPath& start, const pk::SkPath& end, float weight, pk::SkPath& out) {
    return start.interpolate(end, weight, &out);
}

bool pk_path_get_last_pt(const pk::SkPath& p, SkPoint& out) {
    pk::SkPoint q{};
    bool ok = p.getLastPt(&q);
    if (ok) {
        out.fX = q.fX;
        out.fY = q.fY;
    }
    return ok;
}

void pk_path_set_last_pt(pk::SkPath& p, float x, float y) {
    p.setLastPt(x, y);
}

std::uint32_t pk_path_segment_masks(const pk::SkPath& p) {
    return p.getSegmentMasks();
}

bool pk_path_has_multiple_contours(const pk::SkPath& p) {
    return p.hasMultipleContours();
}

void pk_path_add_path_offset(pk::SkPath& dst, const pk::SkPath& src, float dx, float dy, bool extend) {
    dst.addPath(src, dx, dy,
                extend ? pk::SkPath::kExtend_AddPathMode : pk::SkPath::kAppend_AddPathMode);
}

void pk_path_reverse_add_path(pk::SkPath& dst, const pk::SkPath& src) {
    dst.reverseAddPath(src);
}

void pk_path_transform(pk::SkPath& p, rust::Slice<const float> mat9) {
    if (mat9.size() != 9) {
        return;
    }
    pk::SkMatrix m;
    m.set9(mat9.data());
    p.transform(m);
}

void pk_path_transform_to(const pk::SkPath& src, rust::Slice<const float> mat9, pk::SkPath& dst) {
    if (mat9.size() != 9) {
        return;
    }
    pk::SkMatrix m;
    m.set9(mat9.data());
    src.transform(m, &dst);
}

namespace {

void matrix_rw(rust::Slice<float> m9, const std::function<void(pk::SkMatrix&)>& fn) {
    if (m9.size() != 9) {
        return;
    }
    pk::SkMatrix m;
    m.set9(m9.data());
    fn(m);
    m.get9(m9.data());
}

pk::SkMatrix matrix_ro(rust::Slice<const float> m9) {
    pk::SkMatrix m;
    if (m9.size() == 9) {
        m.set9(m9.data());
    }
    return m;
}

}  // namespace

void pk_matrix_reset(rust::Slice<float> m9) {
    matrix_rw(m9, [](pk::SkMatrix& m) { m.reset(); });
}

void pk_matrix_set_all(rust::Slice<float> m9, float v0, float v1, float v2, float v3, float v4, float v5,
                       float v6, float v7, float v8) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setAll(v0, v1, v2, v3, v4, v5, v6, v7, v8); });
}

void pk_matrix_set_translate(rust::Slice<float> m9, float dx, float dy) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setTranslate(dx, dy); });
}

void pk_matrix_set_scale(rust::Slice<float> m9, float sx, float sy) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setScale(sx, sy); });
}

void pk_matrix_set_scale_pivot(rust::Slice<float> m9, float sx, float sy, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setScale(sx, sy, px, py); });
}

void pk_matrix_set_rotate(rust::Slice<float> m9, float degrees) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setRotate(degrees); });
}

void pk_matrix_set_rotate_pivot(rust::Slice<float> m9, float degrees, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setRotate(degrees, px, py); });
}

void pk_matrix_set_sin_cos(rust::Slice<float> m9, float sin_v, float cos_v) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setSinCos(sin_v, cos_v); });
}

void pk_matrix_set_sin_cos_pivot(rust::Slice<float> m9, float sin_v, float cos_v, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setSinCos(sin_v, cos_v, px, py); });
}

void pk_matrix_set_skew(rust::Slice<float> m9, float kx, float ky) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setSkew(kx, ky); });
}

void pk_matrix_set_skew_pivot(rust::Slice<float> m9, float kx, float ky, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setSkew(kx, ky, px, py); });
}

void pk_matrix_set_scale_translate(rust::Slice<float> m9, float sx, float sy, float tx, float ty) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.setScaleTranslate(sx, sy, tx, ty); });
}

void pk_matrix_set_concat(rust::Slice<float> out9, rust::Slice<const float> a9, rust::Slice<const float> b9) {
    if (out9.size() != 9 || a9.size() != 9 || b9.size() != 9) {
        return;
    }
    pk::SkMatrix a;
    pk::SkMatrix b;
    a.set9(a9.data());
    b.set9(b9.data());
    pk::SkMatrix r;
    r.setConcat(a, b);
    r.get9(out9.data());
}

void pk_matrix_pre_translate(rust::Slice<float> m9, float dx, float dy) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preTranslate(dx, dy); });
}

void pk_matrix_pre_scale(rust::Slice<float> m9, float sx, float sy) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preScale(sx, sy); });
}

void pk_matrix_pre_scale_pivot(rust::Slice<float> m9, float sx, float sy, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preScale(sx, sy, px, py); });
}

void pk_matrix_pre_rotate(rust::Slice<float> m9, float degrees) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preRotate(degrees); });
}

void pk_matrix_pre_rotate_pivot(rust::Slice<float> m9, float degrees, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preRotate(degrees, px, py); });
}

void pk_matrix_pre_skew(rust::Slice<float> m9, float kx, float ky) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preSkew(kx, ky); });
}

void pk_matrix_pre_skew_pivot(rust::Slice<float> m9, float kx, float ky, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.preSkew(kx, ky, px, py); });
}

void pk_matrix_pre_concat(rust::Slice<float> m9, rust::Slice<const float> o9) {
    if (m9.size() != 9 || o9.size() != 9) {
        return;
    }
    pk::SkMatrix o;
    o.set9(o9.data());
    matrix_rw(m9, [&o](pk::SkMatrix& m) { m.preConcat(o); });
}

void pk_matrix_post_translate(rust::Slice<float> m9, float dx, float dy) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postTranslate(dx, dy); });
}

void pk_matrix_post_scale(rust::Slice<float> m9, float sx, float sy) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postScale(sx, sy); });
}

void pk_matrix_post_scale_pivot(rust::Slice<float> m9, float sx, float sy, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postScale(sx, sy, px, py); });
}

void pk_matrix_post_rotate(rust::Slice<float> m9, float degrees) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postRotate(degrees); });
}

void pk_matrix_post_rotate_pivot(rust::Slice<float> m9, float degrees, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postRotate(degrees, px, py); });
}

void pk_matrix_post_skew(rust::Slice<float> m9, float kx, float ky) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postSkew(kx, ky); });
}

void pk_matrix_post_skew_pivot(rust::Slice<float> m9, float kx, float ky, float px, float py) {
    matrix_rw(m9, [=](pk::SkMatrix& m) { m.postSkew(kx, ky, px, py); });
}

void pk_matrix_post_concat(rust::Slice<float> m9, rust::Slice<const float> o9) {
    if (m9.size() != 9 || o9.size() != 9) {
        return;
    }
    pk::SkMatrix o;
    o.set9(o9.data());
    matrix_rw(m9, [&o](pk::SkMatrix& m) { m.postConcat(o); });
}

bool pk_matrix_set_rect_to_rect(rust::Slice<float> m9, const SkRect& src, const SkRect& dst,
                                std::int32_t scale_to_fit) {
    if (m9.size() != 9) {
        return false;
    }
    pk::SkMatrix m;
    bool ok = m.setRectToRect(to_native_rect(src), to_native_rect(dst),
                             static_cast<pk::SkMatrix::ScaleToFit>(scale_to_fit));
    m.get9(m9.data());
    return ok;
}

std::uint32_t pk_matrix_get_type(rust::Slice<const float> m9) {
    return static_cast<std::uint32_t>(matrix_ro(m9).getType());
}

bool pk_matrix_is_identity(rust::Slice<const float> m9) {
    return matrix_ro(m9).isIdentity();
}

bool pk_matrix_is_scale_translate(rust::Slice<const float> m9) {
    return matrix_ro(m9).isScaleTranslate();
}

bool pk_matrix_rect_stays_rect(rust::Slice<const float> m9) {
    return matrix_ro(m9).rectStaysRect();
}

bool pk_matrix_has_perspective(rust::Slice<const float> m9) {
    return matrix_ro(m9).hasPerspective();
}

bool pk_matrix_is_finite_matrix(rust::Slice<const float> m9) {
    return matrix_ro(m9).isFinite();
}

bool pk_matrix_invert(rust::Slice<const float> src9, rust::Slice<float> dst9) {
    if (src9.size() != 9 || dst9.size() != 9) {
        return false;
    }
    pk::SkMatrix m;
    m.set9(src9.data());
    pk::SkMatrix inv;
    if (!m.invert(&inv)) {
        return false;
    }
    inv.get9(dst9.data());
    return true;
}

void pk_matrix_map_xy(rust::Slice<const float> m9, float x, float y, SkPoint& out) {
    pk::SkPoint p = matrix_ro(m9).mapXY(x, y);
    out.fX = p.fX;
    out.fY = p.fY;
}

bool pk_matrix_map_rect(rust::Slice<const float> m9, const SkRect& src, SkRect& dst) {
    pk::SkMatrix m = matrix_ro(m9);
    pk::SkRect d{};
    bool axis = m.mapRect(&d, to_native_rect(src));
    dst = from_native_rect(d);
    return axis;
}

void pk_matrix_map_rect_scale_translate(rust::Slice<const float> m9, const SkRect& src, SkRect& dst) {
    pk::SkMatrix m = matrix_ro(m9);
    pk::SkRect d{};
    m.mapRectScaleTranslate(&d, to_native_rect(src));
    dst = from_native_rect(d);
}

void pk_matrix_map_origin(rust::Slice<const float> m9, SkPoint& out) {
    pk::SkPoint p = matrix_ro(m9).mapOrigin();
    out.fX = p.fX;
    out.fY = p.fY;
}

float pk_matrix_get_min_scale(rust::Slice<const float> m9) {
    return matrix_ro(m9).getMinScale();
}

float pk_matrix_get_max_scale(rust::Slice<const float> m9) {
    return matrix_ro(m9).getMaxScale();
}

bool pk_matrix_get_min_max_scales(rust::Slice<const float> m9, float& min_s, float& max_s) {
    pk::SkScalar s[2]{};
    if (!matrix_ro(m9).getMinMaxScales(s)) {
        return false;
    }
    min_s = s[0];
    max_s = s[1];
    return true;
}

bool pk_matrix_equals(rust::Slice<const float> a9, rust::Slice<const float> b9) {
    if (a9.size() != 9 || b9.size() != 9) {
        return false;
    }
    pk::SkMatrix a;
    pk::SkMatrix b;
    a.set9(a9.data());
    b.set9(b9.data());
    return a == b;
}

std::size_t pk_matrix_write_to_memory(rust::Slice<const float> m9, rust::Slice<std::uint8_t> buf) {
    if (m9.size() != 9) {
        return 0;
    }
    constexpr std::size_t n = 9 * sizeof(float);
    if (buf.size() < n) {
        return 0;
    }
    std::memcpy(buf.data(), m9.data(), n);
    return n;
}

std::size_t pk_matrix_read_from_memory(rust::Slice<float> m9, rust::Slice<const std::uint8_t> buf) {
    if (m9.size() != 9) {
        return 0;
    }
    constexpr std::size_t n = 9 * sizeof(float);
    if (buf.size() < n) {
        return 0;
    }
    std::memcpy(m9.data(), buf.data(), n);
    return n;
}

int32_t pk_path_count_points(const pk::SkPath& p) {
    return p.countPoints();
}

int32_t pk_path_count_verbs(const pk::SkPath& p) {
    return p.countVerbs();
}

void pk_path_get_point(const pk::SkPath& p, int32_t index, SkPoint& out) {
    pk::SkPoint q = p.getPoint(index);
    out.fX = q.fX;
    out.fY = q.fY;
}

void pk_path_compute_tight_bounds(const pk::SkPath& p, SkRect& out) {
    out = from_native_rect(p.computeTightBounds());
}

bool pk_path_is_last_contour_closed(const pk::SkPath& p) {
    return p.isLastContourClosed();
}

bool pk_path_conservatively_contains_rect(const pk::SkPath& p, const SkRect& r) {
    pk::SkRect nr = to_native_rect(r);
    return p.conservativelyContainsRect(nr);
}

bool pk_path_is_rect(const pk::SkPath& p, SkRect& rect, bool& is_closed, Direction& direction) {
    pk::SkRect nr{};
    pk::SkPathDirection dir{};
    bool ok = p.isRect(&nr, &is_closed, &dir);
    rect = from_native_rect(nr);
    direction = static_cast<Direction>(static_cast<std::uint32_t>(dir));
    return ok;
}

bool pk_path_contains(const pk::SkPath& p, float x, float y) {
    return p.contains(x, y);
}

PathFillType pk_path_fill_type_bits(const pk::SkPath& p) {
    return static_cast<PathFillType>(static_cast<std::uint8_t>(p.getFillType()) & 3);
}

void pk_path_set_fill_type_bits(pk::SkPath& p, PathFillType v) {
    p.setFillType(static_cast<pk::SkPathFillType>(static_cast<std::uint8_t>(v) & 3));
}

void pk_path_toggle_inverse_fill_type(pk::SkPath& p) {
    p.toggleInverseFillType();
}

void pk_path_move_to(pk::SkPath& p, float x, float y) {
    p.moveTo(x, y);
}

void pk_path_line_to(pk::SkPath& p, float x, float y) {
    p.lineTo(x, y);
}

void pk_path_quad_to(pk::SkPath& p, float x1, float y1, float x2, float y2) {
    p.quadTo(x1, y1, x2, y2);
}

void pk_path_cubic_to(pk::SkPath& p, float x1, float y1, float x2, float y2, float x3,
                      float y3) {
    p.cubicTo(x1, y1, x2, y2, x3, y3);
}

void pk_path_conic_to(pk::SkPath& p, float x1, float y1, float x2, float y2, float w) {
    p.conicTo(x1, y1, x2, y2, w);
}

void pk_path_arc_to(pk::SkPath& p, float x1, float y1, float x2, float y2, float radius) {
    p.arcTo(x1, y1, x2, y2, radius);
}

void pk_path_close(pk::SkPath& p) {
    p.close();
}

void pk_path_add_poly(pk::SkPath& p, rust::Slice<const SkPoint> pts, bool close) {
    const int count = static_cast<int>(pts.size());
    if (count <= 0) {
        p.addPoly(nullptr, 0, close);
        return;
    }
    p.addPoly(reinterpret_cast<const pk::SkPoint*>(pts.data()), count, close);
}

void pk_path_add_rect(pk::SkPath& p, const SkRect& rect, Direction dir, RectCorner start) {
    pk::SkRect r = to_native_rect(rect);
    p.addRect(r, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)),
              static_cast<std::uint32_t>(start));
}

void pk_path_add_oval(pk::SkPath& p, const SkRect& rect, Direction dir) {
    p.addOval(to_native_rect(rect), static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_add_oval_start(pk::SkPath& p, const SkRect& rect, Direction dir, RectCorner start) {
    p.addOval(to_native_rect(rect), static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)),
              static_cast<std::uint32_t>(start));
}

void pk_path_add_circle(pk::SkPath& p, float cx, float cy, float radius, Direction dir) {
    p.addCircle(cx, cy, radius,
                static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_add_round_rect(pk::SkPath& p, const SkRect& rect, float rx, float ry, Direction dir) {
    p.addRoundRect(to_native_rect(rect), rx, ry,
                   static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_add_rrect(pk::SkPath& p, const SkRRect& rr, Direction dir) {
    pk::SkRRect n = to_native_rrect(rr);
    p.addRRect(n, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_add_rrect_start(pk::SkPath& p, const SkRRect& rr, Direction dir, RectCorner start) {
    pk::SkRRect n = to_native_rrect(rr);
    p.addRRect(n, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)),
               static_cast<std::uint32_t>(start));
}

bool pk_path_is_rrect(const pk::SkPath& p, SkRRect& out) {
    pk::SkRRect nr{};
    bool ok = p.isRRect(&nr);
    out = from_native_rrect(nr);
    return ok;
}

bool pk_path_equals(const pk::SkPath& a, const pk::SkPath& b) {
    return a == b;
}

std::unique_ptr<PathBuilderHolder> pk_path_builder_new() {
    return std::make_unique<PathBuilderHolder>();
}

void pk_path_builder_reset(PathBuilderHolder& h) {
    h.b.reset();
}

PathFillType pk_path_builder_fill_type(const PathBuilderHolder& h) {
    return static_cast<PathFillType>(static_cast<std::uint8_t>(h.b.fillType()));
}

void pk_path_builder_set_fill_type(PathBuilderHolder& h, PathFillType ft) {
    h.b.setFillType(static_cast<pk::SkPathFillType>(static_cast<std::uint8_t>(ft)));
}

void pk_path_builder_toggle_inverse_fill_type(PathBuilderHolder& h) {
    h.b.toggleInverseFillType();
}

std::unique_ptr<pk::SkPath> pk_path_builder_snapshot(const PathBuilderHolder& h) {
    return std::make_unique<pk::SkPath>(h.b.snapshot());
}

std::unique_ptr<pk::SkPath> pk_path_builder_detach(PathBuilderHolder& h) {
    return std::make_unique<pk::SkPath>(h.b.detach());
}

void pk_path_builder_move_to(PathBuilderHolder& h, float x, float y) {
    h.b.moveTo(x, y);
}

void pk_path_builder_line_to(PathBuilderHolder& h, float x, float y) {
    h.b.lineTo(x, y);
}

void pk_path_builder_quad_to(PathBuilderHolder& h, float x1, float y1, float x2, float y2) {
    h.b.quadTo(x1, y1, x2, y2);
}

void pk_path_builder_cubic_to(PathBuilderHolder& h, float x1, float y1, float x2, float y2, float x3,
                               float y3) {
    h.b.cubicTo(x1, y1, x2, y2, x3, y3);
}

void pk_path_builder_close(PathBuilderHolder& h) {
    h.b.close();
}

void pk_path_builder_add_rect(PathBuilderHolder& h, const SkRect& rect, Direction dir,
                              RectCorner start) {
    pk::SkRect r = to_native_rect(rect);
    h.b.addRect(r, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)),
                static_cast<std::uint32_t>(start));
}

void pk_path_builder_add_oval(PathBuilderHolder& h, const SkRect& rect, Direction dir) {
    h.b.addOval(to_native_rect(rect),
                static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_builder_add_circle(PathBuilderHolder& h, float cx, float cy, float radius,
                                Direction dir) {
    h.b.addCircle(cx, cy, radius,
                  static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_builder_add_round_rect(PathBuilderHolder& h, const SkRect& rect, float rx, float ry,
                                    Direction dir) {
    pk::SkRRect rr;
    rr.setRectXY(to_native_rect(rect), rx, ry);
    h.b.addRRect(rr, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_builder_add_rrect(PathBuilderHolder& h, const SkRRect& rr, Direction dir) {
    pk::SkRRect n = to_native_rrect(rr);
    h.b.addRRect(n, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)));
}

void pk_path_builder_add_rrect_start(PathBuilderHolder& h, const SkRRect& rr, Direction dir,
                                     RectCorner start) {
    pk::SkRRect n = to_native_rrect(rr);
    h.b.addRRect(n, static_cast<pk::SkPathDirection>(static_cast<std::uint32_t>(dir)),
                 static_cast<std::uint32_t>(start));
}

void pk_path_builder_add_path(PathBuilderHolder& h, const pk::SkPath& src) {
    h.b.addPath(src);
}

std::unique_ptr<PathIterInner> pk_path_iter_new(const pk::SkPath& path, bool force_close) {
    return std::make_unique<PathIterInner>(path, force_close);
}

PathVerb pk_path_iter_next(PathIterInner& it, SkPoint& p0, SkPoint& p1, SkPoint& p2, SkPoint& p3) {
    pk::SkPoint pts[4]{};
    auto verb = it.iter.next(pts);
    p0 = SkPoint{pts[0].fX, pts[0].fY};
    p1 = SkPoint{pts[1].fX, pts[1].fY};
    p2 = SkPoint{pts[2].fX, pts[2].fY};
    p3 = SkPoint{pts[3].fX, pts[3].fY};
    return static_cast<PathVerb>(static_cast<std::uint32_t>(verb));
}

std::unique_ptr<PathMeasureHolder> pk_measure_new() {
    return std::make_unique<PathMeasureHolder>();
}

std::unique_ptr<PathMeasureHolder> pk_measure_from_path(const pk::SkPath& path, bool force_closed,
                                                        float res_scale) {
    auto h = std::make_unique<PathMeasureHolder>();
    h->m = std::make_unique<pk::SkPathMeasure>(path, force_closed, res_scale);
    return h;
}

void pk_measure_set_path(PathMeasureHolder& m, const pk::SkPath& path, bool force_closed) {
    m.m->setPath(&path, force_closed);
}

float pk_measure_length(PathMeasureHolder& m) {
    return m.m->getLength();
}

bool pk_measure_get_pos_tan(PathMeasureHolder& m, float distance, SkPoint& position,
                            SkPoint& tangent) {
    pk::SkPoint pos{}, tan{};
    bool ok = m.m->getPosTan(distance, &pos, &tan);
    position = SkPoint{pos.fX, pos.fY};
    tangent = SkPoint{tan.fX, tan.fY};
    return ok;
}

bool pk_measure_get_segment(PathMeasureHolder& m, float start_d, float stop_d, pk::SkPath& dst,
                            bool start_with_move_to) {
    return m.m->getSegment(start_d, stop_d, &dst, start_with_move_to);
}

bool pk_measure_is_closed(PathMeasureHolder& m) {
    return m.m->isClosed();
}

bool pk_measure_next_contour(PathMeasureHolder& m) {
    return m.m->nextContour();
}

std::unique_ptr<PaintHolder> pk_paint_new() {
    return std::make_unique<PaintHolder>();
}

std::unique_ptr<PaintHolder> pk_paint_clone(const PaintHolder& ph) {
    auto np = std::make_unique<PaintHolder>();
    np->p = ph.p;
    return np;
}

void pk_paint_set_fill(PaintHolder& ph) {
    ph.p.setStyle(pk::SkPaint::kFill_Style);
}

void pk_paint_set_stroke(PaintHolder& ph, bool enable) {
    ph.p.setStroke(enable);
}

void pk_paint_set_style(PaintHolder& ph, PaintStyle style) {
    ph.p.setStyle(static_cast<pk::SkPaint::Style>(static_cast<std::uint8_t>(style)));
}

void pk_paint_set_stroke_width(PaintHolder& ph, float width) {
    ph.p.setStrokeWidth(width);
}

void pk_paint_set_stroke_miter(PaintHolder& ph, float miter) {
    ph.p.setStrokeMiter(miter);
}

void pk_paint_set_stroke_cap(PaintHolder& ph, StrokeCap cap) {
    ph.p.setStrokeCap(static_cast<pk::SkPaint::Cap>(static_cast<std::uint32_t>(cap)));
}

void pk_paint_set_stroke_join(PaintHolder& ph, StrokeJoin join) {
    ph.p.setStrokeJoin(static_cast<pk::SkPaint::Join>(static_cast<std::uint8_t>(join)));
}

bool pk_paint_get_fill_path(const PaintHolder& ph, const pk::SkPath& src, pk::SkPath& dst) {
    return ph.p.getFillPath(src, &dst, nullptr, 1.0f);
}

std::unique_ptr<StrokeRecHolder> pk_stroke_rec_new(StrokeRecInit init) {
    auto style = (init == StrokeRecInit::Hairline) ? pk::SkStrokeRec::kHairline_InitStyle
                                                   : pk::SkStrokeRec::kFill_InitStyle;
    return std::make_unique<StrokeRecHolder>(style);
}

void pk_stroke_rec_set_fill(StrokeRecHolder& h) {
    h.rec.setFillStyle();
}

void pk_stroke_rec_set_hairline(StrokeRecHolder& h) {
    h.rec.setHairlineStyle();
}

void pk_stroke_rec_set_stroke_style(StrokeRecHolder& h, float width, bool saf) {
    h.rec.setStrokeStyle(width, saf);
}

StrokeRecStyleTag pk_stroke_rec_get_style(const StrokeRecHolder& h) {
    return static_cast<StrokeRecStyleTag>(static_cast<std::uint32_t>(h.rec.getStyle()));
}

float pk_stroke_rec_width(const StrokeRecHolder& h) {
    return h.rec.getWidth();
}

StrokeCap pk_stroke_rec_cap(const StrokeRecHolder& h) {
    return static_cast<StrokeCap>(static_cast<std::uint32_t>(h.rec.getCap()));
}

void pk_stroke_rec_set_cap(StrokeRecHolder& h, StrokeCap cap) {
    h.rec.setStrokeParams(static_cast<pk::SkPaint::Cap>(static_cast<std::uint32_t>(cap)),
                          h.rec.getJoin(), h.rec.getMiter());
}

StrokeJoin pk_stroke_rec_join(const StrokeRecHolder& h) {
    return static_cast<StrokeJoin>(static_cast<std::uint8_t>(h.rec.getJoin()));
}

void pk_stroke_rec_set_join(StrokeRecHolder& h, StrokeJoin join) {
    h.rec.setStrokeParams(h.rec.getCap(),
                          static_cast<pk::SkPaint::Join>(static_cast<std::uint8_t>(join)),
                          h.rec.getMiter());
}

float pk_stroke_rec_miter_limit(const StrokeRecHolder& h) {
    return h.rec.getMiter();
}

void pk_stroke_rec_set_stroke_params(StrokeRecHolder& h, StrokeCap cap, StrokeJoin join,
                                     float miter_limit) {
    h.rec.setStrokeParams(static_cast<pk::SkPaint::Cap>(static_cast<std::uint32_t>(cap)),
                          static_cast<pk::SkPaint::Join>(static_cast<std::uint8_t>(join)),
                          miter_limit);
}

float pk_stroke_rec_inflation_radius(const StrokeRecHolder& h) {
    return h.rec.getInflationRadius();
}

bool pk_stroke_rec_apply_to_path(const StrokeRecHolder& h, pk::SkPath& dst, const pk::SkPath& src) {
    return h.rec.applyToPath(&dst, src);
}

bool pk_op(const pk::SkPath& one, const pk::SkPath& two, PathOp op, pk::SkPath& result) {
    return pk::Op(one, two, static_cast<pk::SkPathOp>(static_cast<std::uint32_t>(op)), &result);
}

std::unique_ptr<OpBuilderHolder> pk_op_builder_new() {
    return std::make_unique<OpBuilderHolder>();
}

void pk_op_builder_add(OpBuilderHolder& h, const pk::SkPath& path, PathOp op) {
    h.b.add(path, static_cast<pk::SkPathOp>(static_cast<std::uint32_t>(op)));
}

bool pk_op_builder_resolve(OpBuilderHolder& h, pk::SkPath& result) {
    return h.b.resolve(&result);
}

bool pk_simplify(const pk::SkPath& path, pk::SkPath& result) {
    return pk::Simplify(path, &result);
}

bool pk_tight_bounds(const pk::SkPath& path, SkRect& out) {
    pk::SkRect nr{};
    bool ok = pk::TightBounds(path, &nr);
    out = from_native_rect(nr);
    return ok;
}

void pk_rrect_new_empty(SkRRect& out) {
    pk::SkRRect n{};
    out = from_native_rrect(n);
}

void pk_rrect_set_rect_xy(SkRRect& rr, const SkRect& rect, float rx, float ry) {
    pk::SkRRect n = to_native_rrect(rr);
    pk::SkRect nr = to_native_rect(rect);
    n.setRectXY(nr, rx, ry);
    rr = from_native_rrect(n);
}

void pk_rrect_set_oval(SkRRect& rr, const SkRect& rect) {
    pk::SkRRect n = to_native_rrect(rr);
    pk::SkRect nr = to_native_rect(rect);
    n.setOval(nr);
    rr = from_native_rrect(n);
}

void pk_rrect_set_rect_radii(SkRRect& rr, const SkRect& rect, rust::Slice<const SkPoint> radii) {
    if (radii.size() != 4) {
        return;
    }
    pk::SkPoint pr[4];
    for (size_t i = 0; i < 4; ++i) {
        pr[i] = to_native_point(radii[i]);
    }
    pk::SkRRect n = to_native_rrect(rr);
    pk::SkRect rrect = to_native_rect(rect);
    n.setRectRadii(rrect, pr);
    rr = from_native_rrect(n);
}

bool pk_rrect_is_valid(const SkRRect& rr) {
    return to_native_rrect(rr).isValid();
}

std::unique_ptr<PathEffectHolder> pk_corner_effect_make(float radius) {
    if (radius <= 0.0f) {
        return nullptr;
    }
    auto h = std::make_unique<PathEffectHolder>();
    h->effect = pk::SkCornerPathEffect::Make(radius);
    if (!h->effect) {
        return nullptr;
    }
    return h;
}

std::unique_ptr<PathEffectHolder> pk_dash_effect_make(rust::Slice<const float> intervals,
                                                      float phase) {
    if (intervals.empty() || (intervals.size() % 2) != 0) {
        return nullptr;
    }
    auto h = std::make_unique<PathEffectHolder>();
    h->effect = pk::SkDashPathEffect::Make(intervals.data(), static_cast<int>(intervals.size()),
                                            phase);
    if (!h->effect) {
        return nullptr;
    }
    return h;
}

bool pk_path_effect_filter(const PathEffectHolder& h, pk::SkPath& dst, const pk::SkPath& src,
                           StrokeRecHolder& rec, const SkRect& cull) {
    if (!h.effect) {
        return false;
    }
    pk::SkRect c = to_native_rect(cull);
    return h.effect->filterPath(&dst, src, &rec.rec, &c);
}
