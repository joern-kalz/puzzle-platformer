use wasm_bindgen::prelude::*;

mod character;
mod sprite_sheet;

use character::Character;
use sprite_sheet::SpriteSheet;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    pixel_buffer: Vec<u8>,
    sprite_sheet: SpriteSheet,
    character: Character,
}

#[wasm_bindgen]
impl World {
    /// Create a new World instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World {
            width: WIDTH,
            height: HEIGHT,
            pixel_buffer: vec![0u8; (WIDTH * HEIGHT * 4) as usize],
            sprite_sheet: SpriteSheet::new(),
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
        );
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}