use binary_parser;

pub struct CHIP8 {
    V: [u8; 16],  // V0 - VF registers
    I: u16,
    delay_timer: u8,
    sound_timer: u8,
    RAM: [u8; 0xFFF],  // 4 KB of RAM
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            V: [0; 16],
            I: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            RAM: [0; 0xFFF],
        }
    }
    pub fn load_binary(&mut self, file_path: &str) {
        binary_parser::load_binary_to_memory(file_path, &mut self.RAM[0x200..]);
    }

    pub fn print_first_16_bytes_of_ram(&self) {
        println!("{:?}", &self.RAM[0x200..0x210]);
    }
}
