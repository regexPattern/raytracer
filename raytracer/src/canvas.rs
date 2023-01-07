use std::collections::HashMap;

use image::{ImageBuffer, Rgb, RgbImage};

use crate::color::{self, Color};

#[derive(Debug)]
pub struct Canvas {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pixels: HashMap<(u32, u32), Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = HashMap::new();

        Self {
            width,
            height,
            pixels,
        }
    }

    pub(crate) fn pixel_at(&self, x: u32, y: u32) -> &Color {
        self.pixels.get(&(x, y)).unwrap_or(&color::consts::BLACK)
    }

    pub(crate) fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels.insert((x, y), color);
    }

    pub fn to_image(&self) -> RgbImage {
        let mut img_buf = ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let Color { red, green, blue } = self.pixel_at(x, y);

            let red = (red * 255.0) as u8;
            let green = (green * 255.0) as u8;
            let blue = (blue * 255.0) as u8;

            *pixel = Rgb([red, green, blue]);
        }

        img_buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for x in 0..c.width {
            for y in 0..c.height {
                assert_eq!(c.pixel_at(x, y), &color::consts::BLACK);
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);

        c.write_pixel(2, 3, color::consts::RED);

        assert_eq!(c.pixel_at(2, 3), &color::consts::RED);
    }

    #[test]
    fn creating_an_image_buffer_from_a_canvas_pixels() {
        let mut c = Canvas::new(5, 3);

        let c1 = Color {
            red: 1.5,
            green: 0.0,
            blue: 0.0,
        };

        let c2 = Color {
            red: 0.0,
            green: 0.5,
            blue: 0.0,
        };

        let c3 = Color {
            red: -0.5,
            green: 0.0,
            blue: 1.0,
        };

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let img = c.to_image();

        assert_eq!(img[(0, 0)], Rgb([255, 0, 0]));
        assert_eq!(img[(2, 1)], Rgb([0, 127, 0]));
        assert_eq!(img[(4, 2)], Rgb([0, 0, 255]));
    }
}
