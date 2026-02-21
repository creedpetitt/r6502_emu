use crate::cpu::{CPU, FLAG_ZERO, FLAG_NEGATIVE};

pub fn execute(cpu: &mut CPU, opcode: u8) {
    match opcode {
        0xA9 => {
            let value = cpu.fetch();
            cpu.register_a = value;
            update_zero_and_negative_flags(cpu, cpu.register_a);
        }
        0x8D => {
            let lo = cpu.fetch() as u16;
            let hi = cpu.fetch() as u16;
            let address = (hi << 8) | lo;
            cpu.bus.write(address, cpu.register_a);
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