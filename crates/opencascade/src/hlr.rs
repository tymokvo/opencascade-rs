use crate::primitives::{Compound, Shape};
use glam::DMat4;
use opencascade_sys::ffi;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Undefined,
    IsoLine,
    OutLine,
    // TODO: Consider naming these explicitly
    Rg1Line,
    RgNLine,
    Sharp,
}
impl EdgeType {
    pub fn to_occ(&self) -> ffi::HLRBRep_TypeOfResultingEdge {
        match self {
            EdgeType::Undefined => ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_Undefined,
            EdgeType::IsoLine => ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_IsoLine,
            EdgeType::OutLine => ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_OutLine,
            EdgeType::Rg1Line => ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_Rg1Line,
            EdgeType::RgNLine => ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_RgNLine,
            EdgeType::Sharp => ffi::HLRBRep_TypeOfResultingEdge::HLRBRep_Sharp,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EdgeVis {
    /// Edge is visible to the projection plane
    V,
    /// Edge is hidden from projection plane
    H,
}
impl EdgeVis {
    pub fn to_occ(&self) -> bool {
        match &self {
            Self::V => true,
            Self::H => false,
        }
    }
}

pub fn project(
    shape: &Shape,
    edge_types: impl IntoIterator<Item = (EdgeType, EdgeVis)>,
    _plane: &DMat4,
) -> Shape {
    let algo = ffi::Handle_HLRBRep_Algo_ctor();
    ffi::HLRBRep_Algo_Add(&algo, &shape.inner);
    let proj = ffi::HLRAlgo_Projector_from_ax2(&ffi::gp_Ax2_ctor(
        &ffi::new_point(0.0, 0.0, 0.0),
        &ffi::gp_Dir_ctor(1.0, 1.0, 1.0),
    ));
    ffi::HLRBRep_Algo_Projector(&algo, &proj);
    ffi::HLRBRep_Algo_Update(&algo);
    ffi::HLRBRep_Algo_Hide(&algo);

    let mut ts = ffi::HLRBRep_HLRToShape_ctor(&algo);
    let mut shapes = vec![];
    for (typ, vis) in edge_types {
        let ts_pin = ts.pin_mut();
        let res = ffi::HLRBRep_HLRToShape_CompoundOfEdges(ts_pin, typ.to_occ(), vis.to_occ(), true);
        if !res.IsNull() {
            shapes.push(Shape::from_shape(&res));
        }
    }
    Compound::from_shapes(shapes).into()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::Shape;

    #[test]
    fn project_simple() {
        let c = Shape::cube(1.0);

        let res = project(&c, [(EdgeType::Sharp, EdgeVis::V)], &glam::DMat4::IDENTITY);
        let edges = res.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 9);
    }

    #[test]
    fn project_outline() {
        let c = Shape::cube(1.0);

        // A cube should have no outline as its edges are all sharp
        let res = project(&c, [(EdgeType::OutLine, EdgeVis::V)], &glam::DMat4::IDENTITY);
        let edges = res.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 0);

        let c = Shape::sphere(1.0).build();

        // A sphere should have a single, circular outline
        let res = project(&c, [(EdgeType::OutLine, EdgeVis::V)], &glam::DMat4::IDENTITY);
        let edges = res.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
    }
}
