use crate::color::Color;
use std::collections::HashMap;
use std::io::{self, Write};

const PPM_TEXTWIDTH: u8 = 70;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pixels: HashMap<Coordinate, Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: HashMap::new(),
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, c: Color) -> Result<(), String> {
        if !self.contains(x, y) {
            return Err(
                format!(
                    "Invalid `x: {}` and `y: {}` values. Must be inside canvas limits {{ width: {}, height: {} }}",
                    x, y, self.width, self.height
                )
            );
        }

        let coordinate = Coordinate { x, y };
        self.pixels.insert(coordinate, c);

        Ok(())
    }

    fn pixel_at(&self, x: i32, y: i32) -> Result<Color, String> {
        if !self.contains(x, y) {
            return Err(
                format!(
                    "Invalid `x: {}` and `y: {}` values. Must be inside canvas limits {{ width: {}, height: {} }}",
                    x, y, self.width, self.height
                )
            );
        }

        let coordinate = Coordinate { x, y };
        let color = match self.pixels.get(&coordinate) {
            Some(pixel) => pixel.to_owned(),
            None => Color::new(0.0, 0.0, 0.0),
        };

        Ok(color)
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    pub fn to_ppm<T: Write>(&self, w: &mut T) -> io::Result<()> {
        let mut lines = Vec::new();

        lines.push("P3".to_string());
        lines.push(format!("{} {}", self.width, self.height));
        lines.push("255".to_string());

        for y in 0..self.height {
            let mut line = String::new();

            for x in 0..self.width {
                let pixel = self.pixel_at(x, y).unwrap();
                for color in [pixel.red(), pixel.green(), pixel.blue()].iter() {
                    let byte = &format!("{} ", color);
                    if line.len() + byte.len() > PPM_TEXTWIDTH.into() {
                        lines.push(line.trim().to_string());
                        line.clear();
                    }
                    line.push_str(byte);
                }
            }

            lines.push(line.trim().to_string());
        }

        writeln!(w, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use tempfile::NamedTempFile;

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
        let c = Canvas::new(10, 20);

        assert_eq!(c.pixel_at(5, 5), Ok(Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, color).unwrap();

        assert_eq!(canvas.pixel_at(2, 3), Ok(color));
    }

    #[test]
    fn canvas_contains_coordinate() {
        let c = Canvas::new(10, 20);

        assert!(c.contains(5, 5));
        assert!(!c.contains(100, 100));
    }

    #[test]
    fn write_pixel_outside_canvas() {
        let mut c = Canvas::new(10, 20);

        assert_eq!(
            c.write_pixel(100, 100, Color::new(1.0, 2.0, 3.0)),
            Err("Invalid `x: 100` and `y: 100` values. Must be inside canvas limits { width: 10, height: 20 }".to_string())
        );
    }

    #[test]
    fn get_pixel_outside_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(
            c.pixel_at(100, 100),
            Err("Invalid `x: 100` and `y: 100` values. Must be inside canvas limits { width: 10, height: 20 }".to_string())
        );
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let mut f = NamedTempFile::new().unwrap();

        c.to_ppm(&mut f).unwrap();

        let f = File::open(f.path()).unwrap();
        let reader = BufReader::new(f);

        let mut lines = reader.lines().map(|l| l.unwrap());

        assert_eq!(lines.next(), Some("P3".to_string()));
        assert_eq!(lines.next(), Some("5 3".to_string()));
        assert_eq!(lines.next(), Some("255".to_string()));
    }

    #[test]
    fn constructing_ppm_body() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1).unwrap();
        c.write_pixel(2, 1, c2).unwrap();
        c.write_pixel(4, 2, c3).unwrap();

        let mut f = NamedTempFile::new().unwrap();

        c.to_ppm(&mut f).unwrap();

        let f = File::open(f.path()).unwrap();
        let reader = BufReader::new(f);

        let mut lines = reader.lines().map(|l| l.unwrap());

        lines.next();
        lines.next();
        lines.next();

        assert_eq!(
            lines.next(),
            Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0".to_string())
        );
        assert_eq!(
            lines.next(),
            Some("0 0 0 0 0 0 0 127 0 0 0 0 0 0 0".to_string())
        );
        assert_eq!(
            lines.next(),
            Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255".to_string())
        );
    }

    #[test]
    fn ppm_splitting_long_lines() {
        let mut c = Canvas::new(10, 2);

        for y in 0..c.height {
            for x in 0..c.width {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6)).unwrap();
            }
        }

        let mut f = NamedTempFile::new().unwrap();

        c.to_ppm(&mut f).unwrap();

        let f = File::open(f.path()).unwrap();
        let reader = BufReader::new(f);

        let mut lines = reader.lines().map(|l| l.unwrap());

        lines.next();
        lines.next();
        lines.next();

        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204".to_string())
        );
        assert_eq!(
            lines.next(),
            Some("153 255 204 153 255 204 153 255 204 153 255 204 153".to_string())
        );
    }

    #[test]
    fn ppm_ended_by_newline() {
        let c = Canvas::new(3, 1);
        let mut f = NamedTempFile::new().unwrap();

        c.to_ppm(&mut f).unwrap();

        let f = File::open(f.path()).unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(reader.fill_buf().unwrap().last(), Some(&b'\n'));
    }
}
