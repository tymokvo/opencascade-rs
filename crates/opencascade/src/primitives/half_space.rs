use super::{face::Face, solid::Solid};
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

pub struct HalfSpace {
    pub(crate) inner: UniquePtr<ffi::BRepPrimAPI_MakeHalfSpace>,
}
impl HalfSpace {
    /// Create a `HalfSpace` from a `Face` and a point that indicates on which
    /// side of the face the solid half of the space should be located.
    pub fn new(face: &Face, point: &DVec3) -> Self {
        Self {
            inner: ffi::BRepPrimAPI_MakeHalfSpace_ctor(
                &face.inner,
                &ffi::new_point(point.x, point.y, point.z),
            ),
        }
    }

    /// Create a `HalfSpace` from a `Face`, using its center and normal to
    /// create the positive side. The `normal_vector_sign` can be used to invert
    /// the side of the face that is used to create the solid.
    pub fn from_face(face: &Face, normal_vector_sign: f64) -> Self {
        let p = face.center_of_mass() + face.normal_at_center() * normal_vector_sign.signum();
        Self::new(face, &p)
    }

    pub fn solid(&self) -> Solid {
        Solid::from_solid(self.inner.Solid())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{primitives::IntoShape, workplane::Workplane};
    use glam::DVec3;

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

    fn contains(solid: &Solid, point: &DVec3, tolerance: f64) -> ffi::TopAbs_State {
        let mut sc = ffi::BRepClass3d_SolidClassifier_ctor();
        sc.pin_mut().Load(&solid.into_shape().inner);
        sc.pin_mut().Perform(&ffi::new_point(point.x, point.y, point.z), tolerance);
        sc.State()
    }

    #[test]
    fn create_new() {
        let s = HalfSpace::new(&unit_face(), &glam::dvec3(0.0, 0.0, 1.0)).solid();

        assert_eq!(
            //
            contains(&s, &glam::dvec3(0.0, 0.0, 0.0), 0.01),
            ffi::TopAbs_State::TopAbs_ON
        );
        assert_eq!(
            //
            contains(&s, &glam::dvec3(0.0, 0.0, 0.1), 0.01),
            ffi::TopAbs_State::TopAbs_IN
        );
        assert_eq!(
            // NOTE: f64::MAX is not contained. From simple trial-and-error, this
            // is the largest value that is still contained.
            contains(&s, &glam::dvec3(0.0, 0.0, 2.0 * 10f64.powf(100.0)), 0.01),
            ffi::TopAbs_State::TopAbs_IN
        );
        assert_eq!(
            //
            contains(&s, &glam::dvec3(0.0, 0.0, -0.1), 0.01),
            ffi::TopAbs_State::TopAbs_OUT
        );
    }

    #[test]
    fn create_from_face() {
        // Test a face with an inverted normal
        let s = HalfSpace::from_face(&unit_face(), -1.0).solid();

        assert_eq!(
            //
            contains(&s, &glam::dvec3(0.0, 0.0, 0.1), 0.01),
            ffi::TopAbs_State::TopAbs_OUT
        );

        assert_eq!(
            //
            contains(&s, &glam::dvec3(0.0, 0.0, -0.1), 0.01),
            ffi::TopAbs_State::TopAbs_IN
        );
    }
}
