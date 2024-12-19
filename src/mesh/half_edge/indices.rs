use std::ops::{Deref, Index};

use nalgebra::Point2;
use super::Parent;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ParentIndex(usize);

impl Deref for ParentIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<ParentIndex> for Vec<HalfEdgeIndex> {
    type Output = HalfEdgeIndex;

    fn index(&self, index: ParentIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl Index<ParentIndex> for Vec<Parent> {
    type Output = Parent;

    fn index(&self, index: ParentIndex) -> &Self::Output {
        &self[index.0]
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

impl Index<HalfEdgeIndex> for Vec<HalfEdgeIndex> {
    type Output = HalfEdgeIndex;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl Index<HalfEdgeIndex> for Vec<ParentIndex> {
    type Output = ParentIndex;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl Index<HalfEdgeIndex> for Vec<VertexIndex> {
    type Output = VertexIndex;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct VertexIndex(usize);

impl Index<VertexIndex> for Vec<Point2<f64>> {
    type Output = Point2<f64>;

    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self[index.0]
    }
}