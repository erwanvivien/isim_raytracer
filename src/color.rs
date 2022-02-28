//! Color representation

use crate::Vector;

/// Color representation in rgb
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Color {
    pub(crate) v: Vector,
}

type ColorTuple = (u8, u8, u8);

#[allow(dead_code)]
impl Color {
    pub fn r(&self) -> u8 {
        let v = self.v.clamp(0f64, 255f64);
        v.x as u8
    }

    pub fn g(&self) -> u8 {
        let v = self.v.clamp(0f64, 255f64);
        v.y as u8
    }

    pub fn b(&self) -> u8 {
        let v = self.v.clamp(0f64, 255f64);
        v.z as u8
    }

    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            v: Vector::new(r as f64, g as f64, b as f64),
        }
    }

    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255);

    pub const fn to_vec(&self) -> Vector {
        self.v
    }

    pub fn values(&self) -> ColorTuple {
        (self.r(), self.g(), self.b())
    }

    fn value(&self) -> u32 {
        let (r, g, b) = self.values();
        (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8
    }
}

impl From<Vector> for Color {
    fn from(v: Vector) -> Self {
        Color {
            v: v.clamp(0f64, 255f64),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn colors() {
        assert_eq!(Color::RED.values(), (255, 0, 0));
        assert_eq!(Color::GREEN.values(), (0, 255, 0));
        assert_eq!(Color::BLUE.values(), (0, 0, 255));
    }

    #[test]
    fn to_value() {
        assert_eq!(0xFF000000, Color::RED.value())
    }
}
