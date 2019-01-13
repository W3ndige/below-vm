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
        self.memory[address as usize] = value;
    }

    pub fn get_byte(&mut self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn get_word(&mut self, address: u16) -> u16 {
        let word: u16 = (self.memory[address as usize] | 
                        self.memory[(address + 1) as usize] << 8) as u16;
        return word;
    }

}