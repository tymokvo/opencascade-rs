use super::{face::Face, solid::Solid};
use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct HalfSpace {
    pub(crate) inner: UniquePtr<ffi::BRepPrimAPI_MakeHalfSpace>,
}
impl HalfSpace {
    pub fn new(face: &Face) -> Self {
        let p = face.center_of_mass() + face.normal_at_center();
        Self {
            inner: ffi::BRepPrimAPI_MakeHalfSpace_ctor(&face.inner, &ffi::new_point(p.x, p.y, p.z)),
        }
    }

    pub fn solid(&self) -> Solid {
        Solid::from_solid(self.inner.Solid())
    }
}

#[cfg(test)]
mod test {
    use crate::workplane::Workplane;

    use super::*;

    fn unit_face() -> Face {
        let sk = Workplane::xy().sketch();
        // Create a right triangle with CCW winding in quadrant 1
        // This is to ensure that the derived normal is along the positive Z axis
        Face::from_wire(
            &sk //
                .line_to(1.0, 0.0)
                .line_to(0.0, 1.0)
                .line_to(0.0, 0.0)
                .wire(),
        )
    }

    #[test]
    fn create_half_space() {
        let f = unit_face();

        let s = HalfSpace::new(&f).solid();

        assert!(s.contains(&glam::dvec3(0.0, 0.0, 0.001), 0.0001));
        assert!(!s.contains(&glam::dvec3(0.0, 0.0, -0.001), 0.0001));
    }
}
