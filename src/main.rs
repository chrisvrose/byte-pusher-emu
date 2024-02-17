use simple_logger::SimpleLogger;
use crate::emu::iomem::MemoryMappedIO;
use crate::emu::mem::RamMemory;
use crate::emu::mmu::{Memory, MappedMemory};
use crate::misc::emulator_error::EmulatorError;

mod emu;
mod args;
mod misc;

fn main() ->Result<(),EmulatorError> {
    SimpleLogger::new().env().init().unwrap();

    let mmio = MemoryMappedIO::new();
    let ram = RamMemory::try_new()?;
    let mmu = MappedMemory::new(mmio, ram);
    for i in 0..10 {
        log::info!("Memory at {} is {}",i,mmu.try_get_byte(i)?);
    }
    Ok(())
}
