use crate::emu::mmu::{MappedMemory, Memory};
use crate::misc::result::EmulatorResult;

#[derive(Debug)]
pub enum CpuState{
    Running,
    Paused
}

#[derive(Debug)]
pub struct Cpu{
    mapped_memory: MappedMemory,
}

impl Cpu{
    pub fn new(mapped_memory: MappedMemory)->Cpu{
        Cpu{
            mapped_memory
        }
    }
    pub fn cycle()->EmulatorResult<()>{
        todo!();
        Ok(())
    }
    fn set_pc(&self,value: &[u8;3])->EmulatorResult<()>{
            todo!()
        // for bytete in value {
        //     self.mapped_memory.try_set_byte()
        // }try_set_byte
    }
}





#[cfg(test)]
mod test{

    #[test]
    pub fn construct(){

    }

}