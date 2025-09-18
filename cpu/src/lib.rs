pub const DRAM_SIZE: u64 = 1024 * 1024 * 128; // 128KB

struct Cpu {
    regs: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut regs: [u64; 32] = [0; 32];
        regs[2] = DRAM_SIZE;
        Self {
            regs,
            pc: 0,
            dram: code,
        }
    }
}
