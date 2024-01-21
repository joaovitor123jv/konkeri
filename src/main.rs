extern crate sdl2;

use clap::Parser;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration as StdDuration;
use chrono::Duration;

mod cli;
mod globals;
mod map;
mod point;
mod rect;
mod time_debugger;

use globals::Global;
use rect::Rect;
// use point::Point;
use map::Map;
use time_debugger::TimeDebugger;

pub fn run() -> Result<(), String> {
    let mut globals = Global::from_cli_args(cli::CliArgs::parse());

    let mut time_debugger = TimeDebugger::new();

    let png = Path::new("assets/cursor.png");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let delay = Duration::seconds(10);
    let game_started_at = time_debugger.get_current_time();
    let mut should_store_time = false;

    let mut events_times: Vec<Duration> = [].to_vec();
    let mut draw_color_times: Vec<Duration> = [].to_vec();
    let mut render_times: Vec<Duration> = [].to_vec();
    let mut copy_times: Vec<Duration> = [].to_vec();
    let mut present_times: Vec<Duration> = [].to_vec();

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


    'mainloop: loop {
        ::std::thread::sleep(StdDuration::new(0, 1_000_000_000u32 / globals.fps_amount));
        canvas.clear();

        time_debugger.reset();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::MouseButtonDown { .. } => is_clicking = true,
                Event::MouseButtonUp { .. } => is_clicking = false,
                Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(width, height) => {
                            globals.update_window_dimensions(width as u32, height as u32);
                            map.calc_zoomed_values(&globals);
                        }
                        _ => {}
                    };
                }

                Event::MouseWheel { y, .. } => {
                    globals.apply_zoom(y);
                    map.calc_zoomed_values(&globals);
                }
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    rectangle.x = x - 64;
                    rectangle.y = y - 256;

                    if is_clicking {
                        globals.apply_offset(xrel, yrel);
                        map.calc_zoomed_values(&globals);
                    }
                }
                _ => {}
            }
        }
        if should_store_time {
            events_times.push(time_debugger.get_elapsed_time());
        }

        i = (i + 1) % 255;
        time_debugger.reset();
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        if should_store_time {
           draw_color_times.push(time_debugger.get_elapsed_time());
        }

        time_debugger.reset();
        map.render(&mut canvas, &globals);
        if should_store_time {
            render_times.push(time_debugger.get_elapsed_time());
        }

        time_debugger.reset();
        canvas.copy(&texture, None, rectangle)?; // barril (cursor)
        if should_store_time {
            copy_times.push(time_debugger.get_elapsed_time());
        }

        time_debugger.reset();
        canvas.present();
        if should_store_time {
            present_times.push(time_debugger.get_elapsed_time());
        }

        if (time_debugger.get_current_time() - game_started_at) > delay {
            should_store_time = true;
        }
    }

    println!("Duração media dos eventos: {:?} ms", events_times.iter().sum::<Duration>().num_milliseconds() / events_times.len() as i64);
    println!("Duração media dos set_draw_color: {:?} ms", draw_color_times.iter().sum::<Duration>().num_milliseconds() / draw_color_times.len() as i64);
    println!("Duração media dos render: {:?} ms", render_times.iter().sum::<Duration>().num_milliseconds() / render_times.len() as i64);
    println!("Duração media dos canvas.copy: {:?} ms", copy_times.iter().sum::<Duration>().num_milliseconds() / copy_times.len() as i64);
    println!("Duração media dos canvas.present: {:?} ms", present_times.iter().sum::<Duration>().num_milliseconds() / present_times.len() as i64);

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
