use std::array::TryFromSliceError;
use std::fmt::Debug;
use std::io::Error;
use crate::misc::emulator_error::EmulatorError::EmulatorIOError;

#[derive(Debug, Copy, Clone)]
pub enum DeviceType {
    CPU,
    RAM,
    KEYBOARD,
    AUDIO,
    GRAPHICS,
}

#[derive(Debug)]
pub enum EmulatorError {
    AllocationFailure(DeviceType, &'static str),
    UnreachableMemory(DeviceType, u32),
    InvalidColor(u8),
    EmulatorIOError(Error),
    OtherError(String),
}

impl From<TryFromSliceError> for EmulatorError {
    fn from(_: TryFromSliceError) -> Self {
        EmulatorError::UnreachableMemory(DeviceType::RAM, 0x5a5a)
    }
}

impl From<std::io::Error> for EmulatorError {
    fn from(value: Error) -> Self {
        EmulatorIOError(value)
    }
}




