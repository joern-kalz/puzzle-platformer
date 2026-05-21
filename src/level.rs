use crate::level::character::CharacterUpdateResult;
use crate::package::LevelParams;
use crate::screen::{Background, Buffer, DrawParams, FrameSet};
use character::{Action, Character};
use hud::{ActionButton, Hud};

mod character;
mod hud;

const SPAWN_INTERVAL_IN_MS: f64 = 2000.0;
const DOOR_WIDTH: i32 = 38;
const DOOR_HEIGHT: i32 = 44;
const CHARACTER_COUNT: i32 = 10;
const END_X: i32 = 100;
const END_Y: i32 = 100;

pub struct Level {
    characters: Vec<Character>,
    hud: Hud,
    last_spawn_time_in_ms: f64,
    params: LevelParams,
    left_count: i32,
    dead_count: i32,
    state: State,
}

enum State {
    Playing,
    Succeeded,
    Failed,
}

impl Level {
    pub fn new(width: i32, height: i32, params: LevelParams) -> Level {
        Level {
            characters: vec![],
            hud: Hud::new(width, height),
            last_spawn_time_in_ms: f64::MIN,
            params,
            left_count: 0,
            dead_count: 0,
            state: State::Playing,
        }
    }

    pub fn update(&mut self, background: &mut impl Background, time_in_ms: f64) {
        if self.left_count + self.dead_count >= CHARACTER_COUNT {
            self.state = if self.left_count > 0 {
                State::Succeeded
            } else {
                State::Failed
            };

            return;
        }

        self.spawn_if_due(time_in_ms);
        let mut i = 0;

        while i < self.characters.len() {
            match self.characters[i].update(background, self.params.door) {
                None => i += 1,
                Some(CharacterUpdateResult::Left) => {
                    self.characters.remove(i);
                    self.left_count += 1
                }
                Some(CharacterUpdateResult::Dead) => {
                    self.characters.remove(i);
                    self.dead_count += 1
                }
            }
        }
    }

    fn spawn_if_due(&mut self, time_in_ms: f64) {
        let time_since_spawn_in_ms = time_in_ms - self.last_spawn_time_in_ms;
        let count = self.left_count + self.dead_count + self.characters.len() as i32;

        if time_since_spawn_in_ms > SPAWN_INTERVAL_IN_MS && count < CHARACTER_COUNT {
            let character = Character::new(self.params.spawn.x, self.params.spawn.y);
            self.characters.push(character);
            self.last_spawn_time_in_ms = time_in_ms;
        }
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        self.draw_door(buffer);

        match self.state {
            State::Succeeded => buffer.draw(DrawParams::new(END_X, END_Y, FrameSet::TheEnd)),
            State::Failed => buffer.draw(DrawParams::new(END_X, END_Y, FrameSet::GameOver)),
            State::Playing => {
                for character in &self.characters {
                    character.draw(buffer);
                }

                self.hud.draw(buffer);
            }
        }
    }

    fn draw_door(&self, buffer: &mut impl Buffer) {
        let x = self.params.door.x - DOOR_WIDTH / 2;
        let y = self.params.door.y - DOOR_HEIGHT;
        buffer.draw(DrawParams::new(x, y, FrameSet::Door));
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        match self.state {
            State::Succeeded | State::Failed => true,
            State::Playing => {
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
        }
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        match self.state {
            State::Succeeded | State::Failed => {
                self.state = State::Playing;
                self.characters.clear();
                self.last_spawn_time_in_ms = f64::MIN;
                self.left_count = 0;
                self.dead_count = 0;
            }
            State::Playing => {
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
    }
}
