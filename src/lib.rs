use wasm_bindgen::prelude::*;

mod character;
mod image;

use character::Character;
use image::Image;

const SPRITE_SHEET_DATA: &[u8] = include_bytes!("../assets/sprite_sheet.png");
const SPRITE_SHEET_WIDTH: i32 = 360;
const SPRITE_SHEET_HEIGHT: i32 = 60;

const LEVEL_DATA: &[u8] = include_bytes!("../assets/level.png");
const LEVEL_WIDTH: i32 = 400;
const LEVEL_HEIGHT: i32 = 400;

#[wasm_bindgen]
pub struct World {
    screen: Image,
    background: Image,
    sprite_sheet: Image,
    character: Character,
}

#[wasm_bindgen]
impl World {
    /// Create a new World instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World {
            screen: Image::new(LEVEL_WIDTH, LEVEL_HEIGHT),
            background: Image::new_from_asset(LEVEL_DATA, LEVEL_WIDTH, LEVEL_HEIGHT),
            sprite_sheet: Image::new_from_asset(
                SPRITE_SHEET_DATA,
                SPRITE_SHEET_WIDTH,
                SPRITE_SHEET_HEIGHT,
            ),
            character: Character::new(LEVEL_WIDTH, LEVEL_HEIGHT),
        }
    }

    /// Get the width
    pub fn get_width(&self) -> i32 {
        self.screen.width
    }

    /// Get the height
    pub fn get_height(&self) -> i32 {
        self.screen.height
    }

    pub fn get_pixel_buffer_ptr(&self) -> *const u8 {
        self.screen.data.as_ptr()
    }

    pub fn update_frame(&mut self, time: f64) {
        self.screen.data.copy_from_slice(&self.background.data);

        self.character.update(&self.screen, time);

        self.character.draw(&mut self.screen, &self.sprite_sheet);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
