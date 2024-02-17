use std::ops::Index;
use crate::emu::mmu::Memory;
use crate::misc::emulator_error::DeviceType::RAM;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::result::EmulatorResult;

const MEM_LENGTH: usize = 2 << 24;

#[derive(Clone, Debug)]
pub struct RamMemory {
    data: Box<[u8; MEM_LENGTH]>,
}

impl RamMemory {
    pub fn try_new() -> EmulatorResult<RamMemory> {
        let alloc_result = vec![0; MEM_LENGTH].into_boxed_slice();
        let data = alloc_result.try_into().map_err(|err|{
            EmulatorError::AllocationFailure(RAM, "Allocation failed")
        })?;
        Ok(RamMemory {
            data
        })
    }
}


impl Memory for RamMemory {
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8> {
        log::trace!("Fetch RAM memory at address {}",address);
        let x = *self.data.get(address as usize).ok_or(EmulatorError::UnreachableMemory(RAM, address))?;
        Ok(x)
    }

    fn try_set_byte(&mut self, address: u32, value: u8) -> EmulatorResult<()> {
        if address>= MEM_LENGTH as u32 {
            return Err(EmulatorError::UnreachableMemory(RAM, address))
        }
        self.data[address as usize] = value;
        Ok(())
    }
}