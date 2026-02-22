use crate::cpu::{CPU, FLAG_ZERO, FLAG_NEGATIVE, FLAG_CARRY, FLAG_DECIMAL, FLAG_INTERRUPT, FLAG_OVERFLOW, FLAG_UNUSED, FLAG_BREAK};
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

        0xAA => { // TAX
            cpu.register_x = cpu.register_a;
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }
        0xA8 => { // TAY
            cpu.register_y = cpu.register_a;
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }
        0x8A => { // TXA
            cpu.register_a = cpu.register_x;
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }
        0x98 => { // TYA
            cpu.register_a = cpu.register_y;
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }

        0xE8 => { // INX
            cpu.register_x = cpu.register_x.wrapping_add(1);
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }
        0xC8 => { // INY
            cpu.register_y = cpu.register_y.wrapping_add(1);
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }
        0xCA => { // DEX
            cpu.register_x = cpu.register_x.wrapping_sub(1);
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }
        0x88 => { // DEY
            cpu.register_y = cpu.register_y.wrapping_sub(1);
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }
        0x48 => {
            cpu.push_stack(cpu.register_a)
        }
        0x08 => {
            let mut flags = cpu.status;
            flags |= FLAG_UNUSED;
            flags |= FLAG_BREAK;
            cpu.push_stack(flags)
        }
        0x68 => {
            cpu.register_a = cpu.pop_stack();
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }
        0x28 => {
            cpu.status = cpu.pop_stack();
            cpu.status &= !FLAG_BREAK;
            cpu.status |= FLAG_UNUSED;
        }
        0x38 => {
            cpu.status |= FLAG_CARRY
        }
        0xF8 => {
            cpu.status |= FLAG_DECIMAL
        }
        0x78 => {
            cpu.status |= FLAG_INTERRUPT
        }
        0x18 => {
            cpu.status &= !FLAG_CARRY
        }
        0xD8 => {
            cpu.status &= !FLAG_DECIMAL
        }
        0x58 => {
            cpu.status &= !FLAG_INTERRUPT
        }
        0xB8 => {
            cpu.status &= !FLAG_OVERFLOW
        }
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