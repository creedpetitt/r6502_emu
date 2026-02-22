pub struct Bus {
    pub memory: [u8; 65536],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: [0; 65536],
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        // Cast address to usize for array indexing
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}