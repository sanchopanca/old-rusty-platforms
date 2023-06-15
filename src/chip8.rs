use crate::binary_parser;
use crate::chip8_display::CHIP8Display;

const MEMORY_SIZE: usize = 0xFFF; // 4KB

#[allow(dead_code)]
pub struct CHIP8<'a> {
    v: [u8; 16],        // V0 - VF registers
    i: u16,             // address register
    stack: [usize; 16], // stack
    sp: usize,          // stack pointer
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; MEMORY_SIZE], // 4 KB of ram
    ca: usize,              // current address
    video_memory: [[u8; 4]; 8],
    display: &'a mut dyn CHIP8Display,
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
            display,
        }
    }
    pub fn load_from_file(&mut self, file_path: &str) {
        binary_parser::load_binary_to_memory(file_path, &mut self.ram[0x200..]).unwrap();
        self.display.clear();
        self.display.update(&self.video_memory)
    }

    #[allow(dead_code)]
    pub fn load_from_memory(&mut self, memory_slice: &[u8]) {
        for (i, byte) in memory_slice.iter().enumerate() {
            self.ram[i + 0x200] = *byte;
        }
    }

    pub fn print_first_16_bytes_of_ram(&self) {
        println!("{:?}", &self.ram[0x200..0x210]);
    }

    #[allow(dead_code)]
    fn execute_opcode(&mut self) {
        let first_nymble = self.ram[self.ca] >> 4;

        self.ca = match first_nymble {
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
                panic!("Unreachable");
            }
        }
    }

    fn not_implemented(&self) {
        println!(
            "Not implemented {:x}{:x}",
            self.ram[self.ca],
            self.ram[self.ca + 1]
        );
    }

    fn warning(&self, message: &str) {
        print!(
            "Illegal instruction {:x}{:x} at {:x} skipped",
            self.ram[self.ca],
            self.ram[self.ca + 1],
            self.ca
        );
        println!("{}", message);
    }

    fn execute_0_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        if second_nymble != 0x0 {
            self.not_implemented();
            return self.ca + 2;
        }
        let second_byte = self.ram[self.ca + 1];
        match second_byte {
            0xE0 => {
                // video
                self.not_implemented();
                self.ca + 2
            }
            0xEE => {
                self.sp -= 1;
                // TODO check stack limits
                // TODO check memory limits
                self.stack[self.sp] + 2
            }
            _ => {
                self.warning("Illegal opcode");
                self.ca + 2
            }
        }
    }

    // uncoditional jump
    fn execute_1_opcode(&mut self) -> usize {
        let second_nymble = (self.ram[self.ca] & 0xF) as usize;
        let second_byte = self.ram[self.ca + 1] as usize;
        let address = second_byte + (second_nymble << 8);
        if address >= MEMORY_SIZE {
            self.warning("Jump ouside of the memory");
            self.ca + 2
        } else if address == self.ca {
            self.warning("Endless loop");
            self.ca + 2
        } else {
            address
        }
    }

    fn execute_2_opcode(&mut self) -> usize {
        let second_nymble = (self.ram[self.ca] & 0xF) as usize;
        let second_byte = self.ram[self.ca + 1] as usize;
        let address = second_byte + (second_nymble << 8);
        self.stack[self.sp] = self.ca;
        // TODO check stack limits
        self.sp += 1;
        address
    }

    // skip if equal
    fn execute_3_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca + 1];
        if self.v[second_nymble as usize] == second_byte {
            self.ca + 4
        } else {
            self.ca + 2
        }
    }

    // skip if not equal
    fn execute_4_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca + 1];
        if self.v[second_nymble as usize] != second_byte {
            self.ca + 4
        } else {
            self.ca + 2
        }
    }

    // skip if two registers are equal
    fn execute_5_opcode(&mut self) -> usize {
        let last_nymble = self.ram[self.ca + 1] & 0xF;
        if last_nymble != 0 {
            self.warning("Illegal opcode");
            return self.ca + 2;
        }
        let x = self.ram[self.ca] & 0xF; // second nymble
        let y = self.ram[self.ca + 1] >> 4; // third nymble
        if self.v[x as usize] == self.v[y as usize] {
            self.ca + 4
        } else {
            self.ca + 2
        }
    }

    // store value in a register
    fn execute_6_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca + 1];
        self.v[second_nymble as usize] = second_byte;
        self.ca + 2
    }

    fn execute_7_opcode(&mut self) -> usize {
        let register = (self.ram[self.ca] & 0xF) as usize;
        let second_byte = self.ram[self.ca + 1];
        let current_value = self.v[register];
        self.v[register] = current_value.wrapping_add(second_byte);
        self.ca + 2
    }

    fn execute_8_opcode(&mut self) -> usize {
        let last_nymble = self.ram[self.ca + 1] & 0xF;
        let x = (self.ram[self.ca] & 0xF) as usize; // second nymble
        let y = (self.ram[self.ca + 1] >> 4) as usize; // third nymble
        match last_nymble {
            0x0 => self.v[x] = self.v[y],
            0x1 => self.v[x] |= self.v[y],
            0x2 => self.v[x] &= self.v[y],
            0x3 => self.v[x] ^= self.v[y],
            0x4 => {
                let sum: u16 = self.v[x] as u16 + self.v[y] as u16;
                if sum & 0xFF00 != 0 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = (sum & 0x00FF) as u8;
            }
            0x5 => {
                if self.v[x] < self.v[y] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            }
            0x6 => {
                self.v[0xF] = self.v[x] & 0b0000_0001;
                self.v[x] >>= 1;
            }
            0x7 => {
                if self.v[y] < self.v[x] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            }
            0xE => {
                self.v[0xF] = (self.v[x] & 0b1000_0000) >> 7;
                self.v[x] <<= 1;
            }
            _ => {
                self.warning("Illegal opcode");
            }
        }
        self.ca + 2
    }

    fn execute_9_opcode(&mut self) -> usize {
        let x = self.ram[self.ca] & 0xF; // second nymble
        let y = self.ram[self.ca + 1] >> 4; // third nymble
        if x != y {
            self.ca + 4
        } else {
            self.ca + 2
        }
    }

    fn execute_a_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        let address: u16 = (second_nymble as u16) << (8 + (self.ram[self.ca + 1] as u16));
        self.i = address;
        self.ca + 2
    }

    fn execute_b_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        let address: usize = (second_nymble as usize) << (8 + (self.ram[self.ca + 1] as usize));
        address + self.v[0] as usize // TODO check that the sum less than max address
    }

    fn execute_c_opcode(&mut self) -> usize {
        let second_nymble = self.ram[self.ca] & 0xF;
        let second_byte = self.ram[self.ca + 1];
        let r: u8 = rand::random();
        self.v[second_nymble as usize] = r & second_byte;
        self.ca + 2
    }

    fn execute_d_opcode(&mut self) -> usize {
        // video
        self.not_implemented();
        self.ca + 2
    }

    fn execute_e_opcode(&mut self) -> usize {
        // keyborad
        self.not_implemented();
        self.ca + 2
    }

    fn execute_f_opcode(&mut self) -> usize {
        let second_byte = self.ram[self.ca + 1];
        let second_nymble = self.ram[self.ca] & 0xF;
        match second_byte {
            0x07 => self.v[second_nymble as usize] = self.delay_timer,
            0x0A => {
                // keyboard
                self.not_implemented();
            }
            0x15 => self.delay_timer = self.v[second_nymble as usize],
            0x18 => self.sound_timer = self.v[second_nymble as usize],
            0x1E => {
                self.i += self.v[second_nymble as usize] as u16;
                if self.i > 0xFFF {
                    self.v[0xF] = 1;
                }
            }
            0x29 => {
                // font
                self.not_implemented();
            }
            0x33 => {
                self.not_implemented();
            }
            0x55 => {
                // TODO check borders
                for x in 0..second_nymble {
                    self.ram[self.i as usize + x as usize] = self.v[x as usize];
                }
            }
            0x65 => {
                // TODO check borders
                for x in 0..second_nymble {
                    self.v[x as usize] = self.ram[self.i as usize + x as usize];
                }
            }
            _ => {
                self.warning("Illegal opcode");
            }
        }
        self.ca + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chip8_display::DummyCHIP8Display;

    #[test]
    fn test_jump() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 1204  -- jump to the address Ox204, which is two instructions down
        // 0000  -- nothing here
        // CC00  -- we should jump here
        chip8.load_from_memory(&[0x12, 0x04, 0x00, 0x00, 0xCC, 0x00]);
        chip8.execute_opcode();
        println!("{:x}", chip8.ca);
        assert_eq!(chip8.ram[chip8.ca], 0xCC);
    }

    #[test]
    fn test_store() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6001  -- store the value 1 in register 0
        // 6102  -- store the value 2 in register 1
        // 6203  -- store the value 3 in register 2
        // 6304  -- store the value 4 in register 3
        // 6405  -- store the value 5 in register 4
        // 6506  -- store the value 6 in register 5
        // 6607  -- store the value 7 in register 6
        // 6708  -- store the value 8 in register 7
        // 6809  -- store the value 9 in register 8
        // 690A  -- store the value 10 in register 9
        // 6A0B  -- store the value 11 in register A
        // 6B0C  -- store the value 12 in register B
        // 6C0D  -- store the value 13 in register C
        // 6D0E  -- store the value 14 in register D
        // 6E0F  -- store the value 15 in register E
        // 6F10  -- store the value 16 in register F

        chip8.load_from_memory(&[
            0x60, 0x01, 0x61, 0x02, 0x62, 0x03, 0x63, 0x04, 0x64, 0x05, 0x65, 0x06, 0x66, 0x07,
            0x67, 0x08, 0x68, 0x09, 0x69, 0x0A, 0x6A, 0x0B, 0x6B, 0x0C, 0x6C, 0x0D, 0x6D, 0x0E,
            0x6E, 0x0F, 0x6F, 0x10,
        ]);

        for i in 0..15 {
            chip8.execute_opcode();
            assert_eq!(chip8.v[i], (i + 1) as u8);
        }
    }

    #[test]
    fn test_skip3() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6001  -- store the value 1 in register 0
        // 3001  -- skip the next instruction if the value in register 0 is equal to 1
        // 0000  -- nothing here
        // 3000  -- we should jump here and do another check
        // 0300  -- we should end up here
        chip8.load_from_memory(&[0x60, 0x01, 0x30, 0x01, 0x00, 0x00, 0x30, 0x00, 0x03, 0x00]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // successful skip
        assert_eq!(chip8.ca, 0x206);

        chip8.execute_opcode(); // unsuccessful skip
        assert_eq!(chip8.ca, 0x208);
        assert_eq!(chip8.ram[chip8.ca], 0x03);
    }

    #[test]
    fn test_skip4() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6D0A  -- store the value A in register D
        // 4D0A  -- skip the next instruction if the value in register D is not equal to A (this condition doesn't hold, so no skip)
        // 400A  -- skip the next instruction if the value in register D is not equal to 0
        // 0000  -- we should skip this instruction
        // 0400  -- we should end up here
        chip8.load_from_memory(&[0x6D, 0x0A, 0x4D, 0x0A, 0x40, 0x0A, 0x00, 0x00, 0x04, 0x00]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // unsuccessful skip
        assert_eq!(chip8.ca, 0x204);

        chip8.execute_opcode(); // successful skip
        assert_eq!(chip8.ca, 0x208);
        assert_eq!(chip8.ram[chip8.ca], 0x04);
    }

    #[test]
    fn test_skip5() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 5AB0  -- skip the following instruction if the value in registers A and B are equal (they are)
        // 0000  -- nothing here
        // 6BFF  -- store FF in register A
        // 5AB0  -- skip the following instruction if the value in registers A and B are equal (they are not)
        // 0500  -- we should end up here
        chip8.load_from_memory(&[0x5A, 0xB0, 0x00, 0x00, 0x6B, 0xFF, 0x5A, 0xB0, 0x05, 0x00]);
        chip8.execute_opcode(); // skip
        assert_eq!(chip8.ca, 0x204);

        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // unsuccessful skip
        assert_eq!(chip8.ca, 0x208);
        assert_eq!(chip8.ram[chip8.ca], 0x05);
    }

    #[test]
    fn test_adding_constant() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6001  -- store the value 1 in register 0
        // 70F0  -- add the value 0xF0 to register 0
        // 7000  -- add 0 to register 0 (it shouldn't chage)
        // 7010  -- add 0x10 to register 0, it should wrap around
        chip8.load_from_memory(&[0x60, 0x01, 0x70, 0xF0, 0x70, 0x00, 0x70, 0x10]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // add
        assert_eq!(chip8.v[0], 0xF1);

        chip8.execute_opcode(); // add 0
        assert_eq!(chip8.v[0], 0xF1);

        chip8.execute_opcode(); // add 0x10
        assert_eq!(chip8.v[0], 0x01);
    }

    #[test]
    fn test_copy_register() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6001  -- store the value 1 in register 0
        // 8100  -- copy v0 to v1
        chip8.load_from_memory(&[0x60, 0x01, 0x81, 0x00]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // copy
        assert_eq!(chip8.v[1], 0x01);
    }

    #[test]
    fn test_or() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 605F  -- store 0b0101_1111 in v0
        // 6FAA  -- store 0b1010_1010 in vF
        // 80F1  -- v0 |= vF
        chip8.load_from_memory(&[0x60, 0x5F, 0x6F, 0xAA, 0x80, 0xF1]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        assert_eq!(chip8.v[0x0], 0b0101_1111);
        assert_eq!(chip8.v[0xF], 0b1010_1010);

        chip8.execute_opcode(); // or
        assert_eq!(chip8.v[0x0], 0b1111_1111); // changed
        assert_eq!(chip8.v[0xF], 0b1010_1010); // unchanged
    }

    #[test]
    fn test_and() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6055  -- store 0b0101_0101 in v0
        // 6FAA  -- store 0b1010_1010 in vF
        // 80F2  -- v0 &= vF
        chip8.load_from_memory(&[0x60, 0x55, 0x6F, 0xAA, 0x80, 0xF2]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        assert_eq!(chip8.v[0x0], 0b0101_0101);
        assert_eq!(chip8.v[0xF], 0b1010_1010);

        chip8.execute_opcode(); // or
        assert_eq!(chip8.v[0x0], 0b0000_0000); // changed
        assert_eq!(chip8.v[0xF], 0b1010_1010); // unchanged
    }

    #[test]
    fn test_xor() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 605D  -- store 0b0101_1101 in v0
        // 6FAA  -- store 0b1010_1010 in vF
        // 80F3  -- v0 ^= vF
        chip8.load_from_memory(&[0x60, 0x5D, 0x6F, 0xAA, 0x80, 0xF3]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        assert_eq!(chip8.v[0x0], 0b0101_1101);
        assert_eq!(chip8.v[0xF], 0b1010_1010);

        chip8.execute_opcode(); // or
        assert_eq!(chip8.v[0x0], 0b1111_0111); // changed
        assert_eq!(chip8.v[0xF], 0b1010_1010); // unchanged
    }

    #[test]
    fn test_add() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 6CF0  -- store the value F0 in vC
        // 6D01  -- store the value 1 to vD
        // 6E10  -- store the value 10 in vD
        // 6F33  -- store the value 33 in vF
        // 8CD4  -- vC += vD
        // 8EC4  -- vE += vC
        chip8.load_from_memory(&[
            0x6C, 0xF0, 0x6D, 0x01, 0x6E, 0x10, 0x6F, 0x33, 0x8C, 0xD4, 0x8E, 0xC4,
        ]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        assert_eq!(chip8.v[0xF], 0x33); // it is about to change, so let's check if our write worked

        chip8.execute_opcode(); // add 1
        assert_eq!(chip8.v[0xC], 0xF1); // changed
        assert_eq!(chip8.v[0xD], 0x01); // unchanged
        assert_eq!(chip8.v[0xF], 0x00); // carry is 0

        chip8.execute_opcode(); // add 10
        assert_eq!(chip8.v[0xC], 0xF1); // unchanged
        assert_eq!(chip8.v[0xE], 0x01); // overflow
        assert_eq!(chip8.v[0xF], 0x01); // carry is 1
    }

    #[test]
    fn test_sub() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 650A  -- store the value A in v5
        // 6609  -- store the value 9 in v6
        // 6702  -- store the value 2 in v7
        // 6F66  -- store the value 66 in vF
        // 8565  -- v5 -= v6
        // 8575  -- v5 -= v7
        chip8.load_from_memory(&[
            0x65, 0x0A, 0x66, 0x09, 0x67, 0x02, 0x6F, 0x66, 0x85, 0x65, 0x85, 0x75,
        ]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        assert_eq!(chip8.v[0xF], 0x66); // it is about to change, so let's check if our write worked

        chip8.execute_opcode(); // sub 9
        assert_eq!(chip8.v[0x5], 0x01); // changed
        assert_eq!(chip8.v[0x6], 0x09); // unchanged
        assert_eq!(chip8.v[0xF], 0x01); // no borrow

        chip8.execute_opcode(); // sub 2
        assert_eq!(chip8.v[0x5], 0xFF); // undereflow
        assert_eq!(chip8.v[0xF], 0x00); // borrow
    }

    #[test]
    fn test_other_sub() {
        let mut display = DummyCHIP8Display::new();
        let mut chip8 = CHIP8::new(&mut display);

        // 650A  -- store the value A in v5
        // 6609  -- store the value 9 in v6
        // 6702  -- store the value 2 in v7
        // 6F66  -- store the value 66 in vF
        // 8567  -- v5 = v6 - v5
        // 8757  -- v7 = v5 - v7
        chip8.load_from_memory(&[
            0x65, 0x0A, 0x66, 0x09, 0x67, 0x02, 0x6F, 0x66, 0x85, 0x67, 0x87, 0x57,
        ]);
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        chip8.execute_opcode(); // store
        assert_eq!(chip8.v[0xF], 0x66); // it is about to change, so let's check if our write worked

        chip8.execute_opcode(); // sub 9
        assert_eq!(chip8.v[0x5], 0xFF); // underflow
        assert_eq!(chip8.v[0x6], 0x09); // unchanged
        assert_eq!(chip8.v[0xF], 0x00); // borrow

        chip8.execute_opcode(); // sub 2
        assert_eq!(chip8.v[0x7], 0xFD); // changed
        assert_eq!(chip8.v[0xF], 0x01); // no borrow
    }
}
