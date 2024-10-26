pub use boundary::*;
pub use cells::*;
pub use edges::*;
pub use nalgebra::Point2;
pub use neighbor::*;

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
    is_finished: bool,
}

impl<T: Cell2D> MeshBlock2D<T> {
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

    /// Checks if the mesh is valid, mostly used internally. When using the API the mesh should be valid at any moment.
    pub fn check(&self) -> Result<(), String> {
        todo!()
    }

    /// Returns the current state of the mesh.
    /// Finished means that every cell has neighbors (either an other cell or a boundary).
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    /// Checks if the mesh is finished and change its status accordingly.
    /// If the mesh is not finished, returns an error.
    pub fn finish_mesh(&mut self) -> Result<(), String> {
        todo!();
    }

    pub fn edit_mesh(&mut self) {}

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
    pub fn cell_nodes(&self, cell_idx: usize) -> Option<&Vec<Point2<f64>>> {
        match self.cells.get(cell_idx) {
            None => None,
            Some(cell) => Some(cell.nodes(&self.nodes)),
        }
    }

    /// Returns an immutable reference to each edges of a cell.
    pub fn cell_edges(&self, cell_idx: usize) -> Option<&Vec<Edge2D>> {
        match self.cells.get(cell_idx) {
            None => None,
            Some(cell) => Some(cell.edges(&self.edges)),
        }
    }
}
