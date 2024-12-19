use std::ops::Deref;

#[derive(Copy, Clone, Debug, Default)]
pub struct ParentIndex(u32);

impl Deref for ParentIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct HalfEdgeIndex(u32);

impl Deref for HalfEdgeIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct VertexIndex(u32);

impl Deref for VertexIndex {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
