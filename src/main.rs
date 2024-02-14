use std::{cell::RefCell, rc::Rc, time::Duration};

use bus::Bus;
use clap::Parser;
use config::Config;
use cursor::Cursor;
use file::LocalFile;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use tab::Tab;

use crate::{lifecycle::Lifecycle, render::Renderable};

mod config;
mod cursor;
mod file;
mod render;
mod tab;
mod lifecycle;

/// A text editor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let config = Rc::new(RefCell::new(Config::default()));

    let mut event_bus: Bus<Event> = Bus::new(10);

    let mut tabs: Vec<Tab> = vec![];
    let selected_tab: usize = 0;

    // TODO: Look into some kind of Dependency Injection solution that can allow me to resolve these without having to manually pass them their deps (config, etc.)
    let tab = Tab::new(
        LocalFile::new(args.path, config.clone())?,
        Cursor::new(config.clone()),
        event_bus.add_rx(),
        config.clone(),
    )?;
    tabs.push(tab);

    run(&mut tabs[selected_tab], event_bus, config.clone())
}

fn run(tab: &mut Tab, mut event_bus: Bus<Event>, config: Rc<RefCell<Config>>) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Editor", 800, 600) // TODO: Make resizable
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Load a font
    let mut font = ttf_context.load_font(&config.borrow().font_path, config.borrow().font_size)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    'mainloop: loop {
        // TODO: Stop handling key events here, instead make them available to whatever may need to listen to them (e.g. cursor, etc.)
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => event_bus.broadcast(event),
            }
        }

        tab.tick()?;

        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();

        tab.render(&mut canvas, &font)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
