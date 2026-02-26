# R6502 Emulator (Rust)

A cycle-accurate, functionally complete MOS 6502 CPU core written in Rust. Designed to be modular and easily embeddable into broader system emulators (such as an NES or Commodore 64).

## Key Features

*   **100% Instruction Coverage:** Fully implements all 151 official documented opcodes.
*   **Fully Verified:** Flawlessly passes the rigorous [Klaus Dormann 6502 Functional Test](https://github.com/Klaus2m5/6502_65C02_functional_tests) suite.
*   **Cycle-Accurate Timing:** Emulates authentic CPU clock cycles, dynamically calculating `+1` cycle penalties when memory indexing or branching crosses a 256-byte page boundary.
*   **Real-time Trace Logger:** Built-in instruction disassembler and register state logger for debugging.

### Authentic Hardware Bugs Emulated
This emulator follows the exact behavior of the original 1975 NMOS 6502 silicon, including its famous bugs:
*   **`JMP` Indirect Page Wrap Bug:** If an indirect jump vector falls exactly on a page boundary (e.g., `$02FF`), the CPU incorrectly fetches the high byte from the start of that same page (`$0200`) rather than the next page (`$0300`).
*   **Decimal Mode (BCD) Flag Illusion:** During BCD `ADC` and `SBC` operations, the hardware calculates the Zero (Z), Negative (N), and Overflow (V) flags based on the *underlying base-2 binary math* running in the background, rather than the actual base-10 decimal result.
*   **The Phantom B-Flag:** The CPU Status Register only possesses 6 physical wires. Bits 4 and 5 do not exist. When the status is pushed to the stack, the emulator correctly manipulates these bits to inform the OS whether the interrupt was triggered by Software (`BRK`/`PHP`) or Hardware (`IRQ`/`NMI`).

---

## Loading and Running Programs
The emulator was designed with a clean API, making it trivial to load programs and control execution state.

```rust
use cpu::CPU;

fn main() {
    // Create a new CPU
    let mut cpu = CPU::new();

    // Write your 6502 machine code
    let program = vec![
        0xA9, 0x05,       // LDA #$05
        0x69, 0x0A,       // ADC #$0A (A = 15)
        0x4C, 0x04, 0x80  // JMP $8004
    ];

    // Load it into memory and execute
    cpu.load_and_run(program);
    
    assert_eq!(cpu.register_a, 15);
    println!("Executed in {} clock cycles.", cpu.cycles);
}
```

## Trace Logger / Debugger
The emulator includes a built-in trace logger that disassembles machine code on the fly and outputs the exact hardware state of the CPU before every instruction. 

To run a binary file with the trace logger enabled, use the `--trace` flag:
```bash
cargo run --release -- --trace my_rom.bin
```

**Example Output:**
```text
8000  A9 05           LDA $05 A:00 X:00 Y:00 P:24 SP:FD
8002  69 0A           ADC $0A A:05 X:00 Y:00 P:24 SP:FD
8004  AA                  TAX A:0F X:00 Y:00 P:24 SP:FD
8005  F8                  SED A:0F X:0F Y:00 P:24 SP:FD
8006  A9 05           LDA $05 A:0F X:0F Y:00 P:2C SP:FD
8008  18                  CLC A:05 X:0F Y:00 P:2C SP:FD
8009  69 10           ADC $10 A:05 X:0F Y:00 P:2C SP:FD
800B  4C 0B 80      JMP $800B A:15 X:0F Y:00 P:6C SP:FD
```

## Running the Klaus Dormann Test
To prove the CPU's functional accuracy, you can run the official Klaus 6502 test suite. A helper script is provided to download and execute the 64KB `.bin` test file automatically.

```bash
# Make sure the script is executable
chmod +x run_klaus.sh

# Run the test
./run_klaus.sh
```

**Success Output:**
If the emulator halts at address `$3469`, the test suite has passed successfully.
```text
Loading binary file: 6502_functional_test.bin
Starting execution at $0400...

Execution trapped in infinite loop at $3469.
```

## Running Built-in Tests
Executing the project without arguments will run a suite of internal unit tests validating Branching, Bit Shifting, Flags, and BCD Arithmetic.

```bash
cargo run --quiet
```