use std::fmt::Debug;

#[derive(Debug)]
pub enum DeviceType {
    RAM,
    MMU,
    /// Program counter
    PC,
    KEYBOARD,
    AUDIO,
    GRAPHICS,
}

#[derive(Debug)]
pub enum EmulatorError {
    AllocationFailure(DeviceType, &'static str),
    UnreachableMemory(DeviceType, u32),
    InvalidColor(u8)
}





