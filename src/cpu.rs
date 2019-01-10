const NUM_REGS:     usize = 8;   

pub struct CPU {
    pc:         u16,                    /* Program counter */
    flags:      u16,                    /* Flag register */
    pub registers:  [u16; NUM_REGS],    /* Eight 16-bit registers */
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc:         0,
            flags:      0,
            registers: [0; NUM_REGS],
        }
    }

    pub fn get_pc(&mut self) -> u16 {
        return self.pc;
    }

    pub fn step_pc(&mut self, step_count: u16) {
        self.pc += step_count;
    }


}