use crate::misc::error::EmulatorError;
use crate::misc::result::EmulatorResult;

/// A Color as represented in BytePusher.
/// It is a byte that contains a packed 6 digit red, green and blue
#[derive(Debug, Copy, Clone)]
pub struct Color(u8);


impl Color {
    /// Red color offset in the colour byte
    const RED_MULT: u8 = 36;
    /// green color offset in the colour byte
    const GREEN_MULT: u8 = 6;
    /// Only first 216 color indices are used.
    const COLOR_MAX: u8 = 215;
    const COLOR_FACTOR_8_BIT: u8 = 0x33;
    /// This constructs a valid color from rgb triplet
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        let red = red / Self::COLOR_FACTOR_8_BIT;
        let green = green / Self::COLOR_FACTOR_8_BIT;
        let blue = blue / Self::COLOR_FACTOR_8_BIT;
        Color(red * Self::RED_MULT + green * Self::GREEN_MULT + blue)
    }
    pub fn try_new(in_mem_color: u8) -> EmulatorResult<Color> {
        if in_mem_color > Self::COLOR_MAX {
            return Err(EmulatorError::InvalidColor(in_mem_color));
        }
        Ok(Color(in_mem_color))
    }
    /// wrap to black if needed
    pub fn new(in_mem_color: u8) -> Color {
        if in_mem_color > Self::COLOR_MAX {
            log::trace!("Invalid color {}, using 0", in_mem_color);
            Color(0)
        } else {
            Color(in_mem_color)
        }
    }
    pub fn get_mem_byte(&self) -> u8 {
        self.0
    }
    /// This fetches the rgb triplet
    pub fn get_rgb(self) -> (u8, u8, u8) {
        let r = self.0 / Self::RED_MULT;
        let gb_byte_remainder = self.0 % Self::RED_MULT;
        let g = gb_byte_remainder / Self::GREEN_MULT;
        let b = gb_byte_remainder % Self::GREEN_MULT;
        (r * Self::COLOR_FACTOR_8_BIT, g * Self::COLOR_FACTOR_8_BIT, b * Self::COLOR_FACTOR_8_BIT)
    }
}


#[cfg(test)]
mod tests {
    use crate::graphics::color::Color;

    #[test]
    pub fn test_from_mem_zero() {
        let color = Color::try_new(0).unwrap();
        assert_eq!((0, 0, 0), color.get_rgb())
    }

    #[test]
    pub fn test_from_mem_invalid() {
        let color = Color::try_new(0xff);
        assert!(color.is_err())
    }

    #[test]
    pub fn test_from_mem_max() {
        let color = Color::try_new(Color::COLOR_MAX).unwrap();
        assert_eq!((255, 255, 255), color.get_rgb())
    }

    #[test]
    pub fn from_rgb_zero() {
        let color = Color::from_rgb(0, 0, 0);
        assert_eq!(0, color.get_mem_byte())
    }

    #[test]
    pub fn from_rgb_max() {
        let color = Color::from_rgb(255, 255, 255);
        assert_eq!(Color::COLOR_MAX, color.get_mem_byte())
    }
}