use crate::tuple::{Color, Tuple};
use std::collections::HashMap;
use std::io::{self, Write};

pub struct Canvas {
    width: i32,
    height: i32,
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

    pub fn write_pixel(&mut self, x: i32, y: i32, c: Color) {
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

    pub fn to_ppm<T: Write>(&self, w: &mut T) -> io::Result<()> {
        let mut lines = Vec::new();

        lines.push(String::from("P3"));
        lines.push(format!("{} {}", self.width, self.height));
        lines.push(String::from("255"));

        for y in 0..self.height {
            let mut colors_in_row = Vec::new();

            for x in 0..self.width {
                let color = self.pixel_at(x, y);
                colors_in_row.push(format!(
                    "{} {} {}",
                    color.red(),
                    color.green(),
                    color.blue()
                ));
            }

            lines.push(colors_in_row.join(" "));
        }

        write!(w, "{}", lines.join("\n"))
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl From<Tuple> for Coordinate {
    fn from(t: Tuple) -> Coordinate {
        Coordinate { x: t.x as i32, y: t.y as i32 }
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
    #[should_panic(
        expected = "{x, y} values must be inside canvas limits { width: 10, height: 20 }"
    )]
    fn write_pixel_outside_canvas() {
        let mut c = Canvas::new(10, 20);

        c.write_pixel(100, 100, Color::new(1.0, 2.0, 3.0));
    }

    #[test]
    #[should_panic(
        expected = "{x, y} values must be inside canvas limits { width: 10, height: 20 }"
    )]
    fn get_pixel_outside_canvas() {
        let c = Canvas::new(10, 20);

        c.pixel_at(100, 100);
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let mut f = NamedTempFile::new().unwrap();

        c.to_ppm(&mut f).unwrap();

        let f = File::open(f.path()).unwrap();
        let reader = BufReader::new(f);

        let mut lines = reader.lines().map(|l| l.unwrap());

        assert_eq!(lines.next(), Some(String::from("P3")));
        assert_eq!(lines.next(), Some(String::from("5 3")));
        assert_eq!(lines.next(), Some(String::from("255")));
    }

    #[test]
    fn constructing_ppm_body() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

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
            Some(String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"))
        );
        assert_eq!(
            lines.next(),
            Some(String::from("0 0 0 0 0 0 0 127 0 0 0 0 0 0 0"))
        );
        assert_eq!(
            lines.next(),
            Some(String::from("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"))
        );
    }
    /*
    #[test]
    fn ppm_splitting_long_lines() {
        let mut c = Canvas::new(10, 2);

        for y in 0..c.height {
            for x in 0..c.width {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
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
        lines.next();

        assert_eq!(
            lines.next(),
            Some(String::from(
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
            ))
        );
        assert_eq!(
            lines.next(),
            Some(String::from(
                "153 255 204 153 255 204 153 255 204 153 255 204 153"
            ))
        );
    } */
}
