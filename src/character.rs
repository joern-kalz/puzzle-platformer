const SPRITE_WIDTH: i32 = 60;
const SPRITE_HEIGHT: i32 = 60;
const NUM_SPRITES: i32 = 6;

const BASE_LEFT: i32 = 17;
const BASE_RIGHT: i32 = 39;
const BASE_Y: i32 = 49;

use crate::image::Image;

/// Character struct containing position and sprite state
pub struct Character {
    pub dest_x: i32,
    pub dest_y: i32,
    pub sprite_index: i32,
}

impl Character {
    /// Create a new Character instance at the center of the screen
    pub fn new(screen_width: i32, screen_height: i32) -> Character {
        Character {
            dest_x: screen_width / 2,
            dest_y: screen_height / 2,
            sprite_index: 0,
        }
    }

    /// Update the sprite index based on time
    pub fn update(&mut self, screen: &Image, time: f64) {
        if self.is_on_ground(screen) {
            self.dest_x += 1;
        } else {
            self.dest_y += 1;
        }

        self.sprite_index = (time / 200.0) as i32 % NUM_SPRITES;
    }

    fn is_on_ground(&self, screen: &Image) -> bool {
        for x in BASE_LEFT..=BASE_RIGHT {
            if screen.get_pixel(self.dest_x + x, self.dest_y + BASE_Y)[3] > 0 {
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
        );
    }
}
