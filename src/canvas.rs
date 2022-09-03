use std::collections::HashMap;
use std::io::{self, Write};

use crate::tuple::Color;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
    x: u32,
    y: u32,
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: HashMap<Coordinate, Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: HashMap::new(),
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), String> {
        let coordinate = Coordinate { x, y };
        if !self.contains(&coordinate) {
            return Err(format!(
                "invalid x: `{}` and y: `{}` values.\
                must be between canvas limits width: `{}` and height: `{}`",
                x, y, self.width, self.height
            ));
        }

        self.pixels.insert(coordinate, color);
        Ok(())
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Option<Color> {
        let coordinate = Coordinate { x, y };
        if !self.contains(&coordinate) {
            return None;
        }

        let color = match self.pixels.get(&coordinate) {
            Some(color) => *color,
            None => Color::black(),
        };

        Some(color)
    }

    fn contains(&self, coordinate: &Coordinate) -> bool {
        (0..self.width).contains(&coordinate.x) && (0..self.height).contains(&coordinate.y)
    }

    pub fn to_ppm(&self, w: &mut impl Write) -> Result<(), io::Error> {
        let mut buffer: Vec<String> = Vec::new();

        buffer.push("P3".to_string());
        buffer.push(format!("{} {}", self.width, self.height));
        buffer.push("255".to_string());

        for y in 0..self.height {
            let mut line = String::new();

            for x in 0..self.width {
                let pixel = self.pixel_at(x, y).unwrap();
                let colors = [
                    Color::clamp(pixel.red),
                    Color::clamp(pixel.green),
                    Color::clamp(pixel.blue),
                ];

                for color in colors {
                    let byte: String = format!("{} ", color);
                    if line.len() + byte.len() > 70 {
                        buffer.push(line.trim().to_string());
                        line.clear();
                    }
                    line.push_str(&byte);
                }
            }

            buffer.push(line.trim().to_string());
        }

        writeln!(w, "{}", buffer.join("\n"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use tempfile::NamedTempFile;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        assert_eq!(c.pixel_at(5, 5), Some(Color::black()));
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red).unwrap();

        assert_eq!(c.pixel_at(2, 3), Some(red));
    }

    #[test]
    fn writing_pixel_outside_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color::black();

        assert_eq!(
            canvas.write_pixel(100, 100, color),
            Err("invalid x: `100` and y: `100` values.\
                must be between canvas limits width: `10` and height: `20`"
                .to_string())
        );

        assert_eq!(canvas.pixel_at(100, 100), None);
    }

    #[test]
    fn canvas_contains_coordinate() {
        let c = Canvas::new(10, 20);

        assert!(c.contains(&Coordinate { x: 5, y: 5 }));
        assert!(!c.contains(&Coordinate { x: 100, y: 100 }));
    }

    #[test]
    #[ignore]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let mut file = NamedTempFile::new().unwrap();

        c.to_ppm(&mut file).unwrap();

        let file = File::open(file.path()).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines().map(|l| l.unwrap());

        assert_eq!(lines.next(), Some("P3".to_string()));
        assert_eq!(lines.next(), Some("5 3".to_string()));
        assert_eq!(lines.next(), Some("255".to_string()));
    }

    #[test]
    #[ignore]
    fn constructing_ppm_pixels_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1).unwrap();
        c.write_pixel(2, 1, c2).unwrap();
        c.write_pixel(4, 2, c3).unwrap();

        let mut file = NamedTempFile::new().unwrap();

        c.to_ppm(&mut file).unwrap();

        let file = File::open(file.path()).unwrap();
        let reader = BufReader::new(file);
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
        assert_eq!(lines.next(), None);
    }

    #[test]
    #[ignore]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);

        for x in 0..c.width {
            for y in 0..c.height {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6)).unwrap();
            }
        }

        let mut file = NamedTempFile::new().unwrap();

        c.to_ppm(&mut file).unwrap();

        let file = File::open(file.path()).unwrap();
        let reader = BufReader::new(file);
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
        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204".to_string())
        );
        assert_eq!(
            lines.next(),
            Some("153 255 204 153 255 204 153 255 204 153 255 204 153".to_string())
        );
        assert_eq!(lines.next(), None);
    }

    #[test]
    #[ignore]
    fn ppm_files_terminated_by_newline_char() {
        let c = Canvas::new(1, 1);

        let mut file = NamedTempFile::new().unwrap();

        c.to_ppm(&mut file).unwrap();

        let file = File::open(file.path()).unwrap();
        let mut reader = BufReader::new(file);

        assert_eq!(reader.fill_buf().unwrap().last(), Some(&b'\n'));
    }
}
