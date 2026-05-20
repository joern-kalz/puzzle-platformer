use image::GenericImageView;
use image::{imageops, ImageReader, Rgba, RgbaImage};

pub use crate::screen::frame_sets::FrameSet;
use crate::screen::frame_sets::POSITIONS;

mod frame_sets;

const SPRITE_SHEET_DATA: &[u8] = include_bytes!("../assets/sprite_sheet.png");
const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;

pub trait Buffer {
    fn draw(&mut self, params: DrawParams);
}

pub trait Background {
    fn draw(&mut self, params: DrawParams);
    fn get_pixel(&self, x: i32, y: i32) -> Rgba<u8>;
    fn erase(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub struct Screen {
    buffer: RgbaImage,
    background: RgbaImage,
    sprite_sheet: RgbaImage,
}

pub struct DrawParams {
    pub x: i32,
    pub y: i32,
    pub frame_set: FrameSet,
    pub frame_index: i32,
    pub mirror_x: bool,
    pub mirror_y: bool,
}

impl Screen {
    pub fn new() -> Screen {
        let background = RgbaImage::new(WIDTH as u32, HEIGHT as u32);

        let sprite_sheet = ImageReader::new(std::io::Cursor::new(SPRITE_SHEET_DATA))
            .with_guessed_format()
            .expect("Failed to guess image format of sprite sheet")
            .decode()
            .expect("Failed to decode sprite sheet")
            .to_rgba8();

        let buffer = RgbaImage::new(WIDTH as u32, HEIGHT as u32);

        Screen {
            buffer,
            background,
            sprite_sheet,
        }
    }

    pub fn load_background(&mut self, data: &[u8]) {
        let background = ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .expect("Failed to guess image format of level image")
            .decode()
            .expect("Failed to decode level image")
            .to_rgba8();

        self.background = background;
    }

    pub fn clear(&mut self) {
        self.buffer.copy_from_slice(&self.background)
    }

    pub fn width(&self) -> i32 {
        WIDTH
    }

    pub fn height(&self) -> i32 {
        HEIGHT
    }

    pub fn data(&self) -> *const u8 {
        self.buffer.as_ptr()
    }
}

impl Buffer for Screen {
    fn draw(&mut self, params: DrawParams) {
        draw(&self.sprite_sheet, &mut self.buffer, params);
    }
}

impl Background for Screen {
    fn draw(&mut self, params: DrawParams) {
        draw(&self.sprite_sheet, &mut self.background, params);
    }

    fn get_pixel(&self, x: i32, y: i32) -> Rgba<u8> {
        let x_outside = x < 0 || x >= self.background.width() as i32;
        let y_outside = y < 0 || y >= self.background.height() as i32;

        if x_outside || y_outside {
            return Rgba([0, 0, 0, 0]);
        }

        self.background.get_pixel(x as u32, y as u32).clone()
    }

    fn erase(&mut self, x: i32, y: i32, width: i32, height: i32) {
        for x in x..x + width {
            for y in y..y + height {
                if x >= 0
                    && x < self.background.width() as i32
                    && y >= 0
                    && y < self.background.height() as i32
                {
                    self.background
                        .put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 0]));
                }
            }
        }
    }

    fn width(&self) -> i32 {
        self.background.width() as i32
    }

    fn height(&self) -> i32 {
        self.background.height() as i32
    }
}

fn draw(src: &RgbaImage, dst: &mut RgbaImage, params: DrawParams) {
    let frames = POSITIONS[params.frame_set as usize];
    let frame = frames[params.frame_index as usize];

    let src = src.view(frame.0, frame.1, frame.2, frame.3);
    let mut src = src.to_image();

    if params.mirror_x {
        src = imageops::flip_horizontal(&src);
    }

    if params.mirror_y {
        src = imageops::flip_vertical(&src);
    }

    imageops::overlay(dst, &src, params.x as i64, params.y as i64);
}
