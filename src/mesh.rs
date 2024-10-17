pub use cells::*;
pub use edges::*;
pub use neighbor::*;
pub use points::*;
pub use vectors::*;
pub mod cells;
pub mod edges;
pub mod neighbor;
pub mod points;
pub mod vectors;

/// Represents a 2D mesh with cells (any type implementing the `Cell2D` trait), edges (`Edge2D`) and points (`Point2D`) informations.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh2D<T: Cell2D> {
    pub nodes: Vec<Point2D>,
    pub edges: Vec<Edge2D>,
    pub cells: Vec<T>,
}

impl<T: Cell2D> Mesh2D<T> {
    /// Creates a new `Mesh2D`.
    /// Takes ownership of data to prevent cloning (data structure are expected to be huge in standard context).
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2D::new(0.0, 1.0), Point2D::new(1.0, 3.0), Point2D::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None])];
    ///
    /// let mesh = Mesh2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let nodes = vec![Point2D::new(0.0, 1.0), Point2D::new(1.0, 3.0), Point2D::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None])];
    ///
    /// assert_eq!(mesh, Mesh2D::<Triangle> {nodes, edges, cells,});
    /// ```
    #[inline(always)]
    pub fn new(nodes: Vec<Point2D>, edges: Vec<Edge2D>, cells: Vec<T>) -> Mesh2D<T> {
        Mesh2D {
            nodes,
            edges,
            cells,
        }
    }
}
