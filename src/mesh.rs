use cells::Cell2D;
pub use edges::Edge2D;
pub use points::Point2D;
pub use vectors::Vector2D;

pub mod cells;
pub mod edges;
pub mod neighbor;
pub mod points;
pub mod vectors;

struct Mesh2D<T: Cell2D> {
    points: Vec<Point2D>,
    edges: Vec<Edge2D>,
    cells: Vec<T>,
}
