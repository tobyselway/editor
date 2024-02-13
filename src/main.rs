use std::{cell::RefCell, rc::Rc, time::Duration};

use clap::Parser;
use config::Config;
use cursor::Cursor;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use tab::Tab;

use crate::render::Renderable;

mod config;
mod cursor;
mod render;
mod tab;

/// A text editor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let config = Rc::new(RefCell::new(Config::default()));

    let mut tabs: Vec<Tab> = vec![];
    let selected_tab: usize = 0;

    let tab = Tab::new(args.path, Cursor::new(config.clone()), config.clone())?;
    tabs.push(tab);

    run(&mut tabs[selected_tab], config.clone())
}

fn run(tab: &mut Tab, config: Rc<RefCell<Config>>) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Editor", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Load a font
    let mut font = ttf_context.load_font(&config.borrow().font_path, config.borrow().font_size)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    fn char_idx_to_byte(str: &String, idx: usize) -> Result<usize, String> {
        // if idx <= str.len() {
        //     return Ok(str.char_indices().count()); // TODO: Fix eol insertion & backspace
        // }
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

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => tab.cursor.x += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => tab.cursor.x -= 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => tab.cursor.y -= 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => tab.cursor.y += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    let line = &mut tab.lines[tab.cursor.y as usize];
                    line.remove(char_idx_to_byte(&line, tab.cursor.x as usize - 1)?);
                    tab.cursor.x -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Delete),
                    ..
                } => {
                    let line = &mut tab.lines[tab.cursor.y as usize];
                    line.remove(char_idx_to_byte(&line, tab.cursor.x as usize)?);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Home),
                    ..
                } => {
                    tab.cursor.x = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::End),
                    ..
                } => {
                    let line = &mut tab.lines[tab.cursor.y as usize];
                    tab.cursor.x = line.chars().count() as u32;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => {
                    config.borrow_mut().line_height += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    config.borrow_mut().line_height -= 1;
                }
                Event::TextInput { text, .. } => {
                    // println!("Input: \"{}\"", text);
                    type_text(&mut tab.lines, &mut tab.cursor.x, &tab.cursor.y, text)?;
                }
                // Event::TextEditing {
                //     text,
                //     start,
                //     length,
                //     ..
                // } => {
                //     // TODO: Not sure what to do here
                //     // This is primarily used for composing on CJK
                //     // My current guess is I should just replace from (cursor_x + start) to (cursor_x + start + length) with text
                //     // But I haven't gotten around to testing this with a Chinese, Japanese, or Korean keyboard layout yet

                //     // println!("Editing: \"{}\" s: {}  l: {}", text, start, length);
                //     // type_text(&mut tab.lines, &mut tab.cursor.x, &tab.cursor.y, text)?;
                // }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();

        tab.render(&mut canvas, &font)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
