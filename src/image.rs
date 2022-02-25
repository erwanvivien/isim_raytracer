//! Basic image representation with `Color` list
use crate::color::Color;

/// Contains list of `color::Color`
pub struct Image {
    height: usize,
    width: usize,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(height: usize, width: usize) -> Self {
        Image {
            height,
            width,
            pixels: Vec::with_capacity(height * width),
        }
    }
}

mod tests {
    use crate::image::Image;

    #[test]
    fn new() {
        let image = Image::new(16, 16);
        assert_eq!(image.pixels.capacity(), image.height * image.width)
    }
}
