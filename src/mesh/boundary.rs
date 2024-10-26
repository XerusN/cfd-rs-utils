use nalgebra::Point2;

use super::Edge2D;

/// Defines a boundary for meshing purpose.
/// In 2D the boundary is a closed (geometrically speaking) edge list.
/// It also contains an index referecing the boundary condition block to which each edge is linked.
pub struct Boundary2D {
    edges_idx: Vec<usize>,
    boundary_condition_idx: Vec<usize>,
}

impl Boundary2D {
    pub fn new(
        global_edges: &[Edge2D],
        global_nodes: &[Point2<f64>],
        global_boundary_condition: &[String],
        edges_idx: Vec<usize>,
        boundary_condition_idx: Vec<usize>,
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
}
