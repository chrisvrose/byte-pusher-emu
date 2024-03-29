use std::cell::{Ref, RefCell};

use crate::misc::error::DeviceType::RAM;
use crate::misc::error::EmulatorError;
use crate::misc::endian::{read_big_endian_u24, write_big_endian_u16};
use crate::misc::result::EmulatorResult;

pub const MEM_LENGTH: usize = 2 << 23;

/// mapped I/O + RAM.
pub trait Memory {
    /// Get the value (24bit) at the address(24bit)
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8>;
    /// Set the value at the 24bit address
    fn try_set_byte(&self, address: u32, value: u8) -> EmulatorResult<()>;
    fn try_set_u16(&self, address: u32, value: u16) -> EmulatorResult<()>;
}

#[derive(Clone, Debug)]
pub struct RamMemory {
    data: RefCell<Box<[u8; MEM_LENGTH]>>,
}

impl RamMemory {
    pub fn try_from(existing_data: &[u8]) -> EmulatorResult<RamMemory> {
        let alloc_result = vec![0u8; MEM_LENGTH].into_boxed_slice();
        // get box of fixed size
        let mut fixed_size_alloc_box: Box<[u8; MEM_LENGTH]> = alloc_result.try_into().unwrap();
        fixed_size_alloc_box.copy_from_slice(&existing_data);

        Ok(RamMemory {
            data: RefCell::new(fixed_size_alloc_box)
        })
    }

    pub fn try_get_u24(&self, index: u32) -> EmulatorResult<u32> {
        const U24_LEN: usize = 3;
        let index_usize = index as usize;
        let data = self.data.borrow();
        let a = data.get(index_usize..(index_usize + U24_LEN)).ok_or(EmulatorError::UnreachableMemory(RAM, index))?;
        Ok(read_big_endian_u24(a.try_into()?))
    }

    /// write a block of memory into an output slice
    pub fn try_copy_block(&self, address: u32, output: &mut [u8]) -> EmulatorResult<()> {
        let address = address as usize;
        let data = self.data.borrow();
        let data_requested_slice = data.get(address..address + output.len()).ok_or(EmulatorError::UnreachableMemory(RAM, address as u32))?;
        output.copy_from_slice(data_requested_slice);
        Ok(())
    }

    pub fn get_data_ref(&self) -> Ref<Box<[u8; MEM_LENGTH]>> {
        self.data.borrow()
    }

}

impl Memory for RamMemory {
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8> {
        // log::trace!("Fetch RAM memory at address {}",address);
        let data = self.data.borrow();
        let x = *data.get(address as usize).ok_or(EmulatorError::UnreachableMemory(RAM, address))?;
        Ok(x)
    }

    fn try_set_byte(&self, address: u32, value: u8) -> EmulatorResult<()> {
        if address >= MEM_LENGTH as u32 {
            return Err(EmulatorError::UnreachableMemory(RAM, address));
        }
        let mut data = self.data.borrow_mut();
        data[address as usize] = value;
        Ok(())
    }

    fn try_set_u16(&self, address: u32, value: u16) -> EmulatorResult<()> {
        let address = address as usize;
        let mut data_ref = self.data.borrow_mut();
        let data = data_ref.get_mut(address..(address + 2)).ok_or(EmulatorError::UnreachableMemory(RAM, address as u32))?;

        write_big_endian_u16(value, data.try_into()?);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::emu::memory::{MEM_LENGTH, RamMemory};
    use crate::emu::memory::Memory;

    const EXAMPLE_ADDRESS: u32 = 0x24;
    fn get_box_data()->Box<[u8; MEM_LENGTH]>{
        let alloc_result = vec![0u8; MEM_LENGTH].into_boxed_slice();
        // get box of fixed size
        let fixed_size_alloc_box: Box<[u8; MEM_LENGTH]> = alloc_result.try_into().unwrap();
        fixed_size_alloc_box
    }
    #[test]
    pub fn get_mem() {
        let data = get_box_data();
        let ram_result = RamMemory::try_from(data.as_ref());
        assert!(ram_result.is_ok())
    }

    #[test]
    pub fn get_set_memory() {
        const EXAMPLE_DATA: u8 = 0xa5;
        let data = get_box_data();

        let ram = RamMemory::try_from(data.as_ref()).unwrap();
        let byte_before = ram.try_get_byte(EXAMPLE_ADDRESS).unwrap();
        ram.try_set_byte(EXAMPLE_ADDRESS, EXAMPLE_DATA).unwrap();
        let byte_after = ram.try_get_byte(EXAMPLE_ADDRESS).unwrap();
        assert_eq!(0, byte_before);
        assert_eq!(EXAMPLE_DATA, byte_after);
    }
}


