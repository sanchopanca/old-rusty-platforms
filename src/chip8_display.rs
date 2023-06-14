pub trait CHIP8Display {
    fn clear(&mut self);
    fn update(&mut self, vide_memory: &[[u8; 4]; 8]);
}

#[allow(dead_code)]
pub struct DummyCHIP8Display {
    x: i8,
}

impl DummyCHIP8Display {
    pub fn new() -> DummyCHIP8Display {
        DummyCHIP8Display { x: 0 }
    }
}

impl CHIP8Display for DummyCHIP8Display {
    fn clear(&mut self) {
        println!("Clear screen");
    }

    fn update(&mut self, _vide_memory: &[[u8; 4]; 8]) {
        println!("Update screen");
    }
}
