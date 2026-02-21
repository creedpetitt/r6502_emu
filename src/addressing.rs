use crate::cpu::CPU;

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    Absolute,
    // Other 10 modes
}

pub fn get_operand_address(cpu: &mut CPU, mode: &AddressingMode) -> u16 {
    match mode {
        AddressingMode::Immediate => {
            let addr = cpu.program_counter;
            cpu.program_counter += 1;
            addr
        }
        AddressingMode::ZeroPage => {
            cpu.fetch() as u16
        }
        AddressingMode::Absolute => {
            let lo = cpu.fetch() as u16;
            let hi = cpu.fetch() as u16;
            (hi << 8) | lo
        }
    }
}