const EPSILON: f64 = 0.00001;

#[derive(Copy, Clone, Debug)]
struct Tuple(f64, f64, f64);

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let self_fields = [self.0, self.1, self.2];
        let other_fields = [other.0, other.1, other.2];
        let fields = self_fields.iter().zip(other_fields);

        for (a, b) in fields {
            if (a - b).abs() > EPSILON {
                return false;
            }
        }

        true
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
        self.tuple == other.tuple && (self.w - other.w).abs() <= EPSILON
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let p1 = Point::new(1., 2., 3.);
        let p2 = Point::new(1., 2., 3.);
        println!("{:?}", p1);
        println!("{:?}", p2);
    }
}
