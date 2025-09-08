use crate::{
    primitives::{Compound, Shape},
    transform,
};
use glam::{DMat4, DVec3};
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

pub enum PlaneSpec {
    OriginNormal(DVec3, DVec3),
    Matrix(DMat4),
}

/// Filter edges from the input shape that match the types and visibility when
/// "viewed" by the input plane.
///
/// E.g. for a unit cube in +X,+Y,+Z and a plane at (-1, -1, -1) with (normalized)
/// normal vector (1, 1, 1), filtered for "sharp, visible" edges, we would
/// expect to see 9 edges:
/// - 3 for the edges meeting at the origin (inner edges of 3 visible faces)
/// - 6 for the outer edges of the 3 visible faces
pub fn filter(
    shape: &Shape,
    edge_types: impl IntoIterator<Item = (EdgeType, EdgeVis)>,
    plane_type: &PlaneSpec,
    project_edges_to_plane: bool,
) -> Shape {
    let algo = ffi::Handle_HLRBRep_Algo_ctor();
    ffi::HLRBRep_Algo_Add(&algo, &shape.inner);
    let proj = match plane_type {
        PlaneSpec::OriginNormal(origin, normal) => {
            ffi::HLRAlgo_Projector_from_ax2(&ffi::gp_Ax2_ctor(
                &ffi::new_point(origin.x, origin.y, origin.z),
                &ffi::gp_Dir_ctor(normal.x, normal.y, normal.z),
            ))
        },
        PlaneSpec::Matrix(m) => {
            let t = transform::gp_trsf(m);
            ffi::HLRAlgo_Projector_from_trsf(&t, false, 0.0)
        },
    };
    ffi::HLRBRep_Algo_Projector(&algo, &proj);
    ffi::HLRBRep_Algo_Update(&algo);
    ffi::HLRBRep_Algo_Hide(&algo);

    let mut ts = ffi::HLRBRep_HLRToShape_ctor(&algo);
    let mut shapes = vec![];
    for (typ, vis) in edge_types {
        let ts_pin = ts.pin_mut();
        let res = ffi::HLRBRep_HLRToShape_CompoundOfEdges(
            ts_pin,
            typ.to_occ(),
            vis.to_occ(),
            !project_edges_to_plane,
        );
        if !res.IsNull() {
            ffi::BRepLibBuildCurves3d(&res);
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

        let res = filter(
            &c,
            [(EdgeType::Sharp, EdgeVis::V)],
            &PlaneSpec::OriginNormal(glam::DVec3::ZERO, glam::dvec3(1.0, 1.0, 1.0)),
            true,
        );
        let edges = res.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 9);
    }

    #[test]
    fn project_outline() {
        let c = Shape::cube(1.0);

        // A cube should have no outline as its edges are all sharp
        let res = filter(
            &c,
            [(EdgeType::OutLine, EdgeVis::V)],
            &PlaneSpec::OriginNormal(glam::DVec3::ZERO, glam::dvec3(1.0, 1.0, 1.0)),
            true,
        );
        let edges = res.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 0);

        let c = Shape::sphere(1.0).build();

        // A sphere should have a single, circular outline
        let res = filter(
            &c,
            [(EdgeType::OutLine, EdgeVis::V)],
            &PlaneSpec::OriginNormal(glam::DVec3::ZERO, glam::dvec3(1.0, 1.0, 1.0)),
            true,
        );
        let edges = res.edges().collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
    }
}
