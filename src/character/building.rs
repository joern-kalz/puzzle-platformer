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
const STONES_LIMIT: i32 = 5;

pub struct Building {
    sprite: Sprite,
    frame_index: i32,
    stone_counter: i32,
}

pub enum BuildingResult {
    None,
    Walking(Sprite),
    Dead,
}

impl Building {
    pub fn new(sprite: Sprite) -> Self {
        Building {
            sprite,
            frame_index: 0,
            stone_counter: 0,
        }
    }

    pub fn update(&mut self, background: &mut impl Background) -> BuildingResult {
        if !self.sprite.is_in_world(background) {
            return BuildingResult::Dead;
        }

        if !self.sprite.is_on_ground(background) {
            return BuildingResult::Walking(self.sprite);
        }

        if self.is_colliding_with_wall(background) || self.stone_counter >= STONES_LIMIT {
            return BuildingResult::Walking(self.sprite);
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
                self.stone_counter += 1;
            }
            _ => (),
        }

        return BuildingResult::None;
    }

    fn is_colliding_with_wall(&self, background: &impl Background) -> bool {
        let x = self.sprite.x + STEP_WIDTH * self.sprite.direction as i32;
        let y = self.sprite.y - STONE_HEIGHT;
        let left = x - COLLIDER_WIDTH / 2;
        let right = x + COLLIDER_WIDTH / 2;
        let top = y - COLLIDER_HEIGHT;
        let bottom = y;

        for x in left..=right {
            for y in top..bottom {
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
            self.frame_index / UPDATES_PER_FRAME,
            -SPRITE_WIDTH / 2,
            -BASE_Y,
        );
    }

    pub fn get_sprite(&self) -> Sprite {
        self.sprite
    }
}
