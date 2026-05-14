use wasm_bindgen::prelude::*;

use character::{Action, Character};
use hud::{ActionButton, Hud};
use screen::Screen;

mod character;
mod hud;
mod screen;

const FRAME_DURATION_IN_MS: f64 = 1000.0 / 30.0;

#[wasm_bindgen]
pub struct World {
    screen: Screen,
    character: Character,
    hud: Hud,
    last_update_time: f64,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        let screen = Screen::new();
        let character = Character::new(250, 300);
        let hud = Hud::new(screen.width(), screen.height());
        World {
            screen,
            character,
            hud,
            last_update_time: 0.0,
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

    pub fn update_frame(&mut self, time_in_ms: f64) {
        if time_in_ms - self.last_update_time < FRAME_DURATION_IN_MS {
            return;
        }

        self.last_update_time = time_in_ms;
        self.character.update(&mut self.screen);
        self.screen.clear();
        self.character.draw(&mut self.screen);
        self.hud.draw(&mut self.screen);
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        self.hud.is_inside(x, y) || self.character.is_inside(x, y)
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        if self.hud.is_inside(x, y) {
            self.hud.on_click(x, y);
        } else if self.character.is_inside(x, y) {
            match self.hud.get_active_action() {
                ActionButton::Build => self.character.perform(Action::Build),
                ActionButton::Dig => self.character.perform(Action::Dig),
                ActionButton::Jump => self.character.perform(Action::Jump),
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
