use std::cell::{Ref, RefCell};
use crate::emu::memory::Memory;
use crate::emu::memory::RamMemory;
use crate::misc::error::DeviceType::GRAPHICS;
use crate::misc::error::EmulatorError;
use crate::misc::result::EmulatorResult;

pub const DEVICE_FRAMEBUFFER_SIZE: usize = 256 * 256;

#[derive(Debug)]
pub struct GraphicsProcessor<'a> {
    ram: &'a RamMemory,
    frame_buffer: RefCell<Box<[u8; DEVICE_FRAMEBUFFER_SIZE]>>,
}

/// Abstracted graphics processor. Calls `[GraphicsAdapter]` with the associated framebuffer
impl<'a> GraphicsProcessor<'a> {
    pub fn try_new(ram_memory: &'a RamMemory) -> EmulatorResult<GraphicsProcessor> {
        let framebuffer = vec![0; DEVICE_FRAMEBUFFER_SIZE].into_boxed_slice()
            .try_into()
            .map_err(|_| {
                EmulatorError::AllocationFailure(GRAPHICS, "Failed to allocate graphics")
            })?;
        Ok(GraphicsProcessor {
            ram: ram_memory,
            frame_buffer: RefCell::new(framebuffer),
        })
    }
    /// Draw the pixels.
    /// Since this is a VM, we just copy and store it in a buffer.
    /// take a copy of FB and store it
    pub fn draw(&self) -> EmulatorResult<()> {
        self.copy_indicated_pixel_data_block()
    }

    fn copy_indicated_pixel_data_block(&self) -> Result<(), EmulatorError> {
        let fb_base_register = (self.ram.try_get_byte(5)? as u32) << 16;
        let mut fb = self.frame_buffer.borrow_mut();

        self.ram.get_block(fb_base_register, fb.as_mut())?;
        Ok(())
    }
    fn set_framebuffer(&self, memory_slice: &[u8; DEVICE_FRAMEBUFFER_SIZE]) {
        let mut fb = self.frame_buffer.borrow_mut();
        fb.copy_from_slice(memory_slice);
    }
    pub fn get_framebuffer(&self) -> Ref<Box<[u8; DEVICE_FRAMEBUFFER_SIZE]>> {
        self.frame_buffer.borrow()
    }
}

#[cfg(test)]
mod test {}