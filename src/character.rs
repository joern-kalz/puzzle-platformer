use wasm_bindgen::prelude::*;

// Sprite sheet dimensions
const SPRITE_WIDTH: u32 = 60;
const SPRITE_HEIGHT: u32 = 60;
const NUM_SPRITES: u32 = 6;

/// Character struct containing position and sprite state
pub struct Character {
    pub dest_x: usize,
    pub dest_y: usize,
    pub sprite_index: u32,
}

impl Character {
    /// Create a new Character instance at the center of the screen
    pub fn new(screen_width: u32, screen_height: u32) -> Character {
        Character {
            dest_x: (screen_width / 2 - SPRITE_WIDTH / 2) as usize,
            dest_y: (screen_height / 2 - SPRITE_HEIGHT / 2) as usize,
            sprite_index: 0,
        }
    }

    /// Update the sprite index based on time
    pub fn update(&mut self, time: f64) {
        self.sprite_index = ((time / 200.0) as u32) % NUM_SPRITES;
    }

    /// Draw the character sprite to the pixel buffer
    pub fn draw(
        &self,
        pixel_buffer: &mut [u8],
        sprite_sheet: &[u8],
        width: u32,
        height: u32,
        sprite_sheet_width: u32,
    ) {
        let sprite_x = (self.sprite_index * SPRITE_WIDTH) as usize;
        let sprite_sheet_width = sprite_sheet_width as usize;
        let width = width as usize;
        let height = height as usize;

        for y in 0..SPRITE_HEIGHT as usize {
            for x in 0..SPRITE_WIDTH as usize {
                let src_idx = (y * sprite_sheet_width + sprite_x + x) * 4;
                let dest_idx = ((self.dest_y + y) * width + self.dest_x + x) * 4;

                // Only copy non-transparent pixels
                let alpha = sprite_sheet[src_idx + 3];
                if alpha > 0 && dest_idx + 3 < pixel_buffer.len() {
                    pixel_buffer[dest_idx] = sprite_sheet[src_idx];     // R
                    pixel_buffer[dest_idx + 1] = sprite_sheet[src_idx + 1]; // G
                    pixel_buffer[dest_idx + 2] = sprite_sheet[src_idx + 2]; // B
                    pixel_buffer[dest_idx + 3] = alpha; // A
                }
            }
        }
    }
}