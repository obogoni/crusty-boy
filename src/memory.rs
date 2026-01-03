const MEMORY_SIZE: usize = 0x10000; //65356 bytes

pub struct MemoryBus {
    pub memory: [u8; MEMORY_SIZE],
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn fetch_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn fetch_word(&self, addr: u16) -> u16 {
        let low = self.memory[addr as usize] as u16;
        let high = self.memory[(addr + 1) as usize] as u16;

        (high << 8) | low
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
