use nalgebra::Point2;
use crate::boundary::Boundary;

/// Array based Half-edge data-structure mesh representation
/// Supports meshes of up to a billion element.
/// Since the crate is built for cfd on a classic computer (not HPC) it is easily enough.
#[derive(Clone, Debug, Default)]
pub struct Mutable2DMesh {
    //```he``` is for Half-edge
    parent_to_he: Vec<Vec<u32>>,

    he_to_vertex: Vec<u32>,
    he_to_twin: Vec<u32>,
    he_to_next_he: Vec<u32>,
    he_to_prev_he: Vec<u32>,
    he_to_parent: Vec<u32>,

    vertex: Vec<Point2<f64>>,
    parent: Vec<Parent>,
}

#[derive(Clone, Debug, Default)]
pub enum Parent {
    #[default]
    None,
    Cell(u32),
    Boundary(Boundary)
}

