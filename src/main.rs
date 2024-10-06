pub mod tests;
pub mod cpu;
pub mod display;
pub mod emulator;
pub mod hex_util;

use std::env;
use std::fs;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let mut emulator = emulator::Emulator::default();

    let args: Vec<String> = env::args().collect();
    let metadata = fs::metadata(&args[1]).expect("unable to read metadata");
    let len = (metadata.len() as usize) + 0x200;
    emulator.ram[0x200..len].copy_from_slice(&fs::read(&args[1]).expect("unable to read file"));

    // window init
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Grep Chip8", 64, 32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    // to do: sound
    loop {

    }
}
