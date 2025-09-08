use opencascade::{
    hlr::{self, EdgeType, EdgeVis},
    primitives::{Compound, IntoShape, Shape},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    // Create a shape that is uniquely identifiable from all 6 +/-(XYZ) axes
    let c = Workplane::xy()
        .sketch()
        .line_to(1.0, 0.0)
        .line_to(1.0, 2.0)
        .line_to(0.0, 2.0)
        .close()
        .to_face()
        .extrude(glam::dvec3(0.0, 0.0, 3.0))
        .into_shape();
    let ls = Shape::sphere(0.5).at(glam::dvec3(1.0, 2.0, 3.0)).build();
    let ss = Shape::sphere(0.25).at(glam::dvec3(0.0, 0.0, 3.0)).build();
    let shape = c.union(&ls).union(&ss).into_shape();

    // Create a vector of shapes to combine for display
    let mut shapes: Vec<Shape> = vec![];

    // Create right, rear, top views
    for (i, n) in [
        //
        glam::DVec3::X,
        glam::DVec3::Y,
        glam::DVec3::Z,
    ]
    .iter()
    .enumerate()
    {
        // Use the `hlr` module to "filter" for edges that are "visible" to the created coordinate system
        let (_, mut p) = hlr::filter(
            &shape,
            [(EdgeType::Sharp, EdgeVis::V), (EdgeType::OutLine, EdgeVis::V)],
            &glam::DVec3::ZERO,
            &n, // The vector will be normalized and derive a coherent coordinate system by OCC
            true,
        );
        // Move the projected curves down to see them all
        p.set_global_translation(glam::dvec3(0.0, 0.0, -1.0 * (i as f64) - 1.0));
        shapes.push(p);
    }

    // Show the projected shape
    shapes.push(shape);
    Compound::from_shapes(shapes).into()
}
