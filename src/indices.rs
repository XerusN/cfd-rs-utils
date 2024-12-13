use std::ops::Deref;

#[derive(Debug, Clone, Copy, Default)]
pub struct NodeIndex(usize);

#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeIndex(usize);

#[derive(Debug, Clone, Copy, Default)]
pub struct CellIndex(usize);

#[derive(Debug, Clone, Copy, Default)]
pub struct BoundaryConditionIndex(usize);

impl Deref for NodeIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for EdgeIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CellIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BoundaryConditionIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
