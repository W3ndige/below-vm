use std::io::Read;
use std::fs::File;

const MAX_MEMORY:   usize = 0xFFFF;

pub struct Memory {
    memory:     [u8; MAX_MEMORY]   /* 0xFFFF bytes of memory */
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory:     [0; MAX_MEMORY]
        }
    }

    pub fn set_byte(&mut self, value: u8, address: u16) {
        /* Memory up to 0x7FFF is only readable */
        if address > 0x7FFF {
            return;
        }

        self.memory[address as usize] = value;
    }

    pub fn get_byte(&mut self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn get_word(&mut self, address: u16) -> u16 {
        let word: u16 = (self.memory[address as usize] as u16) | 
                        (self.memory[(address + 1) as usize] as u16) << 8;
        return word;
    }

    pub fn set_word(&mut self, value: u16, address: u16) {
        let upper = (value & 0xFF) as u8;
        let lower = (value >> 0x8) as u8;
        self.memory[address as usize] = lower;
        self.memory[(address + 1) as usize] = upper;
    }

    pub fn get_registers_index(&mut self, address: u16) -> (usize, usize) {
        let registers: u8 = self.memory[address as usize];
        let src = (registers & 0xF) as usize;
        let dst = (registers >> 0x4) as usize;
        return (src, dst);
    }

    pub fn read_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Unable to open.");
        file.read(&mut self.memory).unwrap();
    }
}