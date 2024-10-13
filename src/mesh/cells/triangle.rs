use super::{Cell2D, Neighbors};
use super::{Edge2D, Point2D, Vector2D};

/// Represents a triangle.
/// Nodes gives the indices of the nodes in the corresponding array.
/// Neighbors tells if the triangle has no neighnour, is a boundary cell, or gives the indices of the neighboring cells.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshTriangle {
    pub edges_idx: [usize; 3],
    pub neighbors_idx: [Neighbors; 3],
}

impl MeshTriangle {
    pub fn nodes<'a>(&self, edges: &'a [Edge2D], nodes: &'a [Point2D]) -> [&'a Point2D; 3] {
        [
            edges[self.edges_idx[0]].first_node(nodes),
            edges[self.edges_idx[1]].first_node(nodes),
            edges[self.edges_idx[2]].first_node(nodes),
        ]
    }

    pub fn edges<'a>(&self, edges: &'a [Edge2D]) -> [&'a Edge2D; 3] {
        [
            &edges[self.edges_idx[0]],
            &edges[self.edges_idx[1]],
            &edges[self.edges_idx[2]],
        ]
    }
}

impl Cell2D for MeshTriangle {
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

/// Abstraction for the MeshTriangle type.
/// Enables to access cells informations in a less verbose way.
/// Since it's working with references can't be used when editing the mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle<'a> {
    pub nodes: [&'a Point2D; 3],
    pub edges: [&'a Edge2D; 3],
    pub mesh_triangle: &'a MeshTriangle,
}
