use super::super::sprite::{Sprite, COLLIDER_HEIGHT, COLLIDER_WIDTH};
use super::super::update_result::UpdateResult;
use crate::screen::{Background, Buffer, FrameSet};

const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;
const FRAME_COUNT: i32 = 11;
const OFFSETS: [i32; FRAME_COUNT as usize] = [0, 0, 0, 0, -4, -1, 0, 1, 4, 8, 8];

pub struct Jumping {
    sprite: Sprite,
    frame_index: i32,
}

impl Jumping {
    pub fn new(sprite: Sprite) -> Self {
        Jumping {
            sprite,
            frame_index: 0,
        }
    }

    pub fn update(&mut self, background: &mut impl Background) -> Option<UpdateResult> {
        if !self.sprite.is_in_world(background) {
            return Some(UpdateResult::Dead);
        }

        if self.frame_index < FRAME_COUNT {
            self.sprite.y += OFFSETS[self.frame_index as usize];
            self.erase(background);
            self.frame_index += 1;
        } else {
            if self.sprite.is_on_ground(background) {
                self.frame_index = 0;
            } else {
                return Some(UpdateResult::Walking(self.sprite));
            }
        }

        None
    }

    fn erase(&mut self, background: &mut impl Background) {
        background.erase(
            self.sprite.x - COLLIDER_WIDTH / 2,
            self.sprite.y - COLLIDER_HEIGHT,
            COLLIDER_WIDTH,
            COLLIDER_HEIGHT,
        );
    }

    pub fn draw(&self, screen: &mut impl Buffer) {
        self.sprite.draw(
            screen,
            FrameSet::Jumping,
            if self.frame_index < 4 {
                self.frame_index
            } else {
                0
            },
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
