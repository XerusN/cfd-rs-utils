use nalgebra::{Point2, Vector2};

use crate::mesh::indices::{BoundaryPatchIndex, CellIndex, FaceIndex, VertexIndex};

use super::{BoundaryPatch, Cell, Computational2DMesh, Face, Patch};

pub fn straight_line(num_cells: usize) -> Computational2DMesh {
    let l = 1.;
    let t = l/num_cells as f64;

    let mut vertices = vec![];
    let mut cells = vec![];
    let mut faces = vec![];
    let boundaries = vec![
        BoundaryPatch::new("left".to_string()),
        BoundaryPatch::new("right".to_string()),
        BoundaryPatch::new("empty".to_string()),
    ];

    vertices.push(Point2::new(0., 0.));
    vertices.push(Point2::new(0., t));
    faces.push(Face::new(
        [VertexIndex(0), VertexIndex(1)],
        (
            Patch::Boundary(BoundaryPatchIndex(0)),
            Patch::Cell(CellIndex(0)),
        ),
        &vertices,
    ));

    for i in 0..num_cells {
        vertices.push(Point2::new((i as f64 + 1.) * l / num_cells as f64, 0.));
        vertices.push(Point2::new((i as f64 + 1.) * l / num_cells as f64, t));
        faces.push(Face::new(
            [VertexIndex(i*2), VertexIndex(i*2+2)],
            (
                Patch::Cell(CellIndex(i)),
                Patch::Boundary(BoundaryPatchIndex(2)),
            ),
            &vertices,
        ));
        faces.push(Face::new(
            [VertexIndex(i*2+1), VertexIndex(i*2+3)],
            (
                Patch::Boundary(BoundaryPatchIndex(2)),
                Patch::Cell(CellIndex(i)),
            ),
            &vertices,
        ));
        if i < num_cells-1 {
            faces.push(Face::new(
                [VertexIndex(i*2+2), VertexIndex(i*2+3)],
                (
                    Patch::Cell(CellIndex(i)),
                    Patch::Cell(CellIndex(i+1)),
                ),
                &vertices,
            ));
        } else {
            faces.push(Face::new(
                [VertexIndex(i*2+2), VertexIndex(i*2+3)],
                (
                    Patch::Cell(CellIndex(i)),
                    Patch::Boundary(BoundaryPatchIndex(1)),
                ),
                &vertices,
            ));
        }
        let last_face = faces.len() - 1;
        cells.push(Cell::new(vec![FaceIndex(last_face), FaceIndex(last_face-1), FaceIndex(last_face-2), FaceIndex(last_face-3)], &vertices, &faces));
    }

    Computational2DMesh {
        cells,
        boundaries,
        faces,
        vertices,
    }
}
