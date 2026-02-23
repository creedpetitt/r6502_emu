# R6502 Emulator (Rust)
A cycle-accurate MOS 6502 CPU emulator written in Rust.

## Project Status
**Implemented Opcodes:** ~90 / 151 
**Addressing Modes:** 11 / 13 

This emulator is currently capable of running simple assembly programs that utilize:
-   **Register Transfers:** (TAX, TAY, TXA, TYA)
-   **Load / Store:** (LDA, LDX, LDY, STA, STX, STY)
-   **Increments / Decrements:** (INX, INY, DEX, DEY, INC, DEC)
-   **Stack Operations:** (PHA, PLA, PHP, PLP, TXS*, TSX*)
-   **Logical Operations:** (AND, ORA, EOR, BIT)
-   **Comparisons:** (CMP, CPX, CPY)
-   **Flag Manipulation:** (SEC, CLC, SED, CLD, SEI, CLI, CLV)
-   **No Operation:** (NOP)


## Features
-   **Full Bus Architecture:** Clean separation of CPU and Memory.
-   **Cycle-Accurate Addressing:** Correct handling of Page Boundaries and Wrapping.
-   **Flag Management:** Accurate Status Register (NV-BDIZC) behavior.
-   **Stack Emulation:** Hardcoded Page 1 Stack (0x0100 - 0x01FF) with correct wrap-around.

## Remaining Work

### Branching
-   [ ] **Branching Instructions:** (BEQ, BNE, BCS, BCC, BMI, BPL, BVS, BVC)
-   [ ] **Jumps:** (JMP Absolute, JMP Indirect)

### Arithmetic 
-   [ ] **Addition:** (ADC - Add with Carry)
-   [ ] **Subtraction:** (SBC - Subtract with Carry)

### Bit Shifting
-   [ ] **Shifts:** (ASL, LSR)
-   [ ] **Rotates:** (ROL, ROR)

### Subroutines
-   [ ] **Function Calls:** (JSR - Jump to Subroutine)
-   [ ] **Returns:** (RTS - Return from Subroutine, RTI - Return from Interrupt)

