use std::cell::RefCell;
use crate::graphics::graphics_adapter::GraphicsAdapter;
use crate::misc::emulator_error::DeviceType::GRAPHICS;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::result::EmulatorResult;

pub const DEVICE_FRAMEBUFFER_SIZE: usize = 256 * 256;

#[derive(Debug)]
pub struct GraphicsProcessor {
    frame_buffer: RefCell<Box<[u8; DEVICE_FRAMEBUFFER_SIZE]>>,
    graphics_adapter: Box<dyn GraphicsAdapter>
}

/// Abstracted graphics processor. Calls `[GraphicsAdapter]`
impl GraphicsProcessor {
    pub fn try_new(graphics_adapter: Box<dyn GraphicsAdapter>) -> EmulatorResult<GraphicsProcessor> {
        let framebuffer = vec![0; DEVICE_FRAMEBUFFER_SIZE].into_boxed_slice()
            .try_into()
            .map_err(|_| {
                EmulatorError::AllocationFailure(GRAPHICS, "Failed to allocate graphics")
            })?;
        Ok(GraphicsProcessor {
            frame_buffer: RefCell::new(framebuffer),
            graphics_adapter
        })
    }
    /// take a copy of FB and
    pub fn draw(&self, memory_slice: &[u8;DEVICE_FRAMEBUFFER_SIZE])->EmulatorResult<()>{
        self.set_framebuffer(memory_slice);
        let fb_immut = self.frame_buffer.borrow();
        self.graphics_adapter.draw(fb_immut)
    }
    fn set_framebuffer(&self, memory_slice: &[u8;DEVICE_FRAMEBUFFER_SIZE]) {
        let mut fb = self.frame_buffer.borrow_mut();
        fb.copy_from_slice(memory_slice);
    }
}

#[cfg(test)]
mod test{

}