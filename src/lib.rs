#[cfg(feature = "instant-xml")]
mod mesh_instantxml;

#[cfg(feature = "xmlity")]
mod mesh_xmlity;

#[cfg(feature = "yaserde")]
mod mesh_yaserde;

#[cfg(feature = "roxmltree")]
mod mesh_roxmltree;

#[cfg(feature = "fast-float")]
mod mesh_instantxml_fast_float;

use std::{io::Read, path::PathBuf};

#[cfg(feature = "instant-xml")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn read_mesh_instant_xml(
    path: &PathBuf,
    expected_triangle_indices_count: usize,
    expected_vertices_count: usize,
) -> mesh_instantxml::Mesh {
    use mesh_instantxml::Mesh;
    let mut file = std::fs::File::open(path).unwrap();
    let mut xml_string = String::new();
    let _ = file.read_to_string(&mut xml_string).unwrap();
    let mesh = instant_xml::from_str::<Mesh>(&xml_string).unwrap();

    assert_eq!(
        mesh.triangles.triangle.len(),
        expected_triangle_indices_count,
        "Number of triangle indices did not match"
    );

    assert_eq!(
        mesh.vertices.vertex.len(),
        expected_vertices_count,
        "Number of Vertices did not match"
    );

    mesh
}

#[cfg(feature = "instant-xml")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn write_mesh_instant_xml(mesh: &mesh_instantxml::Mesh, string_length: usize) {
    let xml_string = instant_xml::to_string(mesh).unwrap();
    assert_eq!(
        xml_string.len(),
        string_length,
        "Serialized string does not match"
    );
}

#[cfg(feature = "xmlity")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn read_mesh_xmlity(
    path: &PathBuf,
    expected_triangle_indices_count: usize,
    expected_vertices_count: usize,
) -> mesh_xmlity::Mesh {
    use mesh_xmlity::Mesh;

    let mut file = std::fs::File::open(path).unwrap();
    let mut xml_string = String::new();
    let _ = file.read_to_string(&mut xml_string).unwrap();
    let mesh = xmlity_quick_xml::from_str::<Mesh>(&xml_string).unwrap();

    assert_eq!(
        mesh.triangles.triangle.len(),
        expected_triangle_indices_count,
        "Number of triangle indices did not match"
    );

    assert_eq!(
        mesh.vertices.vertex.len(),
        expected_vertices_count,
        "Number of Vertices did not match"
    );

    mesh
}

#[cfg(feature = "xmlity")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn write_mesh_xmlity(mesh: &mesh_xmlity::Mesh, string_length: usize) {
    let xml_string = xmlity_quick_xml::ser::to_string(mesh).unwrap();
    assert_eq!(
        xml_string.len(),
        string_length,
        "Serialized string does not match"
    );
}

#[cfg(feature = "yaserde")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn read_mesh_yaserde(
    path: &PathBuf,
    expected_triangle_indices_count: usize,
    expected_vertices_count: usize,
) -> mesh_yaserde::Mesh {
    use mesh_yaserde::Mesh;

    let mut file = std::fs::File::open(path).unwrap();
    let mut xml_string = String::new();
    let _ = file.read_to_string(&mut xml_string).unwrap();
    let mesh = yaserde::de::from_str::<Mesh>(&xml_string).unwrap();

    assert_eq!(
        mesh.triangles.triangle.len(),
        expected_triangle_indices_count,
        "Number of triangle indices did not match"
    );

    assert_eq!(
        mesh.vertices.vertex.len(),
        expected_vertices_count,
        "Number of Vertices did not match"
    );

    mesh
}

#[cfg(feature = "yaserde")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn write_mesh_yaserde(mesh: &mesh_yaserde::Mesh, string_length: usize) {
    let xml_string = yaserde::ser::to_string(mesh).unwrap();
    assert_eq!(
        xml_string.len(),
        string_length,
        "Serialized string does not match"
    );
}

#[cfg(feature = "roxmltree")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn read_mesh_roxmltree(
    path: &PathBuf,
    expected_triangle_indices_count: usize,
    expected_vertices_count: usize,
) -> mesh_roxmltree::Mesh {
    use mesh_roxmltree::Mesh;
    let mut file = std::fs::File::open(path).unwrap();
    let mut xml_string = String::new();
    let _ = file.read_to_string(&mut xml_string).unwrap();
    let mesh = serde_roxmltree::from_str::<Mesh>(&xml_string).unwrap();

    assert_eq!(
        mesh.triangles.triangle.len(),
        expected_triangle_indices_count,
        "Number of triangle indices did not match"
    );

    assert_eq!(
        mesh.vertices.vertex.len(),
        expected_vertices_count,
        "Number of Vertices did not match"
    );

    mesh
}

#[cfg(feature = "fast-float")]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn read_mesh_instant_xml_with_fast_float(
    path: &PathBuf,
    expected_triangle_indices_count: usize,
    expected_vertices_count: usize,
) -> mesh_instantxml_fast_float::Mesh {
    use mesh_instantxml_fast_float::Mesh;
    let mut file = std::fs::File::open(path).unwrap();
    let mut xml_string = String::new();
    let _ = file.read_to_string(&mut xml_string).unwrap();
    let mesh = instant_xml::from_str::<Mesh>(&xml_string).unwrap();

    assert_eq!(
        mesh.triangles.triangle.len(),
        expected_triangle_indices_count,
        "Number of triangle indices did not match"
    );

    assert_eq!(
        mesh.vertices.vertex.len(),
        expected_vertices_count,
        "Number of Vertices did not match"
    );

    mesh
}
