use crate::screen::{Background, Buffer};
use actions::{Building, Digging, Jumping, Walking};
use sprite::{Direction, Sprite};
use update_result::UpdateResult;

mod actions;
mod sprite;
mod update_result;

enum State {
    Walking(Walking),
    Building(Building),
    Digging(Digging),
    Jumping(Jumping),
    Dead,
}

pub struct Character {
    state: State,
}

pub enum Action {
    Build,
    Dig,
    Jump,
}

impl Character {
    pub fn new(x: i32, y: i32) -> Character {
        Character {
            state: State::Walking(Walking::new(Sprite {
                x,
                y,
                direction: Direction::Right,
            })),
        }
    }

    pub fn update(&mut self, background: &mut impl Background) {
        let result = match &mut self.state {
            State::Walking(walking) => walking.update(background),
            State::Building(building) => building.update(background),
            State::Digging(digging) => digging.update(background),
            State::Jumping(jumping) => jumping.update(background),
            State::Dead => None,
        };

        if let Some(result) = result {
            self.state = match result {
                UpdateResult::Walking(sprite) => State::Walking(Walking::new(sprite)),
                UpdateResult::Dead => State::Dead,
            };
        }
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        match &self.state {
            State::Walking(walking) => walking.draw(buffer),
            State::Building(building) => building.draw(buffer),
            State::Digging(digging) => digging.draw(buffer),
            State::Jumping(jumping) => jumping.draw(buffer),
            State::Dead => (),
        }
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        self.get_sprite().map_or(false, |s| s.is_inside(x, y))
    }

    pub fn perform(&mut self, action: Action) {
        if let Some(sprite) = self.get_sprite() {
            match action {
                Action::Build => {
                    self.state = State::Building(Building::new(sprite));
                }
                Action::Dig => {
                    self.state = State::Digging(Digging::new(sprite));
                }
                Action::Jump => {
                    self.state = State::Jumping(Jumping::new(sprite));
                }
            }
        }
    }

    fn get_sprite(&self) -> Option<Sprite> {
        match &self.state {
            State::Walking(walking) => Some(walking.get_sprite()),
            State::Building(building) => Some(building.get_sprite()),
            State::Digging(digging) => Some(digging.get_sprite()),
            State::Jumping(jumping) => Some(jumping.get_sprite()),
            State::Dead => None,
        }
    }
}
