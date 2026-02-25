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
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
    // Implied - handled inline
    // Accumulator - handled inline
}

pub fn get_operand_address(cpu: &mut CPU, mode: &AddressingMode) -> u16 {
    match mode {
        AddressingMode::Absolute => cpu.fetch_u16(),

        AddressingMode::Relative => {
            let offset = cpu.get_operand() as i8;
            let pc = cpu.program_counter;
            pc.wrapping_add_signed(offset as i16)
        }
        AddressingMode::Indirect => {
            let ptr = cpu.fetch_u16();

            let lo = cpu.bus.read(ptr) as u16;
            let hi = if ptr & 0x00FF == 0x00FF {
                cpu.bus.read(ptr & 0xFF00) as u16
            } else {
                cpu.bus.read(ptr + 1) as u16
            };
            (hi << 8) | lo
        }
        AddressingMode::IndirectX => {
            let base = cpu.get_operand();
            let ptr = base.wrapping_add(cpu.register_x);
            let lo = cpu.bus.read(ptr as u16) as u16;
            let hi = cpu.bus.read(ptr.wrapping_add(1) as u16) as u16;
            (hi << 8) | lo
        }
        AddressingMode::IndirectY => {
            let base = cpu.get_operand();
            let lo = cpu.bus.read(base as u16) as u16;
            let hi = cpu.bus.read(base.wrapping_add(1) as u16) as u16;
            let deref_base = (hi << 8) | lo;
            deref_base.wrapping_add(cpu.register_y as u16)
        }
        AddressingMode::Immediate => {
            let addr = cpu.program_counter;
            cpu.program_counter += 1;
            addr
        }
        AddressingMode::AbsoluteX => {
            let base = cpu.fetch_u16();
            base.wrapping_add(cpu.register_x as u16)
        }
        AddressingMode::AbsoluteY => {
            let base = cpu.fetch_u16();
            base.wrapping_add(cpu.register_y as u16)
        }
        AddressingMode::ZeroPage => {
            cpu.get_operand() as u16
        }
        AddressingMode::ZeroPageX => {
            let pos = cpu.get_operand();
            let addr = pos.wrapping_add(cpu.register_x) as u16;
            addr
        }
        AddressingMode::ZeroPageY => {
            let pos = cpu.get_operand();
            let addr = pos.wrapping_add(cpu.register_y) as u16;
            addr
        }
    }
}
