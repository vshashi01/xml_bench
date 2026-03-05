use std::path::PathBuf;

// #[cfg(feature = "instant-xml")]
// use xml_bench::read_mesh_instant_xml;

// #[cfg(feature = "yaserde")]
// use xml_bench::read_mesh_yaserde;

// #[cfg(feature = "xmlity")]
// use xml_bench::read_mesh_xmlity;

// #[cfg(feature = "fast-float")]
// use xml_bench::read_mesh_instant_xml_with_fast_float;

#[cfg_attr(feature = "hotpath", hotpath::main)]
fn main() {
    let path = PathBuf::from("data/mesh-yaserde.xml")
        .canonicalize()
        .unwrap();

    #[cfg(feature = "instant-xml")]
    {
        use xml_bench::read_mesh_instant_xml;
        use xml_bench::write_mesh_instant_xml;

        println!("Reading using instant-xml from : {:?}", path);
        let mesh = read_mesh_instant_xml(&path, 1205286, 573311);
        println!("Writing using instant-xml");
        write_mesh_instant_xml(&mesh, 89695113);
    }

    #[cfg(feature = "yaserde")]
    {
        use xml_bench::read_mesh_yaserde;
        use xml_bench::write_mesh_yaserde;

        println!("Reading using Yaserde from : {:?}", path);
        let mesh = read_mesh_yaserde(&path, 1205286, 573311);
        println!("Writing using Yaserde");
        write_mesh_yaserde(&mesh, 89695151);
    }

    #[cfg(feature = "xmlity")]
    {
        use xml_bench::read_mesh_xmlity;
        use xml_bench::write_mesh_xmlity;

        //xmlity does not currently work with xml with initial element for XML identifiers and encodings.
        let path = PathBuf::from("data/mesh-xmlity.xml")
            .canonicalize()
            .unwrap();
        println!("Reading using Xmlity from : {:?}", path);
        let mesh = read_mesh_xmlity(&path, 1205286, 573311);
        println!("Writing using Xmlity");
        write_mesh_xmlity(&mesh, 87916516);
    }

    #[cfg(feature = "roxmltree")]
    {
        use xml_bench::read_mesh_roxmltree;

        println!("Reading using roxmltree from : {:?}", path);
        let _ = read_mesh_roxmltree(&path, 1205286, 573311);
    }

    #[cfg(feature = "fast-float")]
    {
        use xml_bench::read_mesh_instant_xml_with_fast_float;

        println!(
            "Reading using instant_xml with fast_float2 parsing from : {:?}",
            path
        );
        let _ = read_mesh_instant_xml_with_fast_float(&path, 1205286, 573311);
    }
}
