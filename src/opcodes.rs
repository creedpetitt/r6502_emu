use crate::cpu::{CPU, FLAG_ZERO, FLAG_NEGATIVE, FLAG_CARRY, FLAG_DECIMAL,
                 FLAG_INTERRUPT, FLAG_OVERFLOW, FLAG_UNUSED, FLAG_BREAK};
use crate::addressing::{AddressingMode, get_operand_address};

pub fn execute(cpu: &mut CPU, opcode: u8) {
    match opcode {
        // LDA
        0xA9 => cpu.register_a = load(cpu, &AddressingMode::Immediate),
        0xA5 => cpu.register_a = load(cpu, &AddressingMode::ZeroPage),
        0xAD => cpu.register_a = load(cpu, &AddressingMode::Absolute),
        0xB5 => cpu.register_a = load(cpu, &AddressingMode::ZeroPageX),
        0xBD => cpu.register_a = load(cpu, &AddressingMode::AbsoluteX),
        0xB9 => cpu.register_a = load(cpu, &AddressingMode::AbsoluteY),

        // LDX
        0xA2 => cpu.register_x = load(cpu, &AddressingMode::Immediate),
        0xA6 => cpu.register_x = load(cpu, &AddressingMode::ZeroPage),
        0xAE => cpu.register_x = load(cpu, &AddressingMode::Absolute),
        0xB6 => cpu.register_x = load(cpu, &AddressingMode::ZeroPageY),
        0xBE => cpu.register_x = load(cpu, &AddressingMode::AbsoluteY),

        // LDY
        0xA0 => cpu.register_y = load(cpu, &AddressingMode::Immediate),
        0xA4 => cpu.register_y = load(cpu, &AddressingMode::ZeroPage),
        0xAC => cpu.register_y = load(cpu, &AddressingMode::Absolute),
        0xB4 => cpu.register_y = load(cpu, &AddressingMode::ZeroPageX),
        0xBC => cpu.register_y = load(cpu, &AddressingMode::AbsoluteX),

        // STA
        0x85 => store(cpu, &AddressingMode::ZeroPage, cpu.register_a),
        0x8D => store(cpu, &AddressingMode::Absolute, cpu.register_a),
        0x95 => store(cpu, &AddressingMode::ZeroPageX, cpu.register_a),
        0x9D => store(cpu, &AddressingMode::AbsoluteX, cpu.register_a),
        0x99 => store(cpu, &AddressingMode::AbsoluteY, cpu.register_a),

        // STX
        0x86 => store(cpu, &AddressingMode::ZeroPage, cpu.register_x),
        0x8E => store(cpu, &AddressingMode::Absolute, cpu.register_x),
        0x96 => store(cpu, &AddressingMode::ZeroPageY, cpu.register_x),

        // STY
        0x84 => store(cpu, &AddressingMode::ZeroPage, cpu.register_y),
        0x8C => store(cpu, &AddressingMode::Absolute, cpu.register_y),
        0x94 => store(cpu, &AddressingMode::ZeroPageX, cpu.register_y),

        // Subroutines
        0x20 => jsr(cpu),
        0x60 => rts(cpu),
        0x40 => rti(cpu),
        // AND
        0x29 => and(cpu, &AddressingMode::Immediate),
        0x25 => and(cpu, &AddressingMode::ZeroPage),
        0x35 => and(cpu, &AddressingMode::ZeroPageX),
        0x2D => and(cpu, &AddressingMode::Absolute),
        0x3D => and(cpu, &AddressingMode::AbsoluteX),
        0x39 => and(cpu, &AddressingMode::AbsoluteY),

        // ORA
        0x09 => ora(cpu, &AddressingMode::Immediate),
        0x05 => ora(cpu, &AddressingMode::ZeroPage),
        0x15 => ora(cpu, &AddressingMode::ZeroPageX),
        0x0D => ora(cpu, &AddressingMode::Absolute),
        0x1D => ora(cpu, &AddressingMode::AbsoluteX),
        0x19 => ora(cpu, &AddressingMode::AbsoluteY),

        // EOR
        0x49 => eor(cpu, &AddressingMode::Immediate),
        0x45 => eor(cpu, &AddressingMode::ZeroPage),
        0x55 => eor(cpu, &AddressingMode::ZeroPageX),
        0x4D => eor(cpu, &AddressingMode::Absolute),
        0x5D => eor(cpu, &AddressingMode::AbsoluteX),
        0x59 => eor(cpu, &AddressingMode::AbsoluteY),

        // ASL
        0x0A => cpu.register_a = shift_left(cpu, cpu.register_a),
        0x06 => asl(cpu, &AddressingMode::ZeroPage),
        0x16 => asl(cpu, &AddressingMode::ZeroPageX),
        0x0E => asl(cpu, &AddressingMode::Absolute),
        0x1E => asl(cpu, &AddressingMode::AbsoluteX),

        // LSR
        0x4A => cpu.register_a = shift_right(cpu, cpu.register_a),
        0x46 => lsr(cpu, &AddressingMode::ZeroPage),
        0x56 => lsr(cpu, &AddressingMode::ZeroPageX),
        0x4E => lsr(cpu, &AddressingMode::Absolute),
        0x5E => lsr(cpu, &AddressingMode::AbsoluteX),

        // ROL
        0x2A => cpu.register_a = rotate_left(cpu, cpu.register_a),
        0x26 => rol(cpu, &AddressingMode::ZeroPage),
        0x36 => rol(cpu, &AddressingMode::ZeroPageX),
        0x2E => rol(cpu, &AddressingMode::Absolute),
        0x3E => rol(cpu, &AddressingMode::AbsoluteX),

        // ROR
        0x6A => cpu.register_a = rotate_right(cpu, cpu.register_a),
        0x66 => ror(cpu, &AddressingMode::ZeroPage),
        0x76 => ror(cpu, &AddressingMode::ZeroPageX),
        0x6E => ror(cpu, &AddressingMode::Absolute),
        0x7E => ror(cpu, &AddressingMode::AbsoluteX),

        0xC9 => compare(cpu, &AddressingMode::Immediate, cpu.register_a),
        0xC5 => compare(cpu, &AddressingMode::ZeroPage, cpu.register_a),
        0xD5 => compare(cpu, &AddressingMode::ZeroPageX, cpu.register_a),
        0xCD => compare(cpu, &AddressingMode::Absolute, cpu.register_a),
        0xDD => compare(cpu, &AddressingMode::AbsoluteX, cpu.register_a),
        0xD9 => compare(cpu, &AddressingMode::AbsoluteY, cpu.register_a),

        // CPX
        0xE0 => compare(cpu, &AddressingMode::Immediate, cpu.register_x),
        0xE4 => compare(cpu, &AddressingMode::ZeroPage, cpu.register_x),
        0xEC => compare(cpu, &AddressingMode::Absolute, cpu.register_x),

        // CPY
        0xC0 => compare(cpu, &AddressingMode::Immediate, cpu.register_y),
        0xC4 => compare(cpu, &AddressingMode::ZeroPage, cpu.register_y),
        0xCC => compare(cpu, &AddressingMode::Absolute, cpu.register_y),

        // DEC
        0xC6 => dec(cpu, &AddressingMode::ZeroPage),
        0xD6 => dec(cpu, &AddressingMode::ZeroPageX),
        0xCE => dec(cpu, &AddressingMode::Absolute),
        0xDE => dec(cpu, &AddressingMode::AbsoluteX),

        // INC
        0xE6 => inc(cpu, &AddressingMode::ZeroPage),
        0xF6 => inc(cpu, &AddressingMode::ZeroPageX),
        0xEE => inc(cpu, &AddressingMode::Absolute),
        0xFE => inc(cpu, &AddressingMode::AbsoluteX),

        // BIT
        0x24 => bit(cpu, &AddressingMode::ZeroPage),
        0x2C => bit(cpu, &AddressingMode::Absolute),

        // BNE
        0xD0 => branch(cpu, cpu.status & FLAG_ZERO == 0),
        // BEQ
        0xF0 => branch(cpu, cpu.status & FLAG_ZERO > 0),
        // BCC
        0x90 => branch(cpu, cpu.status & FLAG_CARRY == 0),
        // BCS
        0xB0 => branch(cpu, cpu.status & FLAG_CARRY > 0),
        // BPL
        0x10 => branch(cpu, cpu.status & FLAG_NEGATIVE == 0),
        // BMI
        0x30 => branch(cpu, cpu.status & FLAG_NEGATIVE > 0),
        // BVC
        0x50 => branch(cpu, cpu.status & FLAG_OVERFLOW == 0),
        // BVS
        0x70 => branch(cpu, cpu.status & FLAG_OVERFLOW > 0),

        // JMP
        0x4C => cpu.program_counter = get_operand_address(cpu, &AddressingMode::Absolute),
        0x6C => cpu.program_counter = get_operand_address(cpu, &AddressingMode::Indirect),

        0xAA => tax(cpu),
        0xA8 => tay(cpu),
        0x8A => txa(cpu),
        0x98 => tya(cpu),
        0x9A => txs(cpu),
        0xBA => tsx(cpu),

        0xE8 => inx(cpu),
        0xC8 => iny(cpu),
        0xCA => dex(cpu),
        0x88 => dey(cpu),

        0x48 => pha(cpu),
        0x08 => php(cpu),
        0x68 => pla(cpu),
        0x28 => plp(cpu),

        0x38 => sec(cpu),
        0xF8 => sed(cpu),
        0x78 => sei(cpu),
        0x18 => clc(cpu),
        0xD8 => cld(cpu),
        0x58 => cli(cpu),
        0xB8 => clv(cpu),

        // NOP
        0xEA => { /* Do nothing */ }
        _ => { }
    }
}

fn load(cpu: &mut CPU, mode: &AddressingMode) -> u8 {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    update_zero_and_negative_flags(cpu, value);
    value
}

fn store(cpu: &mut CPU, mode: &AddressingMode, value: u8) {
    let addr = get_operand_address(cpu, mode);
    cpu.bus.write(addr, value);
}

fn compare(cpu: &mut CPU, mode: &AddressingMode, compare_with: u8) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);

    if compare_with >= value {
        cpu.status |= FLAG_CARRY;
    } else {
        cpu.status &= !FLAG_CARRY;
    }
    let result = compare_with.wrapping_sub(value);
    update_zero_and_negative_flags(cpu, result);
}

fn branch(cpu: &mut CPU, condition: bool) {
    let jump_address = get_operand_address(cpu, &AddressingMode::Relative);

    if condition {
        cpu.program_counter = jump_address;
    }
}

fn inc(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let mut value = cpu.bus.read(addr);
    value = value.wrapping_add(1);
    cpu.bus.write(addr, value);
    update_zero_and_negative_flags(cpu, value);
}

fn dec(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let mut value = cpu.bus.read(addr);
    value = value.wrapping_sub(1);
    cpu.bus.write(addr, value);
    update_zero_and_negative_flags(cpu, value);
}

fn and(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    cpu.register_a &= value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn ora(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    cpu.register_a |= value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn eor(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    cpu.register_a ^= value;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn tax(cpu: &mut CPU) {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn tay(cpu: &mut CPU) {
    cpu.register_y = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

fn txa(cpu: &mut CPU) {
    cpu.register_a = cpu.register_x;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn tya(cpu: &mut CPU) {
    cpu.register_a = cpu.register_y;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn txs(cpu: &mut CPU) {
    cpu.stack_pointer = cpu.register_x;
}

fn tsx(cpu: &mut CPU) {
    cpu.register_x = cpu.stack_pointer;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn inx(cpu: &mut CPU) {
    cpu.register_x = cpu.register_x.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn iny(cpu: &mut CPU) {
    cpu.register_y = cpu.register_y.wrapping_add(1);
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

fn dex(cpu: &mut CPU) {
    cpu.register_x = cpu.register_x.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn dey(cpu: &mut CPU) {
    cpu.register_y = cpu.register_y.wrapping_sub(1);
    update_zero_and_negative_flags(cpu, cpu.register_y);
}

fn pha(cpu: &mut CPU) {
    cpu.push_stack(cpu.register_a);
}

fn php(cpu: &mut CPU) {
    let mut flags = cpu.status;
    flags |= FLAG_UNUSED;
    flags |= FLAG_BREAK;
    cpu.push_stack(flags);
}

fn pla(cpu: &mut CPU) {
    cpu.register_a = cpu.pop_stack();
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn plp(cpu: &mut CPU) {
    cpu.status = cpu.pop_stack();
    cpu.status &= !FLAG_BREAK;
    cpu.status |= FLAG_UNUSED;
}

fn bit(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);

    if (cpu.register_a & value) == 0 {
        cpu.status |= FLAG_ZERO;
    } else {
        cpu.status &= !FLAG_ZERO;
    }

    if value & FLAG_NEGATIVE > 0 {
        cpu.status |= FLAG_NEGATIVE;
    } else {
        cpu.status &= !FLAG_NEGATIVE;
    }

    if value & FLAG_OVERFLOW > 0 {
        cpu.status |= FLAG_OVERFLOW;
    } else {
        cpu.status &= !FLAG_OVERFLOW;
    }
}

fn asl(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    let result = shift_left(cpu, value);
    cpu.bus.write(addr, result);
}

fn lsr(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    let result = shift_right(cpu, value);
    cpu.bus.write(addr, result);
}

fn rol(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    let result = rotate_left(cpu, value);
    cpu.bus.write(addr, result);
}

fn ror(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = get_operand_address(cpu, mode);
    let value = cpu.bus.read(addr);
    let result = rotate_right(cpu, value);
    cpu.bus.write(addr, result);
}

fn jsr(cpu: &mut CPU) {
    let target_addr = cpu.fetch_u16();
    let return_addr = cpu.program_counter - 1;
    cpu.push_stack(((return_addr >> 8) & 0xFF) as u8);
    cpu.push_stack((return_addr & 0xFF) as u8);
    cpu.program_counter = target_addr;
}

fn rts(cpu: &mut CPU) {
    let lo = cpu.pop_stack() as u16;
    let hi = cpu.pop_stack() as u16;
    cpu.program_counter = (hi << 8 | lo) + 1;
}

fn rti(cpu: &mut CPU) {
    cpu.status = cpu.pop_stack();
    cpu.status &= !FLAG_BREAK;
    cpu.status |= FLAG_UNUSED;

    let lo = cpu.pop_stack() as u16;
    let hi = cpu.pop_stack() as u16;
    cpu.program_counter = (hi << 8) | lo;
}

fn shift_left(cpu: &mut CPU, value: u8) -> u8 {
    if value & 0x80 != 0 {
        cpu.status |= FLAG_CARRY;
    } else {
        cpu.status &= !FLAG_CARRY;
    }
    let result = value << 1;
    update_zero_and_negative_flags(cpu, result);
    result
}

fn shift_right(cpu: &mut CPU, value: u8) -> u8 {
    if value & 0x01 != 0 {
        cpu.status |= FLAG_CARRY;
    } else {
        cpu.status &= !FLAG_CARRY;
    }
    let result = value >> 1;
    update_zero_and_negative_flags(cpu, result);
    result
}

fn rotate_left(cpu: &mut CPU, value: u8) -> u8 {
    let old_carry = cpu.status & FLAG_CARRY != 0;

    if value & 0x80 != 0 {
        cpu.status |= FLAG_CARRY;
    } else {
        cpu.status &= !FLAG_CARRY;
    }
    let mut result = value << 1;

    if old_carry {
        result |= 0x01;
    }
    update_zero_and_negative_flags(cpu, result);
    result
}

fn rotate_right(cpu: &mut CPU, value: u8) -> u8 {
    let old_carry = cpu.status & FLAG_CARRY != 0;

    if value & 0x01 != 0 {
        cpu.status |= FLAG_CARRY;
    } else {
        cpu.status &= !FLAG_CARRY;
    }
    let mut result = value >> 1;

    if old_carry {
        result |= 0x80;
    }
    update_zero_and_negative_flags(cpu, result);
    result
}

fn update_zero_and_negative_flags(cpu: &mut CPU, result: u8) {
    if result == 0 {
        cpu.status |= FLAG_ZERO;
    } else {
        cpu.status &= !FLAG_ZERO;
    }

    if result & 0b1000_0000 != 0 {
        cpu.status |= FLAG_NEGATIVE;
    } else {
        cpu.status &= !FLAG_NEGATIVE;
    }
}

fn sec(cpu: &mut CPU) {
    cpu.status |= FLAG_CARRY;
}

fn sed(cpu: &mut CPU) {
    cpu.status |= FLAG_DECIMAL;
}

fn sei(cpu: &mut CPU) {
    cpu.status |= FLAG_INTERRUPT;
}

fn clc(cpu: &mut CPU) {
    cpu.status &= !FLAG_CARRY;
}

fn cld(cpu: &mut CPU) {
    cpu.status &= !FLAG_DECIMAL;
}

fn cli(cpu: &mut CPU) {
    cpu.status &= !FLAG_INTERRUPT;
}

fn clv(cpu: &mut CPU) {
    cpu.status &= !FLAG_OVERFLOW;
}