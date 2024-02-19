
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

    /// wrap to black if needed
    pub fn new(in_mem_color: u8) -> Color {
        if in_mem_color > Self::COLOR_MAX {
            Color(0)
        } else {
            Color(in_mem_color)
        }
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
        let color = Color::new(0);
        assert_eq!((0, 0, 0), color.get_rgb())
    }

    #[test]
    pub fn test_from_mem_invalid() {
        let color = Color::new(0xff);
        assert_eq!((0,0,0),color.get_rgb())
    }

    #[test]
    pub fn test_from_mem_max() {
        let color = Color::new(Color::COLOR_MAX);
        assert_eq!((255, 255, 255), color.get_rgb())
    }

}