use crate::emu::iomem::MemoryMappedIO;
use crate::emu::mem::RamMemory;

// 8 bytes that are memory mapped i/o
pub const MMAPPEDIO_END: u32 = RAM_MEM_START-1;
pub const RAM_MEM_START:u32 = 8;
/// Last index of ram
pub const RAM_MEM_END: u32 = 2<<24 - 1;

///
/// mapped I/O + RAM.
pub trait Memory {
    /// Get the value (24bit) at the address(24bit)
    fn get_byte(&self, address: u32) -> u8;
    /// Set the value at the 24bit address
    fn set_byte(&mut self, address: u32, value: u8);
}
#[derive(Debug,Clone)]
pub struct MappedMemory {
    memory_mapped_io: MemoryMappedIO,
    ram_memory: RamMemory,
}
impl MappedMemory {
    pub fn new(memory_mapped_io: MemoryMappedIO, ram_memory: RamMemory)-> MappedMemory {
        MappedMemory {
            memory_mapped_io,
            ram_memory
        }
    }
}

impl Memory for MappedMemory {
    fn get_byte(&self, address: u32) -> u8 {
        match address {
            0..=MMAPPEDIO_END => {
                self.memory_mapped_io.get_byte(address)
            },
            RAM_MEM_START..=RAM_MEM_END =>{
                self.ram_memory.get_byte(address)
            }
            _ => { panic!("Invalid address") }
        }
    }
    fn set_byte(&mut self, address: u32, value: u8) {
        todo!()
    }
}

