use crate::misc::result::EmulatorResult;

/// mapped I/O + RAM.
pub trait Memory {
    /// Get the value (24bit) at the address(24bit)
    fn try_get_byte(&self, address: u32) -> EmulatorResult<u8>;
    /// Set the value at the 24bit address
    fn try_set_byte(&self, address: u32, value: u8) -> EmulatorResult<()>;
}


