use opencascade::{
    hlr::{self, EdgeType, EdgeVis},
    primitives::{Compound, IntoShape, Shape, Wire},
};

pub fn shape() -> Shape {
    let c = Shape::cube(1.0);
    let p = hlr::filter(&c, [(EdgeType::Sharp, EdgeVis::V)], &glam::DMat4::IDENTITY, true);
    let mut shapes: Vec<Shape> = vec![];
    for e in p.edges() {
        shapes.push(e.into_shape());
    }
    Compound::from_shapes(shapes).into()
}
