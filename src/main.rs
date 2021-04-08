extern crate sdl2;

mod cpu;
mod memory;
mod graphics;

use std::time::Duration;

use std::thread;
use cpu::CPU;
use memory::Memory;
use graphics::Graphics;
use sdl2::pixels::Color;
use std::sync::{Arc, Mutex};
use sdl2::{event::Event, keyboard::Keycode, rect::Rect};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Chip8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_logical_size(64, 32).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(3, 169, 244));
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
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {
                    let mut g = shared.lock().unwrap();
                    g.clear();
                }
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}
