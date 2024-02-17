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




#[cfg(test)]
mod tests{
    use crate::emu::mem::RamMemory;
    use crate::emu::mmu::Memory;
    const EXAMPLE_ADDRESS:u32=0x24;

    #[test]
    pub fn get_mem(){
        let ram_result = RamMemory::try_new();
        assert!(ram_result.is_ok())
    }
    #[test]
    pub fn get_set_memory(){
        const EXAMPLE_DATA:u8 = 0xa5;
        let mut ram = RamMemory::try_new().unwrap();
        let byte_before = ram.try_get_byte(EXAMPLE_ADDRESS).unwrap();
        ram.try_set_byte(EXAMPLE_ADDRESS, EXAMPLE_DATA).unwrap();
        let byte_after = ram.try_get_byte(EXAMPLE_ADDRESS).unwrap();
        assert_eq!(0,byte_before);
        assert_eq!(EXAMPLE_DATA,byte_after);
    }
}