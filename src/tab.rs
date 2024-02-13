use std::fs;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, surface::Surface, ttf::Font, video::Window};

use crate::{
    config::{Config, Configurable},
    cursor::Cursor,
    render::Renderable,
};

pub struct Tab {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    config: Config,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Cursor::default(),
            config: Config::default(),
        }
    }
}

impl Configurable for Tab {
    fn config(&mut self, config: &Config) {
        self.config = config.clone();
        self.cursor.config(config);
    }
}

fn text_to_surface(font: &Font, text: &String, color: Color) -> Result<Surface<'static>, String> {
    font.render(text).blended(color).map_err(|e| e.to_string())
}

fn render_surface(
    canvas: &mut Canvas<Window>,
    surface: Surface,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(
        &texture,
        None,
        Some(Rect::new(x, y, surface.width(), surface.height())),
    )
}

impl Renderable for Tab {
    fn render(&self, canvas: &mut Canvas<Window>, font: &Font) -> Result<(), String> {
        for (line_n, line_txt) in self.lines.iter().enumerate() {
            if line_txt.len() > 0 {
                let surface = text_to_surface(
                    &font,
                    &line_txt.to_string(),
                    Color::RGBA(255, 255, 255, 255),
                )?;
                render_surface(
                    canvas,
                    surface,
                    0,
                    line_n as i32 * self.config.line_height,
                )?;
            }
        }
        self.cursor.render(canvas, font)?;
        Ok(())
    }
}

impl Tab {
    pub fn from_file(path: String, config: &Config) -> Result<Self, String> {
        Ok(Self {
            lines: fs::read_to_string(path)
                .map_err(|e| e.to_string())?
                .replace('\t', " ".repeat(config.tab_size as usize).as_str())
                .replace('\r', "")
                .split('\n')
                .map(str::to_string)
                .collect(),
            ..Default::default()
        })
    }
}
