//! 单元测试。Unit tests.

use super::*;

fn rect(l: f32, t: f32, r: f32, b: f32) -> Rect {
    Rect::new(l, t, r, b)
}

#[test]
fn path_fill_type_default_and_set() {
    let path = Path::new();
    assert_eq!(path.fill_type(), PathFillType::Winding);
    assert!(!path.is_inverse_fill_type());

    let mut p = Path::new();
    p.set_fill_type(PathFillType::EvenOdd);
    assert_eq!(p.fill_type(), PathFillType::EvenOdd);
    assert!(p.fill_type().is_even_odd());
    assert!(!p.is_inverse_fill_type());

    p.toggle_inverse_fill_type();
    assert_eq!(p.fill_type(), PathFillType::InverseEvenOdd);
    assert!(p.is_inverse_fill_type());
    p.toggle_inverse_fill_type();
    assert_eq!(p.fill_type(), PathFillType::EvenOdd);

    assert_eq!(PathFillType::InverseWinding.to_non_inverse(), PathFillType::Winding);
    assert_eq!(PathFillType::InverseEvenOdd.to_non_inverse(), PathFillType::EvenOdd);
}

#[test]
fn path_empty() {
    let path = Path::new();
    assert_eq!(path.count_points(), 0);
    assert_eq!(path.count_verbs(), 0);
    assert!(path.is_empty());
    let bounds = path.tight_bounds();
    assert_eq!(bounds.left, 0.0);
    assert_eq!(bounds.top, 0.0);
    assert_eq!(bounds.right, 0.0);
    assert_eq!(bounds.bottom, 0.0);
}

#[test]
fn path_create_and_bounds() {
    let mut path = Path::new();
    path.add_rect(&rect(10.0, 10.0, 110.0, 110.0), Direction::Cw, RectCorner::UpperLeft);
    let bounds = path.tight_bounds();
    assert_eq!(bounds.left, 10.0);
    assert_eq!(bounds.top, 10.0);
    assert_eq!(bounds.right, 110.0);
    assert_eq!(bounds.bottom, 110.0);
}

#[test]
fn path_reset() {
    let mut path = Path::new();
    path.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    assert!(path.count_points() > 0);
    path.reset();
    assert_eq!(path.count_points(), 0);
    assert_eq!(path.count_verbs(), 0);
}

#[test]
fn path_iter_move_line_close() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0)
        .line_to(100.0, 0.0)
        .line_to(100.0, 100.0)
        .close();
    let items: Vec<_> = path.iter(false).collect();
    assert!(items.len() >= 4); // Move, Line, Line, Close
    assert!(matches!(items[0], PathVerbItem::Move(p) if (p.x - 0.0).abs() < 0.001 && (p.y - 0.0).abs() < 0.001));
    if let PathVerbItem::Line(from, to) = &items[1] {
        assert!((from.x - 0.0).abs() < 0.001);
        assert!((to.x - 100.0).abs() < 0.001);
    }
    assert!(matches!(items.last(), Some(PathVerbItem::Close)));
}

#[test]
fn path_iter_empty() {
    let path = Path::new();
    let items: Vec<_> = path.iter(false).collect();
    assert!(items.is_empty());
}

#[test]
fn path_iter_rect() {
    let mut path = Path::new();
    path.add_rect(&rect(10.0, 20.0, 60.0, 70.0), Direction::Cw, RectCorner::UpperLeft);
    let items: Vec<_> = path.iter(false).collect();
    assert!(!items.is_empty());
    assert!(matches!(items[0], PathVerbItem::Move(_)));
}

#[test]
fn path_iter_quad() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).quad_to(50.0, 100.0, 100.0, 0.0);
    let items: Vec<_> = path.iter(false).collect();
    let quads: Vec<_> = items.iter().filter(|i| matches!(i, PathVerbItem::Quad(_, _))).collect();
    assert!(!quads.is_empty());
}

#[test]
fn path_iter_cubic() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).cubic_to(30.0, 50.0, 70.0, 50.0, 100.0, 0.0);
    let items: Vec<_> = path.iter(false).collect();
    let cubics: Vec<_> = items.iter().filter(|i| matches!(i, PathVerbItem::Cubic(_, _, _))).collect();
    assert!(!cubics.is_empty());
}

#[test]
fn path_iter_force_close() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(50.0, 0.0); // open contour
    let items: Vec<_> = path.iter(true).collect();
    assert!(items.iter().any(|i| matches!(i, PathVerbItem::Close)));
}

#[test]
fn path_move_to_line_to_close() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0)
        .line_to(100.0, 0.0)
        .line_to(100.0, 100.0)
        .line_to(0.0, 100.0)
        .close();
    assert!(path.count_points() >= 4);
    assert!(path.is_last_contour_closed());
    let bounds = path.tight_bounds();
    assert!((bounds.width() - 100.0).abs() < 0.001);
    assert!((bounds.height() - 100.0).abs() < 0.001);
}

#[test]
fn path_add_oval() {
    let mut path = Path::new();
    path.add_oval(&rect(0.0, 0.0, 100.0, 50.0), Direction::Cw);
    assert!(path.count_points() > 0);
    let bounds = path.tight_bounds();
    assert!((bounds.width() - 100.0).abs() < 0.001);
    assert!((bounds.height() - 50.0).abs() < 0.001);
}

#[test]
fn path_add_oval_ccw() {
    let mut path = Path::new();
    path.add_oval(&rect(0.0, 0.0, 50.0, 50.0), Direction::Ccw);
    assert!(path.count_points() > 0);
}

#[test]
fn path_add_circle() {
    let mut path = Path::new();
    path.add_circle(50.0, 50.0, 25.0, Direction::Cw);
    assert!(path.count_points() > 0);
    let bounds = path.tight_bounds();
    assert!((bounds.left - 25.0).abs() < 0.001);
    assert!((bounds.top - 25.0).abs() < 0.001);
    assert!((bounds.right - 75.0).abs() < 0.001);
    assert!((bounds.bottom - 75.0).abs() < 0.001);
}

#[test]
fn path_add_round_rect() {
    let mut path = Path::new();
    path.add_round_rect(&rect(0.0, 0.0, 100.0, 60.0), 10.0, 10.0, Direction::Cw);
    assert!(path.count_points() > 0);
    let bounds = path.tight_bounds();
    assert!((bounds.width() - 100.0).abs() < 0.001);
    assert!((bounds.height() - 60.0).abs() < 0.001);
}

#[test]
fn rrect_from_rect_xy() {
    let r = rect(0.0, 0.0, 100.0, 50.0);
    let rr = RRect::from_rect_xy(&r, 10.0, 10.0);
    assert!(!rr.is_empty());
    assert!(rr.is_simple());
    assert!((rr.width() - 100.0).abs() < 0.001);
    assert!((rr.height() - 50.0).abs() < 0.001);
}

#[test]
fn path_add_rrect() {
    let r = rect(10.0, 20.0, 80.0, 70.0);
    let rr = RRect::from_rect_xy(&r, 5.0, 5.0);
    let mut path = Path::new();
    path.add_rrect(&rr, Direction::Cw);
    assert!(path.count_points() > 0);
    let out = path.is_rrect().unwrap();
    assert!((out.rect().left - 10.0).abs() < 0.001);
}

#[test]
fn rrect_from_oval() {
    let r = rect(0.0, 0.0, 50.0, 50.0);
    let rr = RRect::from_oval(&r);
    assert!(rr.is_oval());
}

#[test]
fn rrect_radii() {
    let r = rect(0.0, 0.0, 100.0, 100.0);
    let radii = [
        Radii { x: 10.0, y: 10.0 },
        Radii { x: 15.0, y: 15.0 },
        Radii { x: 10.0, y: 15.0 },
        Radii { x: 5.0, y: 5.0 },
    ];
    let rr = RRect::from_rect_radii(&r, &radii);
    assert!((rr.radii(RectCorner::UpperLeft).x - 10.0).abs() < 0.001);
}

#[test]
fn rrect_empty() {
    let rr = RRect::new();
    assert!(rr.is_empty());
}

#[test]
fn rrect_from_rect() {
    let r = rect(5.0, 10.0, 50.0, 60.0);
    let rr = RRect::from_rect(&r);
    assert!(rr.is_rect());
    assert!((rr.rect().left - 5.0).abs() < 0.001);
    assert!((rr.rect().top - 10.0).abs() < 0.001);
}

#[test]
fn rrect_rect() {
    let r = rect(1.0, 2.0, 11.0, 22.0);
    let rr = RRect::from_rect_xy(&r, 2.0, 2.0);
    let bounds = rr.rect();
    assert!((bounds.left - 1.0).abs() < 0.001);
    assert!((bounds.right - 11.0).abs() < 0.001);
}

#[test]
fn rrect_clone() {
    let r = rect(0.0, 0.0, 30.0, 40.0);
    let rr = RRect::from_rect_xy(&r, 5.0, 5.0);
    let rr2 = rr.clone();
    assert!((rr2.width() - rr.width()).abs() < 0.001);
}

#[test]
fn rrect_is_valid() {
    let rr = RRect::from_rect_xy(&rect(0.0, 0.0, 10.0, 10.0), 2.0, 2.0);
    assert!(rr.is_valid());
}

#[test]
fn path_is_rrect_none_for_line() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(50.0, 50.0);
    assert!(path.is_rrect().is_none());
}

#[test]
fn path_add_rrect_with_start() {
    let rr = RRect::from_rect_xy(&rect(0.0, 0.0, 50.0, 50.0), 5.0, 5.0);
    let mut path = Path::new();
    path.add_rrect_with_start(&rr, Direction::Cw, RectCorner::LowerRight);
    assert!(path.count_points() > 0);
    let out = path.is_rrect().unwrap();
    assert!((out.rect().width() - 50.0).abs() < 0.001);
}

#[test]
fn path_add_rect_corners() {
    let r = rect(0.0, 0.0, 50.0, 50.0);
    for &corner in &[RectCorner::UpperLeft, RectCorner::UpperRight, RectCorner::LowerRight, RectCorner::LowerLeft] {
        let mut path = Path::new();
        path.add_rect(&r, Direction::Cw, corner);
        assert!(path.count_points() > 0);
        assert!(path.is_rect().is_some());
    }
}

#[test]
fn path_get_point() {
    let mut path = Path::new();
    path.move_to(10.0, 20.0).line_to(30.0, 40.0);
    let pt0 = path.get_point(0).unwrap();
    let pt1 = path.get_point(1).unwrap();
    assert!((pt0.x - 10.0).abs() < 0.001);
    assert!((pt0.y - 20.0).abs() < 0.001);
    assert!((pt1.x - 30.0).abs() < 0.001);
    assert!((pt1.y - 40.0).abs() < 0.001);
}

#[test]
fn path_get_point_out_of_range() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(10.0, 10.0);
    assert!(path.get_point(-1).is_none());
    assert!(path.get_point(100).is_none());
}

#[test]
fn path_copy() {
    let mut path1 = Path::new();
    path1.add_rect(&rect(0.0, 0.0, 50.0, 50.0), Direction::Cw, RectCorner::UpperLeft);
    let path2 = Path::from_path(&path1);
    assert_eq!(path1.count_points(), path2.count_points());
    let b1 = path1.tight_bounds();
    let b2 = path2.tight_bounds();
    assert_eq!(b1.left, b2.left);
    assert_eq!(b1.top, b2.top);
    assert_eq!(b1.right, b2.right);
    assert_eq!(b1.bottom, b2.bottom);
}

#[test]
fn path_op_union() {
    let mut path1 = Path::new();
    path1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut path2 = Path::new();
    path2.add_rect(&rect(50.0, 50.0, 150.0, 150.0), Direction::Cw, RectCorner::UpperLeft);
    let result = path_op(&path1, &path2, PathOp::Union).unwrap();
    assert!(result.count_points() > 0);
}

#[test]
fn path_op_intersect() {
    let mut path1 = Path::new();
    path1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut path2 = Path::new();
    path2.add_rect(&rect(50.0, 50.0, 150.0, 150.0), Direction::Cw, RectCorner::UpperLeft);
    let result = path_op(&path1, &path2, PathOp::Intersect).unwrap();
    assert!(result.count_points() > 0);
    let bounds = result.tight_bounds();
    assert!((bounds.left - 50.0).abs() < 0.001);
    assert!((bounds.top - 50.0).abs() < 0.001);
}

#[test]
fn path_op_difference() {
    let mut path1 = Path::new();
    path1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut path2 = Path::new();
    path2.add_rect(&rect(25.0, 25.0, 75.0, 75.0), Direction::Cw, RectCorner::UpperLeft);
    let result = path_op(&path1, &path2, PathOp::Difference).unwrap();
    assert!(result.count_points() > 0);
}

#[test]
fn path_op_xor() {
    let mut path1 = Path::new();
    path1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut path2 = Path::new();
    path2.add_rect(&rect(50.0, 50.0, 150.0, 150.0), Direction::Cw, RectCorner::UpperLeft);
    let result = path_op(&path1, &path2, PathOp::Xor).unwrap();
    assert!(result.count_points() > 0);
}

#[test]
fn path_op_reverse_difference() {
    let mut path1 = Path::new();
    path1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut path2 = Path::new();
    path2.add_rect(&rect(50.0, 50.0, 150.0, 150.0), Direction::Cw, RectCorner::UpperLeft);
    let _result = path_op(&path1, &path2, PathOp::ReverseDifference).unwrap();
}

#[test]
fn path_simplify() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0)
        .line_to(100.0, 0.0)
        .line_to(100.0, 100.0)
        .line_to(50.0, 50.0)
        .line_to(0.0, 100.0)
        .close();
    let _result = simplify(&path).unwrap();
}

#[test]
fn path_pathops_tight_bounds() {
    let mut path = Path::new();
    path.add_rect(&rect(20.0, 30.0, 80.0, 90.0), Direction::Cw, RectCorner::UpperLeft);
    let result = pathops_tight_bounds(&path).unwrap();
    assert!((result.left - 20.0).abs() < 0.001);
    assert!((result.top - 30.0).abs() < 0.001);
    assert!((result.right - 80.0).abs() < 0.001);
    assert!((result.bottom - 90.0).abs() < 0.001);
}

#[test]
fn path_quad_to() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).quad_to(50.0, 100.0, 100.0, 0.0);
    assert!(path.count_points() >= 3);
}

#[test]
fn path_cubic_to() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0)
        .cubic_to(30.0, 50.0, 70.0, 50.0, 100.0, 0.0);
    assert!(path.count_points() >= 4);
}

#[test]
fn path_is_rect() {
    let mut path = Path::new();
    path.add_rect(&rect(10.0, 20.0, 60.0, 70.0), Direction::Cw, RectCorner::UpperLeft);
    let (out_rect, is_closed) = path.is_rect().unwrap();
    assert!(is_closed);
    assert!((out_rect.left - 10.0).abs() < 0.001);
    assert!((out_rect.top - 20.0).abs() < 0.001);
}

#[test]
fn path_is_rect_none_for_oval() {
    let mut path = Path::new();
    path.add_oval(&rect(0.0, 0.0, 50.0, 50.0), Direction::Cw);
    assert!(path.is_rect().is_none());
}

#[test]
fn path_contains() {
    let mut path = Path::new();
    path.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    assert!(path.contains(50.0, 50.0));
    assert!(!path.contains(150.0, 50.0));
}

#[test]
fn path_conservatively_contains_rect() {
    let mut path = Path::new();
    path.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let inner = rect(25.0, 25.0, 75.0, 75.0);
    assert!(path.conservatively_contains_rect(&inner));
}

#[test]
fn point_rect_conversions() {
    let _p = Point::new(1.0, 2.0);
    let r = Rect::new(0.0, 0.0, 10.0, 20.0);
    assert_eq!(r.width(), 10.0);
    assert_eq!(r.height(), 20.0);
    assert!(!r.is_empty());
}

#[test]
fn rect_is_empty() {
    assert!(Rect::new(0.0, 0.0, 0.0, 10.0).is_empty());
    assert!(Rect::new(10.0, 10.0, 5.0, 20.0).is_empty());
}

#[test]
fn op_builder() {
    let mut p1 = Path::new();
    p1.add_rect(&rect(0.0, 0.0, 50.0, 50.0), Direction::Cw, RectCorner::UpperLeft);
    let mut p2 = Path::new();
    p2.add_rect(&rect(25.0, 25.0, 75.0, 75.0), Direction::Cw, RectCorner::UpperLeft);
    let result = OpBuilder::new()
        .add(p1, PathOp::Union)
        .add(p2, PathOp::Union)
        .resolve()
        .unwrap();
    assert!(result.count_points() > 0);
    let bounds = result.tight_bounds();
    assert!(bounds.width() > 50.0);
}

#[test]
fn op_builder_empty() {
    let result = OpBuilder::new().resolve().unwrap();
    assert!(result.is_empty());
}

#[test]
fn op_builder_intersect() {
    let mut p1 = Path::new();
    p1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut p2 = Path::new();
    p2.add_rect(&rect(25.0, 25.0, 75.0, 75.0), Direction::Cw, RectCorner::UpperLeft);
    // First Union (empty ∪ p1 = p1), then Intersect (p1 ∩ p2)
    let result = OpBuilder::new()
        .add(p1, PathOp::Union)
        .add(p2, PathOp::Intersect)
        .resolve()
        .unwrap();
    assert!(result.count_points() > 0);
    let bounds = result.tight_bounds();
    // 交集应在 [25,75]×[25,75] 内
    assert!(bounds.left >= 24.0 && bounds.left <= 26.0);
    assert!(bounds.right >= 74.0 && bounds.right <= 76.0);
}

#[test]
fn op_builder_difference() {
    let mut p1 = Path::new();
    p1.add_rect(&rect(0.0, 0.0, 100.0, 100.0), Direction::Cw, RectCorner::UpperLeft);
    let mut p2 = Path::new();
    p2.add_rect(&rect(25.0, 25.0, 75.0, 75.0), Direction::Cw, RectCorner::UpperLeft);
    let result = OpBuilder::new()
        .add(p1, PathOp::Union)
        .add(p2, PathOp::Difference)
        .resolve()
        .unwrap();
    assert!(result.count_points() > 0);
}

#[test]
fn simplify_returns_path() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0)
        .line_to(100.0, 0.0)
        .line_to(100.0, 100.0)
        .line_to(50.0, 50.0)
        .line_to(0.0, 100.0)
        .close();
    let result = simplify(&path).unwrap();
    assert!(result.count_points() > 0);
}

#[test]
#[ignore] // filterPath may segfault - pathkit path effect integration needs further investigation
fn dash_path_effect() {
    let effect = DashPathEffect::new(&[10.0, 5.0], 0.0).unwrap();
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(100.0, 0.0);
    let dashed = effect.filter_path(&path, 2.0).unwrap();
    assert!(dashed.count_points() > 0);
}

#[test]
#[ignore] // sk_sp drop causes segfault - C++ refcounting not fully integrated
fn dash_path_effect_create() {
    let _effect = DashPathEffect::new(&[10.0, 5.0], 0.0).unwrap();
}

#[test]
#[ignore] // filterPath may segfault - pathkit path effect integration needs further investigation
fn corner_path_effect() {
    let effect = CornerPathEffect::new(5.0).unwrap();
    let mut path = Path::new();
    path.move_to(0.0, 0.0)
        .line_to(50.0, 0.0)
        .line_to(50.0, 50.0)
        .close();
    let rounded = effect.filter_path(&path, 1.0).unwrap();
    assert!(rounded.count_points() > 0);
}

#[test]
#[ignore] // sk_sp drop causes segfault - C++ refcounting not fully integrated
fn corner_path_effect_create() {
    let _effect = CornerPathEffect::new(5.0).unwrap();
}

#[test]
fn stroke_rec_apply_to_path() {
    let mut rec = StrokeRec::new_stroke(4.0, false);
    rec.set_stroke_params(StrokeCap::Round, StrokeJoin::Round, 2.0);
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(100.0, 0.0);
    let stroked = rec.apply_to_path(&path).unwrap();
    assert!(stroked.count_points() > 0);
    let bounds = stroked.tight_bounds();
    assert!(bounds.height() >= 4.0); // stroke expands vertically
}

#[test]
fn stroke_rec_fill_returns_none() {
    let rec = StrokeRec::new_fill();
    let mut path = Path::new();
    path.add_rect(&rect(0.0, 0.0, 50.0, 50.0), Direction::Cw, RectCorner::UpperLeft);
    assert!(rec.apply_to_path(&path).is_none());
}

#[test]
fn stroke_rec_style_width() {
    let rec = StrokeRec::new_stroke(6.0, true);
    assert!(matches!(rec.style(), StrokeStyle::StrokeAndFill { width } if (width - 6.0).abs() < 0.001));
}

#[test]
fn stroke_rec_hairline() {
    let rec = StrokeRec::new_hairline();
    assert!(matches!(rec.style(), StrokeStyle::Hairline));
}

#[test]
fn stroke_rec_inflation_radius() {
    let rec = StrokeRec::new_stroke(4.0, false);
    assert!(rec.inflation_radius() >= 2.0);
}

#[test]
fn stroke_rec_cap_join() {
    let mut rec = StrokeRec::new_stroke(4.0, false);
    assert_eq!(rec.cap(), StrokeCap::Butt);
    assert_eq!(rec.join(), StrokeJoin::Miter);

    rec.set_cap(StrokeCap::Round);
    rec.set_join(StrokeJoin::Round);
    assert_eq!(rec.cap(), StrokeCap::Round);
    assert_eq!(rec.join(), StrokeJoin::Round);

    rec.set_stroke_params(StrokeCap::Square, StrokeJoin::Bevel, 2.0);
    assert_eq!(rec.cap(), StrokeCap::Square);
    assert_eq!(rec.join(), StrokeJoin::Bevel);
    assert!((rec.miter_limit() - 2.0).abs() < 0.001);

    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(50.0, 0.0);
    let stroked = rec.apply_to_path(&path).unwrap();
    assert!(stroked.count_points() > 0);
}

#[test]
fn dash_path_effect_invalid() {
    assert!(DashPathEffect::new(&[], 0.0).is_none());
    assert!(DashPathEffect::new(&[10.0], 0.0).is_none()); // odd count
}

#[test]
fn paint_get_fill_path() {
    let mut paint = Paint::new();
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(4.0);
    paint.set_stroke_cap(StrokeCap::Round);
    paint.set_stroke_join(StrokeJoin::Round);

    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(50.0, 0.0);
    let filled = paint.get_fill_path(&path).unwrap();
    assert!(filled.count_points() > 0);
}

#[test]
fn paint_fill_returns_copy() {
    let paint = Paint::new(); // default fill
    let mut path = Path::new();
    path.add_rect(&rect(10.0, 10.0, 50.0, 50.0), Direction::Cw, RectCorner::UpperLeft);
    let result = paint.get_fill_path(&path).unwrap();
    assert_eq!(result.count_points(), path.count_points());
}

#[test]
fn path_measure_length() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(100.0, 0.0);
    let mut measure = PathMeasure::from_path(&path, false, 1.0);
    let len = measure.length();
    assert!((len - 100.0).abs() < 0.01);
}

#[test]
fn path_measure_pos_tan() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(100.0, 0.0);
    let mut measure = PathMeasure::from_path(&path, false, 1.0);
    let (pos, tan) = measure.pos_tan(50.0).unwrap();
    assert!((pos.x - 50.0).abs() < 0.01);
    assert!((pos.y - 0.0).abs() < 0.01);
    assert!((tan.x - 1.0).abs() < 0.01); // tangent along +x
    assert!((tan.y - 0.0).abs() < 0.01);
}

#[test]
fn path_measure_get_segment() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(100.0, 0.0);
    let mut measure = PathMeasure::from_path(&path, false, 1.0);
    let mut dst = Path::new();
    let ok = measure.get_segment(25.0, 75.0, &mut dst, true);
    assert!(ok);
    assert!(dst.count_points() > 0);
}

#[test]
fn path_measure_empty() {
    let mut measure = PathMeasure::new();
    assert!(measure.length().abs() < 0.001);
    assert!(measure.pos_tan(0.0).is_none());
}

#[test]
fn path_measure_set_path() {
    let mut path = Path::new();
    path.move_to(0.0, 0.0).line_to(50.0, 0.0);
    let mut measure = PathMeasure::new();
    measure.set_path(&path, false);
    assert!((measure.length() - 50.0).abs() < 0.01);
}

#[test]
fn corner_path_effect_invalid() {
    assert!(CornerPathEffect::new(0.0).is_none());
    assert!(CornerPathEffect::new(-1.0).is_none());
}
