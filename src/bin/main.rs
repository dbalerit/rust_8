use std::error::Error;
use chip_8::Chip8;
use std::env;
use std::fs::File;
use std::io::Read;

extern crate chip_8;

fn main() -> Result<(), Box<dyn Error>> {
    let rom = env::args().nth(1).ok_or("Did not get a rom")?;
    let mut raw_bytes = Vec::new();
    let mut raw_rom = File::open(rom)?;
    raw_rom.read_to_end(&mut raw_bytes)?;
    let _chip_8 = Chip8::new(&raw_bytes);
    Ok(())
}
