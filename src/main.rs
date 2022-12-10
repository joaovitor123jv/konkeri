extern crate sdl2;

use clap::Parser;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::time::Duration;

mod cli;
mod globals;
mod map;
mod point;
mod rect;
mod color;
mod button;
use rect::Rect;
use button::Button;
use color::Color;
use globals::Global;
// use point::Point;
use map::Map;

pub fn run() -> Result<(), String> {
    let mut globals = Global::from_cli_args(cli::CliArgs::parse());

    let png = Path::new("assets/cursor.png");
    let mut button = Button::new("Botão de teste");
    button.rect.point.x = 100;
    button.rect.point.y = 100;
    button.rect.width = 100;
    button.rect.height = 100;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            globals.window_width,
            globals.window_height,
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        // .software()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(&png)?;
    let mut map = Map::load("assets/maps/city-iso-2.tmx", &texture_creator);
    map.offset.x = -64;
    map.offset.y = -32;

    // let mut map = Map::load("assets/maps/map-iso-1.tmx", &texture_creator);
    // map.offset.y = -500;
    // map.offset.x = -250;

    let mut i = 0;
    let mut rectangle = Rect::new(0, 0, 128, 256).to_sdl2();

    let mut event_pump = sdl_context.event_pump()?;
    let mut is_clicking: bool = false;

    map.calc_zoomed_values(&globals);


    let mut cursor_x: i32 = 0;
    let mut cursor_y: i32 = 0;

    'mainloop: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::new(i, 64, 255 - i).to_sdl2());
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::MouseButtonDown { .. } => {
                    is_clicking = true;
                    button.update(cursor_x, cursor_y, is_clicking);
                },
                Event::MouseButtonUp { .. } => {
                    is_clicking = false;
                    button.update(cursor_x, cursor_y, is_clicking);
                },
                Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(width, height) => {
                            globals.update_window_dimensions(width as u32, height as u32);
                            map.calc_zoomed_values(&globals);
                        },
                        _ => {}
                    };
                }

                Event::MouseWheel { y, .. } => {
                    globals.apply_zoom(y);
                    map.calc_zoomed_values(&globals);
                },
                Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    cursor_x = x;
                    cursor_y = y;
                    rectangle.x = x - 64;
                    rectangle.y = y - 256;
                    button.update(x, y, is_clicking);

                    if is_clicking {
                        globals.apply_offset(xrel, yrel);
                        map.calc_zoomed_values(&globals);
                    }
                }
                _ => {}
            }
        }

        map.render(&mut canvas, &globals);
        button.render(&mut canvas);

        canvas.copy(&texture, None, rectangle)?; // barril (cursor)
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / globals.fps_amount));
    }

    Ok(())
}

fn copyright_notices() {
    println!("This software makes usage of external libraries MIT licensed to build and/or run. They are listed below");
    println!("\t - \"sdl2\" via https://crates.io/crates/sdl2");
    println!("\t - \"tiled\" via https://crates.io/crates/tiled");
}

fn main() -> Result<(), String> {
    copyright_notices();
    run()?;

    Ok(())
}
