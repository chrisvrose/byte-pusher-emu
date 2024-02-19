use std::fmt::{Debug, Formatter};
use sdl2::render::WindowCanvas;
use crate::emu::graphics::GraphicsProcessor;
use crate::graphics::color::Color;
use crate::misc::emulator_error::EmulatorError;
use crate::misc::result::EmulatorResult;


#[derive(Clone)]
pub struct SDLGraphicsAdapter<'a> {
    pub graphics_processor: &'a GraphicsProcessor<'a>
}

impl <'a> Debug for SDLGraphicsAdapter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SDL2 adapter")
    }
}

impl <'a> SDLGraphicsAdapter<'a> {
    pub fn new(graphics_processor: &'a GraphicsProcessor)->SDLGraphicsAdapter<'a>{
        SDLGraphicsAdapter{
            graphics_processor
        }
    }
    pub fn draw(&self, canvas:&mut WindowCanvas) -> EmulatorResult<()> {
        let fb = self.graphics_processor.get_framebuffer();

        let xyc = fb.iter().enumerate().map(|(i,e)| {
            let i = i as u32;
            let y_coord = (i&0xff00 )>> 8;
            let x_coord = i&0x00ff;
            let color = Color::new(*e);
            (x_coord,y_coord,color)
        });
        for (x,y,c) in xyc{
            canvas.set_draw_color(c.get_rgb());
            let coordinates = (x as i32,y as i32);
            let draw_result = canvas.draw_point(coordinates).map_err(|str|EmulatorError::OtherError(str));

            draw_result?;
        }
        Ok(())
    }
}

