use std::io::Read;
use std::fs::File;

pub struct Memory {
    code:           Vec<u8>,
    pub stack:      Vec<u8>,
    data:           [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            code:       vec![],
            stack:      vec![],
            data:       [0; 0xFFFF]
        }
    }

    /** Code memory functions */

    pub fn code_get_instruction(&mut self, address: usize) -> u8 {
        if address >= self.code.len() {
            return 0;
        }
        return self.code[address as usize];
    }

    pub fn code_get_immediate(&mut self, address: usize) -> u16 {
        if address >= self.code.len() {
            return 0;
        }

        let word: u16 = (self.code[address] as u16) | 
                        (self.code[address + 1] as u16) << 8;
        return word;
    }

    pub fn get_registers_index(&mut self, address: usize) -> (usize, usize) {
        if address >= self.code.len() {
            return (0, 0);
        }

        let registers: u8 = self.code[address as usize];
        let src = (registers & 0xF) as usize;
        let dst = (registers >> 0x4) as usize;
        return (src, dst);
    }

    pub fn get_memory_size(&mut self) -> usize {
        return self.code.len();
    }

    pub fn read_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Unable to open.");
        file.read_to_end(&mut self.code).unwrap();
    }

    /** Data functions */
    pub fn data_get_word(&mut self, address: u16) -> u16 {
        let word: u16 = (self.data[address as usize] as u16) | 
                        (self.data[(address + 1) as usize] as u16) << 8;
        return word;
    }

    pub fn data_set_word(&mut self, data: u16, address: u16) {
        let upper = (data >> 0x8) as u8;
        let lower = (data & 0xFF) as u8;
        self.data[address as usize] = lower;
        self.data[(address + 1) as usize] = upper;
    }

}