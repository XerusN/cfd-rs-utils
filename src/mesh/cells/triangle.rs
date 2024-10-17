use super::{Cell2D, Neighbors};
use super::{Edge2D, Point2D, Vector2D};

/// Represents a triangle.
/// edges and nodes gives the indices of the data in the corresponding array.
/// The nodes data may seem like a duplication but it appeared to me that using 
/// `Neighbors` tells if the triangle has no neighnour, is a boundary cell, or gives the indices of the neighboring cells.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangle {
    pub edges_idx: [usize; 3],
    pub nodes_idx: [usize; 3],
    pub neighbors: [Neighbors; 3],
}

impl Triangle {
    /// Creates a simple new instance of `Triangle`.
    /// `edges` refers to the array containing all edges (not only for this `Triangle`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None]);
    /// let b = Triangle { edges_idx: [0, 1, 2], neighbors: [Neighbors::None, Neighbors::None, Neighbors::None], };
    ///
    /// assert_eq!(a, b);
    /// ```
    #[inline(always)]
    pub fn new(edges_idx: [usize; 3], neighbors: [Neighbors; 3], edges: &[Edge2D]) -> Triangle {
        let mut triangle = Triangle {
            edges_idx,
            nodes_idx: [0; 3],
            neighbors,
        };
        triangle.update_nodes_idx_from_edges(edges);
        triangle
    }

    /// Updates `nodes_idx` using `edges`
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2D::new(0.0, 1.0), Point2D::new(1.0, 3.0), Point2D::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let mut triangle = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges)
    /// 
    /// assert_eq!(triangle.nodes_idx[0] == 0);
    /// 
    /// triangle.edges_idx = [1, 2, 0];
    /// triangle.update_nodes_idx_from_edges(edges);
    ///
    /// assert_eq!(triangle.nodes_idx[0] == 1);
    /// ```
    pub fn update_nodes_idx_from_edges(&mut self, edges: &[Edge2D]) {
        let first_node = edges[self.edges_idx[0]].nodes_idx[0];
        let second_node = if edges[self.edges_idx[1]].nodes_idx[0] == first_node {
            edges[self.edges_idx[1]].nodes_idx[1]
        } else {
            edges[self.edges_idx[1]].nodes_idx[0]
        };
        let third_node = if (edges[self.edges_idx[2]].nodes_idx[0] == first_node)
            & (edges[self.edges_idx[2]].nodes_idx[0] == second_node)
        {
            panic!("Triangle is badly constructed, it only has two nodes")
        } else if (edges[self.edges_idx[2]].nodes_idx[0] == first_node)
            | (edges[self.edges_idx[2]].nodes_idx[0] == second_node)
        {
            edges[self.edges_idx[2]].nodes_idx[1]
        } else {
            edges[self.edges_idx[2]].nodes_idx[0]
        };
        self.nodes_idx = [first_node, second_node, third_node];
    }

    /// Gives a reference to the `Edge2D` data from each edge of the `Triangle`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    ///
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None]);
    /// let b = [&edges[0], &edges[1], &edges[2]];
    ///
    /// assert_eq!(a.edges(&edges), b);
    /// ```
    pub fn edges<'a>(&self, edges: &'a [Edge2D]) -> [&'a Edge2D; 3] {
        [
            &edges[self.edges_idx[0]],
            &edges[self.edges_idx[1]],
            &edges[self.edges_idx[2]],
        ]
    }
}

impl Cell2D for Triangle {
    /// Compute the surface of the 2D cell
    #[inline(always)]
    fn area(&self, nodes: &[Point2D]) -> f64 {
        let nodes = self.nodes(nodes);
        (0.5 * (-nodes[1].y * nodes[2].x
            + nodes[0].y * (-nodes[1].x + nodes[2].x)
            + nodes[0].x * (nodes[1].y - nodes[2].y)
            + nodes[1].x * nodes[2].y))
            .abs()
    }

    /// Computes the center of the cell
    #[inline(always)]
    fn center(&self, nodes: &[Point2D]) -> Point2D {
        let nodes = self.nodes(nodes);
        &(&(nodes[0] + nodes[1]) + nodes[2]) / 3.0
    }

    /// Computes the normals to each edge
    #[inline(always)]
    fn normals(&self, edges: &[Edge2D], nodes: &[Point2D]) -> Vec<Vector2D> {
        let edges = self.edges(edges);
        vec![
            edges[0].to_vector(nodes),
            edges[1].to_vector(nodes),
            edges[2].to_vector(nodes),
        ]
    }

    /// Gives each node of the cell
    #[inline(always)]
    fn nodes<'a>(&self, nodes: &'a [Point2D]) -> Vec<&'a Point2D> {
        vec![&nodes[self.nodes_idx[0]], &nodes[self.nodes_idx[1]], &nodes[self.nodes_idx[2]]]
    }
}
