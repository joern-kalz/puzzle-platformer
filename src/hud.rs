use crate::screen::{DrawParams, FrameSet, Screen};

const BUTTON_WIDTH: i32 = 60;
const BUTTON_HEIGHT: i32 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Stairs,
    Dig,
    Jump,
}

const ACTIONS: [Action; 3] = [Action::Stairs, Action::Dig, Action::Jump];

struct Button {
    x: i32,
    y: i32,
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
        let mut x = (screen_width - ACTIONS.len() as i32 * BUTTON_WIDTH) / 2;
        let y = screen_height - BUTTON_HEIGHT;

        for action in ACTIONS {
            buttons.push(Button { x, y, action });
            x += BUTTON_WIDTH + 10;
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

    pub fn draw(&self, screen: &mut Screen) {
        for button in &self.buttons {
            self.draw_background(screen, button);
            self.draw_icon(screen, button);
        }
    }

    fn draw_background(&self, screen: &mut Screen, button: &Button) {
        let frame_set = if button.action == self.active {
            FrameSet::ButtonPressed
        } else if Some(button.action) == self.hover {
            FrameSet::ButtonHovered
        } else {
            FrameSet::ButtonNormal
        };

        screen.draw(DrawParams {
            x: button.x,
            y: button.y,
            frame_set,
            frame_index: 0,
            mirror_x: false,
            mirror_y: false,
        });
    }

    fn draw_icon(&self, screen: &mut Screen, button: &Button) {
        let frame_set = match button.action {
            Action::Stairs => FrameSet::Building,
            Action::Dig => FrameSet::Digging,
            Action::Jump => FrameSet::Jumping,
        };

        screen.draw(DrawParams {
            x: button.x,
            y: button.y,
            frame_set,
            frame_index: 0,
            mirror_x: false,
            mirror_y: false,
        });
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        for button in &self.buttons {
            let x_inside = x >= button.x && x < button.x + BUTTON_WIDTH;
            let y_inside = y >= button.y && y < button.y + BUTTON_HEIGHT;

            if x_inside && y_inside {
                self.hover = Some(button.action);
                return true;
            }
        }

        self.hover = None;
        return false;
    }
}
