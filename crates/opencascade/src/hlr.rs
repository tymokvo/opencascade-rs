use crate::primitives::Shape;
use glam::DMat4;
use opencascade_sys::ffi;

pub fn project(shape: &Shape, _plane: &DMat4) -> Shape {
    let algo = ffi::HLRBRep_Algo_ctor();
    ffi::HLRBRep_Algo_Add(&algo, &shape.inner);
    let proj = ffi::HLRAlgo_Projector_from_ax2(&ffi::gp_Ax2_ctor(
        &ffi::new_point(0.0, 0.0, 0.0),
        &ffi::gp_Dir_ctor(1.0, 1.0, 1.0),
    ));
    ffi::HLRBRep_Algo_Projector(&algo, &proj);
    ffi::HLRBRep_Algo_Update(&algo);
    ffi::HLRBRep_Algo_Hide(&algo);

    let mut ts = ffi::HLRBRep_HLRToShape_ctor(&algo);
    let res = ffi::HLRBRep_HLRToShape_CompoundOfEdges(
        ts.pin_mut(),
        ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_Sharp,
        false,
        true,
    );
    Shape::from_shape(&res)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::Shape;

    #[test]
    fn project_simple() {
        let c = Shape::cube(1.0);

        let res = project(&c, &glam::DMat4::IDENTITY);
    }
}
