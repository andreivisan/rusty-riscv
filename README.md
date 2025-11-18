# RISC-V Emulator in Rust

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

- Here is the list of ISAs that RV64G includes:
    - RV64I: base integer instructions
    - RV64M: integer multiplication and division instructions
    - RV64A: atomic instructions
    - RV64F: single-precision floating-point instructions
    - RV64D: double-precision floating-point instructions
    - RVZicsr: control and status register instructions
    - RVZifencei: instruction-fetch fence instructions

- As in the book the goal is to run xv6 so we will implement only the instructions
xv6 uses:
    - RV64I 
    - RVZicsr 
    - a part of RV64M 
    - a part of RV64A.

### Install RISC-V Toolchain on MacOS

```bash
brew install riscv64-elf-gcc riscv64-elf-binutils riscv64-elf-gdb
```

- if the command above is not working use the following two commands

```bash
brew tap riscv-software-src/riscv
brew install riscv-gnu-toolchain
```

- add the path to .zshrc or .bashrc

```bash
echo 'export PATH="$(brew --prefix riscv-gnu-toolchain)/bin:$PATH"' >> ~/.zshrc
source .zshrc
```

**Verify the installation**

```bash
riscv64-unknown-elf-gcc --version
riscv64-unknown-elf-objdump --version
```

## Registers

- There are 32 general-purpose registers that are each 64-bit wide in RV64I.
- Each register has a role defined by the ***integer register convention***.

![integer register convention](./docs/images/risc-v-register-convention.png)

- The zero register (x0) is hardwired with all bits equal to 0.
- The sp register (x2) is a stack pointer. It keeps track of a stack.
- A stack is a data structure mainly located at the end of the address space.
It is especially used to store local variables.
- A value of a stack pointer is subtracted in a function prologue, so we need to set it up with a non-0 value.

> ### Stack pointer & function prologue (RISC-V)
>
> * **Stack pointer:**
>
>   * On RISC-V, **`sp` = `x2`**.
>   * It holds a **memory address** that marks the current **top of the stack**.
> * **What is the stack?**
>
>   * A region of memory used for **function calls**:
>
>     * space for **local variables**,
>     * space to **save registers** (like `ra`, `s0`, etc.).
>   * It is managed by **adjusting `sp`**.
>
> ---
>
> ### “Stack grows down” — what it means
>
> * Memory addresses increase like this:
>
>   * low → high: `0x0000_1000`, `0x0000_1008`, `0x0000_1010`, ...
> * **“Stack grows down”** means:
>
>   * When you allocate more stack, **`sp` is decreased** → it moves to a **smaller address**.
>   * So newer stack frames live at **lower addresses** than older ones.
> * Example:
>
>   * Suppose `sp = 0x0000_8000` (top of stack).
>   * Function needs 32 bytes:
>
>     ```asm
>     addi sp, sp, -32   # sp = 0x0000_8000 - 32 = 0x0000_7FE0
>     ```
>   * Now the function’s local stack frame lives roughly in `[0x7FE0 .. 0x7FFF]`.
>
> ---
>
> ### Function prologue / epilogue (call setup & teardown)
>
> * **Prologue** (at the start of a function) typically:
>
>   ```asm
>   addi sp, sp, -N   # reserve N bytes on the stack (stack grows *down*)
>   sd   ra, 0(sp)    # save return address
>   sd   s0, 8(sp)    # save frame pointer or callee-saved regs
>   # ... possibly more saves / local var space
>   ```
>
> * **Epilogue** (at the end of a function) does the reverse:
>
>   ```asm
>   ld   ra, 0(sp)    # restore return address
>   ld   s0, 8(sp)    # restore saved reg
>   addi sp, sp, N    # free N bytes: stack shrinks *up*
>   ret               # jump back to caller (via ra)
>   ```
>
> * Key idea:
>
>   * **Subtract from `sp`** to **allocate** stack space (grow down).
>   * **Add to `sp`** to **free** stack space (shrink back up).
>
> ---
>
> ### Why `sp` must start **non-zero** in your emulator
>
> * Compiled RISC-V code **assumes**:
>
>   * `sp` already points to a **valid stack area** in memory.
> * If you start with `sp = 0`:
>
>   ```asm
>   addi sp, sp, -16   # sp becomes a large wraparound address (0 - 16)
>   sd   ra, 0(sp)     # store to an invalid address (outside your RAM)
>   ```
>
>   * This will either corrupt memory or correctly trigger an out-of-bounds trap in your emulator.
> * In an emulator, you should:
>
>   * Decide that part of RAM is the **stack region**, usually the **top of RAM**.
>   * Example (RAM size = `MEM_SIZE` bytes):
>
>     * Set `sp = MEM_SIZE` (or `MEM_SIZE - 16` to keep it aligned).
>     * Now when the first function does `addi sp, sp, -N`, it moves into valid stack space.
>
> ---
>
> ### Who decides that `x2` is the stack pointer?
>
> * The **ISA** (RISC-V spec) only says: you have registers `x0..x31`.
> * The **ABI / calling convention** says:
>
>   * **`x2` is the stack pointer (`sp`)**.
>   * The **stack grows down**.
>   * `sp` must be **properly aligned** (e.g., 16-byte alignment).
> * Your emulator doesn’t need to “know” what `sp` means, but to run **real compiled code** correctly, you must:
>
>   * **Initialize `sp`** to a good stack address in RAM before starting execution.
>
> ---
>
> ### TL;DR for your emulator
>
> * Treat `sp` (`x2`) as:
>
>   * “The address of the top of the stack; we move it **down** (subtract) to allocate.”
> * Before running a program, do something like:
>
>   * `regs[2] = MEM_SIZE as u64;`
>   * maybe `regs[2] -= 16;` to keep some alignment.
> * Then compiled code like:
>
>   ```asm
>   addi sp, sp, -16
>   sd   ra, 0(sp)
>   ```
>
>   will operate on a real, valid stack region inside your emulated RAM.


