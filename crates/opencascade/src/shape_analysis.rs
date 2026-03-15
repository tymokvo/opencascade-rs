use crate::primitives::{Shape, WireIterator};
use opencascade_sys::ffi;

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

pub fn dispatch_wires(shape: &Shape, max_join_distance: f64) -> DispatchWires {
    let mut edges = ffi::new_HandleTopTools_HSequenceOfShape();

    let mut explorer = ffi::TopExp_Explorer_ctor(&shape.inner, ffi::TopAbs_ShapeEnum::TopAbs_EDGE);
    while explorer.More() {
        ffi::TopTools_HSequenceOfShape_append(edges.pin_mut(), explorer.Current());
        explorer.pin_mut().Next();
    }

    let mut wires = ffi::new_HandleTopTools_HSequenceOfShape();
    ffi::connect_edges_to_wires(edges.pin_mut(), max_join_distance, false, wires.pin_mut());

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
    use crate::primitives::{Compound, Edge, IntoShape};

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

        let dw = dispatch_wires(&Compound::from_shapes(edges).into_shape(), 0.1);

        assert_eq!(dw.closed().count(), 2);
        assert_eq!(dw.open().count(), 0);
    }
}
