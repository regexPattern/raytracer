use crate::color::Color;
use crate::tuple::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Light {
    pub position: Point,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use crate::color;

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let position = Point::new(0.0, 0.0, 0.0);
        let intensity = color::WHITE;

        let light = Light {
            position,
            intensity,
        };

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
