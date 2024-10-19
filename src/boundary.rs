use crate::{Edge2D, Neighbors};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Boundary2D {
    edges_idx: Vec<usize>,
}

impl Boundary2D {
    /// Creates a new Boundary, empty by default to add edges and keep coherent values with edges parents
    pub fn new() -> Boundary2D {
        Boundary2D {
            edges_idx: Vec::new(),
        }
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
        assert!(boundary_edge_idx < global_edges.len());
        self.edges_idx.push(boundary_edge_idx);
        let mut place_found = false;
        for i in 0..global_edges[boundary_edge_idx].parents.len() {
            if let Neighbors::None = global_edges[boundary_edge_idx].parents[i] {
                global_edges[boundary_edge_idx].parents[i] = Neighbors::Boundary(boundary_index);
                place_found = true;
                break;
            }
        }
        assert!(place_found, "No parent place free on the edge")
    }
}
