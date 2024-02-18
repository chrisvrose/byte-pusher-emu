use crate::emu::mmu::Memory;
use crate::misc::emulator_error::DeviceType::PC;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::endian::{read_big_endian_u24, write_big_endian_u24};
use crate::misc::result::EmulatorResult;


#[derive(Debug,Default, Copy, Clone)]
pub struct ProgramCounter {
    /// 24bit location register
    program_counter_register: [u8; 3],
}

impl ProgramCounter {
    // get the current program counter as an address
    pub fn get_pc_value(&self) -> u32 {
        read_big_endian_u24(&self.program_counter_register)
    }
    // assign a value of PC to start execution
    pub fn set_address(&mut self, address: u32) {
        log::debug!("Setting PC as {}",address);
        write_big_endian_u24(address, &mut self.program_counter_register)
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
        log::error!("Unsupported Method!");
        todo!()
    }
}