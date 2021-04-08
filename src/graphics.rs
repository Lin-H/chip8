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
enum Operation {
    Clear,
    Set(u8, u8, Vec<u8>)
}
pub struct Graphics {
    screen: [u8; 8 * 32],
    op: Vec<Operation>
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            screen: [0; 8 * 32],
            op: vec![]
        }
    }
    pub fn set(&mut self, x: u8, y: u8, pixels: Vec<u8>) {
        self.op.push(Operation::Set(x, y, pixels));
    }
    pub fn clear(&mut self) {
        self.op.push(Operation::Clear);
        self.screen = [0; 8 * 32];
    }
}
