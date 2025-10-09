use xmlity::{Deserialize, SerializationGroup, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[xelement(name = "mesh")]
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

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[xelement(name = "vertices")]
pub struct Vertices {
    pub vertex: Vec<Vertex>,
}

/// A vertex in a triangle mesh
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[xelement(
    name = "vertex",
    //namespace = "http://schemas.microsoft.com/3dmanufacturing/core/2015/02",
    // namespace_expr = XmlNamespace::new_dangerous(CORE_NS),
    // preferred_prefix = "core"
)]
pub struct Vertex {
    #[xattribute]
    pub x: f64,

    #[xattribute]
    pub y: f64,

    #[xattribute]
    pub z: f64,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[xelement(name = "triangles")]
pub struct Triangles {
    pub triangle: Vec<Triangle>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, SerializationGroup)]
#[xelement(name = "triangle")]
pub struct Triangle {
    #[xattribute]
    pub v1: usize,

    #[xattribute]
    pub v2: usize,

    #[xattribute]
    pub v3: usize,

    #[xattribute(optional = true)]
    pub p1: Option<usize>,

    #[xattribute(optional = true)]
    pub p2: Option<usize>,

    #[xattribute(optional = true)]
    pub p3: Option<usize>,

    #[xattribute(optional = true)]
    pub pid: Option<usize>,
}

#[cfg(test)]
pub mod xmlity_tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use xmlity_quick_xml::from_str;
    use xmlity_quick_xml::to_string;

    use super::*;

    #[test]
    pub fn toxml_vertex_test() {
        let xml_string = r#"<vertex x="100.5" y="100" z="0"/>"#.to_string();
        let vertex = Vertex {
            x: 100.5,
            y: 100.0,
            z: 0.0,
        };
        let vertex_string = to_string(&vertex).unwrap();

        assert_str_eq!(vertex_string, xml_string);
    }

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
    pub fn toxml_vertices_test() {
        let xml_string = r#"<vertices><vertex x="100" y="110.5" z="0"/><vertex x="0.156" y="55.6896" z="-10"/></vertices>"#.to_string();
        let vertices = Vertices {
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
        };
        let vertices_string = to_string(&vertices).unwrap();

        assert_str_eq!(vertices_string, xml_string)
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
    pub fn toxml_required_fields_triangle_test() {
        let xml_string = r#"<triangle v1="1" v2="2" v3="3"/>"#.to_string();
        let triangle = Triangle {
            v1: 1,
            v2: 2,
            v3: 3,
            p1: None,
            p2: None,
            p3: None,
            pid: None,
        };
        let triangle_string = to_string(&triangle).unwrap();

        assert_eq!(triangle_string, xml_string);
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
    pub fn toxml_with_optional_fields_triangle_test() {
        let xml_string =
            r#"<triangle v1="1" v2="2" v3="3" p1="4" p2="5" p3="6" pid="7"/>"#.to_string();
        let triangle = Triangle {
            v1: 1,
            v2: 2,
            v3: 3,
            p1: Some(4),
            p2: Some(5),
            p3: Some(6),
            pid: Some(7),
        };
        let triangle_string = to_string(&triangle).unwrap();

        assert_eq!(triangle_string, xml_string);
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
    pub fn toxml_triangles_test() {
        let xml_string = r#"<triangles><triangle v1="1" v2="2" v3="3"/><triangle v1="2" v2="3" v3="4"/></triangles>"#.to_string();
        let triangles = Triangles {
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
        };
        let triangles_string = to_string(&triangles).unwrap();

        assert_eq!(triangles_string, xml_string);
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
    pub fn toxml_required_field_mesh_test() {
        let xml_string = r##"<mesh><vertices><vertex x="-1" y="-1" z="0"/><vertex x="1" y="-1" z="0"/><vertex x="1" y="1" z="0"/><vertex x="-1" y="1" z="0"/></vertices><triangles><triangle v1="0" v2="1" v3="2"/><triangle v1="0" v2="2" v3="3"/></triangles></mesh>"##.to_string();
        let mesh = Mesh {
            vertices: Vertices {
                vertex: vec![
                    Vertex {
                        x: -1.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    Vertex {
                        x: 1.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    Vertex {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vertex {
                        x: -1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
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
                    },
                ],
            },
        };
        let mesh_string = to_string(&mesh).unwrap();

        assert_eq!(mesh_string, xml_string);
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
    pub fn toxml_with_optional_field_mesh_test() {
        let xml_string = r##"<mesh><vertices><vertex x="-1" y="-1" z="0"/><vertex x="1" y="-1" z="0"/><vertex x="1" y="1" z="0"/><vertex x="-1" y="1" z="0"/></vertices><triangles><triangle v1="0" v2="1" v3="2" p1="3" pid="7"/><triangle v1="0" v2="2" v3="3" p1="4" p2="5" p3="6"/></triangles></mesh>"##.to_string();
        let mesh = Mesh {
            vertices: Vertices {
                vertex: vec![
                    Vertex {
                        x: -1.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    Vertex {
                        x: 1.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    Vertex {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vertex {
                        x: -1.0,
                        y: 1.0,
                        z: 0.0,
                    },
                ],
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
                    },
                ],
            },
        };
        let mesh_string = to_string(&mesh).unwrap();

        assert_eq!(mesh_string, xml_string);
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
