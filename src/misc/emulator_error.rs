use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum DeviceType {
    RAM,
    MMU,
    /// Program counter
    PC,
    KEYBOARD,
    AUDIO,
    GRAPHICS,
}

#[derive(Debug, Clone)]
pub enum EmulatorError {
    AllocationFailure(DeviceType, &'static str),
    UnreachableMemory(DeviceType, u32),
    InvalidColor(u8)
}





