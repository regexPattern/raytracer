use crate::{
    color::Color,
    matrix::{self, Matrix},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Scheme {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix<4, 4>,
}

impl Scheme {
    pub const fn new(a: Color, b: Color) -> Self {
        let transform = matrix::IDENTITY4X4;

        Self { a, b, transform }
    }
}

#[cfg(test)]
mod tests {
    use crate::color;

    use super::*;

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = Scheme::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.transform, matrix::IDENTITY4X4);
    }
}
