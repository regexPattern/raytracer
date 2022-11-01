use super::{Plane, Shapes, Sphere};

impl From<Plane> for Shapes {
    fn from(p: Plane) -> Self {
        Shapes::Plane(p)
    }
}

impl From<Sphere> for Shapes {
    fn from(s: Sphere) -> Self {
        Shapes::Sphere(s)
    }
}
