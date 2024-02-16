
use crate::emu::mmu::Memory;

#[derive(Debug, Copy, Clone)]
pub struct MemoryMappedIO {
    keyboard_bits: [u8;2],
    program_counter: [u8; 3],
    pixel_reg: u8,
    audio_sample_address_base: [u8; 2],
}

impl MemoryMappedIO {
    pub fn new() -> MemoryMappedIO {
        MemoryMappedIO {
            keyboard_bits: [0,0],
            program_counter: [0, 0, 0],
            pixel_reg: 0,
            audio_sample_address_base: [0, 0],
        }
    }
}

const KEYBOARD_BIT_START: u32 = 0;
const KEYBOARD_BIT_END: u32 = 1;
const PC_START_ADDR: u32 = 2;
const PC_LEN: u32 = 3;
const PC_END_ADDR: u32 = PC_START_ADDR + PC_LEN - 1;

const PIXEL_BASE: u32 = 5;
const AUDIO_SAMPLE_BASE_START: u32 = 6;
const AUDIO_SAMPLE_BASE_LEN: u32 = 2;
const AUDIO_SAMPLE_BASE_END: u32 = AUDIO_SAMPLE_BASE_START + AUDIO_SAMPLE_BASE_LEN - 1;

impl Memory for MemoryMappedIO {
    fn get_byte(&self, address: u32) -> u8 {
        match address {
            KEYBOARD_BIT_START..=KEYBOARD_BIT_END => {
                let addr_usize = address as usize;
                log::trace!("Fetching keyboard bits segment {}",addr_usize);
                self.keyboard_bits[addr_usize]
            },
            PC_START_ADDR..=PC_END_ADDR => {
                let pc_index = (address - PC_START_ADDR) as usize;
                log::trace!("Fetching PC {:?} bit segment {}",self.program_counter,pc_index);
                self.program_counter[pc_index]
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
