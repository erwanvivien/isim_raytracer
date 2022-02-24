use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
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

impl Add<f64> for Vector {
    type Output = Vector;
    fn add(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x + rhs,
            z: self.z + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f64> for Vector {
    type Output = Vector;
    fn sub(self, rhs: f64) -> Self::Output {
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
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
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
        let v1 = Vector::new(1f64, 2f64, -4f64);
        let v2 = Vector::new(5f64, 6f64, -7f64);

        let v = v1 + v2;

        assert_eq!(v, Vector::new(6f64, 8f64, -11f64))
    }

    #[test]
    fn add_num() {
        let v1 = Vector::new(1f64, 2f64, -4f64);

        let v = v1 + 5f64;

        assert_eq!(v, Vector::new(6f64, 7f64, 1f64))
    }

    #[test]
    fn sub_vec() {
        let v1 = Vector::new(1f64, 2f64, -4f64);
        let v2 = Vector::new(5f64, 6f64, -7f64);

        let v = v1 - v2;

        assert_eq!(v, Vector::new(-4f64, -4f64, 3f64))
    }

    #[test]
    fn sub_num() {
        let v1 = Vector::new(1f64, 2f64, -4f64);

        let v = v1 - 3f64;

        assert_eq!(v, Vector::new(-2f64, -1f64, -7f64))
    }

    #[test]
    fn norm_vec() {
        let v1 = Vector::new(2f64, 4f64, 7f64);
        let v2 = Vector::new(-2f64, 1f64, 0f64);

        assert_eq!(v1 * v2, 0f64)
    }

    #[test]
    fn mul_num() {
        let mut v1 = Vector::new(2f64, 4f64, 0f64);
        v1 = v1 * 5f64;

        assert_eq!(v1, Vector::new(10f64, 20f64, 0f64))
    }
}
