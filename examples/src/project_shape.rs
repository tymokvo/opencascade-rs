use opencascade::{
    angle::Angle,
    primitives::{Compound, IntoShape, Shape},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let axes = Workplane::xy()
        .sketch()
        .line_to(1.0, 0.0)
        .move_to(0.0, 0.0)
        .line_to(0.0, 1.0)
        .wire()
        .into_shape();
    let wp = Workplane::xy()
        .rotated(opencascade::angle::RVec {
            x: Angle::Degrees(45.0),
            y: Angle::Degrees(45.0),
            z: Angle::Degrees(0.0),
        })
        .translated(glam::dvec3(0.0, 0.0, 16.0));
    let extrusion = wp.rect(4.0, 8.0).to_face().extrude(wp.normal()).into_shape();
    Compound::from_shapes([axes, extrusion]).into_shape()
}
