/// Defines the kind of neighbors a cell has.
/// If the neighbor is a `Cell`, then it holds the cell index in the corresponding array.
/// If the neighbor is a Boundary, then it holds the index of the boundary (value meaningfull outside of the mesh struct)
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Neighbors {
    Boundary(usize),
    Cell(usize),
    #[default]
    None,
}
