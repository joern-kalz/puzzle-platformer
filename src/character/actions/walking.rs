use image::Pixel;

use super::super::sprite::{Direction, Sprite, COLLIDER_HEIGHT, COLLIDER_WIDTH};
use super::super::update_result::UpdateResult;
use crate::screen::{Background, Buffer, FrameSet};

const NUM_SPRITES: i32 = 6;
const MAX_STEP_HEIGHT: i32 = 10;
const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;

pub struct Walking {
    sprite: Sprite,
    frame_index: i32,
}

impl Walking {
    pub fn new(sprite: Sprite) -> Self {
        Walking {
            sprite,
            frame_index: 0,
        }
    }

    pub fn update(&mut self, background: &impl Background) -> Option<UpdateResult> {
        self.frame_index = (self.frame_index + 1) % (NUM_SPRITES * 2);

        if !self.sprite.is_in_world(background) {
            return Some(UpdateResult::Dead);
        }

        if !self.sprite.is_on_ground(background) {
            self.sprite.y += 1;
            return None;
        }

        for offset in 0..MAX_STEP_HEIGHT {
            if !self.is_colliding_with_wall(background, offset) {
                self.sprite.y -= offset;
                self.sprite.x += 2 * self.sprite.direction as i32;
                return None;
            }
        }

        self.sprite.direction = match self.sprite.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };

        None
    }

    fn is_colliding_with_wall(&self, background: &impl Background, offset: i32) -> bool {
        let left = self.sprite.x
            + match self.sprite.direction {
                Direction::Right => COLLIDER_WIDTH / 2,
                Direction::Left => -COLLIDER_WIDTH / 2 - 2,
            };
        let right = left + 1;
        let top = self.sprite.y - COLLIDER_HEIGHT - offset;
        let bottom = self.sprite.y - offset;

        for x in left..=right {
            for y in top..=bottom {
                if background.get_pixel(x, y).alpha() > 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn draw(&self, screen: &mut impl Buffer) {
        self.sprite.draw(
            screen,
            FrameSet::Walking,
            self.frame_index / 2,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
