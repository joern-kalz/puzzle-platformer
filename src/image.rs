use image::ImageReader;

pub struct Image {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        Image {
            data: vec![0; (width * height * 4) as usize],
            width,
            height: height,
        }
    }

    pub fn new_from_asset(data: &[u8], width: i32, height: i32) -> Image {
        let data = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode sprite sheet");

        // Convert to RGBA bytes
        let data = data.to_rgba8().into_raw();

        Image {
            data,
            width,
            height,
        }
    }

    pub fn draw(
        &mut self,
        x: i32,
        y: i32,
        source: &Image,
        source_x: i32,
        source_y: i32,
        width: i32,
        height: i32,
    ) {
        let mut src_line = (source_y * (source.width as i32) + source_x) * 4;
        let mut dest_line = (y * (self.width as i32) + x) * 4;

        for _ in 0..height {
            let mut src_index = src_line as usize;
            let mut dest_index = dest_line as usize;

            for _ in 0..width {
                if source.data[src_index + 3] > 0 {
                    self.data[dest_index..dest_index + 4]
                        .copy_from_slice(&source.data[src_index..src_index + 4]);
                }

                src_index += 4;
                dest_index += 4;
            }

            src_line += source.width as i32 * 4;
            dest_line += self.width as i32 * 4;
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> [u8; 4] {
        let index = (y * self.width as i32 + x) * 4;

        if index < 0 || index >= self.data.len() as i32 {
            return [0, 0, 0, 0];
        }

        [
            self.data[index as usize],
            self.data[index as usize + 1],
            self.data[index as usize + 2],
            self.data[index as usize + 3],
        ]
    }
}
