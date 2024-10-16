pub use cells::*;
pub use edges::*;
pub use nalgebra::{Point2, Vector2, Unit};
pub use neighbor::*;
pub mod cells;
pub mod edges;
pub mod neighbor;

/// Represents a 2D mesh with cells (any type implementing the `Cell2D` trait), edges (`Edge2D`) and points (`Point2D`) informations.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh2D<T: Cell2D> {
    pub nodes: Vec<Point2<f64>>,
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
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = Mesh2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// assert_eq!(mesh, Mesh2D::<Triangle> {nodes, edges, cells,});
    /// ```
    #[inline(always)]
    pub fn new(nodes: Vec<Point2<f64>>, edges: Vec<Edge2D>, cells: Vec<T>) -> Mesh2D<T> {
        Mesh2D {
            nodes,
            edges,
            cells,
        }
    }

    /// Gives the nodes from the cell indicated by the index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = Mesh2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// assert_eq!(mesh.cell_nodes(0)[0], &Point2::<f64>::new(0.0, 1.0));
    /// ```
    ///
    /// # Panics
    ///
    /// If the `cell_index` is out of bound in `self.cells` it will panic.
    ///
    /// ```rust, should_panic
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = Mesh2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let node = mesh.cell_nodes(1);
    /// ```
    #[inline(always)]
    pub fn cell_nodes(&self, cell_index: usize) -> Vec<&Point2<f64>> {
        self.cells[cell_index].nodes(&self.nodes)
    }

    #[inline(always)]
    pub fn cell_edges(&self, cell_index: usize) -> Vec<&Edge2D> {
        self.cells[cell_index].edges(&self.edges)
    }
}
