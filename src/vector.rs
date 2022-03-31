//! Vector3
use crate::point::Point;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Vector3
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub const ZERO: Vector = Vector::new(0f64, 0f64, 0f64);
    pub const ONE: Vector = Vector::new(1f64, 1f64, 1f64);

    pub const fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    /// Magnitude: sqrt(sum_i(a_i ^ 2))
    pub fn mag(&self) -> f64 {
        (self.x.powf(2f64) + self.y.powf(2f64) + self.z.powf(2f64)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.mag();
        *self / mag
    }

    #[allow(dead_code)]
    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn powf(&self, e: f64) -> Vector {
        Vector {
            x: self.x.powf(e),
            y: self.y.powf(e),
            z: self.z.powf(e),
        }
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn mul(&self, v: Vector) -> Vector {
        Vector {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }

    pub fn div(&self, v: Vector) -> Vector {
        Vector {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }

    #[allow(dead_code)]
    pub fn normal_vec(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.y * v2.z - v2.y * v1.z,
            y: -(v1.x * v2.z - v2.x * v1.z),
            z: v1.x * v2.y - v2.x * v1.y,
        }
    }

    pub fn cross_product(&self, v: &Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn rotate_around(&self, k: &Vector, angle: f64) -> Vector {
        // from here https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula
        // test here https://www.vcalc.com/wiki/vCalc/V3+-+Vector+Rotation
        let cos = angle.cos();
        let sin = angle.sin();

        let k = k.normalize();

        let v1 = *self * cos;
        let v2 = k.cross_product(self) * sin;
        let v3 = k * (k * *self) * (1f64 - cos);

        return v1 + v2 + v3;
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vector {
        Vector {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }

    pub fn map(&self, f: fn(f64) -> f64) -> Vector {
        Vector {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn max(&self) -> f64 {
        *[self.x, self.y, self.z]
            .iter()
            .max_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap()
    }

    pub fn min(&self) -> f64 {
        *[self.x, self.y, self.z]
            .iter()
            .min_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap()
    }
}

impl Add<Vector> for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f64> for Vector {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            z: self.z + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f64> for Vector {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
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
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

#[cfg(test)]
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

    #[test]
    fn div_num() {
        let mut v1 = Vector::new(10f64, 20f64, 0f64);
        v1 = v1 / 5f64;

        assert_eq!(v1, Vector::new(2f64, 4f64, 0f64))
    }

    #[test]
    fn normal_vec1() {
        let v1 = Vector::new(2f64, 3f64, 4f64);
        let v2 = Vector::new(1f64, 2f64, 3f64);

        let nm = Vector::new(1f64, -2f64, 1f64);
        assert_eq!(nm, Vector::normal_vec(&v1, &v2))
    }

    #[test]
    fn normal_vec2() {
        let v1 = Vector::new(2f64, 3f64, 4f64);
        let v2 = Vector::new(1f64, 2f64, 3f64);

        let nm = Vector::new(-1f64, 2f64, -1f64);
        assert_eq!(nm, Vector::normal_vec(&v2, &v1))
    }

    #[test]
    fn cross_product() {
        let v1 = Vector::new(1f64, 3f64, 5f64);
        let v2 = Vector::new(2f64, 4f64, 5f64);

        let cross = v1.cross_product(&v2);
        assert_eq!(cross, Vector::new(-5f64, 5f64, -2f64))
    }

    #[test]
    fn rotate1() {
        let v1 = Vector::new(1f64, 1f64, 1f64);
        let v2 = Vector::new(-1f64, 1f64, -1f64);

        let r = v2.rotate_around(&v1, std::f64::consts::PI);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(0.33, -1.67, 0.33)")
    }

    #[test]
    fn rotate2() {
        let v1 = Vector::new(0f64, 1f64, 0f64);
        let v2 = Vector::new(1f64, 1f64, 1f64);

        let r = v2.rotate_around(&v1, std::f64::consts::PI);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(-1.00, 1.00, -1.00)")
    }

    #[test]
    fn rotate3() {
        let v1 = Vector::new(0f64, -1f64, 0f64);
        let v2 = Vector::new(1f64, 1f64, 1f64);

        let r = v2.rotate_around(&v1, std::f64::consts::PI);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);

        assert_eq!(&*r, "(-1.00, 1.00, -1.00)")
    }

    macro_rules! triplet {
        ($a:tt, $b:expr, $c:expr) => {
            Vector::new(f64::from($a), f64::from($b), f64::from($c))
        };
    }

    #[test]
    fn rotate4() {
        let v1 = Vector::new(1f64, 0.5f64, 0f64);
        let v2 = Vector::new(-0.5f64, 1f64, 0f64);

        let r = v1.rotate_around(&v2, std::f64::consts::PI / 4f64);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(0.71, 0.35, -0.79)")
    }

    #[test]
    fn rotate5() {
        let v1 = triplet!(16, 9, 16);
        let v2 = triplet!(15, 6, 1);

        let r = v1.rotate_around(&v2, std::f64::consts::PI / 4f64);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(20.31, -1.34, 13.36)")
    }

    #[test]
    fn rotate6() {
        let v1 = triplet!(10, 21, 5);
        let v2 = triplet!(11, 1, 17);

        let r = v1.rotate_around(&v2, std::f64::consts::PI / 4f64);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(-3.51, 19.01, 13.86)")
    }

    #[test]
    fn rotate7() {
        let v1 = triplet!(13, 15, 5);
        let v2 = triplet!(8, 9, 15);

        let r = v1.rotate_around(&v2, std::f64::consts::PI / 4f64);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(4.56, 18.54, 7.37)")
    }

    #[test]
    fn rotate8() {
        let v1 = triplet!(20, 12, 15);
        let v2 = triplet!(4, 20, 9);

        let r = v1.rotate_around(&v2, std::f64::consts::PI / 4f64);

        let r = format!("({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
        assert_eq!(&*r, "(21.30, 17.65, 1.86)")
    }
}
