use core::f64;

use super::Edge2D;
use super::{Cell2D, Neighbors};
use indices::indices;
use nalgebra::{Point2, Rotation2, Unit, Vector2};

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
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges);
    /// let b = Triangle { edges_idx: [0, 1, 2], neighbors: [Neighbors::None, Neighbors::None, Neighbors::None], nodes_idx: [0, 1, 2]};
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
    /// let nodes = vec![Point2::<f64>::new(0.0, 1.0), Point2::<f64>::new(1.0, 3.0), Point2::<f64>::new(-1.0, 3.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    /// let mut triangle = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges);
    ///
    /// assert_eq!(triangle.nodes_idx[0], 0);
    ///
    /// triangle.edges_idx = [1, 2, 0];
    /// triangle.update_nodes_idx_from_edges(&edges);
    ///
    /// assert_eq!(triangle.nodes_idx[0], 1);
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
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges);
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    ///
    /// let nodes = vec![Point2::<f64>::new(0.0, 0.0), Point2::<f64>::new(1.0, 0.0), Point2::<f64>::new(0.0, 1.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    ///
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges);
    ///
    /// assert_eq!(a.area(&nodes), 0.5);
    /// ```
    #[inline(always)]
    fn area(&self, nodes: &[Point2<f64>]) -> f64 {
        let nodes = self.nodes(nodes);
        (0.5 * (-nodes[1].y * nodes[2].x
            + nodes[0].y * (-nodes[1].x + nodes[2].x)
            + nodes[0].x * (nodes[1].y - nodes[2].y)
            + nodes[1].x * nodes[2].y))
            .abs()
    }

    /// Computes the center of the cell
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    /// let nodes = vec![Point2::<f64>::new(0.0, 0.0), Point2::<f64>::new(1.0, 0.0), Point2::<f64>::new(0.0, 1.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    ///
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges);
    ///
    /// assert_eq!(a.center(&nodes), Point2::<f64>::new(1./3., 1./3.));
    /// ```
    #[inline(always)]
    fn center(&self, nodes: &[Point2<f64>]) -> Point2<f64> {
        let nodes = self.nodes(nodes);
        Vector2::<f64>::into((nodes[0].coords + nodes[1].coords + nodes[2].coords) / 3.0)
    }

    /// Computes the normals to each edge
    ///
    /// # Example
    ///
    /// ```rust
    /// use cfd_rs_utils::*;
    /// let nodes = vec![Point2::<f64>::new(0.0, 0.0), Point2::<f64>::new(1.0, 0.0), Point2::<f64>::new(0.0, 1.0)];
    /// let edges = vec![Edge2D::new(0, 1), Edge2D::new(1, 2), Edge2D::new(2, 0)];
    ///
    /// let a = Triangle::new([0, 1, 2], [Neighbors::None, Neighbors::None, Neighbors::None], &edges);
    ///
    /// assert_eq!(a.normals(&edges, &nodes)[0].y, -1.0);
    /// ```
    #[inline(always)]
    fn normals(&self, edges: &[Edge2D], nodes: &[Point2<f64>]) -> Vec<Unit<Vector2<f64>>> {
        let rot = Rotation2::new(f64::consts::FRAC_PI_2);
        let cell_center = self.center(nodes);
        self.edges(edges)
            .iter()
            .map(|edge| {
                let centers = edge.center(nodes);
                let normal = rot * edge.to_vector(nodes);
                if normal.dot(&(centers - cell_center)) > 0. {
                    Unit::<Vector2<f64>>::new_normalize(normal)
                } else {
                    Unit::<Vector2<f64>>::new_normalize(-normal)
                }
            })
            .collect()
    }

    /// Gives a reference to each node of the cell
    #[inline(always)]
    fn nodes<'a>(&self, nodes: &'a [Point2<f64>]) -> Vec<&'a Point2<f64>> {
        vec![
            &nodes[self.nodes_idx[0]],
            &nodes[self.nodes_idx[1]],
            &nodes[self.nodes_idx[2]],
        ]
    }

    /// Gives a reference to each node of the cell
    #[inline(always)]
    fn edges<'a>(&self, edges: &'a [Edge2D]) -> Vec<&'a Edge2D> {
        vec![
            &edges[self.edges_idx[0]],
            &edges[self.edges_idx[1]],
            &edges[self.edges_idx[2]],
        ]
    }

    /// Gives a mutable reference to each node of the cell.
    /// Panics if some indices are the same.
    #[inline(always)]
    fn nodes_mut<'a>(&self, nodes: &'a mut [Point2<f64>]) -> Vec<&'a mut Point2<f64>> {
        let (first_node, second_node, third_node) = indices!(
            nodes,
            self.nodes_idx[0],
            self.nodes_idx[1],
            self.nodes_idx[2]
        );
        vec![first_node, second_node, third_node]
    }

    /// Gives a mutable reference to each edge of the cell.
    /// Panics if some indices are the same
    #[inline(always)]
    fn edges_mut<'a>(&self, edges: &'a mut [Edge2D]) -> Vec<&'a mut Edge2D> {
        let (first_edge, second_edge, third_edge) = indices!(
            edges,
            self.edges_idx[0],
            self.edges_idx[1],
            self.edges_idx[2]
        );
        vec![first_edge, second_edge, third_edge]
    }
}
