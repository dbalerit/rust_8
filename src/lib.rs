use std::error::Error;
use cpu::Cpu;

mod cpu;

pub struct Chip8 {
    cpu: Cpu,
}

impl Chip8 {
    pub fn new(rom_bytes: &[u8]) -> Self {
        Chip8 {
            cpu: Cpu::new(rom_bytes)
        }
    }

    pub fn step(&mut self) -> Result<(), Box<dyn Error>> {
        let opcode = self.cpu.fetch();
        self.cpu.execute(opcode)
    }
}
