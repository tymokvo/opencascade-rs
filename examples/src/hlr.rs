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
    let mut shapes: Vec<Shape> = vec![];

    for (i, tr) in [
        glam::DMat4::from_cols_array_2d(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 10.0, 1.0],
        ]),
        glam::DMat4::from_cols_array_2d(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        glam::DMat4::from_cols_array_2d(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        glam::DMat4::from_cols_array_2d(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        glam::DMat4::from_cols_array_2d(&[
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        glam::DMat4::from_cols_array_2d(&[
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
    ]
    .iter()
    .enumerate()
    {
        let p = hlr::filter(
            &shape.transform(&tr),
            [(EdgeType::Sharp, EdgeVis::V), (EdgeType::OutLine, EdgeVis::V)],
            &PlaneSpec::OriginNormal(glam::DVec3::ZERO, glam::DVec3::Z),
            true,
        );
        shapes.push(p.transform(&glam::DMat4::from_translation(glam::dvec3(
            0.0,
            0.0,
            (i as f64) * -1.0 - 1.0,
        ))));
    }

    shapes.push(shape);
    Compound::from_shapes(shapes).into()
}
