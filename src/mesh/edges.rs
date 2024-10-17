use nalgebra::{Point2, Vector2};

/// Struct used to describe edges in 2D.
/// It is intended to be used as part of a mesh so it keeps the indices for the 2 nodes and eventually the indices of the parent cells.
/// Only implemented for 2D since it needs more parent cell than 2 in 3D.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Edge2D {
    pub nodes_idx: [usize; 2],
    pub parent_cells_idx: [Option<usize>; 2],
}

impl Edge2D {
    /// Creates a simple new instance of `Edge2D`.
    /// Does not support parent cells indices, often not needed when creating an edge.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(0, 1);
    /// let b = mesh::Edge2D {
    ///     nodes_idx: [0, 1],
    ///     parent_cells_idx: [None; 2],
    /// };
    ///
    /// assert_eq!(a, b);
    /// ```
    #[inline(always)]
    pub fn new(node_idx: usize, other_node_idx: usize) -> Edge2D {
        Edge2D {
            nodes_idx: [node_idx, other_node_idx],
            parent_cells_idx: [None; 2],
        }
    }

    /// Gives the Point2Ds struct to which nodes_idx are indexing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(0, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.05, 3.0)];
    ///
    /// assert_eq!(a.nodes(&nodes), [&Point2::<f64>::new(0.0, 1.0), &Point2::<f64>::new(0.05, 3.0)]);
    /// ```
    ///
    /// # Panics
    ///
    /// The program panics if the indices in Edge2D are out of bound in nodes.
    ///
    /// ```should_panic
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(0, 2);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.05, 3.0)];
    ///
    /// a.nodes(&nodes);
    /// ```
    #[inline(always)]
    pub fn nodes<'a>(&self, nodes: &'a [Point2<f64>]) -> [&'a Point2<f64>; 2] {
        [&nodes[self.nodes_idx[0]], &nodes[self.nodes_idx[1]]]
    }

    /// Gives the Point2D struct to which the first index is indexing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(0, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.05, 3.0)];
    ///
    /// assert_eq!(a.first_node(&nodes), &Point2::<f64>::new(0.0, 1.0));
    /// ```
    ///
    /// # Panics
    ///
    /// The program panics if the first index in Edge2D is out of bound in nodes.
    ///
    /// ```should_panic
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(2, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.05, 3.0)];
    ///
    /// a.first_node(&nodes);
    /// ```
    #[inline(always)]
    pub fn first_node<'a>(&self, nodes: &'a [Point2<f64>]) -> &'a Point2<f64> {
        &nodes[self.nodes_idx[0]]
    }

    /// Creates an owned Vector2D instance from an edge.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(0, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0)];
    ///
    /// assert_eq!(a.to_vector(&nodes), Vector2::<f64>::new(1.0, 2.0));
    /// ```
    ///
    /// # Panics
    ///
    /// The program panics if one of the index in Edge2D is out of bound in nodes.
    ///
    /// ```should_panic
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(2, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.05, 3.0)];
    ///
    /// a.to_vector(&nodes);
    /// ```
    #[inline(always)]
    pub fn to_vector(&self, nodes: &[Point2<f64>]) -> Vector2<f64> {
        nodes[self.nodes_idx[1]] - nodes[self.nodes_idx[0]]
    }
}
