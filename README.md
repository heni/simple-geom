# simple-geom
Rust Library with simple plane geometry


## Provided Classes
- [`Point2D`](#point2d)
- [`Vector2D`](#vector2d)
- [`Segment2D`](#segment2d)
- [`Line2D`](#line2d)
- [`Polygon2D`](#polygon2d)

## Supported operations

### Point2D
```rust
pub fn add(&self, v: &Vector2D) -> Self; /** return the sum of point & vector **/
pub fn sub(&self, p0: &Point2D) -> Vector2D; /** return the vector difference between two points **/
pub fn to_vec(&self) -> Vector2D; /** transform point to radius-vector; the same as difference between point and origin **/
pub fn distance_to(&self, p: &Point2D) -> f64;  /** return the distance between points **/
```

### Vector2D
```rust
pub fn len(&self) -> f64;  /** return the length of vector **/
pub fn add(&self, v: &Vector2D) -> Self;  /** return the sum of vectors **/
pub fn add_mut(&mut self, v: &Vector2D) -> &Self;  /** mutably add second vector to the given **/
pub fn kmul(&self, k: f64) -> Self;  /** return scaled by k vector **/
pub fn dot(&self, v: &Vector2D) -> f64;  /** return dot product result for the vectors **/
pub fn cross(&self, v: &Vector2D) -> f64;  /** return cross product result for the 2d vectors **/
pub fn unit(&self) -> Option<Self>;  /** return scaled to the lenght=1 vector if possible **/
pub fn perpendicular(&self) -> Self;  /** return vector perpendicular to the given one **/
```

### Segment2D
```rust
pub fn intersect_segment(&self, o: &Segment2D) -> SegmentIntersection; /** return the intersection relation (with intersection point if needed) for 2 segments **/
pub fn intersect_line(&self, l: &Line2D) -> SegmentIntersection;  /** return the intersection relation (with intersection point if needed) for given segment and the line **/
```

### Line2D
```rust
pub fn point_relation(&self, p: &Point2D) -> PointLineRelation;  /** check either point lies on the given line **/
pub fn segment_relation(&self, s: &Segment2D) -> SegmentLineRelation;  /** check the way which segment relates to the given line (Intersects, Left, LeftTouch, Right, RightTouch, OnLine) **/
```

### Polygon2D
```rust
pub fn len(&self) -> usize;  /** return the number of segments for the given polygon **/
pub fn points(&self) -> impl Iterator<Item = Point2D> + '_;  /** iterate over the vertices of the given polygon **/
pub fn intersect_with_semiplane(&self, l: &Line2D, left_semiplane: bool) -> Option<Polygon2D>;  /** return the intersection of polygon and semiplane **/
pub fn intersect_with_semiplane_mut(&mut self, l: &Line2D, less_mode: bool) -> bool;  /** mutable version of polygon and semiplane intersection **/
pub fn skip_short_edge(&self, tol: f64) -> Self;  /** prune the polygon by removing too small edges (by given tolerance) **/
```
