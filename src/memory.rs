/*
前512 bytes为系统自用，软件内存地址从 0x200 开始 VIPs with 4096 bytes of RAM (and modern implementations) 程序可用地址为0x200 - 0xE8F 3216B
在此之后的 96 bytes 0xEA0-0xEFF 用于call stack 内部使用和其他变量
最高位 256 bytes (0xF00-0xFFF) 用于显示刷新
big-endian
*/

pub struct Memory {
    pub memory: [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            memory: [0; 4096]
        }
    }
    // 设置内存
    pub fn set(&mut self, data: [u8; 3216]) {
        let mut m = &self.memory[0x200..];
    }
}