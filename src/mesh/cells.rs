use super::edges::*;
pub use super::neighbor::*;
use nalgebra::Point2;
pub use triangle::*;
use crate::EdgeIndex;

pub mod triangle;

pub trait Cell2D: Sized {
    /// Creates a valid new instance of a cell.
    fn new(edge_idx: EdgeIndex) -> Result<Self, String>;

    /// Returns the edges of the cell.
    fn edges(&self, global_edges: &[Edge2D]) -> &Vec<Edge2D>;
    
    /// Returns the nodes of the cell.
    fn nodes(&self, global_nodes: &[Point2<f64>]) -> &Vec<Point2<f64>>;

    /// Returns the nodes of the cell.
    /// Defined at unsafe since it uses points positions which may not be valid when editing the mesh.
    /// Is abstracted has safe in the FinishedBlockMesh2D.
    unsafe fn fast_nodes(&self) -> &Vec<Point2<f64>>;
    
    /// Computes the surface.
    fn surface(&self, global_edges: &[Edge2D], global_nodes: &[Point2<f64>]) -> f64;
    
    /// Computes the surface.
    /// Defined at unsafe since it uses points positions which may not be valid when editing the mesh.
    /// Is abstracted has safe in the FinishedBlockMesh2D.
    unsafe fn fast_surface(&self) -> f64;
    
    /// Computes the length of the edges.
    fn edges_length(&self, global_edges: &[Edge2D], global_nodes: &[Point2<f64>]) -> Vec<f64>;
    
    /// Computes the length of the edges.
    /// Defined at unsafe since it is meant to use points positions (known internally) which may not be valid when editing the mesh.
    /// Is abstracted has safe in the FinishedMeshBlock2D.
    fn fast_edges_length(&self) -> Vec<f64>;
    
    /// Updates the point positions to match the real one in the cell.
    /// Should not be useful in FinishedMeshBlock.
    fn update_nodes(&mut self);
}
