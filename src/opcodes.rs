use crate::cpu::{CPU, FLAG_ZERO, FLAG_NEGATIVE};
use crate::addressing::{AddressingMode, get_operand_address};

pub fn execute(cpu: &mut CPU, opcode: u8) {
    match opcode {
        // LDA
        0xA9 => cpu.register_a = load(cpu, &AddressingMode::Immediate),
        0xA5 => cpu.register_a = load(cpu, &AddressingMode::ZeroPage),
        0xAD => cpu.register_a = load(cpu, &AddressingMode::Absolute),

        // LDX
        0xA2 => cpu.register_x = load(cpu, &AddressingMode::Immediate),
        0xA6 => cpu.register_x = load(cpu, &AddressingMode::ZeroPage),
        0xAE => cpu.register_x = load(cpu, &AddressingMode::Absolute),

        // LDY
        0xA0 => cpu.register_y = load(cpu, &AddressingMode::Immediate),
        0xA4 => cpu.register_y = load(cpu, &AddressingMode::ZeroPage),
        0xAC => cpu.register_y = load(cpu, &AddressingMode::Absolute),

        // STA
        0x85 => store(cpu, &AddressingMode::ZeroPage, cpu.register_a),
        0x8D => store(cpu, &AddressingMode::Absolute, cpu.register_a),

        // STX
        0x86 => store(cpu, &AddressingMode::ZeroPage, cpu.register_x),
        0x8E => store(cpu, &AddressingMode::Absolute, cpu.register_x),

        // STY
        0x84 => store(cpu, &AddressingMode::ZeroPage, cpu.register_y),
        0x8C => store(cpu, &AddressingMode::Absolute, cpu.register_y),

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