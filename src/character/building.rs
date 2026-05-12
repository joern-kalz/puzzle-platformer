use image::Pixel;

use super::sprite::{Direction, Sprite, COLLIDER_HEIGHT, COLLIDER_WIDTH};
use crate::screen::{Background, Buffer, DrawParams, FrameSet};

const UPDATES_PER_FRAME: i32 = 4;
const STEP_WIDTH: i32 = 15;
const STONE_WIDTH: i32 = 16;
const STONE_HEIGHT: i32 = 6;
const STONE_X: i32 = 8;
const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;

use wasm_bindgen::prelude::*;

// Import 'console.log' from JavaScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: i32, b: i32);
}

pub struct Building {
    sprite: Sprite,
    frame_index: i32,
}

pub enum UpdateResult {
    None,
    SwitchToWalking(Sprite),
}

impl Building {
    pub fn new(sprite: Sprite) -> Self {
        Building {
            sprite,
            frame_index: 0,
        }
    }

    pub fn update(&mut self, background: &mut impl Background) -> UpdateResult {
        if !self.sprite.is_on_ground(background) {
            return UpdateResult::SwitchToWalking(self.sprite);
        }

        if self.is_colliding_with_wall(background) {
            return UpdateResult::SwitchToWalking(self.sprite);
        }

        self.frame_index += 1;

        match self.frame_index {
            16 => background.draw(DrawParams {
                x: match self.sprite.direction {
                    Direction::Right => self.sprite.x + STONE_X,
                    Direction::Left => self.sprite.x - STONE_X - STONE_WIDTH,
                },
                y: self.sprite.y - STONE_HEIGHT + 1,
                frame_set: FrameSet::Stone,
                frame_index: 0,
                mirror_x: false,
                mirror_y: false,
            }),
            24 => {
                self.frame_index = 0;
                self.sprite.x += STEP_WIDTH * self.sprite.direction as i32;
                self.sprite.y -= STONE_HEIGHT;
            }
            _ => (),
        }

        return UpdateResult::None;
    }

    fn is_colliding_with_wall(&self, background: &impl Background) -> bool {
        log_many(self.sprite.x, self.sprite.y);
        let x = self.sprite.x + STEP_WIDTH * self.sprite.direction as i32;
        let y = self.sprite.y - STONE_HEIGHT;
        let left = x - COLLIDER_WIDTH / 2;
        let right = x + COLLIDER_WIDTH / 2;
        let top = y - COLLIDER_HEIGHT;
        let bottom = y;

        for x in left..=right {
            for y in top..bottom {
                if background.get_pixel(x, y).alpha() > 0 {
                    log_many(x, y);
                    return true;
                }
            }
        }

        false
    }

    pub fn draw(&self, background: &mut impl Buffer) {
        self.sprite.draw(
            background,
            FrameSet::Building,
            self.frame_index / UPDATES_PER_FRAME,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
