use std::cell::Ref;
use std::fmt::Debug;
use crate::misc::result::EmulatorResult;

pub trait GraphicsAdapter: Debug {
    fn draw(&self, frame_buf: Ref<Box<[u8; 65536]>>) -> EmulatorResult<()>;
}

#[derive(Debug, Clone)]
pub struct SDLGraphicsAdapter {
}

impl GraphicsAdapter for SDLGraphicsAdapter {
    fn draw(&self, frame_buffer: Ref<Box<[u8; 65536]>>) -> EmulatorResult<()> {
        todo!()
    }
}

