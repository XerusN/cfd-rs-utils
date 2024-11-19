use nalgebra::Point2;

use super::neighbor::Neighbor2D;
use crate::indices::*;

/// Represents a 2D edge, keeps in memory the two neighbors (cell or boundary) and 2 indices pointing to nodes in a mesh data structure.
/// The limitation of 2D is enforced by the number of parents being set to 2, also easier in term of methods, 2D implementations are much easier, and some don't make sense in 3D.
/// The position of the neighbor in the edge array is of prior importance.
#[derive(Debug, Clone, Default)]
pub struct Edge2D {
    nodes_idx: [NodeIndex; 2],
    neighbors: [Neighbor2D; 2],
}

impl Edge2D {
    /// Creates a new instance of an edge
    pub fn new(global_nodes: &[Point2<f64>], nodes_idx: [NodeIndex; 2]) -> Result<Self, String> {
        todo!()
    }

    /// Checks that the edge is valid (existing nodes and edge is a non-zero vector)
    pub fn check(&self, global_nodes: &[Point2<f64>]) {
        todo!()
    }

    /// Returns an immutable reference to the nodes indices of the edge
    pub fn nodes_idx(&self) -> &[NodeIndex; 2] {
        &self.nodes_idx
    }

    /// Returns an immutable reference to the neighbors of the edge
    pub fn neighbors(&self) -> &[Neighbor2D; 2] {
        &self.neighbors
    }
}
