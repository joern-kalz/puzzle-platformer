use crate::character::sprite::{
    is_on_ground, COLLIDER_BOTTOM, COLLIDER_MARGIN_X, COLLIDER_TOP, SPRITE_WIDTH,
};
use crate::character::state::State;
use crate::screen::{DrawParams, FrameSet, Screen};

const NUM_SPRITES: i32 = 6;
const MAX_STEP_HEIGHT: i32 = 10;

#[derive(Clone, Copy)]
pub enum Direction {
    Left = -1,
    Right = 1,
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

    fn is_colliding_with_wall(&self, screen: &Screen, offset: i32) -> bool {
        let x = match self.direction {
            Direction::Right => self.x + SPRITE_WIDTH - COLLIDER_MARGIN_X,
            Direction::Left => self.x + COLLIDER_MARGIN_X,
        };

        for y in COLLIDER_TOP..COLLIDER_BOTTOM {
            if screen.get_pixel(x, y + self.y - offset)[3] > 0 {
                return true;
            }
        }

        false
    }
}

impl State for Walking {
    fn update(&mut self, screen: &Screen, time: f64) -> Option<Box<dyn State>> {
        self.sprite_index = (time / 100.0) as i32 % NUM_SPRITES;

        if !is_on_ground(self.x, self.y, screen) {
            self.y += 1;
            return None;
        }

        for offset in 0..MAX_STEP_HEIGHT {
            if !self.is_colliding_with_wall(screen, offset) {
                self.y -= offset;
                self.x += self.direction as i32;
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
