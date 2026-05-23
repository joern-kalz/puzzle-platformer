use super::super::sprite::Sprite;
use super::super::update_result::UpdateResult;
use crate::scene::screen::{Buffer, FrameSet};

const NUM_SPRITES: i32 = 4;
const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;
const ANIMATION_LENGTH: i32 = 50;
const TARGET_OFFSET: f32 = 0.2;

pub struct Leaving {
    sprite: Sprite,
    frame_index: i32,
}

impl Leaving {
    pub fn new(sprite: Sprite) -> Self {
        Leaving {
            sprite,
            frame_index: 0,
        }
    }

    pub fn update(&mut self) -> Option<UpdateResult> {
        if self.frame_index >= ANIMATION_LENGTH {
            return Some(UpdateResult::Left);
        }

        self.frame_index += 1;

        None
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        let progress = self.frame_index as f32 / ANIMATION_LENGTH as f32;
        let scale = 1.0 - progress;
        let offset = BASE_Y as f32 * TARGET_OFFSET * progress;

        self.sprite.draw_scaled(
            buffer,
            FrameSet::Leaving,
            (self.frame_index / 2) % NUM_SPRITES,
            -((SPRITE_WIDTH / 2) as f32 * scale) as i32,
            -(BASE_Y as f32 * scale) as i32 - offset as i32,
            scale,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
