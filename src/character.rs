use building::Building;
use walking::Walking;

use crate::screen::{Background, Buffer};

mod building;
mod sprite;
mod walking;

enum State {
    Walking(Walking),
    Building(Building),
}

pub struct Character {
    state: State,
}

pub enum Action {
    Stairs,
}

impl Character {
    pub fn new(x: i32, y: i32) -> Character {
        Character {
            state: State::Walking(Walking::new(sprite::Sprite {
                x,
                y,
                direction: sprite::Direction::Right,
            })),
        }
    }

    pub fn update(&mut self, background: &mut impl Background) {
        match &mut self.state {
            State::Walking(walking) => walking.update(background),
            State::Building(building) => match building.update(background) {
                building::UpdateResult::None => (),
                building::UpdateResult::SwitchToWalking(sprite) => {
                    self.state = State::Walking(Walking::new(sprite))
                }
            },
        };
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        match &self.state {
            State::Walking(walking) => walking.draw(buffer),
            State::Building(building) => building.draw(buffer),
        }
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        self.get_sprite().is_inside(x, y)
    }

    pub fn perform(&mut self, action: Action) {
        let sprite = self.get_sprite();

        match action {
            Action::Stairs => {
                self.state = State::Building(Building::new(sprite));
            }
        }
    }

    fn get_sprite(&self) -> sprite::Sprite {
        match &self.state {
            State::Walking(walking) => walking.get_sprite(),
            State::Building(building) => building.get_sprite(),
        }
    }
}
