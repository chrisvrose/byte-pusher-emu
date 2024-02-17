use byteorder::{BigEndian, ByteOrder};

pub struct MemoryOperations();

impl MemoryOperations {
    /// read a 24bit endian value, and place it in a u32 num
    pub fn read_big_endian_u24(input: &[u8; 3]) -> u32 {
        BigEndian::read_u24(input)
    }

    pub fn read_big_endian_u16(input: &[u8; 2]) -> u16 {
        BigEndian::read_u16(input)
    }
    /// Write 24-bit endian number into slice
    pub fn write_big_endian_u24(input: u32, output_slice: &mut [u8; 3]) {
        BigEndian::write_u24(output_slice, input);
    }
}
