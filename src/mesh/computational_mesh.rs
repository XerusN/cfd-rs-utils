use nalgebra::{Point2, Vector2};

use crate::boundary::Boundary;
use crate::geometry::*;

use super::{
    indices::{BoundaryPatchIndex, CellIndex, FaceIndex, HalfEdgeIndex, ParentIndex, VertexIndex},
    Base2DMesh, Parent,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Patch {
    #[default]
    Empty,
    Cell(CellIndex),
    Boundary(BoundaryPatchIndex),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Face {
    vertices: [VertexIndex; 2],
    area: f64,
    normal: Vector2<f64>,
    patches: (Patch, Patch),
}

impl Face {
    /// Patches needs to be given in an order such that the first parent sees the face defined in the trigonometric order
    pub fn new(
        vertices: [VertexIndex; 2],
        patches: (Patch, Patch),
        vertices_glob: &[Point2<f64>],
    ) -> Self {
        let area = line_length(&[vertices_glob[vertices[0].0], vertices_glob[vertices[1].0]]);
        let normal = -line_normal(&[vertices_glob[vertices[0].0], vertices_glob[vertices[1].0]]);

        Face {
            vertices,
            area,
            normal,
            patches,
        }
    }

    pub fn vertices(&self) -> &[VertexIndex; 2] {
        &self.vertices
    }

    /// From the first patch to the second
    pub fn normal(&self) -> &Vector2<f64> {
        &self.normal
    }

    pub fn area(&self) -> f64 {
        self.area
    }

    pub fn patches(&self) -> &(Patch, Patch) {
        &self.patches
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cell {
    volume: f64,
    centroid: Point2<f64>,
    faces: Vec<FaceIndex>,
    vertices: Vec<VertexIndex>,
}

impl Cell {
    pub fn new(faces: Vec<FaceIndex>, vertices_glob: &[Point2<f64>], faces_glob: &[Face]) -> Self {
        let mut vertices = vec![];
        for face in faces
            .iter()
            .map(|f_id| &faces_glob[f_id.0])
            .collect::<Vec<&Face>>()
        {
            for i in 0..2 {
                if !vertices.contains(&face.vertices[i]) {
                    vertices.push(face.vertices[i]);
                }
            }
        }
        assert_eq!(vertices.len(), 3);
        let points = [
            vertices_glob[vertices[0].0],
            vertices_glob[vertices[1].0],
            vertices_glob[vertices[2].0],
        ];
        let volume = triangle_area(&points);
        let centroid = triangle_centroid(&points);
        Cell {
            volume,
            centroid,
            faces,
            vertices,
        }
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn centroid(&self) -> &Point2<f64> {
        &self.centroid
    }

    pub fn faces_id(&self) -> &[FaceIndex] {
        &self.faces
    }

    pub fn faces<'a>(&self, faces_glob: &'a [Face]) -> Vec<&'a Face> {
        self.faces
            .iter()
            .map(|f_id| &faces_glob[f_id.0])
            .collect::<Vec<&Face>>()
    }

    pub fn vertices_id(&self) -> &[VertexIndex] {
        &self.vertices
    }

    pub fn vertices<'a>(&self, vertices_glob: &'a [Point2<f64>]) -> Vec<&'a Point2<f64>> {
        self.vertices
            .iter()
            .map(|f_id| &vertices_glob[f_id.0])
            .collect::<Vec<&Point2<f64>>>()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoundaryPatch {
    boundary: Boundary,
    faces: Vec<FaceIndex>,
}

impl BoundaryPatch {
    pub fn new(boundary: Boundary, faces: Vec<FaceIndex>) -> Self {
        BoundaryPatch { boundary, faces }
    }

    pub fn boundary(&self) -> &Boundary {
        &self.boundary
    }

    pub fn faces_id(&self) -> &[FaceIndex] {
        &self.faces
    }

    pub fn faces<'a>(&self, faces_glob: &'a [Face]) -> Vec<&'a Face> {
        self.faces
            .iter()
            .map(|f_id| &faces_glob[f_id.0])
            .collect::<Vec<&Face>>()
    }
}

/// Contains all the topological and geometric data needed by a mesh
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Computational2DMesh {
    cells: Vec<Cell>,
    boundaries: Vec<BoundaryPatch>,
    faces: Vec<Face>,
    vertices: Vec<Point2<f64>>,
}

impl Computational2DMesh {
    
    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }
    
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }
    
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }
    
    pub fn faces(&self) -> &[Face] {
        &self.faces
    }
    
    pub fn new_from_he(mesh: Base2DMesh) -> Self {
        let mut vertices = Vec::with_capacity(mesh.vertices_len());
        for i in 0..mesh.vertices_len() {
            vertices.push(mesh.vertices(VertexIndex(i)));
        }

        let mut parent_to_patch = vec![];
        let mut cell_id = 0;
        let mut boundary_id = 0;

        for parent in mesh.parents() {
            match *parent {
                Parent::None => panic!(),
                Parent::Cell => {
                    parent_to_patch.push(Patch::Cell(CellIndex(cell_id)));
                    cell_id += 1;
                }
                Parent::Boundary(_) => {
                    parent_to_patch.push(Patch::Boundary(BoundaryPatchIndex(boundary_id)));
                    boundary_id += 1;
                }
            }
        }

        let mut he_to_face = vec![];
        let mut faces = vec![];

        let mut i = 0;
        for (he, &HalfEdgeIndex(twin)) in mesh.he_to_twin().iter().enumerate() {
            match he.cmp(&twin) {
                std::cmp::Ordering::Less => {
                    he_to_face.push(FaceIndex(i));
                    let patches = (
                        parent_to_patch[mesh.he_to_parent()[he].0].clone(),
                        parent_to_patch[mesh.he_to_parent()[twin].0].clone(),
                    );
                    faces.push(Face::new(
                        mesh.vertices_from_he(HalfEdgeIndex(he)),
                        patches,
                        &vertices,
                    ));
                    i += 1;
                }
                std::cmp::Ordering::Greater => he_to_face.push(he_to_face[he]),
                std::cmp::Ordering::Equal => panic!(),
            };
        }

        let mut cells = vec![];
        let mut boundaries = vec![];

        for (parent, patch) in parent_to_patch.iter().enumerate() {
            match *patch {
                Patch::Empty => (),
                Patch::Boundary(id) => {
                    let boundary = match mesh.parents()[parent].clone() {
                        Parent::Boundary(boundary) => boundary,
                        _ => panic!(
                            "Bad construction of parent_to_patch at index : {:?}",
                            parent
                        ),
                    };
                    let faces_loc = mesh
                        .he_from_parent(ParentIndex(parent))
                        .iter()
                        .map(|he| he_to_face[he.0])
                        .collect();
                    if id.0 != boundaries.len() {
                        panic!("Wrong construction of boundary");
                    }
                    boundaries.push(BoundaryPatch::new(boundary, faces_loc));
                }
                Patch::Cell(id) => {
                    match mesh.parents()[parent] {
                        Parent::Cell => (),
                        _ => panic!(
                            "Bad construction of parent_to_patch at index : {:?}",
                            parent
                        ),
                    };
                    let faces_loc = mesh
                        .he_from_parent(ParentIndex(parent))
                        .iter()
                        .map(|he| he_to_face[he.0])
                        .collect();
                    if id.0 != cells.len() {
                        panic!("Wrong construction of boundary");
                    }
                    cells.push(Cell::new(faces_loc, &vertices, &faces));
                }
            }
        }

        Self {
            cells,
            faces,
            boundaries,
            vertices,
        }
    }
}
