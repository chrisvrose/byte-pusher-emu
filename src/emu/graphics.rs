use crate::misc::emulator_error::DeviceType::GRAPHICS;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::result::EmulatorResult;

pub const DEVICE_FRAMEBUFFER_SIZE: usize = 256 * 256;

pub struct GraphicsProcessor {
    framebuffer: Box<[u8; DEVICE_FRAMEBUFFER_SIZE]>,
}

/// Abstracted graphics processor. Refer GraphicsAdapter
impl GraphicsProcessor {
    pub fn try_new() -> EmulatorResult<GraphicsProcessor> {
        let framebuffer = vec![0; DEVICE_FRAMEBUFFER_SIZE].into_boxed_slice()
            .try_into()
            .map_err(|_| {
                EmulatorError::AllocationFailure(GRAPHICS, "Failed to allocate graphics")
            })?;
        Ok(GraphicsProcessor {
            framebuffer
        })
    }
    pub fn set_framebuffer(&mut self,memory_slice: &[u8]){
        self.framebuffer.copy_from_slice(memory_slice);
    }
    pub fn get_framebuffer(&self) -> &[u8;DEVICE_FRAMEBUFFER_SIZE] {
        &self.framebuffer
    }
}

