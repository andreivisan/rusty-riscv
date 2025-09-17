pub const DRAM_SIZE: u64 = 1024 * 1024 * 128; // 128KB

struct Cpu {
    regs: [u64; 32],
    pc: u64,
    code: Vec<u8>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
