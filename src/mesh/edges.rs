use super::{Point2D, Vector2D};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Edge2D {
    pub nodes_idx: [usize; 2],
    pub parent_cells_idx: [Option<usize>; 2],
}

impl Edge2D {
    pub fn nodes<'a>(&self, nodes: &'a Vec<Point2D>) -> [&'a Point2D; 2] {
        [&nodes[self.nodes_idx[0]], &nodes[self.nodes_idx[1]]]
    }

    pub fn first_node<'a>(&self, nodes: &'a Vec<Point2D>) -> &'a Point2D {
        &nodes[self.nodes_idx[0]]
    }

    pub fn to_vector(&self, nodes: &Vec<Point2D>) -> Vector2D {
        nodes[self.nodes_idx[0]].vector_to(&nodes[self.nodes_idx[0]])
    }
}
