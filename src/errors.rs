use thiserror::Error;

use crate::mesh::indices::*;

#[derive(Clone, Debug, Default, Error, PartialEq)]
pub enum MeshError {
    #[default]
    #[error(
        "An Unspecified error happened, you can blame the crate developer for the lack of details"
    )]
    Unspecified,
    #[error("Value not in the right range (expected in {expected:?}, got {got:?}")]
    WrongFloatValue { got: f64, expected: (f64, f64) },
    #[error("An HalfEdgeIndex is out of bound in array (got {got:?}, there are only {len:?} half-edges)")]
    HalfEdgeIndexOutOfBound{ got: HalfEdgeIndex, len: usize},
    #[error("An VertexIndex is out of bound in array (got {got:?}, there are only {len:?} vertices)")]
    VertexIndexOutOfBound{ got: VertexIndex, len: usize},
    #[error("An ParentIndex is out of bound in array (got {got:?}, there are only {len:?} parents)")]
    ParentIndexOutOfBound{ got: ParentIndex, len: usize},
}
