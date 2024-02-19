use std::fs::File;
use std::io::Read;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use simple_logger::SimpleLogger;
use crate::emu::cpu::Cpu;
use crate::emu::graphics::GraphicsProcessor;
use crate::emu::ram::{MEM_LENGTH, RamMemory};
use crate::emu::mmu::{Memory};
use crate::graphics::graphics_adapter::SDLGraphicsAdapter;

use crate::misc::result::EmulatorResult;

mod emu;
mod args;
mod misc;
mod graphics;

fn main() -> EmulatorResult<()> {
    let mut fileLoc = File::open("rom.BytePusher").unwrap();
    let mut filebytes = vec![0u8; MEM_LENGTH];
    let x = fileLoc.read(&mut filebytes).unwrap();
    assert!(x < MEM_LENGTH);
    SimpleLogger::new().env().init().unwrap();
    let ram = RamMemory::try_from(filebytes.as_slice())?;

    let (mut canvas,mut event_pump) = initiate_sdl();

    let graphics_processor = GraphicsProcessor::try_new(&ram)?;
    let sdl2_graphics_adapter = SDLGraphicsAdapter::new(&graphics_processor);
    let cpu = Cpu::new(&ram,&graphics_processor);

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                event => {
                    log::info!("Received window event {:?}",event);
                }
            }

        }

        cpu.cycle()?;
        // The rest of the game loop goes here...
        sdl2_graphics_adapter.draw(&mut canvas)?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn initiate_sdl() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 512, 512)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}
