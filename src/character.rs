use crate::image::Image;
use state::State;
use walking::Walking;

mod state;
mod walking;

pub struct Character {
    state: Box<dyn State>,
}

impl Character {
    pub fn new(screen_width: i32, screen_height: i32) -> Character {
        Character {
            state: Box::new(Walking::new(screen_width / 2, screen_height / 2)),
        }
    }

    pub fn update(&mut self, screen: &Image, time: f64) {
        if let Some(new_state) = self.state.update(screen, time) {
            self.state = new_state;
        }
    }

    pub fn draw(&self, screen: &mut Image, sprite_sheet: &Image) {
        self.state.draw(screen, sprite_sheet);
    }
}
