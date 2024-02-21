use crate::emu::memory::{Memory, RamMemory};
use crate::misc::result::EmulatorResult;


pub struct Keyboard<'a>{
    dirty: bool,
    bitflags: u16,
    ram: &'a RamMemory
}

impl<'a> Keyboard<'a>{
    pub fn new(ram:&'a RamMemory)->Keyboard<'a>{
        Keyboard {
            dirty: true,
            bitflags:0,
            ram
        }
    }

    pub fn key_down(&mut self,x:u8){
        self.bitflags |= 1<<x;
        log::trace!("Key Down - state {}",self.bitflags);
        self.dirty = true
    }
    pub fn key_up(&mut self,x:u8){
        self.bitflags &= !((1<<x) as u16);
        log::debug!("Key Up - state {}",self.bitflags);
        self.dirty = true
    }
    pub fn flush_keyboard(&mut self)->EmulatorResult<()>{
        if self.dirty {
            log::debug!("Flushing keyboard {}",self.bitflags);
            self.ram.try_set_u16(0, self.bitflags)?;
            self.dirty = false;
        }
        Ok(())
    }
}