use super::*;
use float_cmp::approx_eq;

fn poly_points(p: &Polygon2D) -> Vec<f64> {
    p.points().flat_map(|pt| [pt.x(), pt.y()]).collect::<Vec<_>>()
}

#[test]
fn point_ctor() {
    let p = Point2D::new(1., 1.);
    assert_eq!(p.x(), 1.);
    assert_eq!(p.y(), 1.);
}

#[test]
fn polygon_01() {
    let mut p = Polygon2D::with_points(&[Point2D::new(0., 0.), Point2D::new(1., 0.), Point2D::new(1., 1.), Point2D::new(0., 1.)]);
    let res = p.intersect_with_semiplane_mut(&Line2D::with_points(&Point2D::new(0., 0.5), &Point2D::new(0.5, 0.5)), true);
    assert!(res);
    assert!(approx_eq!(&[f64],
        &poly_points(&p),
        &[1., 0.5, 1., 1., 0., 1., 0., 0.5],
        epsilon = 1e-5
    ));
}

#[test]
fn polygon_02() {
    let mut p = Polygon2D::with_points(&[Point2D::new(-300., -300.), Point2D::new(300., -300.), Point2D::new(300., 300.), Point2D::new(-300., 300.)]);
    let mut res;
    let e = Vector2D::new(1., 0.);
    res = p.intersect_with_semiplane_mut(&Line2D::new(e, -10.), true);
    assert!(res);
    assert!(approx_eq!(&[f64],
        &poly_points(&p),
        &[-10., -300., 300., -300., 300., 300., -10., 300.],
        epsilon = 1e-5
    ));
    
    res = p.intersect_with_semiplane_mut(&Line2D::new(e, 70.), false);
    assert!(res);
    assert!(approx_eq!(&[f64],
        &poly_points(&p),
        &[-10., -300., 70., -300., 70., 300., -10., 300.],
        epsilon = 1e-5
    ));
}

#[test]
fn normalize_zero() {
    let v = Point2D::origin().to_vec();
    let u = v.unit();
    assert!(u.map_or(true, |u| approx_eq!(f64, 1.0, u.len())));
}
