use crate::{
    scene::package::Vec2d,
    scene::screen::{Background, Buffer},
};
use actions::{Building, Digging, Exploding, Falling, Jumping, Leaving, Walking};
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
    Falling(Falling),
    Leaving(Leaving),
    Exploding(Exploding),
}

pub struct Character {
    state: State,
}

pub enum Action {
    Build,
    Dig,
    Jump,
}

pub enum CharacterUpdateResult {
    Dead,
    Left,
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

    pub fn update(
        &mut self,
        background: &mut impl Background,
        door: Vec2d,
    ) -> Option<CharacterUpdateResult> {
        let result = match &mut self.state {
            State::Walking(walking) => walking.update(background, door),
            State::Building(building) => building.update(background),
            State::Digging(digging) => digging.update(background),
            State::Jumping(jumping) => jumping.update(background),
            State::Falling(falling) => falling.update(background),
            State::Leaving(leaving) => leaving.update(),
            State::Exploding(exploding) => exploding.update(),
        };

        let Some(result) = result else {
            return None;
        };

        match result {
            UpdateResult::Dead => Some(CharacterUpdateResult::Dead),
            UpdateResult::Left => Some(CharacterUpdateResult::Left),
            UpdateResult::Leaving(sprite) => {
                self.state = State::Leaving(Leaving::new(sprite));
                None
            }
            UpdateResult::Falling(sprite) => {
                self.state = State::Falling(Falling::new(sprite));
                None
            }
            UpdateResult::Walking(sprite) => {
                self.state = State::Walking(Walking::new(sprite));
                None
            }
            UpdateResult::Exploding(sprite) => {
                self.state = State::Exploding(Exploding::new(sprite));
                None
            }
        }
    }

    pub fn z_index(&self) -> i32 {
        match &self.state {
            State::Leaving(leaving) => leaving.get_frame_index(),
            _ => -1,
        }
    }

    pub fn draw(&self, buffer: &mut impl Buffer) {
        match &self.state {
            State::Walking(walking) => walking.draw(buffer),
            State::Building(building) => building.draw(buffer),
            State::Digging(digging) => digging.draw(buffer),
            State::Jumping(jumping) => jumping.draw(buffer),
            State::Falling(falling) => falling.draw(buffer),
            State::Leaving(leaving) => leaving.draw(buffer),
            State::Exploding(exploding) => exploding.draw(buffer),
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
            State::Falling(falling) => Some(falling.get_sprite()),
            State::Leaving(leaving) => Some(leaving.get_sprite()),
            State::Exploding(exploding) => Some(exploding.get_sprite()),
        }
    }
}
