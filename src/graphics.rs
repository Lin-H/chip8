/*
a monochrome screen of 64 × 32 pixels. The top-left corner of the screen is assigned (x,y) coordinates of (0x00, 0x00), and the bottom-right is assigned (0x3F, 0x1F).
00 |----------------------------------> 3F
   |                                  |
   |                                  |
   |              screen              |
   |                                  |
   |                                  |
   |                                  |
   |----------------------------------|
1F                                    3F,1F
*/
extern crate sdl2;

use sdl2::{event::Event, rect::Rect};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub struct Graphics {
    sdl_context: sdl2::Sdl
}

impl Graphics {
    pub fn new() -> Result<Graphics, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 640, 320)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window.into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_logical_size(64, 32).unwrap();

        // let mut event_pump = sdl_context.event_pump()?;
        // 'running: loop {
        //     for event in event_pump.poll_iter() {
        //         match event {
        //             Event::Quit { .. }
        //             | Event::KeyDown {
        //                 keycode: Some(Keycode::Escape),
        //                 ..
        //             } => break 'running,
        //             _ => {}
        //         }
        //     }

        //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        //     // The rest of the game loop goes here...
        // }
        Ok(Graphics {
            sdl_context: sdl_context
        })
    }
    pub fn display(&self) {
    }
}