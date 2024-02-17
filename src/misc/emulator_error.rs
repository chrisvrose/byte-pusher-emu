use std::fmt::Debug;

#[derive(Debug)]
pub enum DeviceType {
    RAM,
    MMU,
    KEYBOARD,
    AUDIO,
    GRAPHICS,
}

#[derive(Debug)]
pub enum EmulatorError {
    AllocationError(DeviceType, &'static str),
    UnreachableMemoryError(DeviceType, u32),
    InvalidColorError(u8)
}





