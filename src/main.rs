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
use crate::emu::memory::{MEM_LENGTH, RamMemory};
use crate::graphics::graphics_adapter::SDLGraphicsAdapter;

use crate::misc::result::EmulatorResult;

mod emu;
mod args;
mod misc;
mod graphics;

fn main() -> EmulatorResult<()> {
    let (file_bytes, x) = try_load_rom()?;
    assert!(x < MEM_LENGTH);
    SimpleLogger::new().env().init().unwrap();
    let ram = RamMemory::try_from(file_bytes.as_slice())?;

    let (mut canvas, mut event_pump, draw_factor) = initiate_sdl();

    let graphics_processor = GraphicsProcessor::try_new(&ram)?;
    let sdl2_graphics_adapter = SDLGraphicsAdapter::new(&graphics_processor);
    let cpu = Cpu::new(&ram, &graphics_processor);

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },

                event => {
                    log::trace!("Received window event {:?}",event);
                }
            }
        }

        // The rest of the game loop goes here...
        cpu.cycle()?;
        // draw graphics
        sdl2_graphics_adapter.draw(&mut canvas, draw_factor)?;
        // TODO render audio

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn try_load_rom() -> EmulatorResult<(Vec<u8>, usize)> {
    let mut file_bytes = vec![0u8; MEM_LENGTH];

    let mut file_handle = File::open("rom.BytePusher")?;
    let x = file_handle.read(&mut file_bytes)?;

    Ok((file_bytes, x))
}

fn initiate_sdl() -> (WindowCanvas, EventPump, u32) {
    const BASE_RESOLUTION: u32 = 256;
    const DRAW_FACTOR: u32 = 4;
    const WINDOW_RESOLUTION: u32 = BASE_RESOLUTION * DRAW_FACTOR;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("byte-pusher-emu", WINDOW_RESOLUTION, WINDOW_RESOLUTION)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump, DRAW_FACTOR)
}
