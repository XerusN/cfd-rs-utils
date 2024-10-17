use super::edges::Edge2D;
use super::neighbor::Neighbors;
use super::{Point2D, Vector2D};
pub use triangle::Triangle;

/// Trait used to define methods that 2D cells should implement in order to be used in a 2D cfd solver.
pub trait Cell2D {
    /// Compute the surface of the cell.
    fn area(&self, nodes: &[Point2D]) -> f64;

    /// Computes the center of the cell.
    fn center(&self, nodes: &[Point2D]) -> Point2D;

    /// Computes the normals to each edge.
    fn normals(&self, edges: &[Edge2D], nodes: &[Point2D]) -> Vec<Vector2D>;

    /// Gives a reference to each node of the cell.
    fn nodes<'a>(&self, nodes: &'a [Point2D]) -> Vec<&'a Point2D>;

    /// Gives a reference to each edge of the cell.
    fn edges<'a>(&self, edges: &'a [Edge2D]) -> Vec<&'a Edge2D>;

    /// Gives a mutable reference to each node of the cell.
    fn nodes_mut<'a>(&self, nodes: &'a mut [Point2D]) -> Vec<&'a mut Point2D>;

    /// Gives a mutable reference to each edge of the cell.
    fn edges_mut<'a>(&self, edges: &'a mut [Edge2D]) -> Vec<&'a mut Edge2D>;
}

pub mod triangle;
