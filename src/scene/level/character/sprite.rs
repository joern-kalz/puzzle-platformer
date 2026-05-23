use image::Pixel;

use crate::scene::screen::{Background, Buffer, DrawParams, FrameSet};

pub const COLLIDER_WIDTH: i32 = 25;
pub const COLLIDER_HEIGHT: i32 = 36;
pub const SELECTION_MARGIN: i32 = 10;

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
    pub fn left(&self) -> i32 {
        self.x - (COLLIDER_WIDTH - 1) / 2
    }
    pub fn right(&self) -> i32 {
        self.x + (COLLIDER_WIDTH - 1) / 2
    }
    pub fn top(&self) -> i32 {
        self.y - (COLLIDER_HEIGHT - 1)
    }
    pub fn bottom(&self) -> i32 {
        self.y
    }
    pub fn width(&self) -> i32 {
        COLLIDER_WIDTH
    }
    pub fn height(&self) -> i32 {
        COLLIDER_HEIGHT
    }

    pub fn is_on_ground(&self, background: &impl Background) -> bool {
        for x in self.left()..=self.right() {
            if background.get_pixel(x, self.bottom() + 1).alpha() > 0 {
                return true;
            }
        }

        false
    }

    pub fn is_in_world(&self, background: &impl Background) -> bool {
        self.left() < background.width()
            && self.right() >= 0
            && self.top() < background.height()
            && self.bottom() >= 0
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        let left = self.left() - SELECTION_MARGIN;
        let right = self.right() + SELECTION_MARGIN;
        let top = self.top() - SELECTION_MARGIN;
        let bottom = self.bottom() + SELECTION_MARGIN;

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
        buffer.draw(
            DrawParams::new(self.x + offset_x, self.y + offset_y, frame_set)
                .frame_index(frame_index)
                .mirror_x(self.direction == Direction::Left),
        );
    }

    pub fn draw_scaled(
        &self,
        buffer: &mut impl Buffer,
        frame_set: FrameSet,
        frame_index: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) {
        buffer.draw(
            DrawParams::new(self.x + offset_x, self.y + offset_y, frame_set)
                .frame_index(frame_index)
                .mirror_x(self.direction == Direction::Left)
                .scale(scale),
        );
    }
}
