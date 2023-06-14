use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod binary_parser;
mod chip8;
mod chip8_display;

fn main() {
    let mut dummy_display = chip8_display::DummyCHIP8Display::new();
    let mut chip: chip8::CHIP8 = chip8::CHIP8::new(&mut dummy_display);
    chip.load_binary("Cargo.toml");
    chip.print_first_16_bytes_of_ram();
    let x: u8 = rand::random();
    println!("{}", x);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(400.0, 300.0);
        let scaled_size = LogicalSize::new(400.0 * 3.0, 300.0 * 3.0);
        WindowBuilder::new()
            .with_title("Rusty Platforms - CHIP-8")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(400, 300, surface_texture).unwrap()
    };

    pixels.clear_color(Color::BLUE);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
