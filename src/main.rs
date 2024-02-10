use std::{collections::HashMap, fs, time::Duration};

use clap::Parser;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, TextureCreator},
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
};

/// A text editor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

struct Config {
    font_path: String,
    font_size: u16,
    line_height: i32,
    tab_size: u16,
}

struct Tab {
    contents: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let config = Config {
        font_path: "./fonts/ttf/JetBrainsMono-Regular.ttf".into(),
        font_size: 16,
        line_height: 24,
        tab_size: 4,
    };

    let tab = Tab {
        contents: fs::read_to_string(args.path).map_err(|e| e.to_string())?,
    };

    run(tab, config)
}

fn text_to_surface(font: &Font, text: &String, color: Color) -> Result<Surface<'static>, String> {
    font.render(text).blended(color).map_err(|e| e.to_string())
}

fn render_surface(
    texture_creator: &TextureCreator<WindowContext>,
    canvas: &mut Canvas<Window>,
    surface: Surface,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(
        &texture,
        None,
        Some(Rect::new(x, y, surface.width(), surface.height())),
    )
}

fn run(tab: Tab, config: Config) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Editor", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(config.font_path, config.font_size)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let mut cursor_x = 0;
    let mut cursor_y = 0;

    let cursor_height = config.font_size as i32;
    let cursor_height_padding = (config.line_height - cursor_height) / 2;

    let mut lines: Vec<String> = tab
        .contents
        .replace('\t', " ".repeat(config.tab_size as usize).as_str())
        .replace('\r', "")
        .split('\n')
        .map(str::to_string)
        .collect();

    fn type_char(lines: &mut Vec<String>, cursor_x: &mut i32, cursor_y: &i32, c: char) {
        lines[*cursor_y as usize].insert(*cursor_x as usize, c);
        *cursor_x += 1;
    }

    let mut shift_pressed = false;

    fn keymap(shift_pressed: bool) -> HashMap<Keycode, char> {
        let key_to_char: HashMap<Keycode, char> = HashMap::from([
            (Keycode::A, 'a'),
            (Keycode::B, 'b'),
            (Keycode::C, 'c'),
            (Keycode::D, 'd'),
            (Keycode::E, 'e'),
            (Keycode::F, 'f'),
            (Keycode::G, 'g'),
            (Keycode::H, 'h'),
            (Keycode::I, 'i'),
            (Keycode::J, 'j'),
            (Keycode::K, 'k'),
            (Keycode::L, 'l'),
            (Keycode::M, 'm'),
            (Keycode::N, 'n'),
            (Keycode::O, 'o'),
            (Keycode::P, 'p'),
            (Keycode::Q, 'q'),
            (Keycode::R, 'r'),
            (Keycode::S, 's'),
            (Keycode::T, 't'),
            (Keycode::U, 'u'),
            (Keycode::V, 'v'),
            (Keycode::W, 'w'),
            (Keycode::X, 'x'),
            (Keycode::Y, 'y'),
            (Keycode::Z, 'z'),
            (Keycode::Num0, '0'),
            (Keycode::Num1, '1'),
            (Keycode::Num2, '2'),
            (Keycode::Num3, '3'),
            (Keycode::Num4, '4'),
            (Keycode::Num5, '5'),
            (Keycode::Num6, '6'),
            (Keycode::Num7, '7'),
            (Keycode::Num8, '8'),
            (Keycode::Num9, '9'),
            (Keycode::Space, ' '),
            (Keycode::Comma, ','),
            (Keycode::Period, '.'),
            (Keycode::Minus, '-'),
        ]);
    
        let key_to_char_shift: HashMap<Keycode, char> = HashMap::from([
            (Keycode::A, 'A'),
            (Keycode::B, 'B'),
            (Keycode::C, 'C'),
            (Keycode::D, 'D'),
            (Keycode::E, 'E'),
            (Keycode::F, 'F'),
            (Keycode::G, 'G'),
            (Keycode::H, 'H'),
            (Keycode::I, 'I'),
            (Keycode::J, 'J'),
            (Keycode::K, 'K'),
            (Keycode::L, 'L'),
            (Keycode::M, 'M'),
            (Keycode::N, 'N'),
            (Keycode::O, 'O'),
            (Keycode::P, 'P'),
            (Keycode::Q, 'Q'),
            (Keycode::R, 'R'),
            (Keycode::S, 'S'),
            (Keycode::T, 'T'),
            (Keycode::U, 'U'),
            (Keycode::V, 'V'),
            (Keycode::W, 'W'),
            (Keycode::X, 'X'),
            (Keycode::Y, 'Y'),
            (Keycode::Z, 'Z'),
            (Keycode::Num0, '='),
            (Keycode::Num1, '!'),
            (Keycode::Num2, '"'),
            (Keycode::Num3, '#'),
            (Keycode::Num4, '$'),
            (Keycode::Num5, '%'),
            (Keycode::Num6, '&'),
            (Keycode::Num7, '/'),
            (Keycode::Num8, '('),
            (Keycode::Num9, ')'),
            (Keycode::Comma, ';'),
            (Keycode::Period, ':'),
            (Keycode::Minus, '_'),
        ]);

        if shift_pressed {
            key_to_char_shift
        } else {
            key_to_char
        }
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
                    keycode: Some(Keycode::LShift),
                    ..
                } => shift_pressed = true,
                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    ..
                } => shift_pressed = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => cursor_x += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => cursor_x -= 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => cursor_y -= 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => cursor_y += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    lines[cursor_y as usize].remove(cursor_x as usize - 1);
                    cursor_x -= 1;
                },
                Event::KeyDown {
                    keycode: Some(key),
                    ..
                } => match keymap(shift_pressed).get(&key) {
                    Some(c) => type_char(&mut lines, &mut cursor_x, &cursor_y, *c),
                    None => {},
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();

        for (line_n, line_txt) in lines.iter().enumerate() {
            if line_txt.len() > 0 {
                let surface = text_to_surface(
                    &font,
                    &line_txt.to_string(),
                    Color::RGBA(255, 255, 255, 255),
                )?;
                render_surface(
                    &texture_creator,
                    &mut canvas,
                    surface,
                    0,
                    line_n as i32 * config.line_height,
                )?;
            }
        }

        let char_width = 10; // TODO: Figure out how to calculate this from font

        canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        canvas.draw_line(
            Point::new(
                cursor_x * char_width as i32 - 1,
                cursor_y * config.line_height + cursor_height_padding,
            ),
            Point::new(
                cursor_x * char_width as i32 - 1,
                (cursor_y + 1) * config.line_height - cursor_height_padding,
            ),
        )?;
        canvas.draw_line(
            Point::new(
                cursor_x * char_width as i32,
                cursor_y * config.line_height + cursor_height_padding,
            ),
            Point::new(
                cursor_x * char_width as i32,
                (cursor_y + 1) * config.line_height - cursor_height_padding,
            ),
        )?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
