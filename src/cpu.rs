use std::error::Error;

const SCREEN_HEIGHT: usize = 64;
const SCREEN_WIDTH: usize = 32;

pub struct Cpu {
    ram: Ram,
    regs: Registers,
    stack: Stack,
    screen: Screen,
    _keyboard: Keyboard,
}

impl Cpu {
    pub fn new(rom_bytes: &[u8]) -> Cpu {
        Cpu {
            ram: Ram::new(rom_bytes),
            regs: Registers::new(),
            stack: Stack::new(),
            screen: Screen::new(),
            _keyboard: Keyboard::new(),
        }
    }

    pub fn fetch(&self) -> u16 {
        let l_byte = self.ram.0[self.regs.pc.0 as usize] as u16;
        let r_byte = self.ram.0[(self.regs.pc.0 as usize) + 1] as u16;
        (l_byte << 8 | r_byte)
    }

    pub fn execute(&mut self, opcode: u16) -> Result<(), Box<dyn Error>> {
        match opcode {
            0x00E0 => {
                Ok(self.screen.clear_screen())
            }
            0x00EE => {
                self.regs.pc.0 = self.stack.pop(&mut self.regs.sp)?;
                Ok(())
            }
            //0x0nnn =>
            //0x1nnn =>
            //0x2nnn =>
            //0x3xkk =>
            //0x4xkk =>
            //0x5xy0 =>
            //0x6xkk =>
            //0x7xkk =>
            //0x8xy0 =>
            //0x8xy1 =>
            //0x8xy2 =>
            //0x8xy3 =>
            //0x8xy4 =>
            //0x8xy5 =>
            //0x8xy6 =>
            //0x8xy7 =>
            //0x8xyE =>
            //0x9xy0 =>
            //0xAnnn =>
            //0xBnnn =>
            //0xCxkk =>
            //0xDxyn =>
            //0xEx9E =>
            //0xExA1 =>
            //0xFx07 =>
            //0xFx0A =>
            //0xFx15 =>
            //0xFx18 =>
            //0xFx1E =>
            //0xFx29 =>
            //0xFx33 =>
            //0xFx55 =>
            //0xFx65 =>
            o => Err(format!("Got undefined opcode {:X}", o))?,
        }
    }
}

struct Ram(Box<[u8]>);

impl Ram {
    fn new(rom_bytes: &[u8]) -> Ram {
        let mut tmp = Box::new([0; 0xFFF]);
        tmp.copy_from_slice(rom_bytes);
        Ram { 0: tmp }
    }
}

struct Stack([u16; 0xF]);

impl Stack {
    fn new() -> Stack {
        Stack { 0: [0; 0xF] }
    }

    fn pop(&mut self, sp: &mut u8) -> Result<u16, Box<dyn Error>> {
        if *sp == 0 {
            Err("Error: stack underflow. Tried to pop without pushing")?
        } else {
            *sp = *sp - 1;
            Ok(self.0[*sp as usize])
        }
    }

    fn push(
        &mut self,
        payload: u16,
        sp: &mut u8,
    ) -> Result<(), Box<dyn Error>> {
        if *sp > (0xF - 1) {
            Err(format!("Error: stack overflow. Tried to access {:X}", *sp))?
        } else {
            self.0[*sp as usize] = payload;
            *sp = *sp + 1;
            Ok(())
        }
    }
}

struct Keyboard([bool; 0xF + 1]);

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard {
            0: [false; 0xF + 1],
        }
    }

    fn set_key(
        &mut self,
        key: usize,
        state: bool,
    ) -> Result<(), Box<dyn Error>> {
        if key > 0xF {
            Err(format!("Error: cannot access key {:X}", key))?
        } else {
            self.0[key] = state;
            Ok(())
        }
    }
}

struct Screen(Box<[bool]>);

impl Screen {
    fn new() -> Screen {
        Screen {
            0: Box::new([false; SCREEN_WIDTH * SCREEN_HEIGHT]),
        }
    }

    fn clear_screen(&mut self) {
        self.0.iter_mut().for_each(|pixel| *pixel = false);
    }
}

struct Registers {
    v_regs: [u8; 16],
    pc: ProgramCounter,
    sound: u8,
    i_reg: u16,
    counter: u8,
    sp: u8,
}

impl Registers {
    fn new() -> Self {
        Registers {
            v_regs: [0; 16],
            pc: ProgramCounter::new(),
            sound: 0,
            i_reg: 0,
            counter: 0,
            sp: 0,
        }
    }
}

struct ProgramCounter(u16);

impl ProgramCounter {
    fn new() -> ProgramCounter {
        ProgramCounter { 0: 0x200 }
    }

    fn increment(&mut self) {
        self.0 += 2;
    }

    fn set_pc(&mut self, addr: u16) -> Result<(), Box<dyn Error>> {
        // 0xFFF - 1 is the largest valid memory address
        if self.0 > (0xFFF - 1) {
            Err(format!("Error: cannot access address {:X}", addr))?
        } else {
            self.0 = addr;
            Ok(())
        }
    }
}
