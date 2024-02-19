use std::array::TryFromSliceError;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum DeviceType {
    CPU,
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
    InvalidColor(u8),
    OtherError(String)
}

impl From<TryFromSliceError> for EmulatorError{
    fn from(value: TryFromSliceError) -> Self {
        EmulatorError::UnreachableMemory(DeviceType::RAM,0x5a5a)
    }
}




