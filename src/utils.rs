pub const EPSILON: f64 = 0.00001;

pub fn approximately_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floats_are_approximately_equal() {
        assert!(approximately_eq(1.0, 1.0));
        assert!(approximately_eq(-1.0, -1.0));
        assert!(approximately_eq(0.0, 0.0 + 0.00001));
        assert!(!approximately_eq(0.0, 0.0 + 0.0001));
    }
}
