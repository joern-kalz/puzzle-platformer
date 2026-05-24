use crate::scene::screen::{Buffer, DrawParams, FrameSet};

const BUTTON_WIDTH: i32 = 60;
const BUTTON_HEIGHT: i32 = 60;
const BUTTON_SPACING: i32 = 10;
const MESSAGE_WIDTH: i32 = 180;
const MESSAGE_HEIGHT: i32 = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScoreOnClickResult {
    Replay,
    Next,
}

struct Button {
    x: i32,
    y: i32,
    action: ScoreOnClickResult,
}

impl Button {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        let x_inside = x >= self.x && x < self.x + BUTTON_WIDTH;
        let y_inside = y >= self.y && y < self.y + BUTTON_HEIGHT;
        x_inside && y_inside
    }
}

pub struct Score {
    buttons: Vec<Button>,
    hover: Option<ScoreOnClickResult>,
    success: bool,
    screen_width: i32,
    screen_height: i32,
}

impl Score {
    pub fn new(screen_width: i32, screen_height: i32, success: bool) -> Self {
        let count = if success { 2 } else { 1 };
        let buttons_width = count * BUTTON_WIDTH;
        let spacings_width = (count - 1) * BUTTON_SPACING;
        let mut x = (screen_width - buttons_width - spacings_width) / 2;
        let y = screen_height / 2 + MESSAGE_HEIGHT;

        let mut buttons = Vec::new();
        buttons.push(Button {
            x,
            y,
            action: ScoreOnClickResult::Replay,
        });

        if success {
            x += BUTTON_WIDTH + BUTTON_SPACING;
            buttons.push(Button {
                x,
                y,
                action: ScoreOnClickResult::Next,
            });
        }

        Self {
            buttons,
            hover: None,
            success,
            screen_width,
            screen_height,
        }
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        self.draw_message(buffer);

        for button in &self.buttons {
            self.draw_background(buffer, button);
            self.draw_icon(buffer, button);
        }
    }

    fn draw_message(&self, buffer: &mut impl Buffer) {
        let frame_set = if self.success {
            FrameSet::TheEnd
        } else {
            FrameSet::GameOver
        };
        let x = (self.screen_width - MESSAGE_WIDTH) / 2;
        let y = (self.screen_height - MESSAGE_HEIGHT) / 2;
        buffer.draw(DrawParams::new(x, y, frame_set));
    }

    fn draw_background(&self, buffer: &mut impl Buffer, button: &Button) {
        let frame_set = if Some(button.action) == self.hover {
            FrameSet::ButtonHovered
        } else {
            FrameSet::ButtonNormal
        };

        buffer.draw(DrawParams::new(button.x, button.y, frame_set));
    }

    fn draw_icon(&self, buffer: &mut impl Buffer, button: &Button) {
        let frame_set = match button.action {
            ScoreOnClickResult::Replay => FrameSet::Replay,
            ScoreOnClickResult::Next => FrameSet::Next,
        };

        buffer.draw(DrawParams::new(button.x, button.y, frame_set));
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        for button in &self.buttons {
            if button.is_inside(x, y) {
                self.hover = Some(button.action);
                return true;
            }
        }

        self.hover = None;
        false
    }

    pub fn on_click(&mut self, x: i32, y: i32) -> Option<ScoreOnClickResult> {
        for button in &self.buttons {
            if button.is_inside(x, y) {
                return Some(button.action);
            }
        }

        None
    }
}
