
use crate::emu::mmu::Memory;
use crate::misc::endian::MemoryOperations;

#[derive(Debug, Copy, Clone)]
pub struct MemoryMappedIO {
    //FIXME use a keyboard
    keyboard_bytes: [u8;2],
    program_counter: [u8; 3],
    //FIXME use a device
    pixel_reg: u8,
    //FIXME use a device
    audio_sample_address_base: [u8; 2],
}

impl MemoryMappedIO {
    pub fn new() -> MemoryMappedIO {
        MemoryMappedIO {
            keyboard_bytes: [0,0],
            program_counter: [0, 0, 0],
            pixel_reg: 0,
            audio_sample_address_base: [0, 0],
        }
    }
}
// 2 byte keyboard bits

const KEYBOARD_BIT_START: u32 = 0;
const KEYBOARD_BIT_END: u32 = 1;

// 3 bytes PC
const PC_START_ADDR: u32 = 2;
const PC_LEN: u32 = 3;
const PC_END_ADDR: u32 = PC_START_ADDR + PC_LEN - 1;

// 1 byte pixel base reg
const PIXEL_BASE: u32 = 5;

// 2 byte audio sample base reg
const AUDIO_SAMPLE_BASE_START: u32 = 6;
const AUDIO_SAMPLE_BASE_LEN: u32 = 2;
const AUDIO_SAMPLE_BASE_END: u32 = AUDIO_SAMPLE_BASE_START + AUDIO_SAMPLE_BASE_LEN - 1;

impl Memory for MemoryMappedIO {
    fn get_byte(&self, address: u32) -> u8 {
        match address {
            KEYBOARD_BIT_START..=KEYBOARD_BIT_END => {
                let addr_usize = address as usize;
                let keyboard_byte = self.keyboard_bytes[addr_usize];
                log::trace!("Fetching keyboard({}) byte segment {} -> {}",MemoryOperations::read_big_endian_u16(&self.keyboard_bytes),address,keyboard_byte);
                keyboard_byte
            },
            PC_START_ADDR..=PC_END_ADDR => {
                let pc_index = (address - PC_START_ADDR) as usize;
                let pc_byte = self.program_counter[pc_index];
                log::trace!("Fetching PC({}) byte segment {} -> {}",MemoryOperations::read_big_endian_u24(&self.program_counter),pc_index,pc_byte);
                pc_byte
            },
            PIXEL_BASE => {
                log::trace!("Fetching pixel base reg {}",self.pixel_reg);
                self.pixel_reg
            },
            AUDIO_SAMPLE_BASE_START => self.audio_sample_address_base[0],
            AUDIO_SAMPLE_BASE_END => self.audio_sample_address_base[1],
            _ => { panic!("Unreachable code") }
        }
    }

    fn set_byte(&mut self, address: u32, val: u8) {
        todo!()
    }
}
