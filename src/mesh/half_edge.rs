use crate::boundary::Boundary;
use indices::*;
use nalgebra::Point2;

pub mod indices;

/// Parent of a half_edge, either a cell or a boundary.
/// ```None``` is meant as an error or temporary state.
#[derive(Clone, Debug, Default)]
pub enum Parent {
    #[default]
    None,
    Cell(u32),
    Boundary(Boundary),
}

/// Array based Half-edge data-structure mesh representation
/// Supports meshes of up to a billion element.
/// Since the crate is built for cfd on a classic computer (not HPC) it is easily enough.
#[derive(Clone, Debug, Default)]
pub struct Mutable2DMesh {
    //```he``` is for Half-edge
    he_to_vertex: Vec<VertexIndex>,
    he_to_twin: Vec<HalfEdgeIndex>,
    he_to_next_he: Vec<HalfEdgeIndex>,
    he_to_prev_he: Vec<HalfEdgeIndex>,
    he_to_parent: Vec<ParentIndex>,

    vertex: Vec<Point2<f64>>,
    parent: Vec<Parent>,

    parent_to_he: Vec<Vec<HalfEdgeIndex>>,
}
