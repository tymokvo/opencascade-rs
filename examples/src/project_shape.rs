use opencascade::{
    angle::Angle,
    hlr,
    primitives::{Compound, IntoShape, Shape},
    shape_analysis,
    workplane::Workplane,
};

fn shape_to_project() -> Shape {
    let wp = Workplane::xy()
        .rotated(opencascade::angle::RVec {
            x: Angle::Degrees(45.0),
            y: Angle::Degrees(45.0),
            z: Angle::Degrees(0.0),
        })
        .translated(glam::dvec3(0.0, 0.0, 16.0));
    wp.rect(4.0, 8.0).to_face().extrude(wp.normal()).into_shape()
}

fn project(shape: &Shape) -> Shape {
    let mut projections = vec![];
    for face in shape.faces() {
        let (_, projection) = hlr::filter(
            &face.into_shape(),
            [(hlr::EdgeType::Sharp, hlr::EdgeVis::V)],
            &glam::DVec3::ZERO,
            &glam::DVec3::Z,
            true,
        );
        let dw = shape_analysis::dispatch_wires(projection.edges(), 0.01);
        for closed in dw.closed() {
            projections.push(closed.to_face().into_shape());
        }
    }
    Compound::from_shapes(projections).into_shape()
}

pub fn shape() -> Shape {
    let axes = Workplane::xy()
        .sketch()
        .line_to(2.0, 0.0)
        .move_to(0.0, 0.0)
        .line_to(0.0, 1.0)
        .wire()
        .into_shape();
    let projectee = shape_to_project();

    let projection = project(&projectee);

    Compound::from_shapes([axes, projectee, projection]).into_shape()
}
