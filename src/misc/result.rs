use crate::misc::emulator_error::EmulatorError;

pub type EmulatorResult<T> = Result<T,EmulatorError>;