use crate::emu::mmu::Memory;

const MEM_LENGTH: usize = 2 << 24;

#[derive(Clone, Debug)]
pub struct RamMemory {
    data: Box<[u8; MEM_LENGTH]>,
}

impl RamMemory {
    pub fn new() -> RamMemory {
        RamMemory {
            data: vec![0; MEM_LENGTH].into_boxed_slice().try_into().expect("Incorrect ram allocation")
        }
    }
}

impl Memory for RamMemory {
    fn get_byte(&self, address: u32) -> u8 {
        log::trace!("Fetch RAM memory at address {}",address);
        let x = *self.data.get(address as usize).expect("Unchecked address fetch");
        x
    }

    fn set_byte(&mut self, address: u32, val: u8) {
        self.data[address as usize] = val;
    }
}