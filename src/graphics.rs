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
pub struct Graphics {
    pub screen: [u8; 8 * 32],
    pub buffer: [u8; 8 * 32],
    pub need_update: bool,
    pub is_clear: bool,
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            screen: [0; 8 * 32],
            buffer: [0; 8 * 32],
            need_update: false,
            is_clear: false
        }
    }
    pub fn set(&mut self, x: u8, y: u8, pixels: Vec<u8>) -> u8 {
        self.need_update = true;
        let mut index = ((y * 64 + x) / 8) as usize;
        let offset = (x % 8) as u32;
        let mut vf = 0;
        // mask1 mask2
        for p in pixels {
            let mask2 = (2u8).pow(offset) - 1;
            let mask1 = !mask2;
            let xor1 = p >> offset ^ self.buffer[index];
            // 若像素点的设置不是8的倍数，需要将u8切分开设置
            let xor2 = if offset > 0 { (p & mask2) << (8 - offset) ^ self.buffer[index + 1] } else { 0 };
             // 检查屏幕像素碰撞
            if vf == 0 && (xor1 != p & mask1 || xor2 != p & mask2) {
                vf = 1;
            }
            self.buffer[index] = xor1;
            if mask2 > 0 && index + 1 < self.buffer.len() {
                self.buffer[index + 1] = xor2;
            }
            index += 1;
        }
        vf
    }
    pub fn clear(&mut self) {
        self.need_update = true;
        self.is_clear = true;
        self.buffer = [0; 8 * 32];
    }
}

/// big endian
pub trait BitSet {
    /// get bit
    fn bit(&self, pos: usize) -> bool;

    /// set bit
    fn set_bit(&mut self, pos: usize, val: bool) -> &mut Self;
}

impl BitSet for u8 {
    fn bit(&self, pos: usize) -> bool {
        if pos >= 8 {
            assert!(pos < 8);
        }
        *self & 1 << pos != 0
    }
    fn set_bit(&mut self, pos: usize, val: bool) -> &mut Self {
        assert!(pos < 8);
        *self = *self & !(1 << pos) | ((val as u8) << pos);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{BitSet, Graphics};

    #[test]
    fn get_bit() {
        let i = 5u8;
        assert!(!i.bit(1));
        assert!(i.bit(2));
    }

    #[test]
    fn set_bit() {
        let mut i = 5u8;
        i.set_bit(1, true);
        assert_eq!(i, 7);
        i.set_bit(0, false);
        assert_eq!(i, 6);
    }
    
    #[test]
    fn set_buffer_0_0() {
        let mut graphics = Graphics::new();
        let test1 = [1,2,3];
        graphics.set(0, 0, test1.to_vec());
        for i in 0..3 {
            assert_eq!(graphics.buffer[i], test1[i]);
        }
    }
    #[test]
    fn set_buffer_1_2() {
        let mut graphics = Graphics::new();
        graphics.set(1, 2, [1,2,3].to_vec());
        let index = ((2 * 64 + 1) / 8) as usize;
        let result = [0, 129, 1, 128];
        for i in 0..4 {
            assert_eq!(graphics.buffer[index + i], result[i]);
        }
    }
    #[test]
    fn clear() {
        let mut graphics = Graphics::new();
        graphics.buffer = [1; 8 * 32];
        graphics.clear();
        assert!(graphics.buffer.iter().all(|&a| a == 0));
        assert!(graphics.is_clear);
        assert!(graphics.need_update);
    }
}