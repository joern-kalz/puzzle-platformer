use crate::action::Action;
use crate::image::Image;

const SPRITE_WIDTH: i32 = 60;
const SPRITE_HEIGHT: i32 = 60;
const ACTIONS: [Action; 3] = [Action::Stairs, Action::Dig, Action::Jump];

struct Button {
    x: i32,
    y: i32,
    sprite_index: i32,
    action: Action,
}

pub struct Hud {
    buttons: Vec<Button>,
    hover: Option<Button>,
}

impl Hud {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        let mut buttons = Vec::new();
        let mut x = (screen_width - ACTIONS.len() as i32 * SPRITE_WIDTH) / 2;
        let y = screen_height - SPRITE_HEIGHT;

        for (i, action) in ACTIONS.iter().enumerate() {
            buttons.push(Button {
                x,
                y,
                action: *action,
                sprite_index: i as i32 + 1,
            });
            x += SPRITE_WIDTH;
        }

        Self {
            buttons,
            hover: None,
        }
    }

    pub fn draw(&self, screen: &mut Image, sprite_sheet: &Image) {
        for button in &self.buttons {
            screen.draw(
                button.x,
                button.y,
                sprite_sheet,
                4 * SPRITE_WIDTH,
                3 * SPRITE_HEIGHT,
                SPRITE_WIDTH,
                SPRITE_HEIGHT,
                false,
            );
            screen.draw(
                button.x,
                button.y,
                sprite_sheet,
                0,
                button.sprite_index * SPRITE_HEIGHT,
                SPRITE_WIDTH,
                SPRITE_HEIGHT,
                false,
            );
        }
    }
}
