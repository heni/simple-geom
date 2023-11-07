#![allow(unused_macros)]
#![allow(unused_imports)]

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    x: f64,
    y: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    x: f64,
    y: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct Segment2D {
    p: Point2D,
    e: Vector2D,
}
#[derive(Clone, Copy, Debug)]
pub struct Line2D {
    n: Vector2D,
    a: f64,
}
#[derive(Clone, Copy, Debug)]
pub enum SegmentIntersection {
    None,
    Point(Point2D),
    Segment(Segment2D),
}
#[derive(Clone, Debug)]
pub struct Polygon2D {
    segments: Vec<Segment2D>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointLineRelation {
    Left,
    OnLine,
    Right,
}
#[derive(Clone, Copy, Debug)]
pub enum SegmentLineRelation {
    Left,
    LeftTouch,
    OnLine,
    Intersects(Point2D),
    Right,
    RightTouch,
}

impl Point2D {
    const ORIGIN: Point2D = Point2D { x: 0., y: 0. };

    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
    pub fn origin() -> Self {
        Self::ORIGIN
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn add(&self, v: &Vector2D) -> Self {
        Point2D {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
    pub fn sub(&self, p0: &Point2D) -> Vector2D {
        Vector2D {
            x: self.x - p0.x,
            y: self.y - p0.y,
        }
    }
    pub fn to_vec(&self) -> Vector2D {
        self.sub(&Self::ORIGIN)
    }
    pub fn distance_to(&self, p: &Point2D) -> f64 {
        self.sub(p).len()
    }
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn add(&self, v: &Vector2D) -> Self {
        Vector2D {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
    pub fn add_mut(&mut self, v: &Vector2D) -> &Self {
        self.x += v.x;
        self.y += v.y;
        self
    }
    pub fn kmul(&self, k: f64) -> Self {
        Vector2D {
            x: k * self.x,
            y: k * self.y,
        }
    }
    pub fn dot(&self, v: &Vector2D) -> f64 {
        self.x * v.x + self.y * v.y
    }
    pub fn cross(&self, v: &Vector2D) -> f64 {
        self.x * v.y - self.y * v.x
    }
    pub fn unit(&self) -> Self {
        self.kmul(1. / self.len())
    }
    pub fn perpendicular(&self) -> Self {
        Vector2D {
            x: -self.y,
            y: self.x,
        }
    }
}

impl Segment2D {
    const EPS: f64 = 1e-8;

    pub fn new(p: &Point2D, e: &Vector2D) -> Self {
        Segment2D { p: *p, e: *e }
    }
    pub fn with_points(p0: &Point2D, p1: &Point2D) -> Self {
        Segment2D {
            p: p0.clone(),
            e: p1.sub(p0),
        }
    }

    pub fn intersect_segment(&self, o: &Segment2D) -> SegmentIntersection {
        let (p00, p01) = (self.p, self.p.add(&self.e));
        let (p10, p11) = (o.p, o.p.add(&o.e));
        if f64::max(p00.x, p01.x) < f64::min(p10.x, p11.x) - Self::EPS
            || f64::min(p00.x, p01.x) > f64::max(p10.x, p11.x) + Self::EPS
            || f64::max(p00.y, p01.y) < f64::min(p10.y, p11.y) - Self::EPS
            || f64::min(p00.y, p01.y) > f64::max(p10.y, p11.y) + Self::EPS
        {
            return SegmentIntersection::None;
        }
        /***
         * Solve the equation:
         * u * self.e.x - v * o.e.x = o.p.x - self.p.x
         * u * self.e.y - v * o.e.y = o.p.y - self.p.y
         ***/
        let d = -self.e.x * o.e.y + self.e.y * o.e.x;
        if d.abs() < Self::EPS {
            return match self.e.x * (o.p.y - self.p.y) - self.e.y * (o.p.x - self.p.x) {
                val if val.abs() > Self::EPS => SegmentIntersection::None,
                _ => {
                    let w = self.e.unit();
                    let (o0, o1) = (p10.sub(&p00).dot(&w), p11.sub(&p00).dot(&w));
                    let (s0, s1) = (0., self.e.len());
                    if o0 < s0 {
                        assert!(o1 + Self::EPS > s0);
                        if o1 < s1 {
                            SegmentIntersection::Segment(Segment2D::with_points(&p00, &p11))
                        } else {
                            SegmentIntersection::Segment(self.clone())
                        }
                    } else if o1 < s0 {
                        assert!(o0 + Self::EPS > s0);
                        if o0 < s1 {
                            SegmentIntersection::Segment(Segment2D::with_points(&p00, &p10))
                        } else {
                            SegmentIntersection::Segment(self.clone())
                        }
                    } else {
                        if o1 > s1 {
                            SegmentIntersection::Segment(Segment2D::with_points(&p10, &p01))
                        } else if o0 > s1 {
                            SegmentIntersection::Segment(Segment2D::with_points(&p11, &p01))
                        } else {
                            SegmentIntersection::Segment(o.clone())
                        }
                    }
                }
            };
        }

        match (-(o.p.x - self.p.x) * o.e.y + (o.p.y - self.p.y) * o.e.x) / d {
            u if u < -Self::EPS || u > 1.0 + Self::EPS => SegmentIntersection::None,
            u if u < Self::EPS => SegmentIntersection::Point(p00),
            u if u > 1.0 - Self::EPS => SegmentIntersection::Point(p01),
            u => {
                let intersection_point = self.p.add(&self.e.kmul(u));
                SegmentIntersection::Point(intersection_point)
            }
        }
    }

    pub fn intersect_line(&self, l: &Line2D) -> SegmentIntersection {
        /***
         * solve the equation (<*,*> - dot product)
         * <self.p + t * self.e, l.n> = l.a
         *
         * t * <self.e, l.n> = l.a - <self.p, l.n>
         ***/
        let p_offset = l.a - l.n.dot(&self.p.to_vec());
        if self.e.dot(&l.n).abs() < Self::EPS {
            if p_offset.abs() < Self::EPS {
                SegmentIntersection::Segment(self.clone())
            } else {
                SegmentIntersection::None
            }
        } else {
            match p_offset / l.n.dot(&self.e) {
                t if t < -Self::EPS || t > 1. + Self::EPS => SegmentIntersection::None,
                t if t < Self::EPS => SegmentIntersection::Point(self.p),
                t => SegmentIntersection::Point(self.p.add(&self.e.kmul(t))),
            }
        }
    }
}

impl Line2D {
    const EPS: f64 = 1e-8;
    pub fn new(normal: Vector2D, a: f64) -> Self {
        Line2D { n: normal, a }
    }
    pub fn with_points(p0: &Point2D, p1: &Point2D) -> Self {
        let n = p1.sub(p0).perpendicular();
        let a = n.dot(&p0.to_vec());
        Line2D { n, a }
    }
    pub fn point_relation(&self, p: &Point2D) -> PointLineRelation {
        let p_offset = self.n.dot(&p.to_vec());
        if p_offset < self.a - Self::EPS {
            PointLineRelation::Left
        } else if p_offset < self.a + Self::EPS {
            PointLineRelation::OnLine
        } else {
            PointLineRelation::Right
        }
    }
    pub fn segment_relation(&self, s: &Segment2D) -> SegmentLineRelation {
        let (p0, p1) = (s.p, s.p.add(&s.e));
        match (self.point_relation(&p0), self.point_relation(&p1)) {
            (a, b) if a == b => match a {
                PointLineRelation::Left => SegmentLineRelation::Left,
                PointLineRelation::Right => SegmentLineRelation::Right,
                PointLineRelation::OnLine => SegmentLineRelation::OnLine,
            },
            (PointLineRelation::OnLine, PointLineRelation::Left)
            | (PointLineRelation::Left, PointLineRelation::OnLine) => {
                SegmentLineRelation::LeftTouch
            }
            (PointLineRelation::OnLine, PointLineRelation::Right)
            | (PointLineRelation::Right, PointLineRelation::OnLine) => {
                SegmentLineRelation::RightTouch
            }
            _ => match s.intersect_line(self) {
                SegmentIntersection::Point(p) => SegmentLineRelation::Intersects(p),
                _ => {
                    panic!("unreachable branch");
                }
            },
        }
    }
}

impl Polygon2D {
    pub fn new(segments: Vec<Segment2D>) -> Self {
        assert!(segments.len() >= 3);
        Polygon2D { segments }
    }

    pub fn with_points(points: &[Point2D]) -> Self {
        let mut segments = Vec::new();
        let n = points.len();
        assert!(n >= 3);
        for i in 0..n {
            segments.push(Segment2D::with_points(&points[i], &points[(i + 1) % n]));
        }
        Polygon2D { segments }
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn points(&self) -> impl Iterator<Item = Point2D> + '_ {
        self.segments.iter().map(|s| s.p)
    }

    pub fn intersect_with_semiplane(&self, l: &Line2D, left_semiplane: bool) -> Option<Polygon2D> {
        enum TouchOrder {
            First,
            Second,
        }
        let l = if !left_semiplane {
            l.clone()
        } else {
            Line2D::new(l.n.kmul(-1.), -l.a)
        };
        let mut out_segments = Vec::new();
        let mut mem_points = Vec::new();
        let mut handle_touch = |p: Point2D, segments: &mut Vec<Segment2D>, order: TouchOrder| {
            if mem_points.is_empty() {
                mem_points.push(p);
            } else {
                let p0 = mem_points.pop().unwrap();
                segments.push(match order {
                    TouchOrder::First => Segment2D::with_points(&p, &p0),
                    TouchOrder::Second => Segment2D::with_points(&p0, &p),
                });
            }
        };

        for s in &self.segments {
            let (p0, p1) = (s.p, s.p.add(&s.e));
            match l.segment_relation(s) {
                SegmentLineRelation::Right
                | SegmentLineRelation::RightTouch
                | SegmentLineRelation::OnLine => { /* skip it */ }
                SegmentLineRelation::Left => {
                    out_segments.push(s.clone());
                }
                SegmentLineRelation::LeftTouch => match l.point_relation(&p0) {
                    PointLineRelation::OnLine => {
                        handle_touch(p0, &mut out_segments, TouchOrder::Second);
                        out_segments.push(s.clone());
                    }
                    _ => {
                        out_segments.push(s.clone());
                        handle_touch(p1, &mut out_segments, TouchOrder::First);
                    }
                },
                SegmentLineRelation::Intersects(p) => match l.point_relation(&p0) {
                    PointLineRelation::Left => {
                        out_segments.push(Segment2D::with_points(&p0, &p));
                        handle_touch(p, &mut out_segments, TouchOrder::First);
                    }
                    PointLineRelation::Right => {
                        handle_touch(p, &mut out_segments, TouchOrder::Second);
                        out_segments.push(Segment2D::with_points(&p, &p1));
                    }
                    _ => {
                        panic!("unreachable branch");
                    }
                },
            }
        }

        if !out_segments.is_empty() {
            assert!(out_segments.len() >= 3);
            Some(Polygon2D::new(out_segments))
        } else {
            None
        }
    }

    pub fn intersect_with_semiplane_mut(&mut self, l: &Line2D, less_mode: bool) -> bool {
        match self.intersect_with_semiplane(l, less_mode) {
            None => false,
            Some(p) => {
                self.segments = p.segments;
                true
            }
        }
    }

    pub fn skip_short_edge(&self, tol: f64) -> Self {
        let mut segments = Vec::new();
        segments.push(self.segments[0]);
        for s in &self.segments[1..] {
            if s.e.len() < tol {
                let last = segments.len() - 1;
                segments[last].e.add_mut(&s.e);
            } else {
                segments.push(s.clone());
            }
        }
        /*
         * specific implementation for the case of near points
         * we should to support invariant len(segments) >= 3
         */
        if segments.len() <= 2 && self.segments.len() >= 3 {
            let p0 = segments[0].p;
            let p1 = segments[0].p.add(&segments[0].e);

            let mut opt_p = self.segments[1].p;
            let mut opt_dist = opt_p.distance_to(&p0).min(opt_p.distance_to(&p1));
            for i in 2..self.segments.len() {
                let p = self.segments[i].p;
                let dist = p.distance_to(&p0).min(p.distance_to(&p1));
                if dist > opt_dist {
                    opt_p = p;
                    opt_dist = dist;
                }
            }
            return Polygon2D::with_points(&[p0, opt_p, p1]);
        }

        Polygon2D { segments }
    }
}
