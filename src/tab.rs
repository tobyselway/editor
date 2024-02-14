use std::{cell::RefCell, rc::Rc};

use bus::BusReader;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, surface::Surface, ttf::Font, video::Window
};

use crate::{
    config::Config, cursor::Cursor, file::ReadFile, lifecycle::Lifecycle, render::Renderable,
};

pub struct Tab {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    config: Rc<RefCell<Config>>,
    event_channel: BusReader<Event>,
}

impl Tab {
    pub fn new(
        file: impl ReadFile,
        cursor: Cursor,
        event_channel: BusReader<Event>,
        config: Rc<RefCell<Config>>,
    ) -> Result<Self, String> {
        Ok(Self {
            lines: file.all_lines(),
            cursor,
            event_channel,
            config: config.clone(),
        })
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
                    line_n as i32 * self.config.borrow().line_height,
                )?;
            }
        }
        self.cursor.render(canvas, font)?;
        Ok(())
    }
}

fn char_idx_to_byte(str: &String, idx: usize) -> Result<usize, String> {
    if idx >= str.char_indices().count() {
        return Ok(str.char_indices().count());
    }
    str.char_indices()
        .nth(idx)
        .ok_or("No valid char index found at cursor position".to_string())
        .map(|(byte_pos, _)| byte_pos)
}

fn type_text(
    lines: &mut Vec<String>,
    cursor_x: &mut u32,
    cursor_y: &u32,
    text: String,
) -> Result<(), String> {
    let line = &mut lines[*cursor_y as usize];
    line.insert_str(char_idx_to_byte(&line, *cursor_x as usize)?, text.as_str());
    *cursor_x += 1;
    Ok(())
}

impl Lifecycle for Tab {
    fn tick(&mut self) -> Result<(), String> {
        loop {
            match self.event_channel.try_recv() {
                Err(_) => break,
                Ok(event) => match event {
                    Event::TextInput { text, .. } => {
                        // println!("Input: \"{}\"", text);
                        type_text(&mut self.lines, &mut self.cursor.x, &self.cursor.y, text)?;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        if (self.cursor.x as usize) < self.lines[self.cursor.y as usize].char_indices().count() {
                            self.cursor.x += 1;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        if self.cursor.x <= 0 {
                            break;
                        }
                        self.cursor.x -= 1;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => {
                        if self.cursor.y <= 0 {
                            break;
                        }
                        self.cursor.y -= 1;
                        if (self.cursor.x as usize) > self.lines[self.cursor.y as usize].char_indices().count() {
                            self.cursor.x = self.lines[self.cursor.y as usize].len() as u32;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        if self.cursor.y as usize >= self.lines.len() - 1 {
                            break;
                        }
                        self.cursor.y += 1;
                        if (self.cursor.x as usize) > self.lines[self.cursor.y as usize].char_indices().count() {
                            self.cursor.x = self.lines[self.cursor.y as usize].len() as u32;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Backspace),
                        ..
                    } => {
                        if self.cursor.x <= 0 {
                            break; // TODO: Remove line break
                        }
                        let line = &mut self.lines[self.cursor.y as usize];
                        line.remove(char_idx_to_byte(&line, self.cursor.x as usize - 1)?);
                        self.cursor.x -= 1;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Delete),
                        ..
                    } => {
                        let line = &mut self.lines[self.cursor.y as usize];
                        line.remove(char_idx_to_byte(&line, self.cursor.x as usize)?);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Home),
                        ..
                    } => {
                        self.cursor.x = 0;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::End),
                        ..
                    } => {
                        let line = &mut self.lines[self.cursor.y as usize];
                        self.cursor.x = line.char_indices().count() as u32;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::F2),
                        ..
                    } => {
                        self.config.borrow_mut().line_height += 1;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::F1),
                        ..
                    } => {
                        self.config.borrow_mut().line_height -= 1;
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}
