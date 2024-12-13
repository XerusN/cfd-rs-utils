use crate::{EdgeIndex, NodeIndex};

use crate::*;
use super::Cell2D;

/// Defines a basic cell
#[derive(Debug, Clone, Default)]
pub struct Triangle {
    edges: [EdgeIndex; 3],
    nodes: [Point2<f64>; 3],
}

impl Cell2D for Triangle {
    /// Creates a valid new instance of a cell.
    fn new(edges_idx: &[EdgeIndex]) -> Result<Self, Error> {
        if edges_idx.len() != 3 {
            return Err(Error::WrongSliceLength { got: edges_idx.len(), expected: 3 });
        }
        Ok(Triangle {
            edges: [edges_idx[0], edges_idx[1], edges_idx[2]],
            nodes: [Point2::default(); 3],
        })
    }

    /// Returns the edges of the cell.
    fn edges<'a>(&self, global_edges: &'a [Edge2D]) -> Vec<&'a Edge2D> {
        vec![&global_edges[*self.edges[0]], &global_edges[*self.edges[1]], &global_edges[*self.edges[2]]]
    }

    /// Returns the nodes of the cell.
    fn nodes<'a>(&self, global_nodes: &'a [Point2<f64>]) -> Vec<&'a Point2<f64>> {
        todo!()
    }

    /// Returns the nodes of the cell.
    /// Defined as unsafe since it uses points positions which may not be valid when editing the mesh.
    /// Is abstracted has safe in the FinishedBlockMesh2D.
    unsafe fn fast_nodes(&self) -> Vec<&Point2<f64>>{
        vec![&self.nodes[0], &self.nodes[1], &self.nodes[2]]
    }

    /// Computes the surface.
    fn surface(&self, global_edges: &[Edge2D], global_nodes: &[Point2<f64>]) -> f64;

    /// Computes the surface.
    /// Defined as unsafe since it uses points positions which may not be valid when editing the mesh.
    /// Is abstracted has safe in the FinishedBlockMesh2D.
    unsafe fn fast_surface(&self) -> f64;

    unsafe fn neighbors(&self) -> f64;

    /// Computes the length of the edges.
    fn edges_length(&self, global_edges: &[Edge2D], global_nodes: &[Point2<f64>]) -> Vec<f64>;

    /// Computes the length of the edges.
    /// Defined as unsafe since it is meant to use points positions (known internally) which may not be valid when editing the mesh.
    /// Is abstracted has safe in the FinishedMeshBlock2D.
    fn fast_edges_length(&self) -> Vec<f64>;

    /// Updates the point positions to match the real one in the cell.
    /// Should not be useful in FinishedMeshBlock.
    fn update_nodes(&mut self);
}
