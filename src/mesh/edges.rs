use super::neighbor::Neighbor2D;

/// Represents a 2D edge, keeps in memory the two neighbors (cell or boundary) and 2 indices pointing to nodes in a mesh data structure.
/// The limitation of 2D is enforced by the number of parents being set to 2, also easier in term of methods, 2D implementations are much easier, and some don't make sense in 3D.
pub struct Edge2D {
    nodes_idx: [usize; 2],
    neighbors: [Neighbor2D; 2],
}