mod binary_parser;
mod chip8;

fn main() {
    let mut chip: chip8::CHIP8 = chip8::CHIP8::new();
    chip.load_binary("/tmp/test");
    chip.print_first_16_bytes_of_ram();
}
