use opencascade::{
    hlr::{self, EdgeType, EdgeVis, PlaneType},
    primitives::{Compound, IntoShape, Shape},
};

pub fn shape() -> Shape {
    let c = Shape::cube(1.0);
    let ls = Shape::sphere(0.5).at(glam::dvec3(1.0, 0.0, 1.0)).build();
    let ss = Shape::sphere(0.25).at(glam::dvec3(0.0, 1.0, 1.0)).build();
    let shape = c.union(&ls).union(&ss).into_shape();
    let mut shapes: Vec<Shape> = vec![];

    for (o, n) in [
        (glam::dvec3(0.0, 0.0, 2.0), glam::dvec3(0.0, 0.0, 1.0)),
        (glam::dvec3(0.0, 0.0, -2.0), glam::dvec3(0.0, 0.0, -1.0)),
    ] {
        let mut p = hlr::filter(
            &shape,
            [(EdgeType::Sharp, EdgeVis::V), (EdgeType::OutLine, EdgeVis::V)],
            &PlaneType::OriginNormal(o, n),
            true,
        );
        p.set_global_translation(o);
        shapes.push(p);
    }
    shapes.push(shape);
    Compound::from_shapes(shapes).into()
}
