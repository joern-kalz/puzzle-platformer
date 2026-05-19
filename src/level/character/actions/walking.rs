use image::Pixel;

use super::super::sprite::{Direction, Sprite};
use super::super::update_result::UpdateResult;
use crate::screen::{Background, Buffer, FrameSet};

const NUM_SPRITES: i32 = 12;
const STEP_WIDTH: i32 = 2;
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
        self.frame_index = (self.frame_index + 1) % NUM_SPRITES;

        if !self.sprite.is_in_world(background) {
            return Some(UpdateResult::Dead);
        }

        if !self.sprite.is_on_ground(background) {
            for _ in 0..MAX_STEP_HEIGHT {
                self.sprite.y += 1;
                if self.sprite.is_on_ground(background) {
                    return None;
                }
            }
            return Some(UpdateResult::Falling(self.sprite));
        }

        for offset in 0..MAX_STEP_HEIGHT {
            if !self.is_colliding_with_wall(background, offset) {
                self.sprite.y -= offset;
                self.sprite.x += STEP_WIDTH * self.sprite.direction as i32;
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
        let left = match self.sprite.direction {
            Direction::Right => self.sprite.right() + 1,
            Direction::Left => self.sprite.left() - STEP_WIDTH,
        };
        let right = left + STEP_WIDTH - 1;
        let top = self.sprite.top() - offset;
        let bottom = self.sprite.bottom() - offset;

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
            self.frame_index,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
