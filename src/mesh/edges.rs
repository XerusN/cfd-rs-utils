use nalgebra::{center, Point2, Vector2};

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

    /// Gives the `Point2`s struct to which nodes_idx are indexing.
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
    /// The program panics if the indices in `Edge2D` are out of bound in nodes.
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

    /// Gives the `Point2` struct to which the first index is indexing.
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

    /// Creates an owned `Vector2` instance from an edge.
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
    /// The program panics if one of the indices in Edge2D is out of bound in nodes.
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

    /// Computes the center of the edge.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(0, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.0, 3.0)];
    ///
    /// assert_eq!(a.center(&nodes), Point2::<f64>::new(0.0, 2.0));
    /// ```
    ///
    /// # Panics
    ///
    /// The program panics if one of the indices in Edge2D is out of bound in nodes.
    ///
    /// ```should_panic
    /// use cfd_rs_utils::*;
    ///
    /// let a = mesh::Edge2D::new(2, 1);
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(0.05, 3.0)];
    ///
    /// a.center(&nodes);
    /// ```
    #[inline(always)]
    pub fn center(&self, nodes: &[Point2<f64>]) -> Point2<f64> {
        center(&nodes[self.nodes_idx[1]], &nodes[self.nodes_idx[0]])
    }
    
    /// Ensures that the edge is properly defined (no out of bound value or duplicated nodes)
    pub fn check(&self, nodes: &[Point2<f64>]) -> Result<(), String> {
        for node in self.nodes_idx {
            if node >= nodes.len() {
                return Err(format!("Node {node} out of bound in edge"))
            }
        }
        if self.nodes_idx[0] == self.nodes_idx[1] {
            let node = self.nodes_idx[0];
            return Err(format!("Both nodes have the same index {node}"))
        }
        Ok(())
    }
    
    
}