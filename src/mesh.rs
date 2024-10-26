pub use nalgebra::Point2;
pub use cells::*;
pub use edges::*;
pub use neighbor::*;
pub use boundary::*;

pub mod cells;
pub mod edges;
pub mod neighbor;
pub mod boundary;

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