use nalgebra::Point2;

use super::Edge2D;
use crate::indices::*;

/// Defines a boundary for meshing purpose.
/// In 2D the boundary is a closed (geometrically speaking) edge list.
/// It also contains an index referecing the boundary condition block to which each edge is linked.
pub struct Boundary2D {
    edges_idx: Vec<EdgeIndex>,
    boundary_condition_idx: Vec<BoundaryConditionIndex>,
}

impl Boundary2D {
    pub fn new(
        global_edges: &[Edge2D],
        global_nodes: &[Point2<f64>],
        global_boundary_condition: &[String],
        edges_idx: Vec<EdgeIndex>,
        boundary_condition_idx: Vec<BoundaryConditionIndex>,
    ) -> Result<Self, String> {
        let boundary = Boundary2D {
            edges_idx,
            boundary_condition_idx,
        };

        match boundary.check(global_edges, global_nodes, global_boundary_condition) {
            Ok(_) => Ok(boundary),
            Err(error) => Err(format!("Invalid boundary, got : {}", error)),
        }
    }

    /// Checks the validity of the boundary
    pub fn check(
        &self,
        global_edges: &[Edge2D],
        global_nodes: &[Point2<f64>],
        global_boundary_condition: &[String],
    ) -> Result<(), String> {
        todo!()
    }

    /// Returns an immutable the indices of the edges contained in this boundary
    pub fn edges_idx(&self) -> &Vec<EdgeIndex> {
        &self.edges_idx
    }

    /// Returns an immutable the indices of the boundary conditions for the edges of this boundary.
    pub fn boundary_condition_idx(&self) -> &Vec<BoundaryConditionIndex> {
        &self.boundary_condition_idx
    }
}
