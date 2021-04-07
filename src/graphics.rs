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

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect::Rect};
use std::time::Duration;

enum Operation {
    Clear,
    Set(Vec<[u8; 8]>)
}

pub struct Graphics {
    screen: [[u8;8]; 32],
    op: Vec<Operation>
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            screen: [[0; 8]; 32],
            op: vec![]
        }
    }
    pub fn set(&self, pixels: Vec<[u8; 8]>) {

    }
    pub fn clear(&self) {
        
    }
}
