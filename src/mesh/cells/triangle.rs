use super::{Cell2D, Neighbors};
use crate::{Point2D, Vector2D};

/// Represents a triangle.
/// Nodes gives the indices of the nodes in the corresponding array.
/// Neighbors tells if the triangle has no neighnour, is a boundary cell, or gives the indices of the neighboring cells.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangle {
    edges_idx: [usize; 3],
    neighbors: [Neighbors; 3],
}