use rand;

use binary_parser;
use chip8_display::CHIP8Display;

const MEMORY_SIZE: usize = 0xFFF; // 4KB

pub struct CHIP8<'a> {
    v: [u8; 16],  // V0 - VF registers
    i: u16,  // address register
    stack: [usize; 16], // stack
    sp: usize, // stack pointer
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; MEMORY_SIZE],  // 4 KB of ram
    ca: usize, // current address
    video_memory: [[u8; 4]; 8],
    display: &'a mut CHIP8Display,
}

impl<'a> CHIP8<'a> {
    pub fn new<T: CHIP8Display>(display: &'a mut T) -> CHIP8 {
        CHIP8 {
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            ram: [0; 0xFFF],
            ca: 0x200,
            video_memory: [[0; 4]; 8],
            display: display,
        }
    }
    pub fn load_binary(&mut self, file_path: &str) {
        binary_parser::load_binary_to_memory(file_path, &mut self.ram[0x200..]);
        self.display.clear();
        self.display.update(&self.video_memory)
    }

    pub fn print_first_16_bytes_of_ram(&self) {
        println!("{:?}", &self.ram[0x200..0x210]);
    }

    fn execute_opcode(&mut self) {
        let first_nymble = self.ram[self.ca] >> 4;

        match first_nymble {
            // each of the following functions
            // MUST move `ca` pointer
            0x0 => self.execute_0_opcode(),
            0x1 => self.execute_1_opcode(),
            0x2 => self.execute_2_opcode(),
            0x3 => self.execute_3_opcode(),
            0x4 => self.execute_4_opcode(),
            0x5 => self.execute_5_opcode(),
            0x6 => self.execute_6_opcode(),
            0x7 => self.execute_7_opcode(),
            0x8 => self.execute_8_opcode(),
            0x9 => self.execute_9_opcode(),
            0xA => self.execute_a_opcode(),
            0xB => self.execute_b_opcode(),
            0xC => self.execute_c_opcode(),
            0xD => self.execute_d_opcode(),
            0xE => self.execute_e_opcode(),
            0xF => self.execute_f_opcode(),
            _ => {
                self.warning("Illegal opcode");
                self.ca += 2;
            },
        }
    }

    fn not_implemented(&self) {
        println!("Not implemented {:x}{:x}", self.ram[self.ca], self.ram[self.ca+1]);
    }

    fn warning(&self, message: &str) {
        print!("Illegal instruction {:x}{:x} at {:x} skipped",
               self.ram[self.ca], self.ram[self.ca+1], self.ca);
        println!("{}", message);
    }

    fn execute_0_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        if second_nymble != 0x0 {
            self.not_implemented();
            self.ca += 2;
            return;
        }
        let second_byte = self.ram[self.ca+1];
        match second_byte {
            0xE0 => {
                // video
                self.not_implemented();
                self.ca += 2;
            },
            0xEE => {
                self.sp -= 1;
                // TODO check stack limits
                // TODO check memory limits
                self.ca = self.stack[self.sp] + 2;
            },
            _ => {
                self.warning("Illegal opcode");
                self.ca += 2;
            }
        }
    }
    fn execute_1_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        let address: usize = second_byte as usize + (second_nymble as usize) << 8;
        if address >= MEMORY_SIZE {
            self.warning("Jump ouside of the memory");
            self.ca += 2;
        } else if address == self.ca {
            self.warning("Endless loop");
            self.ca += 2;
        } else {
            self.ca = address;
        }
    }

    fn execute_2_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        let address: usize = second_byte as usize + (second_nymble as usize) << 8;
        self.stack[self.sp] = self.ca;
        // TODO check stack limits
        self.sp += 1;
        self.ca = address;
    }

    fn execute_3_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        if self.v[second_nymble as usize] == second_byte {
            self.ca += 4
        } else {
            self.ca += 2;
        }
    }

    fn execute_4_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        if self.v[second_nymble as usize] != second_byte {
            self.ca += 4
        } else {
            self.ca += 2;
        }
    }

    fn execute_5_opcode(&mut self) {
        let last_nymble = self.ram[self.ca+1] & 0xF;
        if last_nymble != 0 {
            self.warning("Illegal opcode");
            self.ca += 2;
            return;
        }
        let x = self.ram[self.ca] & 0xF; // second nymble
        let y = self.ram[self.ca+1] >> 4; // third nymble
        if x == y {
            self.ca += 4;
        } else {
            self.ca += 2;
        }
    }

    fn execute_6_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        self.v[second_nymble as usize] = second_byte;
        self.ca += 2;
    }

    fn execute_7_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        self.v[second_nymble as usize] = second_byte;
        self.ca += 2;
    }
    fn execute_8_opcode(&mut self) {
        let last_nymble = self.ram[self.ca+1] & 0xF;
        let x = (self.ram[self.ca] & 0xF) as usize; // second nymble
        let y = (self.ram[self.ca+1] >> 4) as usize; // third nymble
        match last_nymble {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] |= self.v[y], // TODO check bit or logic
            0x2 => self.v[x] &= self.v[y], // TODO check bit or logic
            0x3 => self.v[x] ^= self.v[y], // TODO check bit or logic
            0x4 => {
                let sum: u16 = self.v[x] as u16 + self.v[y] as u16;
                if sum & 0xFF00 != 0 {
                    self.v[0xF] = 1;
                }
                self.v[x] = (sum & 0x00FF) as u8;
            },
            0x5 => {
                if self.v[x] < self.v[y] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }
                self.v[x] -= self.v[y];
            },
            0x6 => {
                self.v[0xF] = self.v[x] & 0b0000_0001;
                self.v[x] >>= 1;
            },
            0x7 => {
                if self.v[y] < self.v[x] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }
                self.v[x] = self.v[y] - self.v[x];
            },
            0xE => {
                self.v[0xF] = (self.v[x] & 0b1000_0000) >> 7;
                self.v[x] <<= 1;
            },
            _ => {
                self.warning("Illegal opcode");
            }
        }
        self.ca += 2;
    }

    fn execute_9_opcode(&mut self) {
        let x = self.ram[self.ca] & 0xF; // second nymble
        let y = self.ram[self.ca+1] >> 4; // third nymble
        if x != y {
            self.ca += 4;
        } else {
            self.ca += 2;
        }
    }

    fn execute_a_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let address: u16 = (second_nymble as u16) << 8 + (self.ram[self.ca+1] as u16);
        self.i = address;
    }

    fn execute_b_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let address: usize = (second_nymble as usize) << 8 + (self.ram[self.ca+1] as usize);
        self.ca = address + self.v[0] as usize; // TODO check that the sum less than max address
    }

    fn execute_c_opcode(&mut self) {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca+1];
        let r: u8 = rand::random();
        self.v[second_nymble as usize] = r & second_byte;
        self.ca += 2;
    }

    fn execute_d_opcode(&mut self) {
        // video
        self.not_implemented();
        self.ca += 2;
    }

    fn execute_e_opcode(&mut self) {
        // keyborad
        self.not_implemented();
        self.ca += 2;
    }

    fn execute_f_opcode(&mut self) {
        let second_byte = self.ram[self.ca+1];
        let second_nymble = self.ram[self.ca] & 0xF;
        match second_byte {
            0x07 => self.v[second_nymble as usize] = self.delay_timer,
            0x0A => {
                // keyboard
                self.not_implemented();
            },
            0x15 => self.delay_timer = self.v[second_nymble as usize],
            0x18 => self.sound_timer = self.v[second_nymble as usize],
            0x1E => {
                self.i += self.v[second_nymble as usize] as u16;
                if self.i > 0xFFF {
                    self.v[0xF] = 1;
                }
            },
            0x29 => {
                // font
                self.not_implemented();
            },
            0x33 => {
                self.not_implemented();
            },
            0x55 => {
                // TODO check borders
                for x in 0..second_nymble {
                    self.ram[self.i as usize + x as usize] = self.v[x as usize];
                }
            },
            0x65 => {
                // TODO check borders
                for x in 0..second_nymble {
                    self.v[x as usize] = self.ram[self.i as usize + x as usize];
                }
            },
            _ => {
                self.warning("Illegal opcode");
            }
        }
        self.ca += 2;
    }

}
