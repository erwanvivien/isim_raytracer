use crate::vector::Vector;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn to_vec(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f64> for Point {
    type Output = Point;
    fn add(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x + rhs,
            z: self.z + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f64> for Point {
    type Output = Point;
    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            z: -self.z,
            y: -self.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

mod tests {
    use crate::point::Point;

    #[test]
    fn add_vec() {
        let v1 = Point::new(1f64, 2f64, -4f64);
        let v2 = Point::new(5f64, 6f64, -7f64);

        let v = v1 + v2;

        assert_eq!(v, Point::new(6f64, 8f64, -11f64))
    }

    #[test]
    fn add_num() {
        let v1 = Point::new(1f64, 2f64, -4f64);

        let v = v1 + 5f64;

        assert_eq!(v, Point::new(6f64, 7f64, 1f64))
    }

    #[test]
    fn sub_vec() {
        let v1 = Point::new(1f64, 2f64, -4f64);
        let v2 = Point::new(5f64, 6f64, -7f64);

        let v = v1 - v2;

        assert_eq!(v, Point::new(-4f64, -4f64, 3f64))
    }

    #[test]
    fn sub_num() {
        let v1 = Point::new(1f64, 2f64, -4f64);

        let v = v1 - 3f64;

        assert_eq!(v, Point::new(-2f64, -1f64, -7f64))
    }

    #[test]
    fn mul_num() {
        let mut v1 = Point::new(2f64, 4f64, 0f64);
        v1 = v1 * 5f64;

        assert_eq!(v1, Point::new(10f64, 20f64, 0f64))
    }
}
