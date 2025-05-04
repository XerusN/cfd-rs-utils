use nalgebra::{Point2, Vector2};

/// Using Heron's formula
pub fn triangle_area(points: &[Point2<f64>; 3]) -> f64 {
    let (a, b, c) = (
        line_length(&[points[0], points[1]]),
        line_length(&[points[1], points[2]]),
        line_length(&[points[2], points[0]]),
    );
    let s = 0.5 * (a + b + c);
    (s*(s-a)*(s-b)*(s-c)).sqrt()
}

pub fn line_length(points: &[Point2<f64>; 2]) -> f64 {
    Vector2::new(points[1].x - points[0].x, points[1].y - points[0].y).norm()
}

/// Rotated counter-clockwise and normalized
pub fn line_normal(points: &[Point2<f64>; 2]) -> Vector2<f64> {
    Vector2::new(points[0].y - points[1].y, points[1].x - points[0].x).normalize()
}

pub fn triangle_centroid(points: &[Point2<f64>; 3]) -> Point2<f64> {
    Point2::new(
        (points[0].x + points[1].x + points[2].x) / 3.,
        (points[0].y + points[1].y + points[2].y) / 3.,
    )
}

pub fn geometric_weighting_factor(
    cell_centroids: &[Point2<f64>; 2],
    face_center: &Point2<f64>,
) -> f64 {
    line_length(&[cell_centroids[1], *face_center])
        / line_length(&[cell_centroids[0], cell_centroids[1]])
}
