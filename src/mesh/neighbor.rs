/// Used to define all possible neighbors for a cell (or parent for an edge)
#[derive(Default, Debug, Clone)]
pub enum Neighbor2D {
    #[default]
    None,
    Cell(usize),
    // Maybe like that?
    Boundary(usize, usize),
}