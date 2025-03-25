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

    unsafe {
        mesh.trimming((VertexIndex(1), VertexIndex(3)), ParentIndex(1))
            .unwrap();
    }

    mesh.0.check_mesh().unwrap();
}

#[test]
fn extract_vertex_from_edge_test_1() {
    let mut mesh = simple_mesh();

    unsafe {
        _ = mesh
            .notching(HalfEdgeIndex(4), Point2::new(0.5, 0.5))
            .unwrap()
    }

    mesh.0.export_vtk("./output/test2.vtk").unwrap();

    mesh.0.check_mesh().unwrap();
}

#[test]
fn combined_test() {
    let mut mesh = simple_mesh();

    mesh.0.export_vtk("./output/test_0.vtk").unwrap();
    
    
    unsafe {
        mesh.trimming((VertexIndex(1), VertexIndex(3)), ParentIndex(1))
            .unwrap();
    }

    mesh.0.export_vtk("./output/test_1.vtk").unwrap();

    mesh.split_edge(HalfEdgeIndex(8), 0.5).unwrap();

    mesh.0.export_vtk("./output/test_2.vtk").unwrap();

    unsafe {
        if let Err(MeshError::ParentDoesNotContainVertex{vertex: _, parent: _}) = mesh.trimming((VertexIndex(4), VertexIndex(0)), ParentIndex(1)) {
            ();
        } else {
            panic!("Trimming did not catch wrong parent use")
        }
        
        mesh.trimming((VertexIndex(4), VertexIndex(0)), ParentIndex(2))
            .unwrap();
    }

    mesh.0.export_vtk("./output/test_3.vtk").unwrap();

    mesh.0.check_mesh().unwrap();
}

#[test]
fn notching_test() {
    let mut mesh = simple_mesh();
    unsafe {
        mesh.notching(HalfEdgeIndex(0), Point2::new(0.5, 0.5)).unwrap();
    }
    let mut boundary= None;
    for (i, parent) in mesh.0.parents().iter().enumerate() {
        if let &Parent::Boundary(_) = parent {
            boundary = Some(i);
            break;
        }
    }
    println!("len boundary {:?}", mesh.0.vertices_from_parent(ParentIndex(boundary.expect("No boundary after trimming"))).len());
    assert!(mesh.0.vertices_from_parent(ParentIndex(boundary.expect("No boundary after trimming"))).len() == 4)
}