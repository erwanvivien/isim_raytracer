use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    pub fn new(x: i64, y: i64, z: i64) -> Vector {
        Vector { x, y, z }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i64> for Vector {
    type Output = Vector;
    fn add(self, rhs: i64) -> Self::Output {
        Vector {
            x: self.x + rhs,
            z: self.z + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<i64> for Vector {
    type Output = Vector;
    fn sub(self, rhs: i64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            z: -self.z,
            y: -self.y,
        }
    }
}

impl Mul for Vector {
    type Output = i64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: i64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

mod tests {
    use crate::vector::Vector;

    #[test]
    fn add_vec() {
        let v1 = Vector::new(1, 2, -4);
        let v2 = Vector::new(5, 6, -7);

        let v = v1 + v2;

        assert_eq!(v, Vector::new(6, 8, -11))
    }

    #[test]
    fn add_num() {
        let v1 = Vector::new(1, 2, -4);

        let v = v1 + 5;

        assert_eq!(v, Vector::new(6, 7, 1))
    }

    #[test]
    fn sub_vec() {
        let v1 = Vector::new(1, 2, -4);
        let v2 = Vector::new(5, 6, -7);

        let v = v1 - v2;

        assert_eq!(v, Vector::new(-4, -4, 3))
    }

    #[test]
    fn sub_num() {
        let v1 = Vector::new(1, 2, -4);

        let v = v1 - 3;

        assert_eq!(v, Vector::new(-2, -1, -7))
    }

    #[test]
    fn norm_vec() {
        let v1 = Vector::new(2, 4, 7);
        let v2 = Vector::new(-2, 1, 0);

        assert_eq!(v1 * v2, 0)
    }

    #[test]
    fn mul_num() {
        let mut v1 = Vector::new(2, 4, 0);
        v1 = v1 * 5;

        assert_eq!(v1, Vector::new(10, 20, 0))
    }
}
