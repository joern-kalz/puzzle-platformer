use wasm_bindgen::prelude::*;

use character::Character;
use hud::Hud;
use image::Image;

mod action;
mod character;
mod hud;
mod image;

const SPRITE_SHEET_DATA: &[u8] = include_bytes!("../assets/sprite_sheet.png");

const LEVEL_DATA: &[u8] = include_bytes!("../assets/level.png");

#[wasm_bindgen]
pub struct World {
    screen: Image,
    background: Image,
    sprite_sheet: Image,
    character: Character,
    hud: Hud,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        let background = Image::new_from_asset(LEVEL_DATA);

        World {
            screen: Image::new(background.width, background.height),
            character: Character::new(background.width, background.height),
            hud: Hud::new(background.width, background.height),
            background,
            sprite_sheet: Image::new_from_asset(SPRITE_SHEET_DATA),
        }
    }

    pub fn get_width(&self) -> i32 {
        self.screen.width
    }

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
        self.hud.draw(&mut self.screen, &self.sprite_sheet);
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        self.hud.on_hover(x, y)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
