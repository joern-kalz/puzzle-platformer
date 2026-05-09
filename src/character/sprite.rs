use crate::screen::Screen;

pub const SPRITE_WIDTH: i32 = 60;
pub const COLLIDER_MARGIN_X: i32 = 17;
pub const COLLIDER_TOP: i32 = 13;
pub const COLLIDER_BOTTOM: i32 = 49;

#[derive(Clone, Copy)]
pub enum Direction {
    Left = -1,
    Right = 1,
}

pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Sprite {
    pub fn is_on_ground(&self, screen: &Screen) -> bool {
        let start_x = self.x + COLLIDER_MARGIN_X;
        let end_x = self.x + SPRITE_WIDTH - COLLIDER_MARGIN_X;
        let y = self.y + COLLIDER_BOTTOM;

        for x in start_x..=end_x {
            if screen.get_pixel(x, y)[3] > 0 {
                return true;
            }
        }
        false
    }
}
