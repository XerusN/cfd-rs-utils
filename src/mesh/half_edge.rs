use crate::{boundary::Boundary, errors::MeshError};
use indices::*;
use nalgebra::Point2;

pub mod indices;

#[cfg(test)]
mod test;

/// Parent of a half_edge, either a cell or a boundary.
/// ```None``` is meant as an error or temporary state.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Parent {
    #[default]
    None,
    Cell(usize),
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
    /// Expects the edges to be sorted (the next edge is starting with the same vertex as the last from the previous edge)
    /// Parents designates the parent at the right side from an edge
    ///
    /// Failing to comply with those invariants might result in unexpected behaviours.
    /// This function is done in a simple version for testing purpose, but will be changed in the future.
    /// If you have any suggestion, do not hesitate to reach out, send an issue or provide a fix.
    ///
    /// # Safety
    ///
    /// The function is marked as unsafe to warn about the very unstable API and the very specific input needed.
    pub unsafe fn new_from_boundary(
        vertices: Vec<Point2<f64>>,
        edge_to_vertices_and_parent: Vec<(VertexIndex, VertexIndex, ParentIndex)>,
        parents: Vec<Parent>,
    ) -> Self {
        let mut parents = parents;

        let mut he_to_vertex =
            Vec::<VertexIndex>::with_capacity(edge_to_vertices_and_parent.len() * 2);
        let mut he_to_twin =
            Vec::<HalfEdgeIndex>::with_capacity(edge_to_vertices_and_parent.len() * 2);
        let mut he_to_next_he =
            Vec::<HalfEdgeIndex>::with_capacity(edge_to_vertices_and_parent.len() * 2);
        let mut he_to_prev_he =
            Vec::<HalfEdgeIndex>::with_capacity(edge_to_vertices_and_parent.len() * 2);
        let mut he_to_parent =
            Vec::<ParentIndex>::with_capacity(edge_to_vertices_and_parent.len() * 2);

        let mut parent_to_first_he = Vec::<HalfEdgeIndex>::with_capacity(parents.len() + 1);

        // All arrays are needed to be built correctly

        let cell = ParentIndex(parents.len());
        parents.push(Parent::Cell(0));

        let mut prev_he_1 = HalfEdgeIndex(0);
        let mut prev_he_2 = HalfEdgeIndex(0);

        for (i, edge) in edge_to_vertices_and_parent.iter().enumerate() {
            let new_he_1 = HalfEdgeIndex(he_to_vertex.len());
            let new_he_2 = HalfEdgeIndex(he_to_vertex.len() + 1);
            he_to_vertex.push(edge.0);
            he_to_vertex.push(edge.1);
            he_to_twin.push(new_he_2);
            he_to_twin.push(new_he_1);

            he_to_parent.push(cell);
            he_to_parent.push(edge.2);

            he_to_next_he.push(HalfEdgeIndex(0));
            he_to_next_he.push(HalfEdgeIndex(0));
            he_to_prev_he.push(HalfEdgeIndex(0));
            he_to_prev_he.push(HalfEdgeIndex(0));

            if i == 0 {
                prev_he_1 = new_he_1;
                prev_he_2 = new_he_2;
                continue;
            }

            he_to_next_he[new_he_2] = prev_he_2;
            he_to_prev_he[new_he_1] = prev_he_1;

            he_to_next_he[prev_he_1] = new_he_1;
            he_to_prev_he[prev_he_2] = new_he_2;

            prev_he_1 = new_he_1;
            prev_he_2 = new_he_2;
        }

        he_to_next_he[HalfEdgeIndex(1)] = prev_he_2;
        he_to_prev_he[HalfEdgeIndex(0)] = prev_he_1;

        he_to_next_he[prev_he_1] = HalfEdgeIndex(0);
        he_to_prev_he[prev_he_2] = HalfEdgeIndex(1);

        for i in 0..parents.len() {
            let current_id = ParentIndex(i);
            for (j, he_parent) in he_to_parent.iter().enumerate() {
                if current_id == *he_parent {
                    parent_to_first_he.push(HalfEdgeIndex(j));
                    break;
                }
            }
        }

        Mutable2DMesh {
            vertices,

            he_to_vertex,
            he_to_twin,
            he_to_next_he,
            he_to_prev_he,
            he_to_parent,

            parents,
            parent_to_first_he,
        }
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
            //println!("{:?}, {:?}", current_he, first_he);
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

    /// Check that the mesh topology is valid.
    /// Used to confirm the topology before switching to an immutable mesh and for test purpose.
    ///
    /// This function aims to be exhaustive but obviously it is not.
    /// For now it mostly checks coherence between arrays and that not indices are out of bound.
    ///
    /// If you find some cases not covered by this function, don't hesitate to submit a pull request or detail what you would like implemented.
    fn check_mesh(&self) -> Result<(), MeshError> {
        //Checks length coherence between HalfEdges arrays
        assert_eq!(self.he_to_next_he.len(), self.he_to_twin.len());
        assert_eq!(self.he_to_prev_he.len(), self.he_to_twin.len());
        assert_eq!(self.he_to_vertex.len(), self.he_to_twin.len());
        assert_eq!(self.he_to_parent.len(), self.he_to_twin.len());

        for vertex in &self.he_to_vertex {
            if *vertex >= VertexIndex(self.vertices.len()) {
                return Err(MeshError::VertexIndexOutOfBound {
                    got: *vertex,
                    len: self.vertices.len(),
                });
            }
        }

        for he in &self.he_to_twin {
            if *he >= HalfEdgeIndex(self.he_to_vertex.len()) {
                return Err(MeshError::HalfEdgeIndexOutOfBound {
                    got: *he,
                    len: self.he_to_vertex.len(),
                });
            }
        }

        for (i, he) in self.he_to_twin.iter().enumerate() {
            if self.he_to_twin[*he] != HalfEdgeIndex(i) {
                return Err(MeshError::TwinNotCorrect {
                    he: HalfEdgeIndex(i),
                    he_twin: *he,
                    he_twin_twin: self.he_to_twin[*he],
                });
            }
        }

        for he in &self.he_to_next_he {
            if *he >= HalfEdgeIndex(self.he_to_vertex.len()) {
                return Err(MeshError::HalfEdgeIndexOutOfBound {
                    got: *he,
                    len: self.he_to_vertex.len(),
                });
            }
        }

        for he in &self.he_to_prev_he {
            if *he >= HalfEdgeIndex(self.he_to_vertex.len()) {
                return Err(MeshError::HalfEdgeIndexOutOfBound {
                    got: *he,
                    len: self.he_to_vertex.len(),
                });
            }
        }

        // Simple check from prev and next
        for (i, next) in self.he_to_next_he.iter().enumerate() {
            let he = HalfEdgeIndex(i);
            if he != self.he_to_prev_he[*next] {
                return Err(MeshError::NextPrevNotCorrect {
                    he,
                    he_next: *next,
                    he_next_prev: self.he_to_prev_he[*next],
                });
            }
        }

        // Might be redundant with the previous check
        for i in 0..self.he_to_vertex.len() {
            let origin = HalfEdgeIndex(i);
            let mut j = 0;
            let mut current_he = self.he_to_next_he[origin];
            while current_he != origin {
                current_he = self.he_to_next_he[current_he];
                j += 1;
                // len/2 could work
                if j >= self.he_to_vertex.len() {
                    break;
                }
            }
        }

        for parent in &self.he_to_parent {
            if *parent >= ParentIndex(self.parents.len()) {
                return Err(MeshError::ParentIndexOutOfBound {
                    got: *parent,
                    len: self.parents.len(),
                });
            }
        }

        for i in 0..self.parents.len() {
            let parent = ParentIndex(i);
            for he in self.he_from_parent(parent) {
                if self.parent_from_he(he) != parent {
                    return Err(MeshError::ParentNotCorrect {
                        parent,
                        he,
                        he_parent: self.parent_from_he(he),
                    });
                }
            }
        }

        Ok(())
    }

    /// Creates a new vertex on an half edge at a distance of ```distance_ratio``` (between 0. and 1.) the HalfEdge length
    pub fn split_edge(
        &mut self,
        he_id: HalfEdgeIndex,
        distance_ratio: f64,
    ) -> Result<(), MeshError> {
        if (distance_ratio >= 1.0) | (distance_ratio <= 0.0) {
            return Err(MeshError::WrongFloatValue {
                got: distance_ratio,
                expected: (0.0, 1.0),
            });
        }

        let new_vertex_id = VertexIndex(self.vertices.len());
        let new_vertex_pos: Point2<f64> = {
            let edge_vertices = self.vertices_from_he(he_id);
            let edge_vertices = (
                self.vertices[edge_vertices[0]],
                self.vertices[edge_vertices[1]],
            );
            edge_vertices.0.lerp(&edge_vertices.1, distance_ratio)
        };

        let he_ids = (he_id, self.twin_from_he(he_id));

        self.vertices.push(new_vertex_pos);

        let new_he_ids = (
            HalfEdgeIndex(self.he_to_twin.len()),
            HalfEdgeIndex(self.he_to_twin.len() + 1),
        );

        self.he_to_vertex.push(new_vertex_id);
        self.he_to_vertex.push(new_vertex_id);

        self.he_to_twin.push(he_ids.1);
        self.he_to_twin.push(he_ids.0);
        self.he_to_twin[he_ids.0] = new_he_ids.1;
        self.he_to_twin[he_ids.1] = new_he_ids.0;

        let next = self.next_he_from_he(he_ids.0);
        self.he_to_next_he[he_ids.0] = new_he_ids.0;
        self.he_to_next_he.push(next);
        self.he_to_prev_he[next] = new_he_ids.0;
        let next = self.next_he_from_he(he_ids.1);
        self.he_to_next_he[he_ids.1] = new_he_ids.1;
        self.he_to_next_he.push(next);
        self.he_to_prev_he[next] = new_he_ids.1;

        self.he_to_prev_he.push(he_ids.0);
        self.he_to_prev_he.push(he_ids.1);

        self.he_to_parent.push(self.parent_from_he(he_ids.0));
        self.he_to_parent.push(self.parent_from_he(he_ids.1));

        Ok(())
    }
}
