
use std::usize;

use super::memory::Memory;

pub struct CPU {
    speed: u16, // 500Hz
    v: [u8; 16], // 16 8bit register
    i: u16,  // 16bit address register
    stack: Vec<u16>,
    memory: Memory,
    /* program counter */
    pc: usize
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        return CPU {
            speed: 500,
            v: [0; 16],
            i: 0,
            memory,
            stack: Vec::new(),
            pc: 0x200
        }
    }
    pub fn run(&mut self) {
        let op = self.get_op();
        self.exec(op);
        self.next();
    }
    fn exec(&mut self, op: u16) {
        let op0 = (op & 0xf000) >> 12;
        let op1 = (op & 0x0f00) >> 8;
        let op2 = (op & 0x00f0) >> 4;
        let op3 = op & 0x000f;
        // let op = ;
        println!("{:?}, {:?}, {:?}, {:?}", op0, op1, op2, op3);
        match (op0, op1, op2, op3) {
            (0, 0, 0, 0) => println!("No Operation"),
            (0, 0, 0xe, 0) => {
                println!("Clear the Screen");
            },
            (0, 0, 0xe, 0xe) => println!("Return from Subroutine"),
            // Jump to location MMM.
            (1, m1, m2, m3) => {
                let address = (m1 << 8) + (m2 << 4) + m3;
                self.pc = address as usize;
            },
            // Call Subroutine.
            (2, m1, m2, m3) => {
                let address = (m1 << 8) + (m2 << 4) + m3;
                self.stack.push(self.pc as u16);
                self.pc = address as usize;
                let op = self.get_op();
                self.exec(op);
            },
            // Skip next Instruction if VX=KK
            (3, x, k1, k2) => {
                let kk = (k1 << 4 + k2) as u8;
                if self.v[x as usize] == kk {
                    self.next();
                }
            },
            // Skip next Instruction if VX≠KK.
            (4, x, k1, k2) => {
                let kk = (k1 << 4 + k2) as u8;
                if self.v[x as usize] != kk {
                    self.next();
                }
            },
            // Skip next Instruction if VX=VY.
            (5, x, y, 0) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.next();
                }
            },
            // Assign Hex value KK to Register VX
            (6, x, k1, k2) => {
                self.v[x as usize] = (k1 << 4 + k2) as u8;
            },
            _ => println!("{:?}{:?}{:?}{:?} not covered", op0, op1, op2, op3)
        }
    }
    fn get_op(&mut self) -> u16 { // 获取当前CPU指令 并向后2 byte移动指针
        let op: u16 = ((self.memory.address[self.pc] as u16) << 8) + self.memory.address[self.pc + 1] as u16;
        return op;
    }
    fn next(&mut self) {
        self.pc += 2;
    }
}
