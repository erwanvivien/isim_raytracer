#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

type ColorTuple = (u8, u8, u8);

impl Color {
    const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    const RED: Color = Color {
        r: 255,
        ..Color::BLACK
    };
    const GREEN: Color = Color {
        g: 255,
        ..Color::BLACK
    };
    const BLUE: Color = Color {
        b: 255,
        ..Color::BLACK
    };
    const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    fn values(&self) -> ColorTuple {
        (self.r, self.g, self.b)
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

mod tests {
    use crate::color::Color;

    #[test]
    fn red() {
        assert_eq!(Color::RED.values(), (255, 0, 0))
    }

    #[test]
    fn green() {
        assert_eq!(Color::GREEN.values(), (0, 255, 0))
    }

    #[test]
    fn blue() {
        assert_eq!(Color::BLUE.values(), (0, 0, 255))
    }
}
