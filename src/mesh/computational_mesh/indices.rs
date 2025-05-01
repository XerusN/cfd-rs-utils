use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FaceIndex(pub usize);

impl Index<FaceIndex> for Vec<VertexIndex> {
    type Output = HalfEdgeIndex;

    fn index(&self, index: ParentIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<ParentIndex> for Vec<Parent> {
    fn index_mut(&mut self, index: ParentIndex) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl fmt::Display for ParentIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}