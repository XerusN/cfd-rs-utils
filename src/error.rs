use thiserror::Error as ThisError;

use crate::{CellIndex, EdgeIndex, NodeIndex};

#[derive(Clone, ThisError, Debug)]
pub enum Error {
    #[error("Tried to reach out of bound cell (index {index:?} when their is {length:?} cells)")]
    CellOutOfBound{index: CellIndex, length: usize},
    #[error("Tried to reach out of bound edge (index {index:?} when their is {length:?} edges)")]
    EdgeOutOfBound{index: EdgeIndex, length: usize},
    #[error("Tried to reach out of bound node (index {index:?} when their is {length:?} nodes)")]
    NodeOutOfBound{index: NodeIndex, length: usize},
}