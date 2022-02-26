//! Color representation

use crate::Vector;

/// Color representation in rgb
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

type ColorTuple = (u8, u8, u8);

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const RED: Color = Color {
        r: 255,
        ..Color::BLACK
    };
    pub const GREEN: Color = Color {
        g: 255,
        ..Color::BLACK
    };
    pub const BLUE: Color = Color {
        b: 255,
        ..Color::BLACK
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const fn to_vec(&self) -> Vector {
        Vector::new(self.r as f64, self.g as f64, self.b as f64)
    }

    fn values(&self) -> ColorTuple {
        (self.r, self.g, self.b)
    }

    fn value(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8
    }

    fn from_tuple(tuple: ColorTuple) -> Color {
        Color {
            r: tuple.0,
            g: tuple.1,
            b: tuple.2,
        }
    }

    fn from_values(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl From<Vector> for Color {
    fn from(v: Vector) -> Self {
        let mut x = v.x;
        let mut y = v.y;
        let mut z = v.z;

        let x = v.x.clamp(0f64, 255f64);
        let y = v.y.clamp(0f64, 255f64);
        let z = v.z.clamp(0f64, 255f64);

        Color {
            r: x as u8,
            g: y as u8,
            b: z as u8,
        }
    }
}

mod tests {
    use crate::color::Color;

    #[test]
    fn colors() {
        assert_eq!(Color::RED.values(), (255, 0, 0));
        assert_eq!(Color::GREEN.values(), (0, 255, 0));
        assert_eq!(Color::BLUE.values(), (0, 0, 255));
    }

    #[test]
    fn from_tuple() {
        assert_eq!(Color::WHITE, Color::from_tuple((255, 255, 255)))
    }

    #[test]
    fn from_values() {
        assert_eq!(Color::RED, Color::from_values(255, 0, 0))
    }

    #[test]
    fn to_value() {
        assert_eq!(0xFF000000, Color::RED.value())
    }
}
