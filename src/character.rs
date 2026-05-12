use self::sprite::{Direction, Sprite};
use building::{Building, BuildingResult};
use walking::Walking;

use crate::{
    character::walking::WalkingResult,
    screen::{Background, Buffer},
};

mod building;
mod sprite;
mod walking;

enum State {
    Walking(Walking),
    Building(Building),
    Dead,
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
            state: State::Walking(Walking::new(Sprite {
                x,
                y,
                direction: Direction::Right,
            })),
        }
    }

    pub fn update(&mut self, background: &mut impl Background) {
        match &mut self.state {
            State::Walking(walking) => match walking.update(background) {
                WalkingResult::None => (),
                WalkingResult::Dead => {
                    self.state = State::Dead;
                }
            },
            State::Building(building) => match building.update(background) {
                BuildingResult::None => (),
                BuildingResult::Walking(sprite) => {
                    self.state = State::Walking(Walking::new(sprite))
                }
                BuildingResult::Dead => {
                    self.state = State::Dead;
                }
            },
            State::Dead => (),
        };
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        match &self.state {
            State::Walking(walking) => walking.draw(buffer),
            State::Building(building) => building.draw(buffer),
            State::Dead => (),
        }
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        self.get_sprite().map_or(false, |s| s.is_inside(x, y))
    }

    pub fn perform(&mut self, action: Action) {
        if let Some(sprite) = self.get_sprite() {
            match action {
                Action::Stairs => {
                    self.state = State::Building(Building::new(sprite));
                }
            }
        }
    }

    fn get_sprite(&self) -> Option<Sprite> {
        match &self.state {
            State::Walking(walking) => Some(walking.get_sprite()),
            State::Building(building) => Some(building.get_sprite()),
            State::Dead => None,
        }
    }
}
