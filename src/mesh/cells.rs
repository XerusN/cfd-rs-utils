use super::edges::*;
pub use super::neighbor::*;
use nalgebra::Point2;
pub use triangle::*;

pub mod triangle;

pub trait Cell2D: Sized {
    /// Creates a valid new instance of a cell
    fn new(global_edges: &[Edge2D], global_nodes: &[Point2<f64>]) -> Result<Self, String>;

    /// Returns the nodes of the cell
    fn nodes(&self, global_nodes: &[Point2<f64>]) -> &Vec<Point2<f64>>;

    /// Returns the edges of the cell
    fn edges(&self, global_edges: &[Edge2D]) -> &Vec<Edge2D>;
}
