use cxx::UniquePtr;
use glam::DMat4;
use opencascade_sys::ffi;

/// Create a `gp_Trsf` from a `DMat4`.
///
/// Note that OCC only allows setting values for the upper 3x4 matrix. I.e. the
/// XYZ components of each column.
///
/// Additionally, OCC ensures orthogonality of the matrix before the method
/// returns.
pub fn gp_trsf(mat: &DMat4) -> UniquePtr<ffi::gp_Trsf> {
    let mut t = ffi::new_transform();
    t.pin_mut().SetValues(
        mat.x_axis.x,
        mat.y_axis.x,
        mat.z_axis.x,
        mat.w_axis.x,
        mat.x_axis.y,
        mat.y_axis.y,
        mat.z_axis.y,
        mat.w_axis.y,
        mat.x_axis.z,
        mat.y_axis.z,
        mat.z_axis.z,
        mat.w_axis.z,
    );
    t
}

pub fn dmat4(trsf: UniquePtr<ffi::gp_Trsf>) -> DMat4 {
    let mut m = ffi::Mat4_Double_ctor();
    trsf.GetMat4(m.pin_mut());
    let mut mat = glam::DMat4::ZERO;
    for r in 0usize..=3 {
        for c in 0usize..=3 {
            mat.col_mut(c)[r] = m.GetValue(r, c);
        }
    }
    mat
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set_gp_trsf_values() {
        // Matrix with permuted axes
        let m = glam::dmat4(
            glam::dvec4(0.0, 0.0, 1.0, 0.0),
            glam::dvec4(1.0, 0.0, 0.0, 0.0),
            glam::dvec4(0.0, 1.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, 0.0, 1.0),
        );

        let t = gp_trsf(&m);

        // 3D diagonal is 0
        assert_eq!(t.Value(1, 1), 0.0);
        assert_eq!(t.Value(2, 2), 0.0);
        assert_eq!(t.Value(3, 3), 0.0);

        // Permuted axes are preserved
        assert_eq!(t.Value(1, 2), 1.0);
        assert_eq!(t.Value(3, 1), 1.0);
        assert_eq!(t.Value(2, 3), 1.0);
    }

    #[test]
    fn set_dmat4_values() {
        let m = glam::dmat4(
            glam::dvec4(0.0, 0.0, 1.0, 0.0),
            glam::dvec4(1.0, 0.0, 0.0, 0.0),
            glam::dvec4(0.0, 1.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, 0.0, 1.0),
        );

        let t = gp_trsf(&m);

        let mm = dmat4(t);

        assert_eq!(m, mm);
    }
}
