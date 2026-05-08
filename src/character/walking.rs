use crate::character::state::State;
use crate::screen::{DrawParams, FrameSet, Screen};

const SPRITE_WIDTH: i32 = 60;
const NUM_SPRITES: i32 = 6;

const COLLIDER_LEFT: i32 = 17;
const COLLIDER_RIGHT: i32 = 39;
const COLLIDER_TOP: i32 = 13;
const COLLIDER_BOTTOM: i32 = 49;

const MAX_STEP_HEIGHT: i32 = 10;

pub enum Direction {
    Left,
    Right,
}

pub struct Walking {
    x: i32,
    y: i32,
    sprite_index: i32,
    direction: Direction,
}

impl Walking {
    pub fn new(x: i32, y: i32) -> Self {
        Walking {
            x,
            y,
            sprite_index: 0,
            direction: Direction::Right,
        }
    }

    fn is_on_ground(&self, screen: &Screen) -> bool {
        for x in COLLIDER_LEFT..=COLLIDER_RIGHT {
            if screen.get_pixel(self.x + x, self.y + COLLIDER_BOTTOM)[3] > 0 {
                return true;
            }
        }
        false
    }

    fn is_colliding_with_wall(&self, screen: &Screen, offset: i32) -> bool {
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
}

impl State for Walking {
    fn update(&mut self, screen: &Screen, time: f64) -> Option<Box<dyn State>> {
        self.sprite_index = (time / 100.0) as i32 % NUM_SPRITES;

        if !self.is_on_ground(screen) {
            self.y += 1;
            return None;
        }

        for offset in 0..MAX_STEP_HEIGHT {
            if !self.is_colliding_with_wall(screen, offset) {
                self.y -= offset;
                self.x += match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                return None;
            }
        }

        self.direction = match self.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        None
    }

    fn draw(&self, screen: &mut Screen) {
        screen.draw(DrawParams {
            x: self.x,
            y: self.y,
            frame_set: FrameSet::Walking,
            frame_index: self.sprite_index,
            mirror_x: matches!(self.direction, Direction::Left),
            mirror_y: false,
        });
    }

    fn _on_hover(&mut self, _x: i32, _y: i32) -> bool {
        false
    }

    fn _on_click(&mut self, _x: i32, _y: i32) -> Option<Box<dyn State>> {
        None
    }
}
