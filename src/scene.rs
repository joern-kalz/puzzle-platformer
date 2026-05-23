use level::{Level, LevelUpdateResult};
use package::PACKAGES;
use score::{Score, ScoreOnClickResult};
use screen::Screen;

mod level;
mod package;
mod score;
mod screen;

const PACKAGE_COUNT: usize = 3;

pub struct Scene {
    screen: Screen,
    state: State,
    package_index: usize,
}

enum State {
    Level(Level),
    Score(Score),
}

impl Scene {
    pub fn new() -> Scene {
        let package = &package::PACKAGES[0];

        let mut screen = Screen::new();
        screen.load_background(package.background);

        let level = Level::new(screen.width(), screen.height(), package.level_params);

        Scene {
            screen,
            state: State::Level(level),
            package_index: 0,
        }
    }

    pub fn width(&self) -> i32 {
        self.screen.width()
    }

    pub fn height(&self) -> i32 {
        self.screen.height()
    }

    pub fn pixel_buffer_ptr(&self) -> *const u8 {
        self.screen.data()
    }

    pub fn update(&mut self, time_in_ms: f64) {
        match &mut self.state {
            State::Level(level) => {
                if let Some(result) = level.update(&mut self.screen, time_in_ms) {
                    let success = match result {
                        LevelUpdateResult::Success => true,
                        LevelUpdateResult::Fail => false,
                    };

                    self.state = State::Score(Score::new(
                        self.screen.width(),
                        self.screen.height(),
                        success,
                    ))
                }
            }
            State::Score(_) => (),
        }

        self.screen.clear();

        match &self.state {
            State::Level(level) => level.draw(&mut self.screen),
            State::Score(score) => score.draw(&mut self.screen),
        }
    }

    pub fn on_hover(&mut self, x: i32, y: i32) -> bool {
        match &mut self.state {
            State::Level(level) => level.on_hover(x, y),
            State::Score(score) => score.on_hover(x, y),
        }
    }

    pub fn on_click(&mut self, x: i32, y: i32) {
        match &mut self.state {
            State::Level(level) => level.on_click(x, y),
            State::Score(score) => {
                if let Some(result) = score.on_click(x, y) {
                    match result {
                        ScoreOnClickResult::Replay => (),
                        ScoreOnClickResult::Next => {
                            self.package_index = (self.package_index + 1) % PACKAGE_COUNT
                        }
                    }

                    let package = &PACKAGES[self.package_index];
                    self.screen.load_background(package.background);
                    self.state = State::Level(Level::new(
                        self.screen.width(),
                        self.screen.height(),
                        package.level_params,
                    ));
                }
            }
        }
    }
}
