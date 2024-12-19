use crate::boundary::Boundary;
use indices::*;
use nalgebra::Point2;

pub mod indices;

/// Parent of a half_edge, either a cell or a boundary.
/// ```None``` is meant as an error or temporary state.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Parent {
    #[default]
    None,
    Cell(u32),
    Boundary(Boundary),
}

/// Array based Half-edge data-structure mesh representation
/// Supports meshes of up to a billion element.
/// Since the crate is built for cfd on a classic computer (not HPC) it is easily enough.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mutable2DMesh {
    //```he``` is for Half-edge
    he_to_vertex: Vec<VertexIndex>,
    he_to_twin: Vec<HalfEdgeIndex>,
    he_to_next_he: Vec<HalfEdgeIndex>,
    he_to_prev_he: Vec<HalfEdgeIndex>,
    he_to_parent: Vec<ParentIndex>,

    vertices: Vec<Point2<f64>>,
    parents: Vec<Parent>,

    parent_to_first_he: Vec<HalfEdgeIndex>,
}

impl Mutable2DMesh {
    /// Creates a mesh with only boundaries defined
    pub fn new_from_boundary(
        vertices: Vec<Point2<f64>>,
        he_to_vertex: Vec<VertexIndex>,
        parents: Vec<Parent>,
    ) -> Self {
        // All arrays are needed to be built correctly
        todo!()
    }

    /// Gets all the vertices of an half-edge
    pub fn vertices_from_he(&self, he_id: HalfEdgeIndex) -> [VertexIndex; 2] {
        [
            self.he_to_vertex[he_id],
            self.he_to_vertex[self.he_to_twin[he_id]],
        ]
    }

    /// Gets all half-edges from a parent (Cell or boundary)
    pub fn he_from_parent(&self, parent_id: ParentIndex) -> Vec<HalfEdgeIndex> {
        let first_he = self.parent_to_first_he[parent_id];
        let mut result = vec![first_he];
        let mut current_he = self.he_to_next_he[first_he];

        while first_he != current_he {
            result.push(current_he);
            current_he = self.he_to_next_he[current_he];
        }

        result
    }

    /// Gets all vertices from a parent (Cell or boundary)
    pub fn vertices_from_parent(&self, parent_id: ParentIndex) -> Vec<VertexIndex> {
        self.he_from_parent(parent_id)
            .into_iter()
            .map(|he_id| self.he_to_vertex[he_id])
            .collect()
    }
    
    /// Gets the parent from an HalfEdge
    pub fn parent_from_he(&self, he_id: HalfEdgeIndex) -> ParentIndex {
        self.he_to_parent[he_id]
    }
    
    /// Gets the twin HalfEdge from an HalfEdge
    pub fn twin_from_he(&self, he_id: HalfEdgeIndex) -> HalfEdgeIndex {
        self.he_to_twin[he_id]
    }
    
    /// Gets the parents adjacent to another.
    /// This may have strange behaviours when used on a boundary.
    pub fn neighbors_from_parent(&self, parent_id: ParentIndex) -> Vec<ParentIndex> {
        self.he_from_parent(parent_id).into_iter().map(|he_id| self.he_to_parent[self.he_to_twin[he_id]]).collect()
    }
    
}
