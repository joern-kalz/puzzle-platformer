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

    pub fn new_from_asset(data: &[u8]) -> Image {
        let data = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode sprite sheet");

        let img = data.to_rgba8();
        let (width, height) = img.dimensions();
        let data = img.into_raw();

        Image {
            data,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn draw(
        &mut self,
        mut x: i32,
        mut y: i32,
        source: &Image,
        mut source_x: i32,
        mut source_y: i32,
        mut width: i32,
        mut height: i32,
        flip_horizontal: bool,
    ) {
        if x < 0 {
            source_x = source_x - x;
            width = width + x;
            x = 0;
        }
        if y < 0 {
            source_y = source_y - y;
            height = height + y;
            y = 0;
        }
        let overflow_x = (x + width - self.width).max(0);
        width = width - overflow_x;
        let overflow_y = (y + height - self.height).max(0);
        height = height - overflow_y;

        let mut src_line = (source_y * (source.width) + source_x) * 4;
        let mut dest_line = (y * (self.width) + x) * 4;

        for _ in 0..height {
            let mut src_index = src_line as usize;
            let mut dest_index = if flip_horizontal {
                dest_line + (width - 1) * 4
            } else {
                dest_line
            } as usize;

            for _ in 0..width {
                if source.data[src_index + 3] > 0 {
                    self.data[dest_index..dest_index + 4]
                        .copy_from_slice(&source.data[src_index..src_index + 4]);
                }

                src_index += 4;

                if flip_horizontal {
                    dest_index -= 4
                } else {
                    dest_index += 4
                };
            }

            src_line += source.width * 4;
            dest_line += self.width * 4;
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> [u8; 4] {
        let index = (y * self.width + x) * 4;

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
