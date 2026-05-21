use super::super::sprite::Sprite;
use super::super::update_result::UpdateResult;
use crate::screen::{Background, Buffer, FrameSet};

const NUM_SPRITES: i32 = 12;
const STEP_HEIGHT: i32 = 4;
const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;
const MAX_FALL_HEIGHT: i32 = 100;

pub struct Falling {
    sprite: Sprite,
    frame_index: i32,
    start_height: i32,
}

impl Falling {
    pub fn new(sprite: Sprite) -> Self {
        Falling {
            sprite,
            frame_index: 0,
            start_height: sprite.y,
        }
    }

    pub fn update(&mut self, background: &impl Background) -> Option<UpdateResult> {
        self.frame_index = (self.frame_index + 1) % NUM_SPRITES;

        if !self.sprite.is_in_world(background) {
            return Some(UpdateResult::Dead);
        }

        for _ in 0..STEP_HEIGHT {
            self.sprite.y += 1;

            if self.sprite.is_on_ground(background) {
                if self.sprite.y - self.start_height > MAX_FALL_HEIGHT {
                    return Some(UpdateResult::Exploding(self.sprite));
                } else {
                    return Some(UpdateResult::Walking(self.sprite));
                }
            }
        }

        None
    }

    pub fn draw(&self, screen: &mut impl Buffer) {
        self.sprite.draw(
            screen,
            FrameSet::Falling,
            self.frame_index,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
