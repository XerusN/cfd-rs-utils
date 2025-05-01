use nalgebra::{Point2, Vector2};

pub fn triangle_area(points: &[Point2<f64>; 3]) -> f64 {
    todo!()
}

pub fn line_length(points: &[Point2<f64>; 2]) -> f64 {
    Vector2::new(points[1].x - points[0].x, points[1].y - points[0].y).norm()
}

pub fn line_normal(points: &[Point2<f64>; 2]) -> Vector2<f64> {
    todo!()
}

pub fn triangle_centroid(points: &[Point2<f64>; 3]) -> Point2<f64> {
    Point2::new((points[0].x + points[1].x + points[2].x)/3., (points[0].y + points[1].y + points[2].y)/3.)
}