use crate::{boundary::Boundary, errors::MeshError};
use indices::*;
use nalgebra::{distance, Point2};

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
    /// Creates a mesh with only boundaries defined.
    pub fn new_from_boundary(
        vertices: Vec<Point2<f64>>,
        he_to_vertex: Vec<VertexIndex>,
        parents: Vec<Parent>,
    ) -> Self {
        // All arrays are needed to be built correctly
        todo!()
    }

    /// Gets all the vertices of an half-edge.
    pub fn vertices_from_he(&self, he_id: HalfEdgeIndex) -> [VertexIndex; 2] {
        [
            self.he_to_vertex[he_id],
            self.he_to_vertex[self.he_to_twin[he_id]],
        ]
    }

    /// Gets all half-edges from a parent (Cell or boundary).
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

    /// Gets all vertices from a parent (Cell or boundary).
    pub fn vertices_from_parent(&self, parent_id: ParentIndex) -> Vec<VertexIndex> {
        self.he_from_parent(parent_id)
            .into_iter()
            .map(|he_id| self.he_to_vertex[he_id])
            .collect()
    }

    /// Gets the parent from an HalfEdge.
    pub fn parent_from_he(&self, he_id: HalfEdgeIndex) -> ParentIndex {
        self.he_to_parent[he_id]
    }

    /// Gets the twin HalfEdge from an HalfEdge.
    pub fn twin_from_he(&self, he_id: HalfEdgeIndex) -> HalfEdgeIndex {
        self.he_to_twin[he_id]
    }

    /// Gets the next HalfEdge from an HalfEdge.
    pub fn next_he_from_he(&self, he_id: HalfEdgeIndex) -> HalfEdgeIndex {
        self.he_to_next_he[he_id]
    }

    /// Gets the previous HalfEdge from an HalfEdge.
    pub fn prev_he_from_he(&self, he_id: HalfEdgeIndex) -> HalfEdgeIndex {
        self.he_to_prev_he[he_id]
    }

    /// Gets the parents adjacent to another.
    /// This may have strange behaviours when used on a boundary.
    pub fn neighbors_from_parent(&self, parent_id: ParentIndex) -> Vec<ParentIndex> {
        self.he_from_parent(parent_id)
            .into_iter()
            .map(|he_id| self.he_to_parent[self.he_to_twin[he_id]])
            .collect()
    }

    /// Gets the parent properties from its index.
    pub fn parent_from_index(&self, parent_id: ParentIndex) -> &Parent {
        &self.parents[parent_id]
    }

    /// Gets a mutable reference to a vertex from its index.
    pub fn vertex_mut_from_index(&mut self, vertex_id: VertexIndex) -> &mut Point2<f64> {
        &mut self.vertices[vertex_id]
    }

    /// Gets a mutable reference to the parent properties from its index.
    pub fn parent_mut_from_index(&mut self, parent_id: ParentIndex) -> &mut Parent {
        &mut self.parents[parent_id]
    }
    
    /// Creates a new vertex on an half edge at a distance of ```distance_ratio``` (between 0. and 1.) the HalfEdge length
    pub fn split_edge(&mut self, he_id: HalfEdgeIndex, distance_ratio: f64) -> Result<(), MeshError>{
        if (distance_ratio >= 1.0) | (distance_ratio <= 0.0) {
            return Err(MeshError::WrongFloatValue { got: distance_ratio, expected: (0.0, 1.0) });
        }
        
        let new_vertex_id = VertexIndex(self.vertices.len());
        let new_vertex_pos: Point2<f64> = {
            let edge_vertices = self.vertices_from_he(he_id);
            let edge_vertices = (self.vertices[edge_vertices[0]], self.vertices[edge_vertices[1]]);
            edge_vertices.0.lerp(&edge_vertices.1, distance_ratio)
        };
        
        let he_ids = (he_id, self.twin_from_he(he_id));
        
        self.vertices.push(new_vertex_pos);
        
        let new_he_ids = (HalfEdgeIndex(self.he_to_twin.len()), HalfEdgeIndex(self.he_to_twin.len() + 1));
        
        self.he_to_vertex.push(new_vertex_id);
        self.he_to_vertex.push(new_vertex_id);
        
        self.he_to_twin.push(he_ids.1);
        self.he_to_twin.push(he_ids.0);
        self.he_to_twin[he_ids.0] = new_he_ids.1;
        self.he_to_twin[he_ids.1] = new_he_ids.0;
        
        let next = self.next_he_from_he(he_ids.0);
        self.he_to_next_he[he_ids.0] = new_he_ids.0;
        self.he_to_next_he.push(next);
        let next = self.next_he_from_he(he_ids.1);
        self.he_to_next_he[he_ids.1] = new_he_ids.1;
        self.he_to_next_he.push(next);
        
        self.he_to_prev_he.push(he_ids.0);
        self.he_to_prev_he.push(he_ids.1);
        
        self.he_to_parent.push(self.parent_from_he(he_ids.0));
        self.he_to_parent.push(self.parent_from_he(he_ids.1));
        
        Ok(())
    }
    
}
