use super::{Cell2D, Neighbors};
use super::{Edge2D, Point2D, Vector2D};

/// Represents a triangle.
/// Nodes gives the indices of the nodes in the corresponding array.
/// `Neighbors` tells if the triangle has no neighnour, is a boundary cell, or gives the indices of the neighboring cells.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangle {
    pub edges_idx: [usize; 3],
    pub neighbors: [Neighbors; 3],
}

impl Triangle {
    /// Creates a simple new instance of `Triangle`.
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
    pub fn new(edges_idx: [usize; 3], neighbors: [Neighbors; 3]) -> Triangle {
        Triangle {
            edges_idx,
            neighbors,
        }
    }

    /// Gives a reference to the `Point2D` data from each node of the `Triangle`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2D::new(0.0, 1.0), Point2D::new(1.0, 3.0), Point2D::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    ///
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None]);
    /// let b = [&nodes[0], &nodes[1], &nodes[2]];
    ///
    /// assert_eq!(a.nodes(&edges, &nodes), b);
    /// ```
    pub fn nodes<'a>(&self, edges: &'a [Edge2D], nodes: &'a [Point2D]) -> [&'a Point2D; 3] {
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
        [&nodes[first_node], &nodes[second_node], &nodes[third_node]]
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
    fn area(&self, edges: &[Edge2D], nodes: &[Point2D]) -> f64 {
        let nodes = self.nodes(edges, nodes);
        (0.5 * (-nodes[1].y * nodes[2].x
            + nodes[0].y * (-nodes[1].x + nodes[2].x)
            + nodes[0].x * (nodes[1].y - nodes[2].y)
            + nodes[1].x * nodes[2].y))
            .abs()
    }

    /// Compute the signed area of the 2D cell
    /// Often useful when building a mesh
    #[inline(always)]
    fn signed_area(&self, edges: &[Edge2D], nodes: &[Point2D]) -> f64 {
        let nodes = self.nodes(edges, nodes);
        0.5 * (-nodes[1].y * nodes[2].x
            + nodes[0].y * (-nodes[1].x + nodes[2].x)
            + nodes[0].x * (nodes[1].y - nodes[2].y)
            + nodes[1].x * nodes[2].y)
    }

    /// Computes the center of the cell
    #[inline(always)]
    fn center(&self, edges: &[Edge2D], nodes: &[Point2D]) -> Point2D {
        let nodes = self.nodes(edges, nodes);
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
    fn nodes<'a>(&self, edges: &'a [Edge2D], nodes: &'a [Point2D]) -> Vec<&'a Point2D> {
        self.nodes(edges, nodes).to_vec()
    }
}
