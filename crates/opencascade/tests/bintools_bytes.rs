use opencascade::primitives::{Shape, ShapeType};

#[test]
fn round_trips_shape_through_bintools_bytes() {
    let shape = Shape::box_from_corners([0.0, 0.0, 0.0].into(), [1.0, 2.0, 3.0].into());
    let bytes = shape.to_brep_bin_bytes();
    assert!(!bytes.is_empty());

    let decoded = Shape::from_brep_bin_bytes(&bytes).expect("BinTools bytes should deserialize");
    assert_eq!(decoded.shape_type(), ShapeType::Solid);
}

#[test]
fn rejects_invalid_bintools_bytes() {
    assert!(Shape::from_brep_bin_bytes(b"not a BinTools shape").is_err());
}
