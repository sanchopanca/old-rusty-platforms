struct CHIP8 {
    V: [u8, 16],  // V0 - VF registers
    I: u16,
    delay_timer: u8,
    sound_timer: u8,
    RAM: [u8, 0xFFF],  // 4 KB of RAM
}

impl CHIP8 {
    fn new() {
        CHIP8 {
            V: [0, 16],
            I: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            RAM: [0, 0xFFF],
        }
    }
    // fn load_binary(&self, file_path: &str) {
    //
    // }
}
