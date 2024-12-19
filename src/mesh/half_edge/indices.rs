use std::ops::Deref;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ParentIndex(usize);

impl Deref for ParentIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct HalfEdgeIndex(usize);

impl Deref for HalfEdgeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct VertexIndex(usize);

impl Deref for VertexIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
