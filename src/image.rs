use image::ImageReader;

pub struct Image {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

pub struct DrawParams<'a> {
    pub x: i32,
    pub y: i32,
    pub source: &'a Image,
    pub source_x: i32,
    pub source_y: i32,
    pub width: i32,
    pub height: i32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
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

    pub fn draw(&mut self, params: DrawParams) {
        let DrawParams {
            x,
            y,
            source,
            source_x,
            source_y,
            width,
            height,
            flip_horizontal,
            flip_vertical,
        } = self.clip(params);

        let mut src_line = (source_y * (source.width) + source_x) * 4;
        let mut dest_line = (y * self.width + x) * 4;
        let mut dest_inc = 4;
        let mut dest_line_inc = self.width * 4;

        if flip_horizontal {
            dest_line += (width - 1) * 4;
            dest_inc = -4;
        }
        if flip_vertical {
            dest_line += (height - 1) * self.width * 4;
            dest_line_inc = -self.width * 4;
        }

        for _ in 0..height {
            self.draw_line(dest_line, source, src_line, width, dest_inc);
            src_line += source.width * 4;
            dest_line += dest_line_inc;
        }
    }

    fn clip<'a>(&mut self, params: DrawParams<'a>) -> DrawParams<'a> {
        let DrawParams {
            mut x,
            mut y,
            source,
            mut source_x,
            mut source_y,
            mut width,
            mut height,
            flip_horizontal,
            flip_vertical,
        } = params;

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

        DrawParams {
            x,
            y,
            source,
            source_x,
            source_y,
            width,
            height,
            flip_horizontal,
            flip_vertical,
        }
    }

    fn draw_line(
        &mut self,
        dest_index: i32,
        source: &Image,
        src_index: i32,
        width: i32,
        dest_inc: i32,
    ) {
        let mut src_index = src_index;
        let mut dest_index = dest_index;

        for _ in 0..width {
            self.draw_pixel(dest_index as usize, source, src_index as usize);
            src_index += 4;
            dest_index += dest_inc;
        }
    }

    fn draw_pixel(&mut self, dest_index: usize, source: &Image, src_index: usize) {
        if source.data[src_index + 3] > 0 {
            let src = &source.data[src_index..src_index + 4];
            self.data[dest_index..dest_index + 4].copy_from_slice(src);
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
