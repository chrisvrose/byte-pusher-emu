use std::fs::File;
use std::io::Read;
use std::time::Duration;
use clap::Parser;
use log::LevelFilter;
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, WindowCanvas};
use simple_logger::SimpleLogger;
use crate::args::BytePusherArgs;
use crate::emu::audio::AudioProcessor;
use crate::emu::cpu::Cpu;
use crate::emu::graphics::{DEVICE_RESOLUTION, GraphicsProcessor};
use crate::emu::keyboard::Keyboard;
use crate::emu::memory::{MEM_LENGTH, RamMemory};
use crate::graphics::graphics_adapter::SDLGraphicsAdapter;

use crate::misc::result::EmulatorResult;

mod emu;
mod args;
mod misc;
mod graphics;

fn main() -> EmulatorResult<()> {
    let BytePusherArgs { file_name ,draw_scale} = BytePusherArgs::parse();
    SimpleLogger::new().with_level(LevelFilter::Info).env().init()?;

    let (file_bytes, ..) = try_load_rom(&file_name)?;

    let (mut canvas, mut event_pump, audio_queue) = initiate_sdl(draw_scale);


    let ram = RamMemory::try_from(file_bytes.as_slice())?;
    let graphics_processor = GraphicsProcessor::try_new(&ram)?;
    let mut audio_processor = AudioProcessor::try_new(&ram, &audio_queue)?;
    let mut keyboard = Keyboard::new(&ram);
    let cpu = Cpu::new(&ram, &graphics_processor);

    let mut sdl2_graphics_adapter = SDLGraphicsAdapter::new(&graphics_processor);

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(x), repeat: false, .. } => {
                    if let Some(key_val) = get_key_index(x) {
                        keyboard.key_down(key_val)
                    }
                }
                Event::KeyUp { keycode: Some(x), repeat: false, .. } => {
                    if let Some(key_val) = get_key_index(x) {
                        keyboard.key_up(key_val)
                    }
                }
                _ => {}
            }
        }

        keyboard.flush_keyboard()?;

        // The rest of the game loop goes here...
        cpu.cycle()?;

        sdl2_graphics_adapter.draw(&mut canvas)?;
        audio_processor.queue()?;
        canvas.present();


        // 60fps - small offset to consider for cpu cycle time
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60 - 2000_000));
    }

    Ok(())
}

/// get index of key pressed. 0..9+A..F provides a u8
fn get_key_index(p0: Keycode) -> Option<u8> {
    match p0 {
        Keycode::Kp0 => Some(0x0),
        Keycode::Kp1 => Some(0x1),
        Keycode::Kp2 => Some(0x2),
        Keycode::Kp3 => Some(0x3),
        Keycode::Kp4 => Some(0x4),
        Keycode::Kp5 => Some(0x5),
        Keycode::Kp6 => Some(0x6),
        Keycode::Kp7 => Some(0x7),
        Keycode::Kp8 => Some(0x8),
        Keycode::Kp9 => Some(0x9),
        Keycode::A => Some(0xA),
        Keycode::B => Some(0xB),
        Keycode::C => Some(0xC),
        Keycode::D => Some(0xD),
        Keycode::E => Some(0xE),
        Keycode::F => Some(0xF),
        _ => None
    }
}

fn try_load_rom(file_name_option: &Option<String>) -> EmulatorResult<(Vec<u8>, usize)> {
    let mut file_bytes = vec![0u8; MEM_LENGTH];

    let bytes_read = if let Some(file_name) = file_name_option {
        let mut file_handle = File::open(file_name.as_str())?;
        file_handle.read(&mut file_bytes)?
    } else {
        0
    };

    Ok((file_bytes, bytes_read))
}

fn initiate_sdl(draw_scale:f32) -> (WindowCanvas, EventPump, AudioQueue<u8>) {
    let window_resolution: u32 = (DEVICE_RESOLUTION as f32 * draw_scale) as u32;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let wanted_spec = AudioSpecDesired {
        channels: Some(1),
        samples: Some(256),
        freq: Some(15360),
    };
    let audio_queue = audio_subsystem.open_queue::<u8, _>(None, &wanted_spec).unwrap();
    audio_queue.resume();


    let window = video_subsystem.window("byte-pusher-emu", window_resolution, window_resolution)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_scale(draw_scale, draw_scale).expect("Setting scale");

    canvas.set_blend_mode(BlendMode::None);
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump, audio_queue)
}
