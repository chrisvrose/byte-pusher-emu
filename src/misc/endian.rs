use byteorder::{BigEndian, ByteOrder};

pub fn read_big_endian_u24(input: &[u8; 3]) -> u32 {
    BigEndian::read_u24(input)
}

pub fn read_big_endian_u16(input: &[u8; 2]) -> u16 {
    BigEndian::read_u16(input)
}

pub fn write_big_endian_u16(input: u16, output_slice: &mut [u8; 2]) {
    BigEndian::write_u16(output_slice, input);
}
