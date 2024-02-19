use std::cell::Ref;
use std::fmt::{Debug, Formatter};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{TextureAccess, WindowCanvas};
use crate::emu::graphics::{DEVICE_FRAMEBUFFER_SIZE, GraphicsProcessor};
use crate::graphics::color::Color;
use crate::misc::result::EmulatorResult;


#[derive(Clone)]
pub struct SDLGraphicsAdapter<'a> {
    color_fb: Box<[u8; DEVICE_FRAMEBUFFER_SIZE * 3]>,
    graphics_processor: &'a GraphicsProcessor<'a>,
}

impl<'a> Debug for SDLGraphicsAdapter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SDL2 adapter")
    }
}

impl<'a> SDLGraphicsAdapter<'a> {
    pub fn new(graphics_processor: &'a GraphicsProcessor) -> SDLGraphicsAdapter<'a> {
        let color_fb_vec = vec![0u8; DEVICE_FRAMEBUFFER_SIZE * 3].into_boxed_slice().try_into().expect("???");
        SDLGraphicsAdapter {
            color_fb: color_fb_vec,
            graphics_processor,
        }
    }
    pub fn draw(&mut self, canvas: &mut WindowCanvas) -> EmulatorResult<()> {
        let fb = self.graphics_processor.get_framebuffer();
        self.fill_my_texture(fb);


        let texture_creator = canvas.texture_creator();

        let mut texture = texture_creator.create_texture(PixelFormatEnum::RGB24, TextureAccess::Streaming, 256, 256).expect("Failed to make texture");
        texture.with_lock(None, |f, _i| {
            f.copy_from_slice(self.color_fb.as_ref())
        }).expect("TODO: panic message");
        canvas.copy(&texture, None, None).expect("Failed to write texture");

        Ok(())
    }
    fn fill_my_texture(&mut self, dev_fb_ref: Ref<Box<[u8; DEVICE_FRAMEBUFFER_SIZE]>>) {
        for (i, e) in dev_fb_ref.iter().enumerate() {
            let color = Color::new(*e).get_rgb();
            self.color_fb[3 * i] = color.0;
            self.color_fb[3 * i + 1] = color.1;
            self.color_fb[3 * i + 2] = color.2;
        }
    }
}



