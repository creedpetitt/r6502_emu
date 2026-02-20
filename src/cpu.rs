use crate::bus::Bus;
pub struct CPU {
    pub register_a: u8,        // 8-bit Accumulator
    pub register_x: u8,        // 8-bit X Register
    pub register_y: u8,        // 8-bit Y Register
    pub status: u8,            // 8-bit Status Register (P)
    pub stack_pointer: u8,     // 8-bit Stack Pointer (S)
    pub program_counter: u16,  // 16-bit Program Counter (PC)
    pub bus: Bus,              // The physical connection to Memory
}

impl CPU{
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
}