use core::f64;

use opencascade::{
    hlr::{self, EdgeType, EdgeVis, PlaneSpec},
    primitives::{Compound, IntoShape, Shape},
    workplane::Workplane,
};

fn gizmo() -> Shape {
    let xy = Workplane::xy().sketch().line_to(1.0, 0.0).move_to(0.0, 0.0).line_to(0.0, 1.0).wire();
    let z = Workplane::xz().sketch().line_to(0.0, 1.0).wire();
    let c = Workplane::xy().circle(0.0, 0.0, 0.5);
    let ltr_x = Workplane::xy()
        .sketch()
        .move_to(0.9, 0.0)
        .line_to(0.95, 0.05)
        .line_to(1.0, 0.1)
        .move_to(1.0, 0.0)
        .line_to(0.95, 0.05)
        .line_to(0.9, 0.1)
        .wire();
    let ltr_y = Workplane::xy()
        .sketch()
        .move_to(0.0, 0.9)
        .line_to(0.05, 0.95)
        .line_to(0.1, 1.0)
        .move_to(0.05, 0.95)
        .line_to(0.0, 1.0)
        .wire();
    Compound::from_shapes(vec![
        xy.into_shape(),
        z.into_shape(),
        c.into_shape(),
        ltr_x.into_shape(),
        ltr_y.into_shape(),
    ])
    .into()
}

pub fn shape() -> Shape {
    let c = Shape::cube(1.0);
    let ls = Shape::sphere(0.5).at(glam::dvec3(1.0, 0.0, 1.0)).build();
    let ss = Shape::sphere(0.25).at(glam::dvec3(0.0, 1.0, 1.0)).build();
    let shape = c.union(&ls).union(&ss).into_shape();
    let mut shapes: Vec<Shape> = vec![];

    for ps in [
        // PlaneType::OriginNormal(glam::dvec3(0.0, 0.0, 0.0), glam::dvec3(-1.0, 0.0, 0.0)),
        PlaneSpec::Matrix(glam::dmat4(
            glam::dvec4(0.0, 1.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, -1.0, 0.0),
            glam::dvec4(-1.0, 0.0, 0.0, 0.0),
            glam::dvec4(4.0, 0.0, 0.0, 1.0),
        )),
        PlaneSpec::Matrix(glam::dmat4(
            glam::dvec4(0.0, 1.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, 1.0, 0.0),
            glam::dvec4(1.0, 0.0, 0.0, 0.0),
            glam::dvec4(6.0, 0.0, 0.0, 1.0),
        )),
        PlaneSpec::Matrix(glam::dmat4(
            glam::dvec4(-1.0, 0.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, -1.0, 0.0),
            glam::dvec4(0.0, -1.0, 0.0, 0.0),
            glam::dvec4(0.0, 4.0, 0.0, 1.0),
        )),
        PlaneSpec::Matrix(glam::dmat4(
            glam::dvec4(1.0, 0.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, -1.0, 0.0),
            glam::dvec4(0.0, -1.0, 0.0, 0.0),
            glam::dvec4(0.0, 6.0, 0.0, 1.0),
        )),
    ] {
        let p = hlr::filter(
            &shape,
            [(EdgeType::Sharp, EdgeVis::V), (EdgeType::OutLine, EdgeVis::V)],
            &ps,
            true,
        );
        shapes.push(p);
        match ps {
            PlaneSpec::Matrix(m) => {
                shapes.push(gizmo().transform(&m));
            },
            _ => {},
        }
    }

    shapes.push(shape);
    Compound::from_shapes(shapes).into()
}
