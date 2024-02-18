use simple_logger::SimpleLogger;
use crate::emu::iomem::MemoryMappedIO;
use crate::emu::ram::RamMemory;
use crate::emu::mmu::{Memory, MappedMemory};
use crate::misc::emulator_error::EmulatorError;
use crate::misc::result::EmulatorResult;

mod emu;
mod args;
mod misc;
mod graphics;

fn main() -> EmulatorResult<()> {
    SimpleLogger::new().env().init().unwrap();

    let mmio = MemoryMappedIO::new();
    let ram = RamMemory::try_new()?;
    let mmu = MappedMemory::new(mmio, ram);
    for i in 0..10 {
        log::info!("Memory at {} is {}",i,mmu.try_get_byte(i)?);
    }

    Ok(())
}
