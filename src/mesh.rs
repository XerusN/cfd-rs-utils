pub use cells::*;
pub use edges::*;
pub use nalgebra::{Point2, Unit, Vector2};
pub use neighbor::*;
pub mod cells;
pub mod edges;
pub mod neighbor;

/// Represents a 2D mesh with cells (any type implementing the `Cell2D` trait), edges (`Edge2D`) and points (`Point2D`) informations.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshBlock2D<T: Cell2D> {
    nodes: Vec<Point2<f64>>,
    edges: Vec<Edge2D>,
    cells: Vec<T>,
}

impl<T: Cell2D> MeshBlock2D<T> {
    /// Creates a new `MeshBlock2D`.
    /// Takes ownership of data to prevent cloning (data structure are expected to be huge in standard context).
    /// It is recommanded to use this with empty vec, else your mesh isn't guaranteed to be correct, and multiple methods might panic.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    /// ```
    #[inline(always)]
    pub fn new(nodes: Vec<Point2<f64>>, edges: Vec<Edge2D>, cells: Vec<T>) -> MeshBlock2D<T> {
        MeshBlock2D {
            nodes,
            edges,
            cells,
        }
    }

    /// Gives an immutable reference to `self.nodes`
    pub fn nodes(&self) -> &Vec<Point2<f64>> {
        &self.nodes
    }

    /// Gives a mutable reference to `self.nodes`
    pub fn nodes_mut(&mut self) -> &mut Vec<Point2<f64>> {
        &mut self.nodes
    }

    /// Gives an immutable reference to `self.edges`
    pub fn edges(&self) -> &Vec<Edge2D> {
        &self.edges
    }

    /// Gives an immutable reference to `self.cells`
    pub fn cells(&self) -> &Vec<T> {
        &self.cells
    }

    /// Adds a node to `self.nodes`
    pub fn add_node(&mut self, node: Point2<f64>) {
        self.nodes.push(node);
    }

    /// Adds an edge to `self.edges`
    pub fn add_edge(&mut self, edge: Edge2D) {
        self.edges.push(edge);
    }

    /// Adds a cell to `self.cells`
    ///
    /// # Panics
    ///
    /// Panics if trying to produce a wrong cell (wrong number of edges, out of bound edges, cell on top of an other one)
    pub fn add_cell_edge(&mut self, cell_edges_idx: &[usize]) {
        assert!(
            cell_edges_idx.len() == T::edge_number(),
            "Wrong number of edges provided"
        );

        
        // checks if the new cell is not upon an other and determines the cell new neighbors
        let mut cell_neighbors: Vec<Neighbors> = Vec::new();
        
        let edge = cell_edges_idx[0];
        assert!(edge < self.edges.len());
        let mut place_found: [[Option<usize>; 2]; 3] = [[None; 2]; 3];
        for i in 0..self.edges[edge].parents.len() {
            let edge_neighbor = &self.edges[edge].parents[i];
            match edge_neighbor {
                Neighbors::None => {
                    if !place_found {
                        //self.edges[*edge].parents[i] = Neighbors::Cell(self.cells.len());
                        place_found[0][i] = Some(i);
                    } else {
                        cell_neighbors.push(edge_neighbor.clone());
                    }
                }
                Neighbors::Boundary(_) => cell_neighbors.push(edge_neighbor.clone()),
                Neighbors::Cell(_) => cell_neighbors.push(edge_neighbor.clone()),
            }
        }
        if !place_found {
            panic!("Trying to create a cell on top of another")
        }
        
        

        let cell = T::new_cell(cell_edges_idx, &cell_neighbors, &self.edges, &self.nodes);

        // Changes the edges parents
        for edge in cell_edges_idx {
            assert!(*edge < self.edges.len());
            for i in 0..self.edges[*edge].parents.len() {
                let edge_neighbor = &self.edges[*edge].parents[i];
                match edge_neighbor {
                    Neighbors::None => {
                        self.edges[*edge].parents[i] = Neighbors::Cell(self.cells.len());
                        break;
                    }
                    Neighbors::Boundary(_) => (),
                    Neighbors::Cell(_) => (),
                }
            }
        }

        self.cells.push(cell);
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
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
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
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let node = mesh.cell_nodes(1);
    /// ```
    #[inline(always)]
    pub fn cell_nodes(&self, cell_index: usize) -> Vec<&Point2<f64>> {
        self.cells[cell_index].nodes(&self.nodes)
    }

    /// Gives a mutable reference to nodes from the cell indicated by the index.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mut mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
    /// mesh.cell_nodes_mut(0)[1].x = 0.;
    /// assert_eq!(mesh.cell_nodes(0)[1], &Point2::<f64>::new(0.0, 3.0));
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
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mut mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let mut node = mesh.cell_nodes_mut(1);
    /// ```
    #[inline(always)]
    pub fn cell_nodes_mut(&mut self, cell_index: usize) -> Vec<&mut Point2<f64>> {
        self.cells[cell_index].nodes_mut(&mut self.nodes)
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
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
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
    /// let cells = vec![Triangle::new(&[0, 1, 2], &[Neighbors::None, Neighbors::None, Neighbors::None], &edges)];
    ///
    /// let mesh = MeshBlock2D::<Triangle>::new(nodes, edges, cells);
    ///
    /// let node = mesh.cell_edges(1);
    /// ```
    #[inline(always)]
    pub fn cell_edges(&self, cell_index: usize) -> Vec<&Edge2D> {
        self.cells[cell_index].edges(&self.edges)
    }

    // /// Ensures that no out of bound value is stored in the mesh, thus ensures that no panic will happen when calling nodes or
    // pub fn check_mesh(&self) -> Result<(), String> {
    //     let node_len = self.nodes.len();
    //     for edge in &self.edges {
    //         for node in edge.nodes_idx {
    //             if node > node_len {
    //                 return Err("An edge is containing an out of bound node".to_string());
    //             }
    //         }
    //     }
    //     for cell in &self.cells {}
    //     Ok(())
    // }
}
