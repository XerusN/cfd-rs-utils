use super::{Cell2D, Neighbors};
use super::{Edge2D, Point2D, Vector2D};

/// Represents a triangle.
/// Nodes gives the indices of the nodes in the corresponding array.
/// Neighbors tells if the triangle has no neighnour, is a boundary cell, or gives the indices of the neighboring cells.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangle {
    edges_idx: [usize; 3],
    neighbors: [Neighbors; 3],
}

impl Triangle {
    pub fn nodes<'a>(&self, edges: &'a Vec<Edge2D>, nodes: &'a Vec<Point2D>) -> [&'a Point2D; 3] {
        [
            &edges[self.edges_idx[0]].first_node(nodes),
            &edges[self.edges_idx[1]].first_node(nodes),
            &edges[self.edges_idx[2]].first_node(nodes),
        ]
    }
}

impl Cell2D for Triangle {
    /// Compute the surface of the 2D cell
    #[inline(always)]
    fn area(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> f64 {
        let nodes = self.nodes(edges, nodes);
        0.5 * (-nodes[1].y * nodes[2].x
            + nodes[0].y * (-nodes[1].x + nodes[2].x)
            + nodes[0].x * (nodes[1].y - nodes[2].y)
            + nodes[1].x * nodes[2].y)
    }

    /// Compute the signed area of the 2D cell
    /// Often useful when building a mesh
    #[inline(always)]
    fn signed_area(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> f64 {}

    /// Computes the center of the cell
    #[inline(always)]
    fn center(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> Point2D {}

    /// Computes the normals to each edge
    #[inline(always)]
    fn normals(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> Vec<Vector2D> {}

    /// Gives each node of the cell
    #[inline(always)]
    fn nodes(&self, edges: &Vec<Edge2D>, nodes: &Vec<Point2D>) -> Vec<Point2D> {}
}
