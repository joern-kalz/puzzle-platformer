use crate::screen::Screen;

pub const SPRITE_WIDTH: i32 = 60;
pub const COLLIDER_MARGIN_X: i32 = 17;
pub const COLLIDER_TOP: i32 = 13;
pub const COLLIDER_BOTTOM: i32 = 49;

pub fn is_on_ground(x: i32, y: i32, screen: &Screen) -> bool {
    let start_x = x + COLLIDER_MARGIN_X;
    let end_x = x + SPRITE_WIDTH - COLLIDER_MARGIN_X;
    let y = y + COLLIDER_BOTTOM;

    for x in start_x..=end_x {
        if screen.get_pixel(x, y)[3] > 0 {
            return true;
        }
    }
    false
}
