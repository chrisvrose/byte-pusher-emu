use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use simple_logger::SimpleLogger;
use crate::emu::cpu::Cpu;
use crate::emu::iomem::MemoryMappedIO;
use crate::emu::ram::RamMemory;
use crate::emu::mmu::{Memory, MappedMemory};
use crate::emu::program_counter::ProgramCounter;

use crate::misc::result::EmulatorResult;

mod emu;
mod args;
mod misc;
mod graphics;

fn main() -> EmulatorResult<()> {
    SimpleLogger::new().env().init().unwrap();
    let program_counter = ProgramCounter::new();
    let mmio = MemoryMappedIO::new(&program_counter);
    let ram = RamMemory::try_new()?;
    let mmu = MappedMemory::new(&mmio,&ram);

    let mut cpu = Cpu::new(&mmu,&program_counter);
    for i in 0..1{
        cpu.cycle();
    }

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    //
    // let window = video_subsystem.window("rust-sdl2 demo", 512, 512)
    //     .position_centered()
    //     .build()
    //     .unwrap();
    // let mut canvas = window.into_canvas().build().unwrap();
    //
    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             event => {
    //                 log::info!("Received window event {:?}",event);
    //             }
    //         }
    //     }
    //     // The rest of the game loop goes here...
    //
    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }

    Ok(())
}
