use opencascade::{
    hlr::{self, EdgeType, EdgeVis},
    primitives::{Compound, Shape},
};

pub fn shape() -> Shape {
    let c = Shape::cube(1.0);

    let p = hlr::project(&c, [(EdgeType::Sharp, EdgeVis::V)], &glam::DMat4::IDENTITY);

    Compound::from_shapes(&[c, p]).into()
}
