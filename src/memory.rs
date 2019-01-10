const MAX_MEMORY:   usize = 0xFFFF;

pub struct Memory {
    memory:     [u16; MAX_MEMORY]   /* 0xFFFF bytes of memory */

}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory:     [0; MAX_MEMORY]
        }
    }

    pub fn get_byte(&mut self, address: u16) -> u16 {
        return self.memory[address as usize];
    }

}