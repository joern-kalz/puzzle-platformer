use wasm_bindgen::prelude::*;
use image::ImageReader;

mod character;

use character::Character;

// Image dimensions
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

// Sprite sheet dimensions
const SPRITE_SHEET_WIDTH: u32 = 360;
const SPRITE_SHEET_HEIGHT: u32 = 60;
const SPRITE_WIDTH: u32 = 60;
const SPRITE_HEIGHT: u32 = 60;
const NUM_SPRITES: u32 = 6;

// Embed the sprite sheet at compile time
const SPRITE_SHEET_DATA: &[u8] = include_bytes!("../assets/sprite_sheet.png");

/// World struct containing the pixel buffer and dimensions
#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    pixel_buffer: Vec<u8>,
    sprite_sheet: Vec<u8>,
    character: Character,
}

#[wasm_bindgen]
impl World {
    /// Create a new World instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        // Decode the embedded PNG sprite sheet
        let img = ImageReader::new(std::io::Cursor::new(SPRITE_SHEET_DATA))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode sprite sheet");

        // Convert to RGBA bytes
        let sprite_sheet = img.to_rgba8().into_raw();

        World {
            width: WIDTH,
            height: HEIGHT,
            pixel_buffer: vec![0u8; (WIDTH * HEIGHT * 4) as usize],
            sprite_sheet,
            character: Character::new(WIDTH, HEIGHT),
        }
    }

    /// Get the width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get pointer to the pixel buffer
    pub fn get_pixel_buffer_ptr(&self) -> *const u8 {
        self.pixel_buffer.as_ptr()
    }

    /// Update the pixel buffer with a new frame
    pub fn update_frame(&mut self, time: f64) {
        // Clear pixel buffer first
        self.pixel_buffer.fill(0);

        // Update character sprite
        self.character.update(time);

        // Draw character to pixel buffer
        self.character.draw(
            &mut self.pixel_buffer,
            &self.sprite_sheet,
            self.width,
            self.height,
            SPRITE_SHEET_WIDTH,
        );
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}