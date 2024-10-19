use super::edges::Edge2D;
use super::neighbor::Neighbors;
use nalgebra::{Point2, Unit, Vector2};
pub use triangle::Triangle;

/// Trait used to define methods that 2D cells should implement in order to be used in a 2D cfd solver.
pub trait Cell2D {
    /// Gives the number of edges for this cell (and thus the number of nodes)
    fn edge_number() -> usize;

    /// Creates a new cell, `global_edges` is used to update nodes_idx
    fn new_cell(
        cell_edges_idx: &[usize],
        cell_neighbors: &[Neighbors],
        global_edges: &[Edge2D],
        global_nodes: &[Point2<f64>],
    ) -> Self;

    /// Compute the surface of the cell.
    fn area(&self, nodes: &[Point2<f64>]) -> f64;

    /// Computes the center of the cell.
    fn center(&self, nodes: &[Point2<f64>]) -> Point2<f64>;

    /// Computes the normals to each edge.
    fn normals(&self, edges: &[Edge2D], nodes: &[Point2<f64>]) -> Vec<Unit<Vector2<f64>>>;

    /// Gives a reference to each node of the cell.
    fn nodes<'a>(&self, nodes: &'a [Point2<f64>]) -> Vec<&'a Point2<f64>>;

    /// Gives a reference to each edge of the cell.
    fn edges<'a>(&self, edges: &'a [Edge2D]) -> Vec<&'a Edge2D>;

    /// Gives a mutable reference to each node of the cell.
    fn nodes_mut<'a>(&self, nodes: &'a mut [Point2<f64>]) -> Vec<&'a mut Point2<f64>>;

    /// Ensures that the cell is properly defined (no out of bound value or duplicated edges/nodes)
    fn check(&self, edges: &[Edge2D], nodes: &[Point2<f64>]) -> Result<(), String>;
}

pub mod triangle;
