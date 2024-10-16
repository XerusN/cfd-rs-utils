use cells::Cell2D;
pub use edges::Edge2D;
pub use points::Point2D;
pub use vectors::Vector2D;

pub mod cells;
pub mod edges;
pub mod neighbor;
pub mod points;
pub mod vectors;

/// Represents a 2D mesh with cells, edges and points informations
pub struct Mesh2D<T: Cell2D> {
    pub nodes: Vec<Point2D>,
    pub edges: Vec<Edge2D>,
    pub cells: Vec<T>,
}

impl<T: Cell2D> Mesh2D<T> {
    /// Creates a new mesh.
    /// Takes ownership of data to prevent cloning
    pub fn new(nodes: Vec<Point2D>, edges: Vec<Edge2D>, cells: Vec<T>) -> Mesh2D<T> {
        Mesh2D {
            nodes,
            edges,
            cells,
        }
    }
}
