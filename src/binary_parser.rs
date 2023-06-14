use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

pub fn load_binary_to_memory(file_path: &str, memory_slice: &mut [u8]) -> Result<(), Error> {
    let metadata = fs::metadata(file_path)?;
    let binary_size: u64 = metadata.len();
    let memory_size = memory_slice.len() as u64;
    if memory_size < binary_size {
        let file_to_big = Error::new(ErrorKind::Other, "Binary file is too big");
        return Err(file_to_big);
    }
    let mut f = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut buffer);
    for (i, byte) in buffer.iter().enumerate() {
        memory_slice[i] = *byte
    }
    Ok(())
}
