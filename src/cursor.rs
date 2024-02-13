use sdl2::{pixels::Color, rect::Point, render::Canvas, ttf::Font, video::Window};

use crate::{
    config::{Config, Configurable},
    render::Renderable,
};

pub struct Cursor {
    pub x: u32,
    pub y: u32,
    config: Config,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            config: Config::default(),
        }
    }
}

impl Configurable for Cursor {
    fn config(&mut self, config: &Config) {
        self.config = config.clone();
    }
}

impl Renderable for Cursor {
    fn render(&self, canvas: &mut Canvas<Window>, _: &Font) -> Result<(), String> {
        let cursor_height = self.config.font_size as i32;
        let cursor_height_padding = (self.config.line_height - cursor_height) / 2;

        let char_width = 10; // TODO: Figure out how to calculate this from font

        canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        for i in -1..=0 {
            // cursor thickness
            canvas.draw_line(
                Point::new(
                    self.x as i32 * char_width + i,
                    self.y as i32 * self.config.line_height + cursor_height_padding,
                ),
                Point::new(
                    self.x as i32 * char_width as i32 + i,
                    (self.y as i32 + 1) * self.config.line_height - cursor_height_padding,
                ),
            )?;
        }
        Ok(())
    }
}
