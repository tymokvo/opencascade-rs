use crate::primitives::{Edge, Shape, WireIterator};
use opencascade_sys::ffi;

/// A grouping of shapes containing open and closed wires that result from a call to [`dispatch_wires`]
pub struct DispatchWires {
    open: Shape,
    closed: Shape,
}
impl DispatchWires {
    pub fn open(&self) -> WireIterator {
        WireIterator::for_shape(&self.open)
    }
    pub fn closed(&self) -> WireIterator {
        WireIterator::for_shape(&self.closed)
    }
}

/// Issue a call to `ShapeAnalysis_FreeBounds::DispatchWires` using an iterator
/// of edges as the input. This function takes in an `HSequenceOfShape`
/// containing edges and attempts to join them into groups of open and closed
/// wires.
///
/// The result type wraps the grouping of wires that have resulted from the
/// analysis.
pub fn dispatch_wires(edges: impl Iterator<Item = Edge>, max_join_distance: f64) -> DispatchWires {
    let mut edge_seq = ffi::new_HandleTopTools_HSequenceOfShape();

    for e in edges {
        ffi::TopTools_HSequenceOfShape_append(
            edge_seq.pin_mut(),
            ffi::cast_edge_to_shape(&e.inner),
        );
    }

    let mut wires = ffi::new_HandleTopTools_HSequenceOfShape();
    ffi::connect_edges_to_wires(edge_seq.pin_mut(), max_join_distance, false, wires.pin_mut());

    let mut closed = ffi::TopoDS_Compound_ctor();
    let mut open = ffi::TopoDS_Compound_ctor();
    ffi::dispatch_wires(&wires, closed.pin_mut(), open.pin_mut());

    DispatchWires {
        open: Shape::from_shape(&ffi::TopoDS_Compound_as_shape(open)),
        closed: Shape::from_shape(&ffi::TopoDS_Compound_as_shape(closed)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::Edge;

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
        .map(|(a, b)| Edge::segment(a, b));

        let dw = dispatch_wires(edges.into_iter(), 0.1);

        assert_eq!(dw.closed().count(), 2);
        assert_eq!(dw.open().count(), 0);
    }
}
