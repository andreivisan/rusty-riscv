RISC-V Emulator in Rust

## Disclaimer

This purpose of this implementation is an educational only. I used as guideline
the [rvemu book](https://book.rvemu.app/).

## RISC-V ISA

- Consists of modules:
    - A base integer ISA
    - Optional extensions.

- In the base integer ISA there are 2 variants:
    - RV32I for a 32-bit architecture.
    - RV64I for a 64-bit architecture.

- There are also General Purpose ISAs that allow complex systems to run on 
RISC-V hardware: RV32G and RV64G.
