use std::io::Read;
use std::fs::File;


pub struct Memory {
    program:    Vec<u8>,
    heap:       Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            program:    vec![],
            heap:       vec![]
        }
    }

    pub fn get_byte(&mut self, address: usize) -> u8 {
        return self.program[address as usize];
    }

    pub fn get_word(&mut self, address: usize) -> u16 {
        let word: u16 = (self.program[address] as u16) | 
                        (self.program[address + 1] as u16) << 8;
        return word;
    }

    pub fn get_registers_index(&mut self, address: usize) -> (usize, usize) {
        let registers: u8 = self.program[address as usize];
        let src = (registers & 0xF) as usize;
        let dst = (registers >> 0x4) as usize;
        return (src, dst);
    }

    pub fn get_memory_size(&mut self) -> usize {
        return self.program.len();
    }

    pub fn read_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Unable to open.");
        file.read_to_end(&mut self.program).unwrap();
    }
}