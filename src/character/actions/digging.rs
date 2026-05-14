use image::Pixel;

use super::super::sprite::{Direction, Sprite, COLLIDER_HEIGHT, COLLIDER_WIDTH};
use super::super::update_result::UpdateResult;
use crate::screen::{Background, Buffer, FrameSet};

const STEP_WIDTH: i32 = 15;
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
            12 => {
                self.sprite.x += STEP_WIDTH / 3 * self.sprite.direction as i32;
            }
            4 => {
                self.erase(background, self.sprite.y - COLLIDER_HEIGHT);
            }
            8 => {
                self.erase(background, self.sprite.y - COLLIDER_HEIGHT / 2);
            }
            16 => {
                if self.is_before_wall(background) {
                    self.sprite.x += STEP_WIDTH / 3 * self.sprite.direction as i32;
                } else {
                    return Some(UpdateResult::Walking(self.sprite));
                }
            }
            20 => {
                self.frame_index = 0;
                self.sprite.x += STEP_WIDTH / 3 * self.sprite.direction as i32;
            }
            _ => {}
        }

        None
    }

    fn erase(&mut self, background: &mut impl Background, y: i32) {
        background.erase(
            match self.sprite.direction {
                Direction::Right => self.sprite.x + COLLIDER_WIDTH / 2,
                Direction::Left => self.sprite.x - COLLIDER_WIDTH / 2 - STEP_WIDTH,
            },
            y,
            STEP_WIDTH,
            COLLIDER_HEIGHT / 2,
        );
    }

    fn is_before_wall(&self, background: &impl Background) -> bool {
        let left = match self.sprite.direction {
            Direction::Right => self.sprite.x + COLLIDER_WIDTH / 2,
            Direction::Left => self.sprite.x - COLLIDER_WIDTH / 2 - STEP_WIDTH,
        };
        let right = left + STEP_WIDTH;
        let top = self.sprite.y - COLLIDER_HEIGHT + 1;
        let bottom = self.sprite.y;

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
            FrameSet::Digging,
            self.frame_index / 4,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
