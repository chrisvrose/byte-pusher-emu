use std::time::Duration;
use sdl2::audio::AudioQueue;

use crate::emu::memory::RamMemory;
use crate::misc::endian::read_big_endian_u16;
use crate::misc::error::DeviceType::AUDIO;
use crate::misc::error::EmulatorError;
use crate::misc::result::EmulatorResult;

pub const AUDIO_BUFFER_SIZE: usize = 256;
const I8_SIGN_BYTE: u8 = 0x80;
const AUDIO_REG_START: usize = 6;
const AUDIO_REG_LEN: usize = 2;
const AUDIO_REG_END: usize = AUDIO_REG_START + AUDIO_REG_LEN -1;

pub struct AudioProcessor<'a> {
    ram: &'a RamMemory,
    frame_buffer: Box<[u8; AUDIO_BUFFER_SIZE]>,
    audio_queue: &'a AudioQueue<u8>,
}

impl<'a> AudioProcessor<'a> {
    pub fn try_new(ram: &'a RamMemory, audio_queue: &'a AudioQueue<u8>) -> EmulatorResult<AudioProcessor<'a>> {
        let frame_buffer = vec![0u8; AUDIO_BUFFER_SIZE].into_boxed_slice()
            .try_into()
            .map_err(|_| {
                EmulatorError::AllocationFailure(AUDIO, "Failed to allocate graphics")
            })?;
        Ok(AudioProcessor {
            ram,
            audio_queue,
            frame_buffer,
        })
    }
}


impl<'a> AudioProcessor<'a> {
    pub fn queue(&mut self) -> EmulatorResult<()> {
        let audio_base_reg = (self.get_audio_base_reg() as u32) << 8;
        let fb = self.frame_buffer.as_mut();

        // The CPU frame timing is just a little less than 60 fps to prevent audio stutter.
        // We will then wait for audio to drain to adjust frame timing
        if self.audio_queue.size() == 0 {
            log::info!("Detected Queue empty! Audio stutter may occur");
        }
        while self.audio_queue.size() > (AUDIO_BUFFER_SIZE / 2) as u32 {
            std::thread::sleep(Duration::from_micros(1))
        }
        self.ram.try_copy_block(audio_base_reg, fb)?;

        //convert to u8 audio format (BytePusher stores it as "i8")
        fb.iter_mut().for_each(|item|{
            *item ^= I8_SIGN_BYTE;
        });
        self.audio_queue.queue_audio(fb).map_err(|s| { EmulatorError::OtherError(Some(AUDIO), s) })
    }
    fn get_audio_base_reg(&self) -> u16 {
        let data = self.ram.get_data_ref();
        let audio_base_reg = data.get(AUDIO_REG_START..=AUDIO_REG_END).unwrap();
        read_big_endian_u16(audio_base_reg.try_into().unwrap())
    }
}