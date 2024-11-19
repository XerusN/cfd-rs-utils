use std::ops::Deref;

pub use boundary::*;
pub use cells::*;
pub use edges::*;
pub use nalgebra::Point2;
pub use neighbor::*;

use crate::{CellIndex, EdgeIndex, NodeIndex};

pub mod boundary;
pub mod cells;
pub mod edges;
pub mod neighbor;

/// Defines a mesh block (a mesh can be constructed with multiple mesh blocks) and all the fonctiannalities that should be implemented for it to be created, edited and used in a cfd contect.
/// The goal is that the API user won't be able to create segfault using this API.
/// The risk was present since (for example) cells are pointing to nodes through indices.
/// The use of f64 for coordinates is enforced since the code is meant to be used in scientific computations.
pub struct MeshBlock2D<T: Cell2D> {
    nodes: Vec<Point2<f64>>,
    edges: Vec<Edge2D>,
    cells: Vec<T>,
    boundaries: Vec<Boundary2D>,
    boundary_conditions: Vec<String>,
}

/// Safe struct for the mesh, guarantees that the mesh block is valid.
pub struct FinishedMeshBlock2D<T: Cell2D>(MeshBlock2D<T>);

impl<T: Cell2D> Deref for FinishedMeshBlock2D<T> {
    type Target = MeshBlock2D<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Enables to edit the mesh without the risk to passing it in a function that does not support an incomplete mesh.
pub struct EditableMeshBlock2D<T: Cell2D>(MeshBlock2D<T>);

impl<T: Cell2D> Deref for EditableMeshBlock2D<T> {
    type Target = MeshBlock2D<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Cell2D> MeshBlock2D<T> {
    /// Returns an immutable reference to each cell.
    pub fn cells(&self) -> &Vec<T> {
        &self.cells
    }

    /// Returns an immutable reference to each edge.
    pub fn edges(&self) -> &Vec<Edge2D> {
        &self.edges
    }

    /// Returns an immutable reference to each node.
    pub fn nodes(&self) -> &Vec<Point2<f64>> {
        &self.nodes
    }

    /// Returns an immutable reference to the boundaries
    pub fn boundaries(&self) -> &Vec<Boundary2D> {
        &self.boundaries
    }

    /// Returns an immutable reference to the boundary conditions.
    pub fn boundary_conditions(&self) -> &Vec<String> {
        &self.boundary_conditions
    }

    /// Returns an immutable reference to each nodes of a cell.
    pub fn cell_nodes(&self, cell_idx: CellIndex) -> Option<&Vec<Point2<f64>>> {
        match self.cells.get(*cell_idx) {
            None => None,
            Some(cell) => Some(cell.nodes(&self.nodes)),
        }
    }

    /// Returns an immutable reference to each edges of a cell.
    pub fn cell_edges(&self, cell_idx: CellIndex) -> Option<&Vec<Edge2D>> {
        match self.cells.get(*cell_idx) {
            None => None,
            Some(cell) => Some(cell.edges(&self.edges)),
        }
    }
    
    pub fn cell(&self, cell_idx: CellIndex) -> Option<&T> {
        self.cells.get(*cell_idx)
    }
    
    pub fn edge(&self, edge_idx: EdgeIndex) -> Option<&Edge2D> {
        self.edges.get(*edge_idx)
    }
    
    pub fn node(&self, node_idx: NodeIndex) -> Option<&Point2<f64>> {
        self.nodes.get(*node_idx)
    }
}

impl<T: Cell2D> FinishedMeshBlock2D<T> {
    /// Enables to edit the mesh
    pub fn edit(self) -> EditableMeshBlock2D<T> {
        todo!()
    }
}


impl<T: Cell2D> EditableMeshBlock2D<T> {
    /// Creates a new instance of mesh with only boundaries.
    pub fn new_from_boundaries(
        nodes: Vec<Point2<f64>>,
        edges: Vec<Edge2D>,
        boundaries: Vec<Boundary2D>,
        boundary_conditions: Vec<String>,
    ) -> Result<Self, String> {
        todo!()
    }

    /// Creates a new instance of mesh (completed mesh only, used to create mesh from file import).
    pub fn new_from_import(
        nodes: Vec<Point2<f64>>,
        edges: Vec<Edge2D>,
        cells: Vec<T>,
        boundaries: Vec<Boundary2D>,
        boundary_conditions: Vec<String>,
    ) -> Result<Self, String> {
        todo!()
    }
    
    /// Checks if the mesh is finished and change its status accordingly.
    /// If the mesh is not finished, returns an error.
    pub fn finish(self) -> Result<FinishedMeshBlock2D<T>, String> {
        match self.check() {
            Err(e) => Err(e),
            Ok(_) => Ok(FinishedMeshBlock2D(self.0)),
        }
    }
    
    /// When doing this, you are not checking the mesh before converting it.
    /// You should be aware that any badly defined mesh might result in undefinied behaviour when using FinishedMeshBlock2D.
    pub unsafe fn finish_without_check(self) -> FinishedMeshBlock2D<T> {
        FinishedMeshBlock2D(self.0)
    }
    
    /// Checks if the mesh is valid, mostly used internally. When using the safe API the mesh should be valid at any moment.
    pub fn check(&self) -> Result<(), String> {
        todo!()
    }
    
}