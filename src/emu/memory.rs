use std::cell::{Ref, RefCell, RefMut};

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
}

#[derive(Clone, Debug)]
pub struct RamMemory {
    data: RefCell<Box<[u8; MEM_LENGTH]>>,
}

impl RamMemory {
    pub fn try_new() -> EmulatorResult<RamMemory> {
        let alloc_result = vec![0; MEM_LENGTH].into_boxed_slice();
        let data = alloc_result.try_into().map_err(|err| {
            EmulatorError::AllocationFailure(RAM, "Allocation failed")
        })?;
        let data_refcell = RefCell::new(data);
        Ok(RamMemory {
            data: data_refcell
        })
    }
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

    pub fn get_block(&self, address: u32, output: &mut [u8]) -> EmulatorResult<()> {
        let address = address as usize;
        let data = self.data.borrow();
        let data_requested_slice = data.get(address..address + output.len()).ok_or(EmulatorError::UnreachableMemory(RAM, address as u32))?;
        output.copy_from_slice(data_requested_slice);
        Ok(())
    }

    pub fn get_data_ref(&self) -> Ref<Box<[u8; MEM_LENGTH]>> {
        self.data.borrow()
    }
    pub fn get_data_ref_mut(&self) -> RefMut<Box<[u8; MEM_LENGTH]>> {
        self.data.borrow_mut()
    }
    /// set keyboard bits
    pub fn set_keyboard(&self,keyboard_bits: u16) {
        let mut keyboard_slice_ref = self.data.borrow_mut();
        let keyboard_slice = keyboard_slice_ref.get_mut(0..2).unwrap();
        write_big_endian_u16(keyboard_bits,keyboard_slice.try_into().unwrap());
    }
}

impl Memory for RamMemory {
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8> {
        log::trace!("Fetch RAM memory at address {}",address);
        let mut data = self.data.borrow_mut();
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
}


#[cfg(test)]
mod tests {
    use crate::emu::memory::RamMemory;
    use crate::emu::memory::Memory;

    const EXAMPLE_ADDRESS: u32 = 0x24;

    #[test]
    pub fn get_mem() {
        let ram_result = RamMemory::try_new();
        assert!(ram_result.is_ok())
    }

    #[test]
    pub fn get_set_memory() {
        const EXAMPLE_DATA: u8 = 0xa5;
        let mut ram = RamMemory::try_new().unwrap();
        let byte_before = ram.try_get_byte(EXAMPLE_ADDRESS).unwrap();
        ram.try_set_byte(EXAMPLE_ADDRESS, EXAMPLE_DATA).unwrap();
        let byte_after = ram.try_get_byte(EXAMPLE_ADDRESS).unwrap();
        assert_eq!(0, byte_before);
        assert_eq!(EXAMPLE_DATA, byte_after);
    }
}


