const SPRITE_WIDTH: usize = 60;
const SPRITE_HEIGHT: usize = 60;
const NUM_SPRITES: usize = 6;

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
            dest_x: (screen_width  / 2 - SPRITE_WIDTH / 2) as usize,
            dest_y: (screen_height / 2 - SPRITE_HEIGHT / 2) as usize,
            sprite_index: 0,
        }
    }

    /// Update the sprite index based on time
    pub fn update(&mut self, time: f64) {
        self.sprite_index = ((time / 200.0) as usize) % NUM_SPRITES;
    }

    /// Draw the character sprite to the pixel buffer
    pub fn draw(
        &self,
        screen: &mut Image,
        sprite_sheet: &Image,
    ) {
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