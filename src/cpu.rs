
use super::memory::Memory;

pub struct CPU {
    speed: u16, // 500Hz
    V: [u8; 16], // 16 8bit register
    I: u16,  // address register
    memory: Memory
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        return CPU {
            speed: 500,
            V: [0; 16],
            I: 0,
            memory
        }
    }
}
