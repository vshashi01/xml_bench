use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename = "mesh")]
pub struct Mesh {
    /// The vertices of the mesh
    ///
    /// This defines the vertices that are part of the mesh, but not the mesh's
    /// structure. See the `triangles` field.
    pub vertices: Vertices,

    /// The triangles that make up the mesh
    ///
    /// Each triangle consists of indices that refer back to the `vertices`
    /// field.
    pub triangles: Triangles,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
#[serde(rename = "vertices")]
pub struct Vertices {
    pub vertex: Vec<Vertex>,
}

/// A vertex in a triangle mesh
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename = "vertex")]
pub struct Vertex {
    pub x: f64,

    pub y: f64,

    pub z: f64,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Triangles {
    pub triangle: Vec<Triangle>,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
#[serde(rename = "triangle")]
pub struct Triangle {
    pub v1: usize,

    pub v2: usize,

    pub v3: usize,

    pub p1: Option<usize>,

    pub p2: Option<usize>,

    pub p3: Option<usize>,

    pub pid: Option<usize>,
}

#[cfg(test)]
mod roxmltree_tests {
    use serde_roxmltree::from_str;

    use super::*;

    #[test]
    pub fn fromxml_vertex_test() {
        let xml_string = r#"<vertex x="100.5" y="100" z="0" />"#.to_string();
        let vertex = from_str::<Vertex>(&xml_string).unwrap();

        assert_eq!(
            vertex,
            Vertex {
                x: 100.5,
                y: 100.0,
                z: 0.0,
            }
        );
    }

    #[test]
    pub fn fromxml_vertices_test() {
        let xml_string = r#"<vertices><vertex x="100" y="110.5" z="0" /><vertex x="0.156" y="55.6896" z="-10" /></vertices>"#.to_string();
        let vertices = from_str::<Vertices>(&xml_string).unwrap();

        assert_eq!(
            vertices,
            Vertices {
                vertex: vec![
                    Vertex {
                        x: 100.,
                        y: 110.5,
                        z: 0.0,
                    },
                    Vertex {
                        x: 0.156,
                        y: 55.6896,
                        z: -10.0,
                    },
                ],
            }
        )
    }

    #[test]
    pub fn fromxml_required_fields_triangle_test() {
        let xml_string = r#"<triangle v1="1" v2="2" v3="3"/>"#.to_string();
        let triangle = from_str::<Triangle>(&xml_string).unwrap();

        assert_eq!(
            triangle,
            Triangle {
                v1: 1,
                v2: 2,
                v3: 3,
                p1: None,
                p2: None,
                p3: None,
                pid: None,
            }
        );
    }

    #[test]
    pub fn fromxml_triangles_test() {
        let xml_string = r#"<triangles><triangle v1="1" v2="2" v3="3" /><triangle v1="2" v2="3" v3="4" /></triangles>"#.to_string();
        let triangles = from_str::<Triangles>(&xml_string).unwrap();

        assert_eq!(
            triangles,
            Triangles {
                triangle: vec![
                    Triangle {
                        v1: 1,
                        v2: 2,
                        v3: 3,
                        p1: None,
                        p2: None,
                        p3: None,
                        pid: None,
                    },
                    Triangle {
                        v1: 2,
                        v2: 3,
                        v3: 4,
                        p1: None,
                        p2: None,
                        p3: None,
                        pid: None,
                    },
                ],
            }
        );
    }

    #[test]
    pub fn fromxml_with_optional_fields_triangle_test() {
        let xml_string = r#"<triangle v1="1" v2="2" v3="3" p1="4" pid="7"/>"#.to_string();
        let triangle = from_str::<Triangle>(&xml_string).unwrap();

        assert_eq!(
            triangle,
            Triangle {
                v1: 1,
                v2: 2,
                v3: 3,
                p1: Some(4),
                p2: None,
                p3: None,
                pid: Some(7),
            }
        );
    }

    #[test]
    pub fn fromxml_required_fields_mesh_test() {
        let xml_string = r##"<mesh><vertices><vertex x="-1" y="-1" z="0"/><vertex x="1" y="-1" z="0"/><vertex x="1" y="1" z="0"/><vertex x="-1" y="1" z="0"/></vertices><triangles><triangle v1="0" v2="1" v3="2"/><triangle v1="0" v2="2" v3="3"/></triangles></mesh>"##.to_string();

        let mesh = from_str::<Mesh>(&xml_string).unwrap();

        assert_eq!(
            mesh,
            Mesh {
                vertices: Vertices {
                    vertex: vec![
                        Vertex {
                            x: -1.0,
                            y: -1.0,
                            z: 0.0
                        },
                        Vertex {
                            x: 1.0,
                            y: -1.0,
                            z: 0.0
                        },
                        Vertex {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0
                        },
                        Vertex {
                            x: -1.0,
                            y: 1.0,
                            z: 0.0
                        }
                    ]
                },
                triangles: Triangles {
                    triangle: vec![
                        Triangle {
                            v1: 0,
                            v2: 1,
                            v3: 2,
                            p1: None,
                            p2: None,
                            p3: None,
                            pid: None,
                        },
                        Triangle {
                            v1: 0,
                            v2: 2,
                            v3: 3,
                            p1: None,
                            p2: None,
                            p3: None,
                            pid: None,
                        }
                    ]
                },
            }
        )
    }

    #[test]
    pub fn fromxml_with_optional_fields_mesh_test() {
        let xml_string = r##"<mesh><vertices><vertex x="-1" y="-1" z="0"/><vertex x="1" y="-1" z="0"/><vertex x="1" y="1" z="0"/><vertex x="-1" y="1" z="0"/></vertices><triangles><triangle v1="0" v2="1" v3="2" p1="3" pid="7"/><triangle v1="0" v2="2" v3="3" p1="4" p2="5" p3="6"/></triangles></mesh>"##.to_string();

        let mesh = from_str::<Mesh>(&xml_string).unwrap();

        assert_eq!(
            mesh,
            Mesh {
                vertices: Vertices {
                    vertex: vec![
                        Vertex {
                            x: -1.0,
                            y: -1.0,
                            z: 0.0
                        },
                        Vertex {
                            x: 1.0,
                            y: -1.0,
                            z: 0.0
                        },
                        Vertex {
                            x: 1.0,
                            y: 1.0,
                            z: 0.0
                        },
                        Vertex {
                            x: -1.0,
                            y: 1.0,
                            z: 0.0
                        }
                    ]
                },
                triangles: Triangles {
                    triangle: vec![
                        Triangle {
                            v1: 0,
                            v2: 1,
                            v3: 2,
                            p1: Some(3),
                            p2: None,
                            p3: None,
                            pid: Some(7),
                        },
                        Triangle {
                            v1: 0,
                            v2: 2,
                            v3: 3,
                            p1: Some(4),
                            p2: Some(5),
                            p3: Some(6),
                            pid: None,
                        }
                    ]
                },
            }
        )
    }
}
