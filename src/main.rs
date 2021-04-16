extern crate sdl2;

mod cpu;
mod memory;
mod graphics;

use graphics::BitSet;
use std::time::Duration;

use std::thread;
use cpu::CPU;
use memory::Memory;
use graphics::Graphics;
use sdl2::pixels::Color;
use std::sync::{Arc, Mutex};
use sdl2::{event::Event, keyboard::Keycode, rect::{Rect, Point}};

pub fn main() -> Result<(), String> {
    let background_color = Color::RGB(37, 17, 1);
    let foreground_color = Color::RGB(232, 233, 243);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Chip8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_logical_size(64, 32).unwrap();

    canvas.set_draw_color(background_color);
    canvas.clear();
    canvas.set_draw_color(foreground_color);
    canvas.fill_rect(Rect::new(10, 1, 30, 30))?;
    canvas.present();

    let graphics = Graphics::new();
    let shared = Arc::new(Mutex::new(graphics));
    let mut cpu = CPU::new(Memory::new(), shared.clone());
    thread::spawn(move || {
        cpu.run();
    });

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let mut graphics = shared.lock().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    graphics.clear();
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
        if graphics.need_update {
            // 清空屏幕
            if graphics.is_clear {
                graphics.screen = graphics.buffer.clone();
                canvas.set_draw_color(background_color);
                canvas.clear();
                continue;
            }
            for i in 0..256 {
                let screen = graphics.screen[i];
                let buffer = graphics.buffer[i];
                if screen == buffer {
                    continue;
                }
                let xor = buffer ^ screen;
                let bit_one = xor & buffer;
                let bit_zero = xor & !buffer;
                let mut points_one: Vec<Point> = Vec::new();
                let mut points_zero: Vec<Point> = Vec::new();
                for p in 0..8 {
                    let x = (i % 8 * 8 + p) as i32;
                    let y = (i % 8) as i32;
                    if bit_one.bit(p) {
                        points_one.push(Point::new(x, y))
                    }
                    if bit_zero.bit(p) {
                        points_zero.push(Point::new(x, y))
                    }
                }
                canvas.set_draw_color(foreground_color);
                canvas.draw_points(points_one.as_slice())?;
                canvas.set_draw_color(background_color);
                canvas.draw_points(points_zero.as_slice())?;
            }
            canvas.present();
        }
    }

    Ok(())
}
