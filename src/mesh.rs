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
    finished: bool,
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
    
    /// Checks if the mesh is valid, mostly used internally.
    /// Quite expensive operation.
    pub fn check(&self) -> Result<(), String> {
        todo!()
    }
    
    
    
}
