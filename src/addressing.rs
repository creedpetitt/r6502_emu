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

pub fn get_operand_address(cpu: &mut CPU, mode: &AddressingMode) -> (u16, bool) {
    match mode {
        AddressingMode::Absolute => (cpu.fetch_u16(), false),

        AddressingMode::Relative => {
            let offset = cpu.get_operand() as i8;
            let pc = cpu.program_counter;
            let addr = pc.wrapping_add_signed(offset as i16);
            (addr, (pc & 0xFF00) != (addr & 0xFF00))
        }
        AddressingMode::Indirect => {
            let ptr = cpu.fetch_u16();

            let lo = cpu.bus.read(ptr) as u16;
            let hi = if ptr & 0x00FF == 0x00FF {
                cpu.bus.read(ptr & 0xFF00) as u16
            } else {
                cpu.bus.read(ptr + 1) as u16
            };
            ((hi << 8) | lo, false)
        }
        AddressingMode::IndirectX => {
            let base = cpu.get_operand();
            let ptr = base.wrapping_add(cpu.register_x);
            let lo = cpu.bus.read(ptr as u16) as u16;
            let hi = cpu.bus.read(ptr.wrapping_add(1) as u16) as u16;
            ((hi << 8) | lo, false)
        }
        AddressingMode::IndirectY => {
            let base = cpu.get_operand();
            let lo = cpu.bus.read(base as u16) as u16;
            let hi = cpu.bus.read(base.wrapping_add(1) as u16) as u16;
            let deref_base = (hi << 8) | lo;
            let addr = deref_base.wrapping_add(cpu.register_y as u16);
            (addr, (deref_base & 0xFF00) != (addr & 0xFF00))
        }
        AddressingMode::Immediate => {
            let addr = cpu.program_counter;
            cpu.program_counter += 1;
            (addr, false)
        }
        AddressingMode::AbsoluteX => {
            let base = cpu.fetch_u16();
            let addr = base.wrapping_add(cpu.register_x as u16);
            (addr, (base & 0xFF00) != (addr & 0xFF00))
        }
        AddressingMode::AbsoluteY => {
            let base = cpu.fetch_u16();
            let addr = base.wrapping_add(cpu.register_y as u16);
            (addr, (base & 0xFF00) != (addr & 0xFF00))
        }
        AddressingMode::ZeroPage => {
            (cpu.get_operand() as u16, false)
        }
        AddressingMode::ZeroPageX => {
            let pos = cpu.get_operand();
            let addr = pos.wrapping_add(cpu.register_x) as u16;
            (addr, false)
        }
        AddressingMode::ZeroPageY => {
            let pos = cpu.get_operand();
            let addr = pos.wrapping_add(cpu.register_y) as u16;
            (addr, false)
        }
    }
}
