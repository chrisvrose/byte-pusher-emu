use crate::emu::graphics::GraphicsProcessor;
use crate::emu::memory::{Memory, RamMemory};
use crate::misc::endian::{read_big_endian_u24, write_big_endian_u24};
use crate::misc::result::EmulatorResult;

#[derive(Debug)]
pub enum CpuState{
    Running,
    Paused
}

#[derive(Debug)]
pub struct Cpu<'a>{
    pub memory: &'a RamMemory,
    pub graphics_processor: &'a GraphicsProcessor<'a>
}

impl <'a> Cpu<'a>{
    const PC_START:usize = 2;
    const PC_LEN:usize = 3;
    const PC_ZERO:[u8;3] = [0;3];
    pub fn new(memory: &'a RamMemory, graphics_processor: &'a GraphicsProcessor<'a>) ->Cpu<'a>{
        Cpu{
            graphics_processor,
            memory
        }
    }
    pub fn get_pc(&self)->u32{
        let memory_slice = self.memory.data.borrow();
        let data = memory_slice.get(Self::PC_START..(Self::PC_START+Self::PC_LEN)).unwrap();
        read_big_endian_u24(data.try_into().unwrap())
    }
    pub fn set_pc(&self,address:u32){
        let mut memory_slice = self.memory.data.borrow_mut();

        let mut pc_big_endian_slice = Self::PC_ZERO;
        write_big_endian_u24(address,&mut pc_big_endian_slice);

        memory_slice[Self::PC_START..(Self::PC_START + Self::PC_LEN)].copy_from_slice(&pc_big_endian_slice);
    }

    pub fn cycle(&self)->EmulatorResult<()>{
        for _i in 0..65536{
            let address_to_execute = self.get_pc();

            //execute p1
            self.copy_u24(address_to_execute)?;
            //execute p2
            let new_pc_location = address_to_execute+2*(Self::PC_LEN as u32) ;

            let new_pc = self.memory.try_get_u24(new_pc_location)?;

            self.set_pc(new_pc);
        }
        log::debug!("Finished internal loop");
        self.graphics_processor.draw()?;
        // TODO send audio
        Ok(())
    }

    fn copy_u24(&self, address_to_execute: u32) -> EmulatorResult<()> {
        let aloc = self.memory.try_get_u24(address_to_execute)?;
        let bloc = self.memory.try_get_u24(address_to_execute+Self::PC_LEN as u32)?;

        self.memory.try_set_byte(bloc,self.memory.try_get_byte(aloc)?)
    }


    fn fetch_triplet(&self, address: u32) -> EmulatorResult<[u8;3]> {
        let first_byte = self.memory.try_get_byte(address)?;
        let second_byte = self.memory.try_get_byte(address+1)?;
        let third_byte = self.memory.try_get_byte(address+2)?;
        let num = [first_byte,second_byte,third_byte];
        Ok(num)
    }
    fn set_triplet(&self, address: u32, val:&[u8;3]) -> EmulatorResult<()> {
        self.memory.try_set_byte(address, val[0])?;
        self.memory.try_set_byte(address + 1, val[1])?;
        self.memory.try_set_byte(address + 2, val[2])?;
        Ok(())
    }
}



#[cfg(test)]
mod test{

    #[test]
    pub fn construct(){

    }

}