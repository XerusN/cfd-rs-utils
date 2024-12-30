use std::ops::{Index, IndexMut};
use std::fmt;

use super::Parent;
use nalgebra::Point2;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ParentIndex(pub usize);

impl Index<ParentIndex> for Vec<HalfEdgeIndex> {
    type Output = HalfEdgeIndex;

    fn index(&self, index: ParentIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<ParentIndex> for Vec<HalfEdgeIndex> {
    fn index_mut(&mut self, index: ParentIndex) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<ParentIndex> for Vec<Parent> {
    type Output = Parent;

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


#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct HalfEdgeIndex(pub usize);

impl Index<HalfEdgeIndex> for Vec<HalfEdgeIndex> {
    type Output = HalfEdgeIndex;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<HalfEdgeIndex> for Vec<HalfEdgeIndex> {
    fn index_mut(&mut self, index: HalfEdgeIndex) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<HalfEdgeIndex> for Vec<ParentIndex> {
    type Output = ParentIndex;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<HalfEdgeIndex> for Vec<ParentIndex> {
    fn index_mut(&mut self, index: HalfEdgeIndex) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<HalfEdgeIndex> for Vec<VertexIndex> {
    type Output = VertexIndex;

    fn index(&self, index: HalfEdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<HalfEdgeIndex> for Vec<VertexIndex> {
    fn index_mut(&mut self, index: HalfEdgeIndex) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl fmt::Display for HalfEdgeIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct VertexIndex(pub usize);

impl Index<VertexIndex> for Vec<Point2<f64>> {
    type Output = Point2<f64>;

    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<VertexIndex> for Vec<Point2<f64>> {
    fn index_mut(&mut self, index: VertexIndex) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl fmt::Display for VertexIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
