use crate::image::{DrawParams, Image};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Stairs,
    Dig,
    Jump,
}

const SPRITE_WIDTH: i32 = 60;
const SPRITE_HEIGHT: i32 = 60;
const ACTIONS: [Action; 3] = [Action::Stairs, Action::Dig, Action::Jump];
const BUTTON_SPRITE_POSITION: (i32, i32) = (4, 3);
const HOVER_BUTTON_SPRITE_POSITION: (i32, i32) = (5, 3);

struct Button {
    x: i32,
    y: i32,
    sprite_index: i32,
    action: Action,
}

pub struct Hud {
    buttons: Vec<Button>,
    hover: Option<Action>,
    active: Action,
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
            x += SPRITE_WIDTH + 10;
        }

        Self {
            buttons,
            hover: None,
            active: Action::Stairs,
        }
    }

    pub fn _get_active_action(&self) -> Action {
        self.active
    }

    pub fn draw(&self, screen: &mut Image, sprite_sheet: &Image) {
        for button in &self.buttons {
            self.draw_sprite(
                screen,
                sprite_sheet,
                (button.x, button.y),
                if self.hover == Some(button.action) {
                    HOVER_BUTTON_SPRITE_POSITION
                } else {
                    BUTTON_SPRITE_POSITION
                },
                self.active == button.action,
            );
            self.draw_sprite(
                screen,
                sprite_sheet,
                (button.x, button.y),
                (0, button.sprite_index),
                false,
            );
        }
    }

    fn draw_sprite(
        &self,
        screen: &mut Image,
        sprite_sheet: &Image,
        position: (i32, i32),
        sprite: (i32, i32),
        flip: bool,
    ) {
        screen.draw(DrawParams {
            x: position.0,
            y: position.1,
            source: sprite_sheet,
            source_x: sprite.0 * SPRITE_WIDTH,
            source_y: sprite.1 * SPRITE_HEIGHT,
            width: SPRITE_WIDTH,
            height: SPRITE_HEIGHT,
            flip_horizontal: flip,
            flip_vertical: flip,
        });
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        for button in &self.buttons {
            if x >= button.x
                && x < button.x + SPRITE_WIDTH
                && y >= button.y
                && y < button.y + SPRITE_HEIGHT
            {
                self.hover = Some(button.action);
                return true;
            }
        }

        self.hover = None;
        return false;
    }
}
