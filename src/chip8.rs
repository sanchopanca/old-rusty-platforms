use rand;

use binary_parser;

const MEMORY_SIZE: usize = 0xFFF; // 4KB

pub struct CHIP8 {
    v: [u8; 16],  // V0 - VF registers
    i: usize,  // FIXME I should be 2 bytes long
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; MEMORY_SIZE],  // 4 KB of ram
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            v: [0; 16],
            i: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            ram: [0; 0xFFF],
        }
    }
    pub fn load_binary(&mut self, file_path: &str) {
        binary_parser::load_binary_to_memory(file_path, &mut self.ram[0x200..]);
    }

    pub fn print_first_16_bytes_of_ram(&self) {
        println!("{:?}", &self.ram[0x200..0x210]);
    }

    fn execute_opcode(&mut self) {
        let first_nymble = self.ram[self.i] >> 4;

        match first_nymble {
            // each of the following functions
            // MUST move `i` pointer
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
                self.not_implemented();
                self.i += 2;
            },
        }
    }

    fn not_implemented(&self) {
        println!("Not implemented {:x}{:x}", self.ram[self.i], self.ram[self.i+1]);
    }

    fn warning(&self, message: &str) {
        print!("Illegal instruction {:x}{:x} at {:x} skipped",
               self.ram[self.i], self.ram[self.i+1], self.i);
        println!("{}", message);
    }

    fn execute_0_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        if second_nymble != 0x0 {
            self.not_implemented();
            self.i += 2;
            return;
        }
        let second_byte = self.ram[self.i+1];
        match second_byte {
            0xE0 => {
                self.not_implemented();
                self.i += 2;
            },
            0xEE => {
                self.not_implemented();
                self.i += 2;
            },
            _ => {
                self.not_implemented();
                self.i += 2;
            }
        }
    }
    fn execute_1_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let second_byte = self.ram[self.i+1];
        let address: u16 = second_byte as u16 + (second_nymble as u16) << 12;
        if address as usize >= MEMORY_SIZE {
            self.warning("Jump ouside of the memory");
            self.i += 2;
            return;
        }
        self.i += address as usize;
    }

    fn execute_2_opcode(&mut self) {
        self.not_implemented();
        self.i += 2;
    }

    fn execute_3_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let second_byte = self.ram[self.i+1];
        if self.v[second_nymble as usize] == second_byte {
            self.i += 4
        } else {
            self.i += 2;
        }
    }

    fn execute_4_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let second_byte = self.ram[self.i+1];
        if self.v[second_nymble as usize] != second_byte {
            self.i += 4
        } else {
            self.i += 2;
        }
    }

    fn execute_5_opcode(&mut self) {
        let last_nymble = self.ram[self.i+1] & 0xF;
        if last_nymble != 0 {
            self.warning("Illegal opcode");
            self.i += 2;
            return;
        }
        let x = self.ram[self.i] & 0xF; // second nymble
        let y = self.ram[self.i+1] >> 4; // third nymble
        if x == y {
            self.i += 4;
        } else {
            self.i += 2;
        }
    }

    fn execute_6_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let second_byte = self.ram[self.i+1];
        self.v[second_nymble as usize] = second_byte;
        self.i += 2;
    }

    fn execute_7_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let second_byte = self.ram[self.i+1];
        self.v[second_nymble as usize] = second_byte;
        self.i += 2;
    }
    fn execute_8_opcode(&mut self) {
        let last_nymble = self.ram[self.i+1] & 0xF;
        let x = (self.ram[self.i] & 0xF) as usize; // second nymble
        let y = (self.ram[self.i+1] >> 4) as usize; // third nymble
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
                self.not_implemented();
            }
        }
        self.i += 2;
    }

    fn execute_9_opcode(&mut self) {
        let x = self.ram[self.i] & 0xF; // second nymble
        let y = self.ram[self.i+1] >> 4; // third nymble
        if x != y {
            self.i += 4;
        } else {
            self.i += 2;
        }
    }

    fn execute_a_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let address: usize = (second_nymble as usize) << 8 + (self.ram[self.i+1] as usize);
        self.i = address;
    }

    fn execute_b_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let address: usize = (second_nymble as usize) << 8 + (self.ram[self.i+1] as usize);
        self.i = address + self.v[0] as usize; // TODO check that the sum less than max address
    }

    fn execute_c_opcode(&mut self) {
        let second_nymble = self.ram[self.i] & 0xF;
        let second_byte = self.ram[self.i+1];
        let r: u8 = rand::random();
        self.v[second_nymble as usize] = r & second_byte;
        self.i += 2;
    }

    fn execute_d_opcode(&mut self) {
        self.not_implemented();
        self.i += 2;
    }

    fn execute_e_opcode(&mut self) {
        self.not_implemented();
        self.i += 2;
    }

    fn execute_f_opcode(&mut self) {
        let second_byte = self.ram[self.i+1];
        let second_nymble = self.ram[self.i] & 0xF;
        match second_byte {
            0x07 => self.v[second_nymble as usize] = self.delay_timer,
            0x0A => {
                self.not_implemented();
            },
            0x15 => self.delay_timer = self.v[second_nymble as usize],
            0x18 => self.sound_timer = self.v[second_nymble as usize],
            0x1E =>  {
                self.not_implemented();
            },
            0x29 => {
                self.not_implemented();
            },
            0x33 => {
                self.not_implemented();
            },
            0x55 => {
                self.not_implemented();
            },
            0x65 => {
                self.not_implemented();
            },
            _ => {
                self.not_implemented();
            }
        }
        self.i += 2;
    }

}
