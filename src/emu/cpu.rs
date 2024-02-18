use crate::emu::mmu::{MappedMemory, Memory};
use crate::emu::program_counter::ProgramCounter;
use crate::misc::emulator_error::{DeviceType, EmulatorError};
use crate::misc::emulator_error::DeviceType::{CPU, PC};
use crate::misc::endian::read_big_endian_u24;
use crate::misc::result::EmulatorResult;

#[derive(Debug)]
pub enum CpuState{
    Running,
    Paused
}

#[derive(Debug)]
pub struct Cpu<'a>{
    mapped_memory:&'a MappedMemory<'a>,
    program_counter: &'a ProgramCounter
}

impl <'a> Cpu<'a>{
    pub fn new(mapped_memory: &'a MappedMemory<'a>,program_counter: &'a ProgramCounter)->Cpu<'a>{
        Cpu{
            mapped_memory,
            program_counter
        }
    }
    pub fn cycle(&self)->EmulatorResult<()>{
        for _i in 0..65536{
            let address_to_execute = self.program_counter.get_pc_value();

            //fetch
            let num1 = self.fetch_triplet(address_to_execute)?;
            self.set_triplet(address_to_execute+3,&num1)?;

            let new_pc = read_big_endian_u24(&self.fetch_triplet(address_to_execute+6)?);
            self.program_counter.set_address(new_pc)?;
        }
        // TODO send graphics
        // TODO send audio
        Ok(())
    }

    fn fetch_triplet(&self, address: u32) -> EmulatorResult<[u8;3]> {
        let first_byte = self.mapped_memory.try_get_byte(address)?;
        let second_byte = self.mapped_memory.try_get_byte(address+1)?;
        let third_byte = self.mapped_memory.try_get_byte(address+2)?;
        let num = [first_byte,second_byte,third_byte];
        Ok(num)
    }
    fn set_triplet(&self, address: u32, val:&[u8;3]) -> EmulatorResult<()> {
        self.mapped_memory.try_set_byte(address, val[0])?;
        self.mapped_memory.try_set_byte(address + 1, val[1])?;
        self.mapped_memory.try_set_byte(address + 2, val[2])?;
        Ok(())
    }
    fn set_pc(&self,value: &[u8;3])->EmulatorResult<()>{
        let x = read_big_endian_u24(value);
        self.program_counter.set_address(x)
    }
}





#[cfg(test)]
mod test{

    #[test]
    pub fn construct(){

    }

}