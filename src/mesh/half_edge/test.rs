use super::*;

fn simple_mesh() -> Mutable2DMesh {
    let parents = vec![Parent::Boundary(Boundary::NoSlip)];
    let vertices = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ];

    let edge_to_vertices_and_parent = vec![
        (VertexIndex(0), VertexIndex(1), ParentIndex(0)),
        (VertexIndex(1), VertexIndex(2), ParentIndex(0)),
        (VertexIndex(2), VertexIndex(3), ParentIndex(0)),
        (VertexIndex(3), VertexIndex(0), ParentIndex(0)),
    ];

    let mesh;

    unsafe {
        mesh = Mutable2DMesh::new_from_boundary(vertices, edge_to_vertices_and_parent, parents);
    }

    mesh
}

#[test]
fn new_from_boundary_test_1() {
    let mesh = simple_mesh();

    assert_eq!(mesh.parents.len(), 2);
    assert_eq!(
        mesh.he_from_parent(ParentIndex(0)).len(),
        mesh.he_from_parent(ParentIndex(1)).len()
    );
}
