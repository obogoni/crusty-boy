pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }
}
