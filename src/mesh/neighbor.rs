/// Defines the kind of neighbors a cell has.
/// If the neighbor is a `Cell`, then it holds the cell index in the corresponding array.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Neighbors {
    Boundary,
    Cell(usize),
    #[default]
    None,
}
