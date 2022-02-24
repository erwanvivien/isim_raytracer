pub struct Image {
    height: usize,
    width: usize,
    pixels: Vec<u32>,
}

impl Image {
    fn new(height: usize, width: usize) -> Self {
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
        assert_eq!(image.pixels.capacity(), 16 * 16)
    }
}
