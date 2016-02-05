use std::io::prelude::*;
use std::fs::File;

pub fn parse(file_path: &str) -> Vec<u16> {
    // This code is for loading big-endian binaries
    let result_vector: Vec<u16> = Vec::new();
    let mut f = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return result_vector,
    };
    let mut buffer: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut buffer);
    let mut result_vector: Vec<u16> = Vec::new();
    if buffer.len() % 2 != 0 {
        return result_vector;
    }
    for i in (0..buffer.len()).step_by(2) {
        let mut dword = buffer[i] as u16;
        dword <<= 8;
        dword += buffer[i+1] as u16;
        result_vector.push(dword);
    }
    result_vector
}
