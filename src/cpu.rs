#![allow(dead_code)]

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
    pub cycles: u64,           // Tracks the number of clock cycles executed
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            stack_pointer: 0xFD,
            program_counter: 0,
            bus: Bus::new(),
            cycles: 0,
        }
    }

    pub fn get_operand(&mut self) -> u8 {
        let address = self.program_counter;
        let data = self.bus.read(address);
        self.program_counter += 1;
        data
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let lo = self.get_operand() as u16;
        let hi = self.get_operand() as u16;
        (hi << 8) | lo
    }

    pub fn step(&mut self) {
        let opcode = self.get_operand();
        self.cycles += crate::cycles::OP_CYCLES[opcode as usize] as u64;
        crate::opcodes::execute(self, opcode);
    }

    pub fn push_stack(&mut self, data: u8) {
        self.bus.write(0x0100 + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.bus.read(0x0100 + self.stack_pointer as u16)
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0xFD; // Traditional starting point for the stack

        self.status = FLAG_INTERRUPT | FLAG_UNUSED;

        // Read where the program starts from these two memory locations
        let lo = self.bus.read(0xFFFC) as u16;
        let hi = self.bus.read(0xFFFD) as u16;

        self.program_counter = (hi << 8) | lo;
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..program.len() {
            self.bus.write(0x8000 + i as u16, program[i]);
        }
        self.bus.write(0xFFFC, 0x00);
        self.bus.write(0xFFFD, 0x80);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn hardware_interrupt(&mut self, vector_addr: u16) {
        self.push_stack((self.program_counter >> 8) as u8);
        self.push_stack((self.program_counter & 0xFF) as u8);

        let mut status = self.status;
        status |= FLAG_UNUSED;
        status &= !FLAG_BREAK;
        self.push_stack(status);

        self.status |= FLAG_INTERRUPT;

        let lo = self.bus.read(vector_addr) as u16;
        let hi = self.bus.read(vector_addr + 1) as u16;
        self.program_counter = (hi << 8) | lo;
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        self.status & flag != 0
    }

    pub fn set_flag(&mut self, flag: u8, condition: bool) {
        if condition {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }

    pub fn nmi(&mut self) {
        self.hardware_interrupt(0xFFFA);
    }

    pub fn irq(&mut self) {
        if self.status & FLAG_INTERRUPT == 0 {
            self.hardware_interrupt(0xFFFE);
        }
    }

    pub fn run(&mut self) {
        self.reset();
        loop {
            let previous_pc = self.program_counter;
            let _opcode = self.bus.read(self.program_counter);
            self.step();

            if self.program_counter == previous_pc || self.program_counter == 0x0000 {
                return;
            }
        }
    }
}
