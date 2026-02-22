use crate::cpu::{CPU, FLAG_ZERO, FLAG_NEGATIVE};
use crate::addressing::{AddressingMode, get_operand_address};

pub fn execute(cpu: &mut CPU, opcode: u8) {
    match opcode {
        // LDA (Load A)
        0xA9 => { // Immediate
            let addr = get_operand_address(cpu, &AddressingMode::Immediate);
            cpu.register_a = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }
        0xA5 => { // Zero Page
            let addr = get_operand_address(cpu, &AddressingMode::ZeroPage);
            cpu.register_a = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }
        0xAD => { // Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.register_a = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }

        // LDX (Load X)
        0xA2 => { // Immediate
            let addr = get_operand_address(cpu, &AddressingMode::Immediate);
            cpu.register_x = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }
        0xA6 => { // Zero Page
            let addr = get_operand_address(cpu, &AddressingMode::ZeroPage);
            cpu.register_x = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }
        0xAE => { // Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.register_x = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }

        // LDY (Load Y)
        0xA0 => { // Immediate
            let addr = get_operand_address(cpu, &AddressingMode::Immediate);
            cpu.register_y = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }
        0xA4 => { // Zero Page
            let addr = get_operand_address(cpu, &AddressingMode::ZeroPage);
            cpu.register_y = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }
        0xAC => { // Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.register_y = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }

        // STA (Store A)
        0x85 => { // Zero Page
            let addr = get_operand_address(cpu, &AddressingMode::ZeroPage);
            cpu.bus.write(addr, cpu.register_a);
        }
        0x8D => { // Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.bus.write(addr, cpu.register_a);
        }

        // STX (Store X)
        0x86 => { // Zero Page
            let addr = get_operand_address(cpu, &AddressingMode::ZeroPage);
            cpu.bus.write(addr, cpu.register_x);
        }
        0x8E => { // Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.bus.write(addr, cpu.register_x);
        }

        // STY (Store Y)
        0x84 => { // Zero Page
            let addr = get_operand_address(cpu, &AddressingMode::ZeroPage);
            cpu.bus.write(addr, cpu.register_y);
        }
        0x8C => { // Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.bus.write(addr, cpu.register_y);
        }

        // TAX (Transfer A to X)
        0xAA => {
            cpu.register_x = cpu.register_a;
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }

        // TAY (Transfer A to Y)
        0xA8 => {
            cpu.register_y = cpu.register_a;
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }

        // TXA (Transfer X to A)
        0x8A => {
            cpu.register_a = cpu.register_x;
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }

        // TYA (Transfer Y to A)
        0x98 => {
            cpu.register_a = cpu.register_y;
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }

        _ => { }
    }
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