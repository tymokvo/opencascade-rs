use crate::primitives::{Compound, Shape};
use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct ShapeAnalysisFreeBounds {
    pub(crate) inner: UniquePtr<ffi::ShapeAnalysis_FreeBounds>,
}
impl ShapeAnalysisFreeBounds {
    pub fn new(shape: &Shape, tolerance: f64, split_closed: bool, split_open: bool) -> Self {
        Self {
            inner: ffi::ShapeAnalysis_FreeBounds_ctor(
                &shape.inner,
                tolerance,
                split_closed,
                split_open,
            ),
        }
    }

    pub fn closed_wires(&self) -> Compound {
        Compound::from_compound(self.inner.GetClosedWires())
    }

    pub fn open_wires(&self) -> Compound {
        Compound::from_compound(self.inner.GetOpenWires())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::{Edge, IntoShape, WireIterator};

    fn v(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> glam::DVec3 {
        glam::dvec3(x.into(), y.into(), z.into())
    }

    #[test]
    fn connect_edges() {
        let edges = [
            // Create disconnected edges of a 1x2x0 rectangle
            (v(0, 0, 0), v(0, 2, 0)),
            (v(0, 0, 0), v(1, 0, 0)),
            (v(1, 2, 0), v(1, 0, 0)),
            (v(1, 2, 0), v(0, 2, 0)),
            // Create disconnected edges of a 1x2x0 rectangle, disjoint from the first
            (v(3, 0, 0), v(3, 2, 0)),
            (v(3, 0, 0), v(4, 0, 0)),
            (v(4, 2, 0), v(4, 0, 0)),
            (v(4, 2, 0), v(3, 2, 0)),
        ]
        .map(|(a, b)| Edge::segment(a, b).into_shape());

        let edge_compound = Compound::from_shapes(edges);

        let sa = ShapeAnalysisFreeBounds::new(&edge_compound.into_shape(), 0.001, false, true);

        let wires = WireIterator {
            explorer: ffi::TopExp_Explorer_ctor(
                &sa.closed_wires().into_shape().inner,
                ffi::TopAbs_ShapeEnum::TopAbs_WIRE,
            ),
        };

        assert_eq!(wires.count(), 2);
    }
}
