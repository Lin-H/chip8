/*
前512 bytes为系统自用，软件内存地址从 0x200 开始 VIPs with 4096 bytes of RAM (and modern implementations) 程序可用地址为0x200 - 0xE8F 3216B
在此之后的 96 bytes 0xEA0-0xEFF 用于call stack 内部使用和其他变量
最高位 256 bytes (0xF00-0xFFF) 用于显示刷新
big-endian
*/

pub struct Memory {
    pub address: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        let mut m = Memory { address: [0; 4096] };
        m.set(
            0,
            vec![
                0xF090, 0x9090, 0xF020, 0x6020, 0x2070, // 0, 1
                0xF010, 0xF080, 0xF0F0, 0x10F0, 0x10F0, // 2, 3
                0x9090, 0xF010, 0x10F0, 0x80F0, 0x10F0, // 4, 5
                0xF080, 0xF090, 0xF0F0, 0x1020, 0x4040, // 6, 7
                0xF090, 0xF090, 0xF0F0, 0x90F0, 0x10F0, // 8, 9
                0xF090, 0xF090, 0x90E0, 0x90E0, 0x90E0, // A, B
                0xF080, 0x8080, 0xF0E0, 0x9090, 0x90E0, // C, D
                0xF080, 0xF080, 0xF0F0, 0x80F0, 0x8080, // E, F
            ],
        );
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
    pub fn get(&self, pos: usize) -> u16 {
        let high = (self.address[pos] as u16) << 8;
        let low = self.address[pos + 1] as u16;
        high + low
    }
}
/*
SONG - TWINKLE, TWINKLE, LITTLE STAR
Description: Plays the song - Twinke, Twinkle, Little Star.
Tones are stored at address 0300H.
Number of Tones(-1) at address 0209H.
[0x6500, 0xA300, 0x7501, 0x2210, 0x3536, 0x1204, 0xF000, 0x0000, 0x6000, 0x6110, 0x6201, 0xF065, 0xF017, 0xF118, 0xF21E, 0x63F4, 0x6400, 0x7401, 0x34FF, 0x1222, 0x7301, 0x33FF, 0x1220, 0x00EE, 0x0F0F, 0x1313, 0x0D0D, 0x1300, 0x0012, 0x1211, 0x1110, 0x100F, 0x0000, 0x1313, 0x1212, 0x1111, 0x1000, 0x0013, 0x1312, 0x1211, 0x1110, 0x0000, 0x0F0F, 0x1313, 0x0D0D, 0x1300, 0x0012, 0x1211, 0x1110, 0x100F, 0x0000]
*/

#[cfg(test)]
mod tests {
    use super::Memory;
    #[test]
    fn test_memory_set() {
        let mut mem = Memory::new();
        mem.set(0x200, vec![0x6001, 0x7002]);
        assert_eq!(mem.address[0x200], 0x60);
        assert_eq!(mem.address[0x201], 0x01);
        assert_eq!(mem.address[0x202], 0x70);
        assert_eq!(mem.address[0x203], 0x02);
    }

    #[test]
    fn fonts_sprite_data() {
        let mem = Memory::new();
        assert_eq!(mem.get(0), 0xF090);
        assert_eq!(mem.get(0x12), 0x10F0);
    }
}
