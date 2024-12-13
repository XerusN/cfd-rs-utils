use super::Error;

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
#[derive(Debug, Clone, Default)]
struct MeshBlock2D<T: Cell2D> {
    nodes: Vec<Point2<f64>>,
    edges: Vec<Edge2D>,
    cells: Vec<T>,
    boundaries: Vec<Boundary2D>,
    boundary_conditions: Vec<String>,
}

/// Safe struct for the mesh, guarantees that the mesh block is valid.
#[derive(Debug, Clone)]
pub struct FinishedMeshBlock2D<T: Cell2D>(MeshBlock2D<T>);

/// Enables to edit the mesh without the risk to passing it in a function that does not support an incomplete mesh.
#[derive(Debug, Clone, Default)]
pub struct EditableMeshBlock2D<T: Cell2D>(MeshBlock2D<T>);

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
    pub fn cell_nodes(&self, cell_idx: CellIndex) -> Option<Vec<&Point2<f64>>> {
        match self.cells.get(*cell_idx) {
            None => None,
            Some(cell) => Some(cell.nodes(&self.nodes)),
        }
    }

    /// Returns an immutable reference to each edges of a cell.
    pub fn cell_edges(&self, cell_idx: CellIndex) -> Option<Vec<&Edge2D>> {
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
        unsafe {
            EditableMeshBlock2D::new_uncheck(
                self.0.nodes,
                self.0.edges,
                self.0.cells,
                self.0.boundaries,
                self.0.boundary_conditions,
            )
        }
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

    // /// Creates a new instance of mesh (completed mesh only, used to create mesh from file import).
    // pub fn new_from_import(
    //     nodes: Vec<Point2<f64>>,
    //     edges: Vec<Edge2D>,
    //     cells: Vec<T>,
    //     boundaries: Vec<Boundary2D>,
    //     boundary_conditions: Vec<String>,
    // ) -> Result<Self, String> {
    //     todo!()
    // }

    /// Creates a new ediatble mesh, no topology check.
    /// 
    /// # Safety
    /// 
    /// Can produce undefined behaviours later if the mesh topology is not right.
    pub unsafe fn new_uncheck(
        nodes: Vec<Point2<f64>>,
        edges: Vec<Edge2D>,
        cells: Vec<T>,
        boundaries: Vec<Boundary2D>,
        boundary_conditions: Vec<String>,
    ) -> Self {
        EditableMeshBlock2D(MeshBlock2D {
            nodes,
            edges,
            cells,
            boundaries,
            boundary_conditions,
        })
    }

    /// Checks if the mesh is finished and change its status accordingly.
    /// If the mesh is not finished, returns an error.
    /// Checks the topology.
    pub fn finish(mut self) -> Result<FinishedMeshBlock2D<T>, Error> {
        match self.check() {
            Err(e) => Err(e),
            Ok(_) => {
                self.0
                    .edges
                    .iter_mut()
                    .for_each(|edge| edge.update_neighbors());
                self.0.cells.iter_mut().for_each(|cell| cell.update_nodes());
                Ok(FinishedMeshBlock2D(self.0))
            }
        }
    }

    /// When doing this, you are not checking the mesh before converting it.
    /// You should be aware that any badly defined mesh might result in undefinied behaviour when using the value later.
    pub unsafe fn finish_without_check(self) -> FinishedMeshBlock2D<T> {
        FinishedMeshBlock2D(self.0)
    }

    /// Checks if the mesh is valid, mostly used internally. When using the safe API the mesh should be valid at any moment.
    /// Checks the topology.
    pub fn check(&self) -> Result<(), Error> {
        todo!()
    }

    /// Add a cell to the mesh.
    /// Will return an error if the edges are not contiguous.
    pub fn add_cells(&mut self, edges_idx: Vec<&[EdgeIndex]>) -> Result<(), Error> {
        todo!()
    }

    pub fn add_edges(&mut self, edges: Vec<Edge2D>) -> Result<(), Error> {
        todo!()
    }

    pub fn add_nodes(&mut self, nodes: Vec<Point2<f64>>) {
        todo!()
    }

    pub fn change_cell(
        &mut self,
        cell_idx: CellIndex,
        edges: Option<&[EdgeIndex]>,
    ) -> Result<(), Error> {
        todo!()
    }

    ///The above cells should be removed first
    pub unsafe fn remove_edge(&mut self, edge_idx: EdgeIndex) -> Result<(), Error> {
        todo!()
    }

    pub fn remove_cell(&mut self, cell_idx: EdgeIndex) -> Result<(), Error> {
        todo!()
    }

    /// The above cells and edges should be removed first.
    /// Quite long operation since the node removal swaps the last node of the array, thus requiring to adjust the indices in the above cells.
    pub unsafe fn remove_node(&mut self, node_idx: NodeIndex) -> Result<(), Error> {
        todo!()
    }

    /// Swaps the contact edge in order to have it connect to a
    pub fn swap_edge(&mut self, cells_idx: (CellIndex, CellIndex)) -> Result<(), Error> {
        todo!()
    }
}
