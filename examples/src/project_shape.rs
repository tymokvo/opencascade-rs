use opencascade::primitives::Shape;

pub fn shape() -> Shape {
    Shape::sphere(16.0).build()
}
