use super::edges::Edge2D;
use super::neighbor::Neighbors;
use super::{Point2D, Vector2D};

/// Trait used to define methods that 2D cells should implement in order to be used in a 2D cfd solver
pub trait Cell2D {
    /// Compute the surface of the 2D cell
    fn area(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> f64;

    /// Compute the signed area of the 2D cell
    /// Often useful when building a mesh
    fn signed_area(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> f64;

    /// Computes the center of the cell
    fn center(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> Point2D;

    /// Computes the normals to each edge
    fn normals(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> Vec<Vector2D>;

    /// Gives each node of the cell
    fn nodes(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> Vec<Point2D>;
}

pub mod triangle;
