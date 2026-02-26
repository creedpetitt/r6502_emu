mod cpu;
mod bus;
mod opcodes;
mod addressing;
mod trace;

use std::env;
use std::fs;
use crate::cpu::CPU;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // CLI
        let is_trace = args.contains(&String::from("--trace"));
        let filename = args.iter().find(|arg| !arg.starts_with("--") && *arg != &args[0]).unwrap_or(&args[1]);

        println!("Loading binary file: {}", filename);
        
        let rom = match fs::read(filename) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", filename, e);
                return;
            }
        };

        let mut cpu = CPU::new();

        for (i, byte) in rom.iter().enumerate() {
            cpu.bus.write(i as u16, *byte);
        }

        // Klaus test starts execution at $0400
        cpu.program_counter = 0x0400;
        
        println!("Starting execution at ${:04X}...", cpu.program_counter);

        // We can't use cpu.run() directly here
        // so we write a custom run loop that tracks the PC.
        loop {
            if is_trace {
                println!("{}", trace::trace(&mut cpu));
            }

            let previous_pc = cpu.program_counter;
            cpu.step();
            
            if cpu.program_counter == previous_pc {
                println!("\nExecution trapped in infinite loop at ${:04X}.", cpu.program_counter);
                println!("(Check the test documentation to see if this address means PASS or FAIL).");
                break;
            }
        }

    } else {
        run_internal_tests();
    }
}

fn run_internal_tests() {
    println!("R6502 Emulator");

    let mut cpu = CPU::new();

    // TEST 1
    let program1 = vec![
        0xA9, 0x01,       // LDA #$01
        0x85, 0x10,       // STA $10
        0xE6, 0x10,       // INC $10
        0xC6, 0x10,       // DEC $10
        0xA9, 0x00,       // LDA #$00
        0x24, 0x10,       // BIT $10
        0x4C, 0x0C, 0x80  // JMP $800C (Infinite Loop at itself to stop emulator)
    ];

    cpu.load(program1);
    cpu.reset();

    println!("Test 1 (INC/DEC/BIT):");
    println!("--- TRACE LOG ---");
    trace_loop(&mut cpu);
    println!("-----------------");

    let mem_val = cpu.bus.read(0x10);
    let z_flag = (cpu.status & 0b0000_0010) > 0;
    
    println!("  Final Mem[0x10]: {} (Expected 1)", mem_val);
    println!("  Zero Flag (After BIT): {} (Expected true)", z_flag);

    if mem_val == 1 && z_flag {
        println!("  -> PASS\n");
    } else {
        println!("  -> FAIL\n");
    }

    // TEST 2
    cpu = CPU::new();
    let program2 = vec![
        0xA9, 0x10,       // LDA #$10
        0xC9, 0x05,       // CMP #$05
        0xC9, 0x20,       // CMP #$20
        0x4C, 0x06, 0x80  // JMP $8006
    ];

    cpu.load(program2);
    cpu.reset();
    
    println!("Test 2 (CMP):");
    println!("--- TRACE LOG ---");
    trace_loop(&mut cpu);
    println!("-----------------");
    
    // Test the final state
    let c_flag_2 = (cpu.status & 0b0000_0001) > 0;
    let n_flag_2 = (cpu.status & 0b1000_0000) > 0;

    println!("  CMP #$20 (16 vs 32) -> Carry: {} (Expected false)", c_flag_2);
    println!("  CMP #$20 (16 vs 32) -> Negative: {} (Expected true)", n_flag_2);

    if !c_flag_2 && n_flag_2 {
        println!("  -> PASS\n");
    } else {
        println!("  -> FAIL\n");
    }

    // TEST 3
    cpu = CPU::new();
    let program3 = vec![
        0xA2, 0x03,       // LDX #$03
        0xCA,             // DEX
        0xD0, 0xFD,       // BNE -3 (0xFD)
        0x4C, 0x05, 0x80  // JMP $8005
    ];

    cpu.load(program3);
    cpu.reset();

    println!("Test 3 (BNE Loop):");
    println!("--- TRACE LOG ---");
    trace_loop(&mut cpu);
    println!("-----------------");
    println!("  Final X Register: {} (Expected 0)", cpu.register_x);

    if cpu.register_x == 0 {
        println!("  -> PASS\n");
    } else {
        println!("  -> FAIL\n");
    }

    // TEST 4
    cpu = CPU::new();
    let program4 = vec![
        0xA9, 0x81,       // LDA #$81
        0x0A,             // ASL A
        0x6A,             // ROR A
        0x4A,             // LSR A
        0x4C, 0x05, 0x80  // JMP $8005
    ];

    cpu.load(program4);
    cpu.reset();
    
    println!("Test 4 (Shifts/Rotates):");
    println!("--- TRACE LOG ---");
    trace_loop(&mut cpu);
    println!("-----------------");

    let a_after_lsr = cpu.register_a;
    let c_after_lsr = (cpu.status & 0b0000_0001) > 0;

    println!("  Final LSR #$81 -> A: {:02X}, C: {} (Expected A: 40, C: true)", a_after_lsr, c_after_lsr);

    if a_after_lsr == 0x40 && c_after_lsr {
        println!("  -> PASS\n");
    } else {
        println!("  -> FAIL\n");
    }

    // TEST 5
    cpu = CPU::new();
    let program5 = vec![
        0xA9, 0x05,       // LDA #$05
        0x69, 0x0A,       // ADC #$0A  (Binary)
        0xAA,             // TAX       (Save binary result to X)
        0xF8,             // SED       (Set Decimal Flag)
        0xA9, 0x05,       // LDA #$05
        0x18,             // CLC
        0x69, 0x10,       // ADC #$10  (Decimal 5 + 10 = 15)
        0x4C, 0x0B, 0x80  // JMP $800B
    ];

    cpu.load(program5);
    cpu.reset();
    
    println!("Test 5 (Arithmetic & BCD):");
    println!("--- TRACE LOG ---");
    trace_loop(&mut cpu);
    println!("-----------------");

    println!("  Binary 5 + A: {:02X} (Expected 0F)", cpu.register_x);
    println!("  Decimal 5 + 10: {:02X} (Expected 15)", cpu.register_a);

    if cpu.register_x == 0x0F && cpu.register_a == 0x15 {
        println!("  -> PASS\n");
    } else {
        println!("  -> FAIL\n");
    }
}

fn trace_loop(cpu: &mut CPU) {
    loop {
        println!("{}", trace::trace(cpu));
        let previous_pc = cpu.program_counter;
        cpu.step();
        if cpu.program_counter == previous_pc || cpu.program_counter == 0x0000 {
            break;
        }
    }
}
