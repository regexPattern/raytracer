use std::ops::Add;

#[derive(Copy, Clone, Debug)]
struct Tuple(f64, f64, f64);

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let diff = self - other;
        let sum: f64 = [diff.0, diff.1, diff.2].iter().sum();
        sum.abs() > f64::EPSILON
    }
}

impl Add for &Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    tuple: Tuple,
    w: f64,
}

impl Coordinate {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            tuple: Tuple(x, y, z),
            w,
        }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.tuple == other.tuple && (self.w - other.w).abs() <= f64::EPSILON
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            tuple: self.tuple + other.tuple,
            w: self.w + other.w,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point(Coordinate);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vector(Coordinate);

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Coordinate::new(x, y, z, 1.))
    }
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Coordinate::new(x, y, z, 0.))
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point(self.0 + other.0)
    }
}

#[derive(Copy, Clone, Debug)]
struct Color(Tuple);

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Tuple(r, g, b))
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}
