
use super::memory::Memory;

pub struct CPU {
    speed: u16, // 500Hz
    V: [u8; 16], // 16 8bit register
    I: u16,  // 16bit address register
    memory: Memory,
    pointer: usize
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        return CPU {
            speed: 500,
            V: [0; 16],
            I: 0,
            memory,
            pointer: 0x200
        }
    }
    pub fn run(&mut self) {
        let op = self.getOP();
        
    }
    fn getOP(&mut self) -> u16 { // 获取当前CPU指令 并向后2 byte移动指针
        let op = self.memory.address[self.pointer] << 8 + self.memory.address[self.pointer + 1];
        self.pointer += 2;
        return op as u16;
    }
}
