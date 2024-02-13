use std::{cell::RefCell, rc::Rc};

use sdl2::{pixels::Color, rect::Point, render::Canvas, ttf::Font, video::Window};

use crate::{
    config::Config,
    render::Renderable,
};

pub struct Cursor {
    pub x: u32,
    pub y: u32,
    config: Rc<RefCell<Config>>,
}

impl Cursor {
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        Self {
            x: 0,
            y: 0,
            config: config.clone(),
        }
    }
}

impl Renderable for Cursor {
    fn render(&self, canvas: &mut Canvas<Window>, font: &Font) -> Result<(), String> {
        let cursor_height = self.config.borrow().font_size as i32;
        let cursor_height_padding = (self.config.borrow().line_height - cursor_height) / 2;

        let char_width = font.size_of_char('A').map_err(|e| e.to_string() )?.0 as i32;

        canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        for i in -1..=0 {
            // cursor thickness
            canvas.draw_line(
                Point::new(
                    self.x as i32 * char_width + i,
                    self.y as i32 * self.config.borrow().line_height + cursor_height_padding,
                ),
                Point::new(
                    self.x as i32 * char_width as i32 + i,
                    (self.y as i32 + 1) * self.config.borrow().line_height - cursor_height_padding,
                ),
            )?;
        }
        Ok(())
    }
}
