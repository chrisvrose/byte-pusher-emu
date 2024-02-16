use simple_logger::SimpleLogger;
use crate::emu::iomem::MemoryMappedIO;
use crate::emu::mem::RamMemory;
use crate::emu::mmu::{Memory, MappedMemory};

mod emu;
mod args;

fn main() {
    SimpleLogger::new().init().unwrap();

    let mmio = MemoryMappedIO::new();
    let ram = RamMemory::new();
    let mmu = MappedMemory::new(mmio, ram);
    for i in 0..10 {
        log::info!("Memory at {} is {}",i,mmu.get_byte(i));
    }

}
