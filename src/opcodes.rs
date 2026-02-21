use crate::cpu::{CPU, FLAG_ZERO, FLAG_NEGATIVE};
use crate::addressing::{AddressingMode, get_operand_address};

pub fn execute(cpu: &mut CPU, opcode: u8) {
    match opcode {
        // Load Accumulator
        0xA9 => { // LDA Immediate
            let addr = get_operand_address(cpu, &AddressingMode::Immediate);
            cpu.register_a = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }

        // Load X Register
        0xA2 => { // LDX Immediate
            let addr = get_operand_address(cpu, &AddressingMode::Immediate);
            cpu.register_x = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_x);
        }

        // Load Y Register
        0xA0 => { // LDY Immediate
            let addr = get_operand_address(cpu, &AddressingMode::Immediate);
            cpu.register_y = cpu.bus.read(addr);
            update_zero_and_negative_flags(cpu, cpu.register_y);
        }

        // Store Accumulator
        0x8D => { // STA Absolute
            let addr = get_operand_address(cpu, &AddressingMode::Absolute);
            cpu.bus.write(addr, cpu.register_a);
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