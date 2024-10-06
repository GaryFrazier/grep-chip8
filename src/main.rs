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
use sdl2::rect;
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
    let window = video_subsystem.window("Grep Chip8", 64 * 10, 32 * 10)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
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
        
        // testing at 10 cycles per frame (600hz)

        for _ in 0..1 {
            cpu::execute_next_instruction(&mut emulator);
        }
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // draw current state to frame
        for i in 0..64 {
            for j in 0..32 {
                if emulator.display_memory[(j * 64) + i] {
                    canvas.fill_rect(rect::Rect::new(i as i32 * 10, j as i32 * 10, 10, 10)).expect("couldnt draw rect :(");
                }
            }
        }

        // to do: sound
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
