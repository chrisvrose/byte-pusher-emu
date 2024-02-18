use crate::emu::graphics::{DEVICE_FRAMEBUFFER_SIZE, GraphicsProcessor};
use crate::misc::result::EmulatorResult;

pub trait GraphicsAdapter{
    fn draw(&mut self,frame_buf:&[u8;DEVICE_FRAMEBUFFER_SIZE])->EmulatorResult<()>;
}

#[derive(Debug, Clone)]
pub struct SDLGraphicsAdapter{
    graphics_processor: GraphicsProcessor
}

impl GraphicsAdapter for SDLGraphicsAdapter{
    fn draw(&mut self, frame_buf: &[u8; DEVICE_FRAMEBUFFER_SIZE]) -> EmulatorResult<()> {
        todo!()
    }
}

