use image::ImageReader;

const SPRITE_SHEET_DATA: &[u8] = include_bytes!("../assets/sprite_sheet.png");
const SPRITE_SHEET_WIDTH: u32 = 360;

pub struct SpriteSheet {
    pub data: Vec<u8>,
    pub width: u32,
}

impl SpriteSheet {
    pub fn new() -> SpriteSheet {
        let img = ImageReader::new(std::io::Cursor::new(SPRITE_SHEET_DATA))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode sprite sheet");

        // Convert to RGBA bytes
        let sprite_sheet = img.to_rgba8().into_raw();

        SpriteSheet { data: sprite_sheet, width: SPRITE_SHEET_WIDTH }
    }
}