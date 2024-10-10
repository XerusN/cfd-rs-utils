/// Defines the kind of neighbors a cell has.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Neighbors {
    Boundary,
    Cell(usize),
    #[default]
    None,
}
