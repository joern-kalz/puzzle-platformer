use crate::screen::Screen;
use state::State;
use walking::Walking;

mod sprite;
mod state;
mod walking;

pub struct Character {
    state: Box<dyn State>,
}

impl Character {
    pub fn new(x: i32, y: i32) -> Character {
        Character {
            state: Box::new(Walking::new(x, y)),
        }
    }

    pub fn update(&mut self, screen: &Screen, time: f64) {
        if let Some(new_state) = self.state.update(screen, time) {
            self.state = new_state;
        }
    }

    pub fn draw(&self, screen: &mut Screen) {
        self.state.draw(screen);
    }
}
