use wasm_bindgen::prelude::*;

use level::Level;
use screen::Screen;

mod level;
mod screen;

const FRAME_DURATION_IN_MS: f64 = 1000.0 / 30.0;

#[wasm_bindgen]
pub struct World {
    screen: Screen,
    last_update_time: f64,
    level: Level,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        let screen = Screen::new();
        let level = Level::new(screen.width(), screen.height());
        World {
            screen,
            last_update_time: 0.0,
            level,
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
        self.level.update(&mut self.screen);
        self.screen.clear();
        self.level.draw(&mut self.screen);
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        self.level.on_hover(x, y)
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        self.level.on_click(x, y);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
