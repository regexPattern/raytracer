use std::collections::HashMap;

use crate::color::{self, Color};

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: HashMap<Pixel, Color>,
}

#[derive(Eq, Hash, PartialEq)]
struct Pixel {
    x: u32,
    y: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: HashMap::new(),
        }
    }

    fn pixel_at(&self, x: u32, y: u32) -> &Color {
        let pixel = Pixel { x, y };
        self.pixels.get(&pixel).unwrap_or(&color::BLACK)
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        let pixel = Pixel { x, y };
        self.pixels.insert(pixel, color);
    }

    pub fn to_ppm(&self) -> String {
        let header = self.gen_ppm_header();
        let data = self.gen_ppm_data();

        format!("{}\n{}\n", header, data)
    }

    fn gen_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height)
    }

    fn gen_ppm_data(&self) -> String {
        const MAX_LINE_WIDTH: usize = 70;

        let mut lines = Vec::new();

        for y in 0..self.height {
            let mut line = String::with_capacity(MAX_LINE_WIDTH);

            for x in 0..self.width {
                let pixel = self.pixel_at(x, y).clamp();
                let colors = [pixel.red, pixel.green, pixel.blue];

                for color in colors {
                    let color = format!("{} ", color);
                    if line.len() + color.len() > MAX_LINE_WIDTH {
                        lines.push(line.trim().to_owned());
                        line.clear();
                    }

                    line.push_str(&color);
                }
            }

            lines.push(line.trim().to_owned());
        }

        lines.join("\n")
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
                assert_eq!(c.pixel_at(x, y), &color::BLACK);
            }
        }
    }

    #[test]
    fn writing_pixel_to_a_canvas() {
        let mut c = Canvas::new(10, 20);

        c.write_pixel(2, 3, color::RED);

        assert_eq!(c.pixel_at(2, 3), &color::RED);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);

        assert_eq!(
            c.gen_ppm_header(),
            "\
P3
5 3
255"
            .to_owned()
        );
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
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

        assert_eq!(
            c.gen_ppm_data(),
            "\
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 127 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
                .to_owned()
        );
    }

    #[test]
    fn constructing_the_entire_ppm() {
        let mut c = Canvas::new(5, 3);
        let color = Color {
            red: 1.5,
            green: 0.0,
            blue: 0.0,
        };

        c.write_pixel(0, 0, color);

        assert_eq!(
            c.to_ppm(),
            "\
P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"
            .to_owned()
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color {
            red: 1.0,
            green: 0.8,
            blue: 0.6,
        };

        for x in 0..c.width {
            for y in 0..c.height {
                c.write_pixel(x, y, color);
            }
        }

        assert_eq!(
            c.gen_ppm_data(),
            "\
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"
                .to_owned()
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);

        let ppm = c.to_ppm();

        assert_eq!(ppm.as_bytes().last().unwrap(), &b'\n');
    }
}
