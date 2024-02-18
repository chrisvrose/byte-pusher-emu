use crate::emu::iomem::MemoryMappedIO;
use crate::emu::ram::RamMemory;
use crate::misc::emulator_error::DeviceType::MMU;

use crate::misc::emulator_error::EmulatorError::UnreachableMemory;
use crate::misc::result::EmulatorResult;

// 8 bytes that are memory mapped i/o
pub const MMAPPEDIO_END: u32 = RAM_MEM_START - 1;
pub const RAM_MEM_START: u32 = 8;
/// Last index of ram
pub const RAM_MEM_END: u32 = 2 << 24 - 1;

///
/// mapped I/O + RAM.
pub trait Memory {
    /// Get the value (24bit) at the address(24bit)
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8>;
    /// Set the value at the 24bit address
    fn try_set_byte(&mut self, address: u32, value: u8) -> EmulatorResult<()>;
}

#[derive(Debug, Clone)]
pub struct MappedMemory {
    memory_mapped_io: MemoryMappedIO,
    ram_memory: RamMemory,
}

impl MappedMemory {
    pub fn new(memory_mapped_io: MemoryMappedIO, ram_memory: RamMemory) -> MappedMemory {
        MappedMemory {
            memory_mapped_io,
            ram_memory,
        }
    }
}

impl Memory for MappedMemory {
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8> {
        let byte_at_addr = match address {
            0..=MMAPPEDIO_END => {
                self.memory_mapped_io.try_get_byte(address)
            }
            RAM_MEM_START..=RAM_MEM_END => {
                let memory_index = address - RAM_MEM_START;
                self.ram_memory.try_get_byte(memory_index)
            }
            _ => { Err(UnreachableMemory(MMU, address)) }
        }?;

        Ok(byte_at_addr)
    }
    fn try_set_byte(&mut self, address: u32, value: u8) -> EmulatorResult<()> {
        todo!()
    }
}

