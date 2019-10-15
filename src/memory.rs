/*
前512 bytes为系统自用，软件内存地址从 0x200 开始 VIPs with 4096 bytes of RAM (and modern implementations) 程序可用地址为0x200 - 0xE8F 3216B
在此之后的 96 bytes 0xEA0-0xEFF 用于call stack 内部使用和其他变量
最高位 256 bytes (0xF00-0xFFF) 用于显示刷新
big-endian
*/

pub struct Memory {
    pub address: [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        let mut m = Memory {
            address: [0; 4096]
        };
        m.address[0x200] = 0x60;
        m.address[0x201] = 0x01;
        m.address[0x202] = 0x70;
        m.address[0x203] = 0x02;
        return m;
    }
    // 设置内存
    pub fn set(&mut self, data: [u8; 3216]) {
        for i in 0..3216 {
            self.address[0x200 + i] = data[i];
        }
    }
}