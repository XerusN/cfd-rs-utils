use super::*;

fn simple_mesh() -> Modifiable2DMesh {
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
        mesh = Modifiable2DMesh::new_from_boundary(vertices, edge_to_vertices_and_parent, parents);
    }

    mesh
}

#[test]
fn new_from_boundary_test_1() {
    let mesh = simple_mesh();

    mesh.0.check_mesh().unwrap();
}

#[test]
fn split_edge_test_1() {
    let mut mesh = simple_mesh();

    mesh.split_edge(HalfEdgeIndex(1), 0.5).unwrap();

    mesh.split_edge(HalfEdgeIndex(2), 0.2).unwrap();

    mesh.0.check_mesh().unwrap();
}

#[test]
fn add_edge_between_vertices_test_1() {
    let mut mesh = simple_mesh();

    mesh.add_edge_between_vertices((VertexIndex(1), VertexIndex(3)), ParentIndex(1))
        .unwrap();

    mesh.0.check_mesh().unwrap();
}
