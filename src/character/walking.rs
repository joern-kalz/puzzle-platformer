use crate::character::sprite::{
    Direction, Sprite, COLLIDER_BOTTOM, COLLIDER_MARGIN_X, COLLIDER_TOP, SPRITE_WIDTH,
};
use crate::character::state::State;
use crate::screen::{DrawParams, FrameSet, Screen};

const NUM_SPRITES: i32 = 6;
const MAX_STEP_HEIGHT: i32 = 10;

pub struct Walking {
    sprite: Sprite,
    sprite_index: i32,
}

impl Walking {
    pub fn new(x: i32, y: i32) -> Self {
        Walking {
            sprite: Sprite {
                x,
                y,
                direction: Direction::Right,
            },
            sprite_index: 0,
        }
    }

    fn is_colliding_with_wall(&self, screen: &Screen, offset: i32) -> bool {
        let x = match self.sprite.direction {
            Direction::Right => self.sprite.x + SPRITE_WIDTH - COLLIDER_MARGIN_X,
            Direction::Left => self.sprite.x + COLLIDER_MARGIN_X,
        };

        for y in COLLIDER_TOP..COLLIDER_BOTTOM {
            if screen.get_pixel(x, y + self.sprite.y - offset)[3] > 0 {
                return true;
            }
        }

        false
    }
}

impl State for Walking {
    fn update(&mut self, screen: &Screen, time: f64) -> Option<Box<dyn State>> {
        self.sprite_index = (time / 100.0) as i32 % NUM_SPRITES;

        if !self.sprite.is_on_ground(screen) {
            self.sprite.y += 1;
            return None;
        }

        for offset in 0..MAX_STEP_HEIGHT {
            if !self.is_colliding_with_wall(screen, offset) {
                self.sprite.y -= offset;
                self.sprite.x += self.sprite.direction as i32;
                return None;
            }
        }

        self.sprite.direction = match self.sprite.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        None
    }

    fn draw(&self, screen: &mut Screen) {
        screen.draw(DrawParams {
            x: self.sprite.x,
            y: self.sprite.y,
            frame_set: FrameSet::Walking,
            frame_index: self.sprite_index,
            mirror_x: matches!(self.sprite.direction, Direction::Left),
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
