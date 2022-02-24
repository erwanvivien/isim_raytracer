use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
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

impl Add<i64> for Point {
    type Output = Point;
    fn add(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x + rhs,
            z: self.z + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<i64> for Point {
    type Output = Point;
    fn sub(self, rhs: i64) -> Self::Output {
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

impl Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
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
        let v1 = Point::new(1, 2, -4);
        let v2 = Point::new(5, 6, -7);

        let v = v1 + v2;

        assert_eq!(v, Point::new(6, 8, -11))
    }

    #[test]
    fn add_num() {
        let v1 = Point::new(1, 2, -4);

        let v = v1 + 5;

        assert_eq!(v, Point::new(6, 7, 1))
    }

    #[test]
    fn sub_vec() {
        let v1 = Point::new(1, 2, -4);
        let v2 = Point::new(5, 6, -7);

        let v = v1 - v2;

        assert_eq!(v, Point::new(-4, -4, 3))
    }

    #[test]
    fn sub_num() {
        let v1 = Point::new(1, 2, -4);

        let v = v1 - 3;

        assert_eq!(v, Point::new(-2, -1, -7))
    }

    #[test]
    fn mul_num() {
        let mut v1 = Point::new(2, 4, 0);
        v1 = v1 * 5;

        assert_eq!(v1, Point::new(10, 20, 0))
    }
}
