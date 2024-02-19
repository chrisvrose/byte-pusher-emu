use crate::misc::error::EmulatorError;

pub type EmulatorResult<T> = Result<T,EmulatorError>;