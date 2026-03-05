use fast_float2::parse;
use instant_xml::*;

//use std::f64;

/// A triangle mesh
#[derive(FromXml, ToXml, PartialEq, Clone, Debug)]
#[xml(rename = "mesh")]
pub struct Mesh {
    /// The vertices of the mesh
    pub vertices: Vertices,

    /// The triangles that make up the mesh
    pub triangles: Triangles,
}

/// A list of vertices, as a struct mainly to comply with easier serde xml
#[derive(ToXml, PartialEq, Clone, Debug)]
#[xml(rename = "vertices")]
pub struct Vertices {
    pub vertex: Vec<Vertex>,
}

impl<'xml> FromXml<'xml> for Vertices {
    fn matches(id: Id<'_>, _field: Option<Id<'_>>) -> bool {
        id == ::instant_xml::Id {
            ns: "",
            name: "vertices",
        }
    }

    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        field: &'static str,
        deserializer: &mut Deserializer<'cx, 'xml>,
    ) -> Result<(), Error> {
        if into.is_some() {
            return Err(Error::DuplicateValue(field));
        }

        let mut vertices: Vec<Vertex> = Vec::with_capacity(30000);

        while let Some(node) = deserializer.next() {
            if let Ok(n) = node
                && let de::Node::Open(element) = n
            {
                //println!("This is element value {:?}", element);
                let mut vertex_value: Option<Vertex> = None;
                let mut nested = deserializer.nested(element);

                if Vertex::deserialize(&mut vertex_value, "vertex", &mut nested).is_ok()
                    && let Some(vertex) = vertex_value
                {
                    vertices.push(vertex);
                };
            }
        }

        vertices.shrink_to_fit();
        *into = Some(Vertices { vertex: vertices });

        Ok(())
    }

    type Accumulator = Option<Self>;
    const KIND: Kind = Kind::Scalar;
}

/// A vertex in a triangle mesh
#[derive(ToXml, PartialEq, Clone, Debug)]
#[xml(rename = "vertex")]
pub struct Vertex {
    #[xml(attribute)]
    pub x: f64,

    #[xml(attribute)]
    pub y: f64,

    #[xml(attribute)]
    pub z: f64,
}

impl<'xml> FromXml<'xml> for Vertex {
    #[inline]
    fn matches(id: ::instant_xml::Id<'_>, _: Option<::instant_xml::Id<'_>>) -> bool {
        id == ::instant_xml::Id {
            ns: "",
            name: "vertex",
        }
    }
    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        _: &'static str,
        deserializer: &mut ::instant_xml::Deserializer<'cx, 'xml>,
    ) -> ::std::result::Result<(), ::instant_xml::Error> {
        use ::instant_xml::Error;
        use ::instant_xml::de::Node;
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        while let Some(node) = deserializer.next() {
            let node = node?;
            match node {
                Node::Attribute(attr) => {
                    let id = deserializer.attribute_id(&attr)?;
                    // println!("Attr value: {:?}", attr.value);
                    match id.name {
                        "x" => {
                            x = parse(attr.value.as_ref()).unwrap_or_default();
                        }

                        "y" => {
                            y = parse(attr.value.as_ref()).unwrap_or_default();
                        }
                        "z" => {
                            z = parse(attr.value.as_ref()).unwrap_or_default();
                        }
                        _ => {
                            let mut nested =
                                deserializer.for_node(Node::AttributeValue(attr.value));
                            nested.ignore()?;
                        }
                    }
                }
                Node::Open(data) => {
                    let mut nested = deserializer.nested(data);
                    nested.ignore()?;
                }
                Node::Text(_) => {}
                _ => {
                    return Err(Error::UnexpectedNode("Unexpected".to_owned()));
                }
            }
        }
        *into = Some(Self { x, y, z });
        Ok(())
    }
    type Accumulator = Option<Self>;
    const KIND: ::instant_xml::Kind = ::instant_xml::Kind::Element;
}

/// A list of triangles, as a struct mainly to comply with easier serde xml
#[derive(ToXml, PartialEq, Clone, Debug)]
#[xml(rename = "triangles")]
pub struct Triangles {
    pub triangle: Vec<Triangle>,
}

impl<'xml> FromXml<'xml> for Triangles {
    fn matches(id: Id<'_>, _field: Option<Id<'_>>) -> bool {
        id == ::instant_xml::Id {
            ns: "",
            name: "triangles",
        }
    }

    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        field: &'static str,
        deserializer: &mut Deserializer<'cx, 'xml>,
    ) -> Result<(), Error> {
        if into.is_some() {
            return Err(Error::DuplicateValue(field));
        }

        let mut triangles: Vec<Triangle> = Vec::with_capacity(10000);
        while let Some(node) = deserializer.next() {
            if let Ok(n) = node
                && let de::Node::Open(element) = n
            {
                let mut triangle_value: Option<Triangle> = None;
                let mut nested = deserializer.nested(element);
                if Triangle::deserialize(&mut triangle_value, field, &mut nested).is_ok()
                    && let Some(vertex) = triangle_value
                {
                    triangles.push(vertex);
                }
            }
        }

        triangles.shrink_to_fit();
        *into = Some(Triangles {
            triangle: triangles,
        });

        Ok(())
    }

    type Accumulator = Option<Self>;

    const KIND: Kind = Kind::Element;
}

/// A triangle in a triangle mesh
///
/// The triangle consists of indices that refer to the vertices of the mesh. See
/// [`TriangleMesh`].
#[derive(ToXml, PartialEq, Clone, Debug)]
#[xml(rename = "triangle")]
pub struct Triangle {
    #[xml(attribute)]
    pub v1: usize,

    #[xml(attribute)]
    pub v2: usize,

    #[xml(attribute)]
    pub v3: usize,

    #[xml(attribute)]
    pub p1: Option<usize>,

    #[xml(attribute)]
    pub p2: Option<usize>,

    #[xml(attribute)]
    pub p3: Option<usize>,

    #[xml(attribute)]
    pub pid: Option<usize>,
}

impl<'xml> FromXml<'xml> for Triangle {
    #[inline]
    fn matches(id: ::instant_xml::Id<'_>, _: Option<::instant_xml::Id<'_>>) -> bool {
        id == ::instant_xml::Id {
            ns: "",
            name: "triangle",
        }
    }
    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        _: &'static str,
        deserializer: &mut ::instant_xml::Deserializer<'cx, 'xml>,
    ) -> ::std::result::Result<(), ::instant_xml::Error> {
        use ::instant_xml::Error;
        use ::instant_xml::de::Node;
        let mut v1: usize = 0;
        let mut v2: usize = 0;
        let mut v3: usize = 0;
        let mut p1: Option<usize> = None;
        let mut p2: Option<usize> = None;
        let mut p3: Option<usize> = None;
        let mut pid: Option<usize> = None;

        while let Some(node) = deserializer.next() {
            let node = node?;
            match node {
                Node::Attribute(attr) => {
                    let id = deserializer.attribute_id(&attr)?;
                    // println!("Attr value: {:?}", attr.value);
                    match id.name {
                        "v1" => {
                            v1 = attr.value.parse().unwrap_or_default();
                        }

                        "v2" => {
                            v2 = attr.value.parse().unwrap_or_default();
                        }
                        "v3" => {
                            v3 = attr.value.parse().unwrap_or_default();
                        }
                        "p1" => {
                            if let Ok(value) = attr.value.parse::<usize>() {
                                p1 = Some(value);
                            }
                        }
                        "p2" => {
                            if let Ok(value) = attr.value.parse::<usize>() {
                                p2 = Some(value);
                            }
                        }
                        "p3" => {
                            if let Ok(value) = attr.value.parse::<usize>() {
                                p3 = Some(value);
                            }
                        }
                        "pid" => {
                            if let Ok(value) = attr.value.parse::<usize>() {
                                pid = Some(value);
                            }
                        }

                        _ => {
                            let mut nested =
                                deserializer.for_node(Node::AttributeValue(attr.value));
                            nested.ignore()?;
                        }
                    }
                }
                Node::Open(data) => {
                    let mut nested = deserializer.nested(data);
                    nested.ignore()?;
                }
                Node::Text(_) => {}
                _ => {
                    return Err(Error::UnexpectedNode("Unexpected".to_owned()));
                }
            }
        }
        *into = Some(Self {
            v1,
            v2,
            v3,
            p1,
            p2,
            p3,
            pid,
        });
        Ok(())
    }
    type Accumulator = Option<Self>;
    const KIND: ::instant_xml::Kind = ::instant_xml::Kind::Element;
}

#[cfg(test)]
pub mod tests {
    use instant_xml::{from_str, to_string};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    pub fn toxml_vertex_test() {
        let xml_string = r#"<vertex x="100.5" y="100" z="0" />"#.to_string();
        let vertex = Vertex {
            x: 100.5,
            y: 100.0,
            z: 0.0,
        };
        let vertex_string = to_string(&vertex).unwrap();

        assert_eq!(vertex_string, xml_string);
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
        let xml_string = r#"<vertices><vertex x="100" y="110.5" z="0" /><vertex x="0.156" y="55.6896" z="-10" /></vertices>"#.to_string();
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

        assert_eq!(vertices_string, xml_string)
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
        let xml_string = r#"<triangle v1="1" v2="2" v3="3" />"#.to_string();
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
        let xml_string = r#"<triangle v1="1" v2="2" v3="3" />"#.to_string();
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
    pub fn toxml_triangles_test() {
        let xml_string = r#"<triangles><triangle v1="1" v2="2" v3="3" /><triangle v1="2" v2="3" v3="4" /></triangles>"#.to_string();
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
    pub fn toxml_mesh_test() {
        let xml_string = r##"<mesh><vertices><vertex x="-1" y="-1" z="0" /><vertex x="1" y="-1" z="0" /><vertex x="1" y="1" z="0" /><vertex x="-1" y="1" z="0" /></vertices><triangles><triangle v1="0" v2="1" v3="2" /><triangle v1="0" v2="2" v3="3" /></triangles></mesh>"##.to_string();
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
            // trianglesets: None,
        };
        let mesh_string = to_string(&mesh).unwrap();

        assert_eq!(mesh_string, xml_string);
    }

    #[test]
    pub fn fromxml_mesh_test() {
        let xml_string = r##"<mesh><vertices><vertex x="-1" y="-1" z="0" /><vertex x="1" y="-1" z="0" /><vertex x="1" y="1" z="0" /><vertex x="-1" y="1" z="0" /></vertices><triangles><triangle v1="0" v2="1" v3="2" /><triangle v1="0" v2="2" v3="3" /></triangles></mesh>"##.to_string();
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
                // trianglesets: None,
            }
        )
    }
}
