use nalgebra::Point2;

use super::neighbor::{self, Neighbor2D};
use crate::{indices::*, Error};

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
    /// The neighbors should be defined such as when going from the first node to the second, the first neighbor is on the right.
    pub fn new(nodes_idx: [NodeIndex; 2], neighbors: [Neighbor2D; 2]) -> Self {
        Edge2D {
            nodes_idx,
            neighbors,
        }
    }

    /// Checks that the edge is valid (existing nodes and neighbors)
    pub fn check(
        &self,
        global_nodes: &[Point2<f64>],
        global_neighbors: &[Neighbor2D],
    ) -> Result<(), Error> {
        for node in self.nodes_idx {
            if *node >= global_nodes.len() {
                return Err(Error::NodeOutOfBound {
                    index: node,
                    length: global_nodes.len(),
                });
            }
        }
        // Add neighbor check
        todo!()
    }

    /// Returns an immutable reference to the nodes indices of the edge
    pub fn nodes_idx(&self) -> &[NodeIndex; 2] {
        &self.nodes_idx
    }

    /// Returns an immutable reference to the neighbors of the edge
    pub unsafe fn neighbors(&self) -> &[Neighbor2D; 2] {
        &self.neighbors
    }

    pub fn update_neighbors(&mut self) {
        todo!()
    }
}
