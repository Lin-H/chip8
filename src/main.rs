extern crate sdl2;

mod cpu;
mod graphics;
mod memory;

use graphics::BitSet;
use std::time::Duration;

use cpu::CPU;
use graphics::Graphics;
use memory::Memory;
use sdl2::pixels::Color;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::{Point, Rect},
};

pub fn main() -> Result<(), String> {
    let background_color = Color::RGB(0, 0, 0);
    let foreground_color = Color::RGB(50, 255, 102);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip8", 640, 320)
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

    let mut graphics = Graphics::new();
    let mut mem = Memory::new();
    mem.set(
        0x200,
        vec![
            0x00E0,
            0xfe as u16 + 0x6300,
            0x6400,
            0x6500,
            0xA500,
            0xF333,
            0xF265,
            0xF029,
            0xD455,
            0xF129,
            0x7408,
            0xD455,
            0xF229,
            0x7408,
            0xD455,
            0xF000,
        ],
    );
    let mut cpu = CPU::new(mem, &mut graphics);
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        cpu.tick();
        let mut graphics = &mut *cpu.graphics;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    println!("press A");
                    canvas.set_draw_color(background_color);
                    canvas.clear();
                    canvas.present();
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        if graphics.need_update {
            // 清空屏幕
            if graphics.is_clear {
                graphics.screen = graphics.buffer.clone();
                canvas.set_draw_color(background_color);
                canvas.clear();
                graphics.is_clear = false;
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
                    let y = (i / 8) as i32;
                    // 从高位，也就是左边开始
                    if bit_one.bit(7 - p) {
                        points_one.push(Point::new(x, y))
                    }
                    if bit_zero.bit(7 - p) {
                        points_zero.push(Point::new(x, y))
                    }
                }
                canvas.set_draw_color(foreground_color);
                canvas.draw_points(points_one.as_slice()).unwrap();
                canvas.set_draw_color(background_color);
                canvas.draw_points(points_zero.as_slice()).unwrap();
            }
            canvas.present();
            graphics.need_update = false;
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / cpu.speed));
    }

    Ok(())
}
