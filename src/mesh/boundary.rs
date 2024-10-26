/// Defines a boundary for meshing purpose.
/// In 2D the boundary is a closed (geometrically speaking) edge list.
/// It also contains an index referecing the boundary condition block to which each edge is linked.
pub struct Boundary2D {
    edges: Vec<usize>,
    boundary_condition: Vec<usize>,
}