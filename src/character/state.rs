use crate::image::Image;

pub trait State {
    fn update(&mut self, screen: &Image, time: f64) -> Option<Box<dyn State>>;
    fn draw(&self, screen: &mut Image, sprite_sheet: &Image);
    fn _on_hover(&mut self, x: i32, y: i32) -> bool;
    fn _on_click(&mut self, x: i32, y: i32) -> Option<Box<dyn State>>;
}
