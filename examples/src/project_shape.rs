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
    let rect = wp.rect(4.0, 8.0).to_face().extrude(wp.normal()).into_shape();
    let cyl = {
        // NOTE: Subtracting a circle here will cause a failure when the
        // subtraction results in a face that has multiple closed edge loops
        wp.circle(0.5, 0.0, 1.0).to_face().extrude(wp.normal()).into_shape()
    };
    rect.subtract(&cyl).into_shape()
}

fn project(shape: &Shape) -> Shape {
    let mut projections = vec![];
    let plane_normal = glam::DVec3::Z;
    for face in shape.faces() {
        if face.normal_at_center().dot(plane_normal) < 0.0 {
            let (_, projection) = hlr::filter(
                &face.into_shape(),
                [(hlr::EdgeType::Sharp, hlr::EdgeVis::V)],
                &glam::DVec3::ZERO,
                &plane_normal,
                true,
            );
            let dw = shape_analysis::dispatch_wires(projection.edges(), 0.01);
            for closed in dw.closed() {
                projections.push(closed.to_face().into_shape());
            }
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
