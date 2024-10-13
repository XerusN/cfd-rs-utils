use cells::{triangle::MeshTriangle, Cell2D};
pub use edges::Edge2D;
pub use points::Point2D;
pub use vectors::Vector2D;

pub mod cells;
pub mod edges;
pub mod neighbor;
pub mod points;
pub mod vectors;

pub struct Mesh2D<T: Cell2D> {
    pub points: Vec<Point2D>,
    pub edges: Vec<Edge2D>,
    pub cells: Vec<T>,
}
