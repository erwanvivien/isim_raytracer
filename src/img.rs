//! Basic image representation with `Color` list
use crate::color::Color;
use std::fs;
use std::io::Write;

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

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[allow(dead_code)]
    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn push(&mut self, c: Color) {
        self.pixels.push(c);
    }

    #[allow(dead_code)]
    pub fn save(&self, filename: &str) -> bool {
        if self.pixels.len() != (self.height * self.width) {
            return false;
        }

        let header = format!(
            "P6 {width} {height} 255\n",
            width = self.width,
            height = self.height
        );

        let mut content = header;

        for chunks in self.pixels.chunks(23) {
            let mut s = String::with_capacity(69);
            for c in chunks {
                let (r, g, b) = c.values();
                s.push(char::from(r));
                s.push(char::from(g));
                s.push(char::from(b));
            }

            content.push_str(&*s);
            content.push('\n');
        }

        let buffer = fs::File::create(filename);
        if buffer.is_err() {
            return false;
        }

        let content = content.as_bytes();
        let mut buffer = buffer.unwrap();

        let mut pos = 0;
        while pos < content.len() {
            let bytes_written = buffer.write(&content[pos..]);
            if bytes_written.is_err() {
                return false;
            }

            pos += bytes_written.unwrap();
        }

        true
    }

    #[allow(dead_code)]
    pub fn save_png(&self, filename: &str) -> bool {
        let tmp = &self
            .pixels
            .iter()
            .map(|c| [c.r(), c.g(), c.b()])
            .flatten()
            .collect::<Vec<u8>>();

        image::save_buffer(
            filename,
            &tmp[..],
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::img::Image;
    use image::GenericImageView;

    #[test]
    fn new() {
        let image = Image::new(16, 16);
        assert_eq!(image.pixels.capacity(), image.height * image.width)
    }

    fn _check(img: &Image, path: &str) -> bool {
        // let _ = img.save_png("images/test_80x80_v1.png");
        let buffer = image::io::Reader::open(path).unwrap().decode().unwrap();

        for (_nb, (x, y, color)) in buffer.pixels().enumerate() {
            let x = x as usize;
            let y = y as usize;
            let pixel = img.pixels()[y * 80 + x];

            if [pixel.r(), pixel.g(), pixel.b(), 255] != color.0 {
                return false;
            }
        }

        return true;
    }
}
