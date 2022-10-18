#![no_std]

mod opcodes;

use opcodes::OPCODES;

struct Register([u8; 2]);

impl Register {
    fn as_u16(&self) -> u16 {
        u16::from_le_bytes(self.0)
    }

    fn add(&mut self, val: u16) {
        self.0 = (u16::from_le_bytes(self.0) + val).to_le_bytes();
    }

    fn low(&self) -> u8 {
        self.0[0]
    }

    fn high(&self) -> u8 {
        self.0[1]
    }

    fn set_low(&mut self, val: u8) {
        self.0[0] = val;
    }

    fn set_high(&mut self, val: u8) {
        self.0[1] = val;
    }
}

pub struct Processor {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,

    ime: bool,
    halted: bool,

    memory: Memory,
}

impl Processor {
    pub fn process(&mut self, opcode: u8) {
        OPCODES[opcode as usize](self);
    }
}

struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn set(&mut self, addr: u16, val: u8) {
        // Cast to usize to make compiler happy; we rely on the u16 having a max value
        // smaller than amount of memory to avoid needing a Result.
        let addr = addr as usize;
        if addr >= 0xC000 && addr < 0xE000 {
            // Make sure to set in echo and local
            self.memory[addr + 0x2000] = val;
        }
        if addr >= 0xE000 {
            self.memory[addr - 0x2000] = val;
        }
        self.memory[addr] = val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
