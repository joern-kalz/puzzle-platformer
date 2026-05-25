use super::package::LevelParams;
use crate::scene::screen::{Background, Buffer, DrawParams, FrameSet};
use character::CharacterUpdateResult;
use character::{Action, Character};
use hud::{ActionButton, Hud};

mod character;
mod hud;

const SPAWN_INTERVAL_IN_MS: f64 = 2000.0;
const DOOR_WIDTH: i32 = 38;
const DOOR_HEIGHT: i32 = 44;
const CHARACTER_COUNT: i32 = 10;
const MAX_CLICK_DISTANCE: f32 = 60.0;

pub struct Level {
    characters: Vec<Character>,
    hud: Hud,
    last_spawn_time_in_ms: f64,
    params: LevelParams,
    left_count: i32,
    dead_count: i32,
}

pub enum LevelUpdateResult {
    Success,
    Fail,
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
        }
    }

    pub fn update(
        &mut self,
        background: &mut impl Background,
        time_in_ms: f64,
    ) -> Option<LevelUpdateResult> {
        if self.left_count + self.dead_count >= CHARACTER_COUNT {
            if self.left_count > 0 {
                return Some(LevelUpdateResult::Success);
            } else {
                return Some(LevelUpdateResult::Fail);
            };
        }

        self.spawn_if_due(time_in_ms);
        self.update_characters(background);

        None
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

    fn update_characters(&mut self, background: &mut impl Background) {
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

    pub fn draw(&self, buffer: &mut impl Buffer) {
        self.draw_door(buffer);

        let mut characters: Vec<&Character> = self.characters.iter().collect();
        characters.sort_by_key(|character| -character.z_index());

        for character in characters {
            character.draw(buffer);
        }

        self.hud.draw(buffer);
    }

    fn draw_door(&self, buffer: &mut impl Buffer) {
        let x = self.params.door.x - DOOR_WIDTH / 2;
        let y = self.params.door.y - DOOR_HEIGHT;
        buffer.draw(DrawParams::new(x, y, FrameSet::Door));
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

        let mut nearest_index: Option<usize> = None;
        let mut nearest_distance: f32 = f32::MAX;

        for (i, character) in self.characters.iter().enumerate() {
            if let Some(d) = character.distance(x, y) {
                if d < nearest_distance && d <= MAX_CLICK_DISTANCE {
                    nearest_distance = d;
                    nearest_index = Some(i);
                }
            }
        }

        if let Some(i) = nearest_index {
            let action = self.hud.get_active_action();
            match action {
                ActionButton::Build => self.characters[i].perform(Action::Build),
                ActionButton::Dig => self.characters[i].perform(Action::Dig),
                ActionButton::Jump => self.characters[i].perform(Action::Jump),
            }
        }
    }
}
