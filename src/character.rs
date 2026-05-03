const SPRITE_WIDTH: i32 = 60;
const SPRITE_HEIGHT: i32 = 60;
const NUM_SPRITES: i32 = 6;

const COLLIDER_LEFT: i32 = 17;
const COLLIDER_RIGHT: i32 = 39;
// const COLLIDER_TOP: i32 = 13;
const COLLIDER_BOTTOM: i32 = 49;

use crate::image::Image;

pub enum Direction {
    Left,
    Right,
}

pub struct Character {
    pub dest_x: i32,
    pub dest_y: i32,
    pub sprite_index: i32,
    pub direction: Direction,
}

impl Character {
    /// Create a new Character instance at the center of the screen
    pub fn new(screen_width: i32, screen_height: i32) -> Character {
        Character {
            dest_x: screen_width / 2,
            dest_y: screen_height / 2,
            sprite_index: 0,
            direction: Direction::Right,
        }
    }

    /// Update the sprite index based on time
    pub fn update(&mut self, screen: &Image, time: f64) {
        if self.is_on_ground(screen) {
            self.dest_x += match self.direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };
        } else {
            self.dest_y += 1;
        }

        self.sprite_index = (time / 200.0) as i32 % NUM_SPRITES;
    }

    fn is_on_ground(&self, screen: &Image) -> bool {
        for x in COLLIDER_LEFT..=COLLIDER_RIGHT {
            if screen.get_pixel(self.dest_x + x, self.dest_y + COLLIDER_BOTTOM)[3] > 0 {
                return true;
            }
        }
        false
    }

    /// Draw the character sprite to the pixel buffer
    pub fn draw(&self, screen: &mut Image, sprite_sheet: &Image) {
        let sprite_x = self.sprite_index * SPRITE_WIDTH;
        screen.draw(
            self.dest_x,
            self.dest_y,
            sprite_sheet,
            sprite_x,
            0,
            SPRITE_WIDTH,
            SPRITE_HEIGHT,
            matches!(self.direction, Direction::Left),
        );
    }
}
