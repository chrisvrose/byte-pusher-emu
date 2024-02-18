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
    fn try_set_byte(&self, address: u32, value: u8) -> EmulatorResult<()>;
}

#[derive(Debug)]
pub struct MappedMemory<'a> {
    memory_mapped_io: &'a MemoryMappedIO<'a>,
    ram_memory: &'a RamMemory,
}

impl <'a> MappedMemory<'a> {
    pub fn new(memory_mapped_io: &'a MemoryMappedIO<'a>, ram_memory:&'a RamMemory) -> MappedMemory<'a> {
        MappedMemory {
            memory_mapped_io,
            ram_memory,
        }
    }
}

impl <'a> Memory for MappedMemory<'a> {
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
    fn try_set_byte(&self, address: u32, value: u8) -> EmulatorResult<()> {
        match address {
            0..=MMAPPEDIO_END => {
                self.memory_mapped_io.try_set_byte(address,value)
            }
            RAM_MEM_START..=RAM_MEM_END => {
                let memory_index = address - RAM_MEM_START;
                self.ram_memory.try_set_byte(memory_index,value)
            }
            _ => { Err(UnreachableMemory(MMU, address)) }
        }
    }
}

