use binary_parser;

const MEMORY_SIZE: usize = 0xFFF; // 4KB

pub struct CHIP8 {
    V: [u8; 16],  // V0 - VF registers
    I: usize,  // FIXME I should be 2 bytes long
    delay_timer: u8,
    sound_timer: u8,
    RAM: [u8; MEMORY_SIZE],  // 4 KB of RAM
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

    fn execute_opcode(&mut self) {
        let first_nymble = self.RAM[self.I] >> 4;

        match first_nymble {
            // each of the following functions
            // MUST move `I` pointer
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
            0xA => self.execute_A_opcode(),
            0xB => self.execute_B_opcode(),
            0xC => self.execute_C_opcode(),
            0xD => self.execute_D_opcode(),
            0xE => self.execute_E_opcode(),
            0xF => self.execute_F_opcode(),
            _ => {
                self.not_implemented();
                self.I += 2;
            },
        }
    }

    fn not_implemented(&self) {
        println!("Not implemented {:x}{:x}", self.RAM[self.I], self.RAM[self.I+1]);
    }

    fn execute_0_opcode(&mut self) {
        let second_nymble = self.RAM[self.I] & 0xF;
        if second_nymble != 0x0 {
            self.not_implemented();
            self.I += 2;
            return;
        }
        let second_byte = self.RAM[self.I+1];
        match second_byte {
            0xE0 => {
                println!("Clearing the screen");
                self.I += 2;
            },
            0xEE => {
                println!("Returning from a subroutine");
                self.I += 2;
            },
            _ => {
                self.not_implemented();
                self.I += 2;
            }
        }
    }
    fn execute_1_opcode(&mut self) {
        let second_nymble = self.RAM[self.I] & 0xF;
        let second_byte = self.RAM[self.I+1];
        let address: u16 = second_byte as u16 + (second_nymble as u16) << 12;
        if address as usize >= MEMORY_SIZE {
            println!("Illegal jump");
            self.I += 2;
            return;
        }
        self.I += address as usize;
    }

    fn execute_2_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_3_opcode(&mut self) {
        let second_nymble = self.RAM[self.I] & 0xF;
        let second_byte = self.RAM[self.I+1];
        if self.V[second_nymble as usize] == second_byte {
            self.I += 4
        } else {
            self.I += 2;
        }
    }

    fn execute_4_opcode(&mut self) {
        let second_nymble = self.RAM[self.I] & 0xF;
        let second_byte = self.RAM[self.I+1];
        if self.V[second_nymble as usize] != second_byte {
            self.I += 4
        } else {
            self.I += 2;
        }
    }

    fn execute_5_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_6_opcode(&mut self) {
        let second_nymble = self.RAM[self.I] & 0xF;
        let second_byte = self.RAM[self.I+1];
        self.V[second_nymble as usize] = second_byte;
        self.I += 2;
    }

    fn execute_7_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }
    fn execute_8_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_9_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_A_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_B_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_C_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_D_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_E_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

    fn execute_F_opcode(&mut self) {
        self.not_implemented();
        self.I += 2;
    }

}
