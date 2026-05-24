use scene::Scene;
use wasm_bindgen::prelude::*;

mod log;
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
        log::log(&format!(
            "Click ({}, {}) at {}",
            x, y, self.last_update_time
        ));
        self.scene.on_click(x, y);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn new_game_has_width_and_height() {
        let game = Game::default();
        assert_ne!(game.get_width(), 0);
        assert_ne!(game.get_height(), 0);
    }

    #[test]
    fn update_frame_respects_frame_duration() {
        let mut game = Game::default();
        game.update_frame(1.0);
        assert_eq!(game.last_update_time, 0.0);
        game.update_frame(FRAME_DURATION_IN_MS);
        assert_eq!(game.last_update_time, FRAME_DURATION_IN_MS);
    }

    #[test]
    fn on_hover_outside_hud_returns_false() {
        let mut game = Game::default();
        assert!(!game.on_hover(0, 0));
    }

    #[test]
    fn on_hover_inside_hud_returns_true() {
        let mut game = Game::default();
        assert!(game.on_hover(131, 369));
    }

    #[test]
    fn succeed_in_first_level_with_jumping() {
        perform_test(&[
            Action {
                time: 1291.1,
                param: ActionParam::Click(225, 56),
            },
            Action {
                time: 30709.4,
                param: ActionParam::AssertSuccess,
            },
        ]);
    }

    #[test]
    fn succeed_in_first_level_with_digging() {
        perform_test(&[
            Action {
                time: 500.1,
                param: ActionParam::Click(119, 356),
            },
            Action {
                time: 1663.1,
                param: ActionParam::Click(240, 40),
            },
            Action {
                time: 30709.6,
                param: ActionParam::AssertSuccess,
            },
        ]);
    }

    #[test]
    fn fail_in_first_level() {
        perform_test(&[
            Action {
                time: 1346.4,
                param: ActionParam::Click(236, 57),
            },
            Action {
                time: 6596.6,
                param: ActionParam::Click(321, 258),
            },
            Action {
                time: 27817.3,
                param: ActionParam::AssertFailure,
            },
        ]);
    }

    #[test]
    fn start_second_level() {
        perform_test(&[
            Action {
                time: 1291.1,
                param: ActionParam::Click(225, 56),
            },
            Action {
                time: 30709.4,
                param: ActionParam::Click(248, 338),
            },
            Action {
                time: 31709.4,
                param: ActionParam::AssertPlaying,
            },
        ]);
    }

    fn perform_test(actions: &[Action]) {
        let mut game = Game::default();
        let mut time = 0.0;

        for action in actions {
            update(&mut game, time, action.time);
            time = action.time;
            save_screen_if_debugging(&game, time);

            match action.param {
                ActionParam::Click(x, y) => game.on_click(x, y),
                ActionParam::AssertSuccess => assert!(
                    is_success(&game),
                    "Expected level completed with success at {time}"
                ),
                ActionParam::AssertFailure => assert!(
                    is_failure(&game),
                    "Expected level completed with failure at {time}"
                ),
                ActionParam::AssertPlaying => {
                    assert!(is_playing(&game), "Expected level not completed at {time}")
                }
            }
        }
    }

    fn save_screen_if_debugging(game: &Game, time: f64) {
        if std::env::var("DEBUG").is_ok() {
            let folder = "debug";
            std::fs::create_dir_all(folder).expect("Cannot create debug folder");
            get_buffer(game)
                .save(format!("{folder}/{time}.png"))
                .expect("Cannot save game screen");
        }
    }

    fn update(game: &mut Game, start_time: f64, end_time: f64) {
        let count = ((end_time - start_time) / FRAME_DURATION_IN_MS).floor() as usize;
        for i in 0..=count {
            let time = start_time + FRAME_DURATION_IN_MS * i as f64;
            game.update_frame(time);
        }
    }

    fn is_success(game: &Game) -> bool {
        check_pixel(&game, 164, 172, Rgba([70, 70, 255, 255]))
    }

    fn is_failure(game: &Game) -> bool {
        check_pixel(&game, 155, 176, Rgba([255, 0, 0, 255]))
    }

    fn is_playing(game: &Game) -> bool {
        !is_success(game) && !is_failure(game)
    }

    fn check_pixel(game: &Game, x: u32, y: u32, expected: Rgba<u8>) -> bool {
        let buffer = get_buffer(game);
        let actual = buffer.get_pixel(x, y);
        &expected == actual
    }

    fn get_buffer(game: &Game) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let data_len = (game.get_width() * game.get_height()) as usize * 4;
        let buffer = game.get_pixel_buffer_ptr();
        let buffer = unsafe { std::slice::from_raw_parts(buffer, data_len) };
        let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            game.get_width() as u32,
            game.get_height() as u32,
            buffer.to_vec(),
        );
        let Some(buffer) = buffer else {
            panic!("Buffer does not have expected size")
        };
        buffer
    }

    struct Action {
        time: f64,
        param: ActionParam,
    }

    enum ActionParam {
        Click(i32, i32),
        AssertSuccess,
        AssertFailure,
        AssertPlaying,
    }
}
