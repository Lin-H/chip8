
use std::usize;
use rand;

use std::sync::{Arc, Mutex};
use super::memory::Memory;
use super::Graphics;

pub struct CPU {
    graphics: Arc<Mutex<Graphics>>,
    speed: u16, // 500Hz
    v: [u8; 16], // 16 8bit register
    i: u16,  // 16bit address register
    stack: Vec<u16>,
    memory: Memory,
    /* program counter */
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    pitch: u8
}

impl CPU {
    pub fn new(memory: Memory, graphics: Arc<Mutex<Graphics>>) -> CPU {
        return CPU {
            graphics,
            speed: 500,
            v: [0; 16],
            i: 0,
            memory,
            stack: Vec::new(),
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            pitch: 0
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
        let op0 = ((op & 0xf000) >> 12) as usize;
        let op1 = ((op & 0x0f00) >> 8) as usize;
        let op2 = ((op & 0x00f0) >> 4) as usize;
        let op3 = (op & 0x000f) as usize;
        println!("{:X}, {:X}, {:X}, {:X}", op0, op1, op2, op3);
        match (op0, op1, op2, op3) {
            (0, 0, 0, 0) => println!("No Operation"),
            (0, 0, 0xE, 0) => {
                self.graphics.lock().unwrap().clear();
                println!("Clear the Screen");
            },
            (0, 0, 0xE, 0xE) => println!("Return from Subroutine"),
            // Jump to location MMM.
            (1, m1, m2, m3) => {
                let address = (m1 << 8) + (m2 << 4) + m3;
                self.pc = address;
            },
            // Call Subroutine.
            (2, m1, m2, m3) => {
                let address = (m1 << 8) + (m2 << 4) + m3;
                self.stack.push(self.pc as u16);
                self.pc = address;
                let op = self.get_op();
                self.exec(op);
            },
            // Skip next Instruction if VX=KK
            (3, x, k1, k2) => {
                let kk = ((k1 << 4) + k2) as u8;
                if self.v[x] == kk {
                    self.next();
                }
            },
            // Skip next Instruction if VX≠KK.
            (4, x, k1, k2) => {
                let kk = ((k1 << 4) + k2) as u8;
                if self.v[x] != kk {
                    self.next();
                }
            },
            // Skip next Instruction if VX=VY.
            (5, x, y, 0) => {
                if self.v[x] == self.v[y] {
                    self.next();
                }
            },
            // Assign Hex value KK to Register VX
            (6, x, k1, k2) => {
                self.v[x] = ((k1 << 4) + k2) as u8;
            },
            // Add KK to VX, VX=VX+KK
            (7, x, k1, k2) => {
                self.v[x] += ((k1 << 4) + k2) as u8;
            },
            // Copy VY to VX, VX=VY
            (8, x, y, 0) => {
                self.v[x] = self.v[y];
            },
            // Logical OR VX with VY, VX=VX│VY
            (8, x, y, 1) => {
                self.v[x] = self.v[x] | self.v[y];
            },
            // Logical AND VX with VY, VX=VX&VY
            (8, x, y, 2) => {
                let x = x;
                self.v[x] = self.v[x] & self.v[y];
            },
            // Logical XOR VX with VY, VX=VX XOR VY
            (8, x, y, 3) => {
                let x = x;
                self.v[x] = self.v[x] ^ self.v[y];
            },
            // Add VY to VX.If result >FF, then VF=1; VX=VX+VY
            (8, x, y, 4) => {
                let sum = self.v[x] + self.v[y];
                self.v[0xf] = if sum >= 16 { 1 } else { 0 };
                self.v[x] = sum % 16;
            },
            // Subtract VY. If VX<VY, then VF=0; VX=VX-VY
            (8, x, y, 5) => {
                let sum;
                if self.v[x] < self.v[y] {
                    self.v[0xf] = 0;
                    sum = self.v[y] - self.v[x];
                } else {
                    sum = self.v[x] - self.v[y];
                }
                self.v[x] = sum;
            },
            // Skip next Instruction if VX≠VY; SKF VX≠VY
            (9, x, y, 0) => {
                if self.v[x] != self.v[y] {
                    self.next();
                }
            },
            // Set memory Index Pointer to MMM; I=MMM
            (0xA, m1, m2, m3) => {
                let address = (m1 << 8) + (m2 << 4) + m3;
                self.i = address as u16;
            },
            // Jump to location MMM+V0; GOTO MMM+V0
            (0xB, m1, m2, m3) => {
                let address = (m1 << 8) + (m2 << 4) + m3 + (self.v[0] as usize);
                self.pc = address;
            },
            // Get random byte, then AND with KK; VX=RND.KK
            (0xC, x, k1, k2) => {
                let rand_byte = rand::random::<u8>();
                self.v[x] = rand_byte & ((k1 << 4) + k2) as u8;
            },
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            (0xD, x, y, n) => {
                println!("Display N-byte pattern at (VX,VY); ");
                let location = self.i as usize;
                let mut pixels: Vec<u8> = vec![];
                for i in 0..n {
                    pixels.push(self.memory.address[location + i]);
                    print!("{:X} ", self.memory.address[location + i]);
                }
                println!("");
                self.graphics.lock().unwrap().set(self.v[x], self.v[y], pixels);
            },
            // Skip if key down =VX. No wait.; SKF VX=KEY
            (0xE, x, 9, 0xE) => {
                println!("Skip if key down =VX. No wait")
            },
            // Skip if key down ≠VX. No wait.; SKF VX≠KEY
            (0xE, x, 0xA, 1) => {
                println!("Skip if key down ≠VX. No wait")
            },
            // Jump to Monitor (CHIPOS); STOP 
            (0xF, 0, 0, 0) => {
                println!("Jump to Monitor (CHIPOS)")
            },
            // Set the delay timer to the value of register VX; VX=TIME
            (0xF, x, 0, 7) => {
                self.v[x] = self.delay_timer;
            },
            // Wait for a keypress and store the result in register VX; VX=KEY
            (0xF, x, 0, 0xA) => {
                println!("Wait for a keypress and store the result in register VX")
            },
            // Initialize Timer. 01=20 mS.; TIME=VX
            (0xF, x, 1, 5) => {
                self.delay_timer = self.v[x];
            },
            // Set the Pitch of the Tone Generator to VX.; PITCH=VX
            (0xF, x, 1, 7) => {
                self.pitch = self.v[x];
            },
            // Sound Tone for 20 timesVX milliseconds; TONE=VX
            (0xF, x, 1, 8) => {
                self.sound_timer = self.v[x];
            },
            // Add VX to Memory Pointer; I=I+VX
            (0xF, x, 1, 0xE) => {
                self.i += x as u16;
            },
            // Set I = location of sprite for digit Vx.
            (0xF, x, 2, 9) => {
                let digit = self.v[x];
                self.i = (digit * 5) as u16;
            },
            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            (0xF, x, 3, 3) => {
                let digit = self.v[x];
                let location = self.i as usize;
                self.memory.address[location] = digit / 100;
                self.memory.address[location + 1] = digit / 10 % 10;
                self.memory.address[location + 2] = digit % 10;
            },
            // Store registers V0 through Vx in memory starting at location I.
            (0xF, x, 5, 5) => {
                let location = self.i as usize;
                for i in 0..=x {
                    self.memory.address[location + i] = self.v[i];
                }
                // 部分文档说S-CHIP8修改i
                self.i += (x + 1) as u16;
            },
            // Read registers V0 through Vx from memory starting at location I.
            (0xF, x, 6, 5) => {
                let location = self.i as usize;
                for i in 0..=x {
                    self.v[i] = self.memory.address[location + i];
                }
                self.i += (x + 1) as u16;
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
    use std::sync::{Arc, Mutex};

    use super::CPU;
    use super::Memory;
    use super::Graphics;
    use rand;
    #[test]
    fn op_add() {
        let graphics = Graphics::new();
        let mut mem = Memory::new();
        mem.set(0x200, vec![0x6001, 0x6102, 0x8014]);
        let mut cpu = CPU::new(mem, Arc::new(Mutex::new(graphics)));
        cpu.run();
        assert_eq!(cpu.v[0x0], 0x3);
    }

    // 测试将hex显示到屏幕上，转为BCD码逐位显示
    #[test]
    fn hex_to_decimal_converter() {
        let graphics = Graphics::new();
        let mut mem = Memory::new();
        let hex: u8 = rand::random::<u8>();
        mem.set(0x200, vec![0x00E0, hex as u16 + 0x6300, 0x6400, 0x6500, 0xA500, 0xF333, 0xF265, 0xF029, 0xD455, 0xF129, 0x7408, 0xD455, 0xF229, 0x7408, 0xD455, 0xF000]);
        let mut cpu = CPU::new(mem, Arc::new(Mutex::new(graphics)));
        cpu.run();
        assert_eq!(hex / 100, cpu.v[0]);
        assert_eq!(hex / 10 % 10, cpu.v[1]);
        assert_eq!(hex % 10, cpu.v[2]);
    }
}