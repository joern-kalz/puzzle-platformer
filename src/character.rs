const SPRITE_WIDTH: i32 = 60;
const SPRITE_HEIGHT: i32 = 60;
const NUM_SPRITES: i32 = 6;

const COLLIDER_LEFT: i32 = 17;
const COLLIDER_RIGHT: i32 = 39;
const COLLIDER_TOP: i32 = 13;
const COLLIDER_BOTTOM: i32 = 49;

const MAX_STEP_HEIGHT: i32 = 10;

use crate::image::Image;

pub enum Direction {
    Left,
    Right,
}

pub struct Character {
    pub x: i32,
    pub y: i32,
    pub sprite_index: i32,
    pub direction: Direction,
}

impl Character {
    pub fn new(screen_width: i32, screen_height: i32) -> Character {
        Character {
            x: screen_width / 2,
            y: screen_height / 2,
            sprite_index: 0,
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self, screen: &Image, time: f64) {
        self.sprite_index = (time / 100.0) as i32 % NUM_SPRITES;

        if !self.is_on_ground(screen) {
            self.y += 1;
            return;
        }

        for offset in 0..MAX_STEP_HEIGHT {
            if !self.is_colliding_with_wall(screen, offset) {
                self.y -= offset;
                self.x += match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                return;
            }
        }

        self.direction = match self.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }

    fn is_on_ground(&self, screen: &Image) -> bool {
        for x in COLLIDER_LEFT..=COLLIDER_RIGHT {
            if screen.get_pixel(self.x + x, self.y + COLLIDER_BOTTOM)[3] > 0 {
                return true;
            }
        }
        false
    }

    fn is_colliding_with_wall(&self, screen: &Image, offset: i32) -> bool {
        let x = match self.direction {
            Direction::Left => self.x + SPRITE_WIDTH - COLLIDER_RIGHT,
            Direction::Right => self.x + COLLIDER_RIGHT,
        };

        for y in (COLLIDER_TOP + self.y - offset)..(COLLIDER_BOTTOM + self.y - offset) {
            if screen.get_pixel(x, y)[3] > 0 {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, screen: &mut Image, sprite_sheet: &Image) {
        let sprite_x = self.sprite_index * SPRITE_WIDTH;
        screen.draw(
            self.x,
            self.y,
            sprite_sheet,
            sprite_x,
            0,
            SPRITE_WIDTH,
            SPRITE_HEIGHT,
            matches!(self.direction, Direction::Left),
        );
    }
}
