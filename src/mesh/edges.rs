use super::Point2D;

pub struct Edge2D {
    pub nodes_idx: [usize; 2],
    pub parent_cells_idx: [Option<usize>; 2],
}