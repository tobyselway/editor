use sdl2::{render::Canvas, ttf::Font, video::Window};

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>, font: &Font) -> Result<(), String>;
}
