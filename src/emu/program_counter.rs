use crate::emu::mmu::{Memory, RAM_MEM_END};
use crate::misc::emulator_error::DeviceType::PC;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::endian::{read_big_endian_u24, write_big_endian_u24};
use crate::misc::result::EmulatorResult;


#[derive(Debug, Default, Clone)]
pub struct ProgramCounter {
    /// 24bit location register
    program_counter_register: [u8; 3],
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            program_counter_register: [0; 3]
        }
    }
    // get the current program counter as an address
    pub fn get_pc_value(&self) -> u32 {
        read_big_endian_u24(&self.program_counter_register)
    }
    // assign a value of PC to start execution
    pub fn set_address(&mut self, address: u32) -> EmulatorResult<()> {
        log::debug!("Setting PC as {}",address);
        if address >= (2 << 24) {
            return Err(EmulatorError::UnreachableMemory(PC, address));
        }
        write_big_endian_u24(address, &mut self.program_counter_register);
        Ok(())
    }
}

/// Allow Using Program counter as mapped memory
impl Memory for ProgramCounter {
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8> {
        log::trace!("Fetching PC({}) byte segment index {}",read_big_endian_u24(&self.program_counter_register),address);
        self.program_counter_register.get(address as usize)
            .map(|e| *e)
            .ok_or(EmulatorError::UnreachableMemory(PC, address))
    }
    /// TODO: Set a byte of PC from the memory.
    fn try_set_byte(&mut self, address: u32, value: u8) -> EmulatorResult<()> {
        match address {
            0..=2 => { self.program_counter_register[address as usize] = value }
            _ => { return Err(EmulatorError::UnreachableMemory(PC, address)); }
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::emu::program_counter::ProgramCounter;

    #[test]
    pub fn setting_address_works() {
        const ADDRESS: u32 = 4;
        let mut pc = ProgramCounter::new();
        let res = pc.set_address(ADDRESS);
        assert!(res.is_ok());
        assert_eq!(ADDRESS, pc.get_pc_value());
    }

    #[test]
    pub fn set_invalid_addr_returns_error() {
        const ADDRESS: u32 = 2 << 24;
        let mut pc = ProgramCounter::new();
        let res = pc.set_address(ADDRESS);
        assert!(res.is_err());
    }
}