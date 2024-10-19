use crate::{Edge2D, Neighbors};

/// Describes a boundary of the domain, needs to be defined in a organised manner (clockwise or counter-clockwise)
#[derive(Default, Clone, PartialEq, Debug)]
pub struct Boundary2D {
    edges_idx: Vec<usize>,
    locked: bool,
}

impl Boundary2D {
    /// Creates a new Boundary, empty by default to add edges and keep coherent values with edges parents
    pub fn new() -> Boundary2D {
        Boundary2D {
            edges_idx: Vec::new(),
            locked: false,
        }
    }

    pub fn status(&self) -> bool {
        self.locked
    }

    pub fn edges_idx(&self) -> &Vec<usize> {
        &self.edges_idx
    }

    /// Adds an edge to the boundary and updates the edge parent accordingly.
    /// `boundary_index` refers to the number wich designate this boundary.
    pub fn add_edge(
        &mut self,
        boundary_index: usize,
        boundary_edge_idx: usize,
        global_edges: &mut [Edge2D],
    ) {
        assert!(!self.locked, "Boundary is already closed");

        assert!(
            boundary_edge_idx < global_edges.len(),
            "boundary_edge_idx is out of bound in global_edges"
        );
        let edge_nodes = global_edges[boundary_edge_idx].nodes_idx;
        let (mut first_boundary_node, mut last_boundary_node) = (false, false);
        for i in edge_nodes {
            if i == *self.edges_idx.last().expect("global_edges is empty") {
                last_boundary_node = true;
            }
            if i == *self.edges_idx.first().expect("global_edges is empty") {
                first_boundary_node = true;
            }
        }

        assert!(
            last_boundary_node,
            "edge is not connected to the last edge of the boundary"
        );

        let mut place_found = false;
        for i in 0..global_edges[boundary_edge_idx].parents.len() {
            if let Neighbors::None = global_edges[boundary_edge_idx].parents[i] {
                global_edges[boundary_edge_idx].parents[i] = Neighbors::Boundary(boundary_index);
                place_found = true;
                break;
            }
        }
        assert!(place_found, "No parent place free on the edge");

        self.edges_idx.push(boundary_edge_idx);

        if first_boundary_node {
            self.locked = true
        }
    }
}
