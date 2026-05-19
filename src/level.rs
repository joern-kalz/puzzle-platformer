use character::{Action, Character};
use hud::{ActionButton, Hud};

use crate::screen::{Background, Buffer};

mod character;
mod hud;

pub struct Level {
    character: Character,
    hud: Hud,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Level {
        let character = Character::new(250, 300);
        let hud = Hud::new(width, height);

        Level { character, hud }
    }

    pub fn update(&mut self, background: &mut impl Background) {
        self.character.update(background);
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        self.character.draw(buffer);
        self.hud.draw(buffer);
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        self.hud.is_inside(x, y) || self.character.is_inside(x, y)
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        if self.hud.is_inside(x, y) {
            self.hud.on_click(x, y);
        } else if self.character.is_inside(x, y) {
            match self.hud.get_active_action() {
                ActionButton::Build => self.character.perform(Action::Build),
                ActionButton::Dig => self.character.perform(Action::Dig),
                ActionButton::Jump => self.character.perform(Action::Jump),
            }
        }
    }
}
