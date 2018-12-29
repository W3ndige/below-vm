use ops::Opcode;

const NUM_REGS:     usize = 8;   
const MAX_MEMORY:   usize = 0xFFFF;

enum Register {
    R0, R1, R2,
    R3, R4, R5,
    R6, R7
}

pub struct VM {
    pc:         u16,                /* Program counter */
    flags:      u16,                /* Flag register */
    registers:  [u16; NUM_REGS],    /* Eight 16-bit registers */
    memory:     [u16; MAX_MEMORY]   /* 0xFFFF bytes of memory */
}

impl VM {
    pub fn new() -> VM {
        VM {
            pc:         0,
            flags:      0,
            registers:  [0; NUM_REGS],
            memory:     [0; MAX_MEMORY]
        }
    }

    pub fn fetch_and_dispach(&mut self) {
        loop {
            if self.pc as usize >= MAX_MEMORY {
                break;
            }

            let op = self.memory[self.pc as usize];
            self.execute_instruction(op);
            self.pc += 1;
        }
    }

    fn execute_instruction(&mut self, op: u16) {

        match Opcode::from(op) {
            Opcode::NOP => {},
            Opcode::LOAD => {

            }

            Opcode::NON => println!("Unimplemented instruction {}", op)
        };
    }
}