use std::collections::HashMap;

use crate::tuple::Color;

struct Canvas {
    width: i32,
    height: i32,
    pixels: HashMap<Coordinate, Color>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: HashMap::new(),
        }
    }

    fn write_pixel(&mut self, x: i32, y: i32, c: Color) {
        if !self.is_inside_canvas(x, y) {
            panic!(
                "{{x, y}} values must be inside canvas limits {{ width: {}, height: {} }}",
                self.width, self.height
            );
        }

        let coordinate = Coordinate { x, y };
        self.pixels.insert(coordinate, c);
    }

    fn pixel_at(&self, x: i32, y: i32) -> Color {
        if !self.is_inside_canvas(x, y) {
            panic!(
                "{{x, y}} values must be inside canvas limits {{ width: {}, height: {} }}",
                self.width, self.height
            );
        }

        let coordinate = Coordinate { x, y };
        match self.pixels.get(&coordinate) {
            Some(pixel) => pixel.to_owned(),
            None => Color::new(0.0, 0.0, 0.0),
        }
    }

    fn is_inside_canvas(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c.pixels.is_empty());
    }

    #[test]
    fn canvas_pixels_are_black() {
        let mut c = Canvas::new(10, 20);

        assert_eq!(c.pixel_at(5, 5), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, color);

        assert_eq!(canvas.pixel_at(2, 3), color);
    }

    #[test]
    fn coordinate_is_inside_canvas() {
        let c = Canvas::new(10, 20);

        assert!(c.is_inside_canvas(5, 5));
        assert!(!c.is_inside_canvas(100, 100));
    }

    #[test]
    #[should_panic(expected = "{x, y} values must be inside canvas limits { width: 10, height: 20 }")]
    fn write_pixel_outside_canvas() {
        let mut c = Canvas::new(10, 20);

        c.write_pixel(100, 100, Color::new(1.0, 2.0, 3.0));
    }

    #[test]
    #[should_panic(expected = "{x, y} values must be inside canvas limits { width: 10, height: 20 }")]
    fn get_pixel_outside_canvas() {
        let c = Canvas::new(10, 20);

        c.pixel_at(100, 100);
    }
}
