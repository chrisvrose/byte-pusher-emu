use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use simple_logger::SimpleLogger;
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
    let mut program_counter = ProgramCounter::new();
    let mut mmio = MemoryMappedIO::new(&mut program_counter);
    let mut ram = RamMemory::try_new()?;
    let mut mmu = MappedMemory::new(&mut mmio,&mut ram);
    // for i in 0..10 {
    //     log::info!("Memory at {} is {}",i,mmu.try_get_byte(i)?);
    // }
    mmu.try_set_byte(0x2,0x1)?;
    let data = program_counter.try_get_byte(0x0)?;
    log::info!("Computed data {}",data);


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
