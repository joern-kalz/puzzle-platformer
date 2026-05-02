use wasm_bindgen::prelude::*;

mod character;
mod image;

use character::Character;
use image::Image;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const SPRITE_SHEET_DATA: &[u8] = include_bytes!("../assets/sprite_sheet.png");
const SPRITE_SHEET_WIDTH: usize = 360;
const SPRITE_SHEET_HEIGHT: usize = 60;

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    screen: Image,
    sprite_sheet: Image,
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
            screen: Image::new(WIDTH, HEIGHT ),
            sprite_sheet: Image::new_from_asset(SPRITE_SHEET_DATA, SPRITE_SHEET_WIDTH, SPRITE_SHEET_HEIGHT),
            character: Character::new(WIDTH, HEIGHT),
        }
    }

    /// Get the width
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the height
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_pixel_buffer_ptr(&self) -> *const u8 {
        self.screen.data.as_ptr()
    }

    pub fn update_frame(&mut self, time: f64) {
        self.screen.data.fill(0);

        // Update character sprite
        self.character.update(time);

        self.character.draw(
            &mut self.screen,
            &self.sprite_sheet,
        );
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}