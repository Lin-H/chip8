
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
        loop {
            let op = self.get_op();
            if op == 0 { break }
            self.exec(op);
            self.next();
        }
    }
    fn exec(&mut self, op: u16) {
        let op0 = (op & 0xf000) >> 12;
        let op1 = (op & 0x0f00) >> 8;
        let op2 = (op & 0x00f0) >> 4;
        let op3 = op & 0x000f;
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
                let kk = ((k1 << 4) + k2) as u8;
                if self.v[x as usize] == kk {
                    self.next();
                }
            },
            // Skip next Instruction if VX≠KK.
            (4, x, k1, k2) => {
                let kk = ((k1 << 4) + k2) as u8;
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
                self.v[x as usize] = ((k1 << 4) + k2) as u8;
            },
            // Add KK to VX, VX=VX+KK
            (7, x, k1, k2) => {
                self.v[x as usize] += ((k1 << 4) + k2) as u8;
            },
            // Copy VY to VX, VX=VY
            (8, x, y, 0) => {
                self.v[x as usize] = self.v[y as usize];
            },
            // Logical OR VX with VY, VX=VX│VY
            (8, x, y, 1) => {
                let x = x as usize;
                self.v[x] = self.v[x] | self.v[y as usize];
            },
            // Logical AND VX with VY, VX=VX&VY
            (8, x, y, 2) => {
                let x = x as usize;
                self.v[x] = self.v[x] & self.v[y as usize];
            },
            // Logical XOR VX with VY, VX=VX XOR VY
            (8, x, y, 3) => {
                let x = x as usize;
                self.v[x] = self.v[x] ^ self.v[y as usize];
            },
            // Add VY to VX.If result >FF, then VF=1; VX=VX+VY
            (8, x, y, 4) => {
                let x = x as usize;
                let y = y as usize;
                let sum = self.v[x] + self.v[y];
                self.v[0xf] = if sum >= 16 { 1 } else { 0 };
                self.v[x] = sum % 16;
            },
            // Subtract VY. If VX<VY, then VF=0; VX=VX-VY
            (8, x, y, 5) => {
                let x = x as usize;
                let y = y as usize;
                let sum;
                if self.v[x] < self.v[y] {
                    self.v[0xf] = 0;
                    sum = self.v[y] - self.v[x];
                } else {
                    sum = self.v[x] - self.v[y];
                }
                self.v[x] = sum;
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

#[cfg(test)]
mod tests {
    use super::CPU;
    use super::Memory;
    #[test]
    fn test_op_add() {
        let mut mem = Memory::new();
        mem.set(0x200, vec![0x6001, 0x6102, 0x8014]);
        let mut cpu = CPU::new(mem);
        cpu.run();
        assert_eq!(cpu.v[0x0], 0x3);
    }
}