
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
        self.exec(op)
    }
    fn exec(&mut self, op: u16) {
        let op0 = (op & 0xf000) >> 12;
        let op1 = (op & 0x0f00) >> 8;
        let op2 = (op & 0x00f0) >> 4;
        let op3 = op & 0x000f;
        println!("{:?}, {:?}, {:?}, {:?}", op0, op1, op2, op3)
    }
    fn getOP(&mut self) -> u16 { // 获取当前CPU指令 并向后2 byte移动指针
        let op: u16 = ((self.memory.address[self.pointer] as u16) << 8) + self.memory.address[self.pointer + 1] as u16;
        self.pointer += 2;
        return op;
    }
}
