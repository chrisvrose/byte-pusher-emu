use byteorder::BigEndian;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::endian::MemoryOperations;
use crate::misc::result::EmulatorResult;

pub struct Color(u8);

const RED_MULT: u8 = 36;
const GREEN_MULT: u8 = 6;

impl Color {
    /// Only first 216 color indices are used.
    const COLOR_MAX:u8 = 215;
    const COLOR_FACTOR_8_BIT: u8 = 0x33;
    /// This constructs a valid color from rgb triplet
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        let red = red / Self::COLOR_FACTOR_8_BIT;
        let green = green / Self::COLOR_FACTOR_8_BIT;
        let blue = blue / Self::COLOR_FACTOR_8_BIT;
        Color(red * RED_MULT + green * GREEN_MULT + blue)
    }
    pub fn try_new(in_mem_color: u8) -> EmulatorResult<Color> {
        if in_mem_color > Self::COLOR_MAX {
            return Err(EmulatorError::InvalidColorError(in_mem_color))
        }
        Ok(Color(in_mem_color))
    }
    pub fn get_mem_byte(&self) -> u8 {
        self.0
    }
    /// This fetches the rgb triplet
    pub fn get_rgb(self) -> [u8; 3] {
        let r = self.0 / RED_MULT;
        let gb_byte_remainder = self.0 % RED_MULT;
        let g = gb_byte_remainder / GREEN_MULT;
        let b = gb_byte_remainder % GREEN_MULT;
        [r * Self::COLOR_FACTOR_8_BIT, g * Self::COLOR_FACTOR_8_BIT, b * Self::COLOR_FACTOR_8_BIT]
    }
}


#[cfg(test)]
mod tests {
    use crate::graphics::color::Color;

    #[test]
    pub fn test_from_mem_zero() {
        let color = Color::try_new(0).unwrap();
        assert_eq!([0u8; 3], color.get_rgb())
    }

    #[test]
    pub fn test_from_mem_invalid() {
        let color = Color::try_new(0xff);
        assert!(color.is_err())
    }
    #[test]
    pub fn test_from_mem_max(){
        let color = Color::try_new(Color::COLOR_MAX).unwrap();
        assert_eq!([255u8;3],color.get_rgb())
    }

    #[test]
    pub fn from_rgb_zero(){
        let color = Color::from_rgb(0,0,0);
        assert_eq!(0,color.get_mem_byte())
    }
    #[test]
    pub fn from_rgb_max(){
        let color = Color::from_rgb(255,255,255);
        assert_eq!(Color::COLOR_MAX,color.get_mem_byte())
    }

}