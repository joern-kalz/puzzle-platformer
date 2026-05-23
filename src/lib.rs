use wasm_bindgen::prelude::*;

use scene::Scene;

mod scene;

const FRAME_DURATION_IN_MS: f64 = 1000.0 / 30.0;

#[wasm_bindgen]
pub struct Game {
    last_update_time: f64,
    scene: Scene,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            last_update_time: 0.0,
            scene: Scene::new(),
        }
    }

    pub fn get_width(&self) -> i32 {
        self.scene.width()
    }

    pub fn get_height(&self) -> i32 {
        self.scene.height()
    }

    pub fn get_pixel_buffer_ptr(&self) -> *const u8 {
        self.scene.pixel_buffer_ptr()
    }

    pub fn update_frame(&mut self, time_in_ms: f64) {
        if time_in_ms - self.last_update_time < FRAME_DURATION_IN_MS {
            return;
        }

        self.last_update_time = time_in_ms;
        self.scene.update(time_in_ms);
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        self.scene.on_hover(x, y)
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        self.scene.on_click(x, y);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
