use crate::cpu::CPU;

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    Absolute,
    ZeroPageX,
    AbsoluteX,
    ZeroPageY,
    AbsoluteY,
    // Other 6 modes
}

pub fn get_operand_address(cpu: &mut CPU, mode: &AddressingMode) -> u16 {
    match mode {
        AddressingMode::Immediate => {
            let address = cpu.program_counter;
            cpu.program_counter += 1;
            address
        }
        AddressingMode::Absolute => cpu.fetch_u16(),

        AddressingMode::AbsoluteX => {
            let base = cpu.fetch_u16();
            base.wrapping_add(cpu.register_x as u16)
        }
        AddressingMode::AbsoluteY => {
            let base = cpu.fetch_u16();
            base.wrapping_add(cpu.register_y as u16)
        }
        AddressingMode::ZeroPage => {
            cpu.fetch() as u16
        }
        AddressingMode::ZeroPageX => {
            let pos = cpu.fetch();
            let address = pos.wrapping_add(cpu.register_x) as u16;
            address
        }
        AddressingMode::ZeroPageY => {
            let pos = cpu.fetch();
            let address = pos.wrapping_add(cpu.register_y) as u16;
            address
        }
    }
}
