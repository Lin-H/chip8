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
        m.set(0x200, vec![0x6002, 0x7002]);
        return m;
    }
    // 设置内存
    pub fn set(&mut self, pos: usize, data: Vec<u16>) {
        let length = data.len();
        let mut i = 0;
        while i < length {
            self.address[pos + i * 2] = (data[i] >> 8) as u8;
            self.address[pos + i * 2 + 1] = data[i] as u8;
            i += 1;
        }
    }
}

#[test]
fn test_memory_set() {
    let mem = Memory::new();
    assert_eq!(mem.address[0x200], 0x60);
    assert_eq!(mem.address[0x201], 0x02);
    assert_eq!(mem.address[0x202], 0x70);
    assert_eq!(mem.address[0x203], 0x02);
}