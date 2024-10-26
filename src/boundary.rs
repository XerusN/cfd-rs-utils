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
        let (mut first_boundary_node, mut last_boundary_node) = (None, None);
        for i in 0..edge_nodes.len() {
            if edge_nodes[i] == *self.edges_idx.last().expect("global_edges is empty") {
                last_boundary_node = Some(i);
            }
            if edge_nodes[i] == *self.edges_idx.first().expect("global_edges is empty") {
                first_boundary_node = Some(i);
            }
        }
        
        let last_boundary_node = last_boundary_node.expect("edge is not connected to the last edge of the boundary");

        let mut place_found = false;
        if let Neighbors::None = global_edges[boundary_edge_idx].parents[last_boundary_node] {
            global_edges[boundary_edge_idx].parents[last_boundary_node] = Neighbors::Boundary(boundary_index);
            place_found = true;
        }
        assert!(place_found, "Expected parent place not available on edge");

        self.edges_idx.push(boundary_edge_idx);

        if first_boundary_node.is_some() {
            self.locked = true
        }
    }
}
