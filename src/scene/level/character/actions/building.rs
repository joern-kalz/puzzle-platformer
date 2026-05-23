use image::Pixel;

use super::super::sprite::{Direction, Sprite};
use super::super::update_result::UpdateResult;
use crate::scene::screen::{Background, Buffer, DrawParams, FrameSet};

const STEP_WIDTH: i32 = 15;
const STONE_WIDTH: i32 = 16;
const STONE_HEIGHT: i32 = 6;
const STONE_X: i32 = 8;
const BASE_Y: i32 = 49;
const SPRITE_WIDTH: i32 = 60;
const STONES_LIMIT: i32 = 5;

pub struct Building {
    sprite: Sprite,
    frame_index: i32,
    stone_counter: i32,
}

impl Building {
    pub fn new(sprite: Sprite) -> Self {
        Building {
            sprite,
            frame_index: 0,
            stone_counter: 0,
        }
    }

    pub fn update(&mut self, background: &mut impl Background) -> Option<UpdateResult> {
        if !self.sprite.is_in_world(background) {
            return Some(UpdateResult::Dead);
        }

        if !self.sprite.is_on_ground(background) {
            return Some(UpdateResult::Walking(self.sprite));
        }

        if self.is_colliding_with_wall(background) || self.stone_counter >= STONES_LIMIT {
            return Some(UpdateResult::Walking(self.sprite));
        }

        self.frame_index += 1;

        match self.frame_index {
            6 => {
                let x = match self.sprite.direction {
                    Direction::Right => self.sprite.x + STONE_X,
                    Direction::Left => self.sprite.x - STONE_X - STONE_WIDTH,
                };
                let y = self.sprite.y - STONE_HEIGHT + 1;
                background.draw(DrawParams::new(x, y, FrameSet::Stone));
                self.stone_counter += 1;
            }
            12 => {
                self.frame_index = 0;
                self.sprite.x += STEP_WIDTH * self.sprite.direction as i32;
                self.sprite.y -= STONE_HEIGHT;
            }
            _ => (),
        }

        None
    }

    fn is_colliding_with_wall(&self, background: &impl Background) -> bool {
        let offset_x = STEP_WIDTH * self.sprite.direction as i32;
        let offset_y = -STONE_HEIGHT;

        for x in (self.sprite.left() + offset_x)..=(self.sprite.right() + offset_x) {
            for y in (self.sprite.top() + offset_y)..=(self.sprite.bottom() + offset_y) {
                if background.get_pixel(x, y).alpha() > 0 {
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
            self.frame_index,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
