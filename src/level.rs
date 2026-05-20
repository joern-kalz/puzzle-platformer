use crate::package::LevelParams;
use crate::screen::{Background, Buffer};
use character::{Action, Character};
use hud::{ActionButton, Hud};

mod character;
mod hud;

const SPAWN_INTERVAL_IN_MS: f64 = 2000.0;

pub struct Level {
    characters: Vec<Character>,
    hud: Hud,
    last_spawn_time_in_ms: f64,
    params: LevelParams,
}

impl Level {
    pub fn new(width: i32, height: i32, params: LevelParams) -> Level {
        Level {
            characters: vec![],
            hud: Hud::new(width, height),
            last_spawn_time_in_ms: f64::MIN,
            params,
        }
    }

    pub fn update(&mut self, background: &mut impl Background, time_in_ms: f64) {
        if time_in_ms - self.last_spawn_time_in_ms > SPAWN_INTERVAL_IN_MS {
            let character = Character::new(self.params.spawn.x, self.params.spawn.y);
            self.characters.push(character);
            self.last_spawn_time_in_ms = time_in_ms;
        }

        for character in &mut self.characters {
            character.update(background);
        }
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        for character in &self.characters {
            character.draw(buffer);
        }

        self.hud.draw(buffer);
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        if self.hud.on_hover(x, y) {
            return true;
        }

        for character in &self.characters {
            if character.is_inside(x, y) {
                return true;
            }
        }

        false
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        if self.hud.on_click(x, y) {
            return;
        }

        for character in &mut self.characters {
            if character.is_inside(x, y) {
                match self.hud.get_active_action() {
                    ActionButton::Build => character.perform(Action::Build),
                    ActionButton::Dig => character.perform(Action::Dig),
                    ActionButton::Jump => character.perform(Action::Jump),
                }
                return;
            }
        }
    }
}
