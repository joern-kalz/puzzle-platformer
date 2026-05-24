use crate::scene::screen::{Buffer, DrawParams, FrameSet};

const BUTTON_WIDTH: i32 = 60;
const BUTTON_HEIGHT: i32 = 60;
const BUTTON_SPACING: i32 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActionButton {
    Dig,
    Build,
    Jump,
}

const ACTION_BUTTONS: [ActionButton; 3] =
    [ActionButton::Dig, ActionButton::Build, ActionButton::Jump];

struct Button {
    x: i32,
    y: i32,
    action: ActionButton,
}

impl Button {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        let x_inside = x >= self.x && x < self.x + BUTTON_WIDTH;
        let y_inside = y >= self.y && y < self.y + BUTTON_HEIGHT;
        x_inside && y_inside
    }
}

pub struct Hud {
    buttons: Vec<Button>,
    hover: Option<ActionButton>,
    active: ActionButton,
}

impl Hud {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        let mut buttons = Vec::new();
        let buttons_width = ACTION_BUTTONS.len() as i32 * BUTTON_WIDTH;
        let spacings_width = (ACTION_BUTTONS.len() as i32 - 1) * BUTTON_SPACING;
        let mut x = (screen_width - buttons_width - spacings_width) / 2;
        let y = screen_height - BUTTON_HEIGHT;

        for action in ACTION_BUTTONS {
            buttons.push(Button { x, y, action });
            x += BUTTON_WIDTH + BUTTON_SPACING;
        }

        Self {
            buttons,
            hover: None,
            active: ActionButton::Jump,
        }
    }

    pub fn get_active_action(&self) -> ActionButton {
        self.active
    }

    pub fn draw(&self, screen: &mut impl Buffer) {
        for button in &self.buttons {
            self.draw_background(screen, button);
            self.draw_icon(screen, button);
        }
    }

    fn draw_background(&self, screen: &mut impl Buffer, button: &Button) {
        let frame_set = if button.action == self.active {
            FrameSet::ButtonPressed
        } else if Some(button.action) == self.hover {
            FrameSet::ButtonHovered
        } else {
            FrameSet::ButtonNormal
        };

        screen.draw(DrawParams::new(button.x, button.y, frame_set));
    }

    fn draw_icon(&self, screen: &mut impl Buffer, button: &Button) {
        let frame_set = match button.action {
            ActionButton::Build => FrameSet::Building,
            ActionButton::Dig => FrameSet::Digging,
            ActionButton::Jump => FrameSet::Jumping,
        };

        screen.draw(DrawParams::new(button.x, button.y, frame_set));
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        for button in &self.buttons {
            if button.is_inside(x, y) && self.active != button.action {
                self.hover = Some(button.action);
                return true;
            }
        }

        self.hover = None;
        false
    }

    pub fn on_click(&mut self, x: i32, y: i32) -> bool {
        for button in &self.buttons {
            if button.is_inside(x, y) {
                self.active = button.action;
                return true;
            }
        }

        false
    }
}
