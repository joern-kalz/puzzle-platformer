use image::Pixel;

use crate::screen::{Background, Buffer, DrawParams, FrameSet};

pub const COLLIDER_WIDTH: i32 = 24;
pub const COLLIDER_HEIGHT: i32 = 36;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left = -1,
    Right = 1,
}

#[derive(Clone, Copy)]
pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Sprite {
    pub fn is_on_ground(&self, background: &impl Background) -> bool {
        let start_x = self.x - COLLIDER_WIDTH / 2;
        let end_x = self.x + COLLIDER_WIDTH / 2;
        let y = self.y + 1;

        for x in start_x..=end_x {
            if background.get_pixel(x, y).alpha() > 0 {
                return true;
            }
        }

        false
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        let left = self.x - COLLIDER_WIDTH / 2;
        let right = self.x + COLLIDER_WIDTH / 2;
        let top = self.y - COLLIDER_HEIGHT;
        let bottom = self.y;

        x >= left && x <= right && y >= top && y <= bottom
    }

    pub fn draw(
        &self,
        buffer: &mut impl Buffer,
        frame_set: FrameSet,
        frame_index: i32,
        offset_x: i32,
        offset_y: i32,
    ) {
        buffer.draw(DrawParams {
            x: self.x + offset_x,
            y: self.y + offset_y,
            frame_set,
            frame_index,
            mirror_x: self.direction == Direction::Left,
            mirror_y: false,
        });
    }
}
