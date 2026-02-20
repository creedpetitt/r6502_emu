use crate::bus::Bus;

pub const FLAG_CARRY: u8     = 0b0000_0001;
pub const FLAG_ZERO: u8      = 0b0000_0010;
pub const FLAG_INTERRUPT: u8 = 0b0000_0100;
pub const FLAG_DECIMAL: u8   = 0b0000_1000;
pub const FLAG_BREAK: u8     = 0b0001_0000;
pub const FLAG_UNUSED: u8    = 0b0010_0000;
pub const FLAG_OVERFLOW: u8  = 0b0100_0000;
pub const FLAG_NEGATIVE: u8  = 0b1000_0000;

pub struct CPU {
    pub register_a: u8,        // 8-bit Accumulator
    pub register_x: u8,        // 8-bit X Register
    pub register_y: u8,        // 8-bit Y Register
    pub status: u8,            // 8-bit Status Register (P)
    pub stack_pointer: u8,     // 8-bit Stack Pointer (S)
    pub program_counter: u16,  // 16-bit Program Counter (PC)
    pub bus: Bus,              // The physical connection to Memory
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            stack_pointer: 0xFD,
            program_counter: 0,
            bus,
        }
    }

    fn fetch(&mut self) -> u8 {
        let address = self.program_counter;
        let data = self.bus.read(address);
        self.program_counter += 1;
        data
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        crate::opcodes::execute(self, opcode);
    }
}