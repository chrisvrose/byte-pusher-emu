use std::cell::Cell;
use crate::emu::mmu::Memory;
use crate::emu::program_counter::ProgramCounter;
use crate::misc::emulator_error::{DeviceType, EmulatorError};
use crate::misc::emulator_error::DeviceType::MMU;
use crate::misc::endian::{read_big_endian_u16};
use crate::misc::result::EmulatorResult;

#[derive(Debug)]
pub struct MemoryMappedIO<'a> {
    //FIXME use a keyboard
    keyboard_bytes: Cell<[u8; 2]>,
    program_counter: &'a ProgramCounter,
    //FIXME use a device
    pixel_reg: Cell<u8>,
    //FIXME use a device
    audio_sample_address_base: Cell<[u8; 2]>,
}

/// Represents the memory mapped segment of IO. Aggregates the mapping logic.
impl<'a> MemoryMappedIO<'a> {
    // 2 byte keyboard bits
    pub const KEYBOARD_BIT_START: u32 = 0;
    const KEYBOARD_BIT_END: u32 = 1;

    // 3 bytes PC
    const PC_START_ADDR: u32 = 2;
    const PC_LEN: u32 = 3;
    const PC_END_ADDR: u32 = Self::PC_START_ADDR + Self::PC_LEN - 1;

    // 1 byte pixel base reg
    pub const PIXEL_BASE: u32 = 5;

    // 2 byte audio sample base reg
    pub const AUDIO_SAMPLE_BASE_START: u32 = 6;
    pub const AUDIO_SAMPLE_BASE_LEN: u32 = 2;
    const AUDIO_SAMPLE_BASE_END: u32 = Self::AUDIO_SAMPLE_BASE_START + Self::AUDIO_SAMPLE_BASE_LEN - 1;

    pub fn new(program_counter: &'a ProgramCounter) -> MemoryMappedIO<'a> {
        MemoryMappedIO {
            keyboard_bytes: Cell::new([0, 0]),
            program_counter,
            pixel_reg: Cell::new(0),
            audio_sample_address_base: Cell::new([0, 0]),
        }
    }
}


impl<'a> Memory for MemoryMappedIO<'a> {
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8> {
        let byte = match address {
            Self::KEYBOARD_BIT_START..=Self::KEYBOARD_BIT_END => {
                let addr_usize = address as usize;
                let keyboard_bytes = self.keyboard_bytes.get();
                let keyboard_byte = keyboard_bytes[addr_usize];
                log::trace!("Fetching keyboard({}) byte segment {} -> {}",read_big_endian_u16(&keyboard_bytes),address,keyboard_byte);
                Ok(keyboard_byte)
            }
            Self::PC_START_ADDR..=Self::PC_END_ADDR => {
                let pc_index = address - Self::PC_START_ADDR;
                self.program_counter.try_get_byte(pc_index)
            }
            Self::PIXEL_BASE => {
                log::trace!("Fetching pixel base reg {}",self.pixel_reg.get());
                Ok(self.pixel_reg.get())
            }
            Self::AUDIO_SAMPLE_BASE_START => Ok(self.audio_sample_address_base.get()[0]),
            Self::AUDIO_SAMPLE_BASE_END => Ok(self.audio_sample_address_base.get()[1]),
            address => {
                Err(EmulatorError::UnreachableMemory(MMU, address))
            }
        };
        byte
    }

    fn try_set_byte(&self, address: u32, byte_value: u8) -> EmulatorResult<()> {
        match address {
            Self::KEYBOARD_BIT_START..=Self::KEYBOARD_BIT_END => {
                let mut keyboard_bytes = self.keyboard_bytes.get();
                log::trace!("Setting keyboard({}) byte segment {} -> {}",read_big_endian_u16(&keyboard_bytes),address,byte_value);
                keyboard_bytes[address as usize] = byte_value;
                self.keyboard_bytes.set(keyboard_bytes);
            }
            Self::PC_START_ADDR..=Self::PC_END_ADDR => {
                let pc_index = address - Self::PC_START_ADDR;
                self.program_counter.try_set_byte(pc_index, byte_value)?
            }
            Self::PIXEL_BASE => {
                log::trace!("Setting pixel base reg to {}",byte_value);
                self.pixel_reg.set(byte_value);
            }
            Self::AUDIO_SAMPLE_BASE_START..=Self::AUDIO_SAMPLE_BASE_END => {
                let audio_reg_index = (address - Self::AUDIO_SAMPLE_BASE_START) as usize;
                let mut audio_sample_address_base = self.audio_sample_address_base.get();
                audio_sample_address_base[audio_reg_index] = byte_value;
                self.audio_sample_address_base.set(audio_sample_address_base);
            }
            _ => { return Err(EmulatorError::UnreachableMemory(MMU, address)); }
        };
        Ok(())
    }
}
