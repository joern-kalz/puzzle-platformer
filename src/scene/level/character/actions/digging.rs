use image::Pixel;

use super::super::sprite::{Direction, Sprite};
use super::super::update_result::UpdateResult;
use crate::scene::screen::{Background, Buffer, FrameSet};

const STEP_WIDTH: i32 = 4;
const STEP_COUNT: i32 = 4;
const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;

pub struct Digging {
    sprite: Sprite,
    frame_index: i32,
}

impl Digging {
    pub fn new(sprite: Sprite) -> Self {
        Digging {
            sprite,
            frame_index: 0,
        }
    }

    pub fn update(&mut self, background: &mut impl Background) -> Option<UpdateResult> {
        if !self.sprite.is_in_world(background) {
            return Some(UpdateResult::Dead);
        }

        if !self.sprite.is_on_ground(background) {
            return Some(UpdateResult::Walking(self.sprite));
        }

        self.frame_index += 1;

        match self.frame_index {
            0 | 4 | 5 | 6 => {
                if self.is_colliding_with_wall(background) {
                    return Some(UpdateResult::Walking(self.sprite));
                }
                self.sprite.x += STEP_WIDTH * self.sprite.direction as i32;
            }
            1..=3 => {
                self.erase(background, self.sprite.height() * self.frame_index / 3);
            }
            7 => {
                if self.is_before_wall(background) {
                    self.frame_index = 0;
                } else {
                    return Some(UpdateResult::Walking(self.sprite));
                }
            }
            _ => {}
        }

        None
    }

    fn is_colliding_with_wall(&self, background: &impl Background) -> bool {
        let left = match self.sprite.direction {
            Direction::Right => self.sprite.right() + 1,
            Direction::Left => self.sprite.left() - STEP_WIDTH,
        };

        for x in left..=(left + STEP_WIDTH - 1) {
            for y in self.sprite.top()..=self.sprite.bottom() {
                if background.get_pixel(x, y).alpha() > 0 {
                    return true;
                }
            }
        }

        false
    }

    fn erase(&mut self, background: &mut impl Background, height: i32) {
        background.erase(
            match self.sprite.direction {
                Direction::Right => self.sprite.right() + 1,
                Direction::Left => self.sprite.left() - STEP_COUNT * STEP_WIDTH,
            },
            self.sprite.top(),
            STEP_COUNT * STEP_WIDTH,
            height,
        );
    }

    fn is_before_wall(&self, background: &impl Background) -> bool {
        let left = match self.sprite.direction {
            Direction::Right => self.sprite.right() + 1,
            Direction::Left => self.sprite.left() - STEP_WIDTH * STEP_COUNT,
        };
        let right = left + STEP_WIDTH * STEP_COUNT - 1;

        for x in left..=right {
            for y in self.sprite.top()..=self.sprite.bottom() {
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
            FrameSet::Digging,
            self.frame_index,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
