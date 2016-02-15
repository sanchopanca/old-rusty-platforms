mod binary_parser;
mod chip8;
mod chip8_display;
extern crate rand;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut dummy_display = chip8_display::DummyCHIP8Display::new();
    let mut chip: chip8::CHIP8 = chip8::CHIP8::new(&mut dummy_display);
    chip.load_binary("/tmp/test");
    chip.print_first_16_bytes_of_ram();
    let x: u8 = rand::random();
    println!("{}", x);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 255));
    renderer.clear();
    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}
