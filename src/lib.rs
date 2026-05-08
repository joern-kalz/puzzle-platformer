use wasm_bindgen::prelude::*;

use character::Character;
use hud::Hud;
use screen::Screen;

mod character;
mod hud;
mod screen;

#[wasm_bindgen]
pub struct World {
    screen: Screen,
    character: Character,
    hud: Hud,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        let screen = Screen::new();
        let character = Character::new(200, 100);
        let hud = Hud::new(screen.width(), screen.height());
        World {
            screen,
            character,
            hud,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.screen.width()
    }

    pub fn get_height(&self) -> i32 {
        self.screen.height()
    }

    pub fn get_pixel_buffer_ptr(&self) -> *const u8 {
        self.screen.data()
    }

    pub fn update_frame(&mut self, time: f64) {
        self.screen.clear();
        self.character.update(&self.screen, time);
        self.character.draw(&mut self.screen);
        self.hud.draw(&mut self.screen);
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
