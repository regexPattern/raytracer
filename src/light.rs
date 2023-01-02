use crate::{color::Color, tuple::Point};

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use crate::color;

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = color::consts::WHITE;
        let position = Point::new(0.0, 0.0, 0.0);

        let light = PointLight {
            position,
            intensity,
        };

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn comparing_lights() {
        let l1 = PointLight {
            position: Point::new(1.0, 2.0, 3.0),
            intensity: color::consts::GREEN,
        };

        let l2 = PointLight {
            position: Point::new(1.0, 2.0, 3.0),
            intensity: color::consts::GREEN,
        };

        let l3 = PointLight {
            position: Point::new(0.0, 1.0, 2.0),
            intensity: color::consts::BLUE,
        };

        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
    }
}
