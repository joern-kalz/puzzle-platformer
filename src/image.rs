use image::ImageReader;

pub struct Image {
    pub data: Vec<u8>,
    pub width: usize,
    pub _height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            data: vec![0; width * height * 4],
            width,
            _height: height,
        }
    }

    pub fn new_from_asset(data: &[u8], width: usize, height: usize) -> Image {
        let data = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode sprite sheet");

        // Convert to RGBA bytes
        let data = data.to_rgba8().into_raw();

        Image { data, width, _height: height }
    }

    pub fn draw(&mut self, x: usize, y: usize, source: &Image, source_x: usize, source_y: usize, width: usize, height: usize) {
        let mut src_line = (source_y * source.width + source_x) * 4;
        let mut dest_line = (y * self.width + x) * 4;

        for _ in 0..height {
            let mut src_index = src_line;
            let mut dest_index = dest_line;

            for _ in 0..width {
                if source.data[src_index + 3] > 0 { 
                    self.data[dest_index..dest_index + 4].copy_from_slice(&source.data[src_index..src_index + 4]);
                }

                src_index += 4;
                dest_index += 4;
            }

            src_line += source.width * 4;
            dest_line += self.width * 4;
        }
    }
}