use std::{fs, time::Duration};

use clap::Parser;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::{Canvas, TextureCreator, TextureQuery}, surface::Surface, ttf::Font, video::{Window, WindowContext}};

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
    font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())
}

fn render_surface(texture_creator: &TextureCreator<WindowContext>, canvas: &mut Canvas<Window>, surface: Surface, x: i32, y: i32) -> Result<(), String> {
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(&texture, None, Some(Rect::new(x, y, surface.width(), surface.height())))
}

fn run(tab: Tab, config: Config) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Editor", 600, 400)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(config.font_path, config.font_size)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    canvas.set_draw_color(Color::RGB(16, 16, 16));
    canvas.clear();

    for (line_n, line_txt) in tab.contents.replace('\t', " ".repeat(config.tab_size as usize).as_str()).replace('\r', "").split('\n').enumerate() {
        if line_txt.len() > 0 {
            let surface = text_to_surface(&font, &line_txt.to_string(), Color::RGBA(255, 255, 255, 255))?;
            render_surface(&texture_creator, &mut canvas, surface, 0, line_n as i32 * config.line_height)?;
        }
    }

    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        // canvas.clear();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
