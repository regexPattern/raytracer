use crate::tuple::{Color, Point};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::white();
        let position = Point::new(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
