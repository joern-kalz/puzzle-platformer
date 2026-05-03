const SPRITE_WIDTH: usize = 60;
const SPRITE_HEIGHT: usize = 60;
const NUM_SPRITES: usize = 6;

const BASE_LEFT: usize = 17;
const BASE_RIGHT: usize = 39;
const BASE_Y: usize = 49;

use crate::image::Image;

/// Character struct containing position and sprite state
pub struct Character {
    pub dest_x: usize,
    pub dest_y: usize,
    pub sprite_index: usize,
}

impl Character {
    /// Create a new Character instance at the center of the screen
    pub fn new(screen_width: usize, screen_height: usize) -> Character {
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

        self.sprite_index = ((time / 200.0) as usize) % NUM_SPRITES;
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
        let sprite_x = (self.sprite_index * SPRITE_WIDTH) as usize;
        screen.draw(
            self.dest_x,
            self.dest_y,
            sprite_sheet,
            sprite_x,
            0,
            SPRITE_WIDTH as usize,
            SPRITE_HEIGHT as usize,
        );
    }
}
