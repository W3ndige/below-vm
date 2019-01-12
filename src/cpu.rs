use memory::Memory;
use opcodes::Opcodes;

const NUM_REGS:     usize = 8;   

pub struct CPU {
    pc:         u16,                    /* Program counter */
    flags:      u16,                    /* Flag register */
    registers:  [u16; NUM_REGS],    /* Eight 16-bit registers */
    memory:     Memory,                 /* Memury structure */

}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc:         0,
            flags:      0,
            registers:  [0; NUM_REGS],
            memory:     Memory::new()
        }
    }

    pub fn init(&mut self) {
        self.memory.set_byte(0x02, 0x01);   // Mov
        self.memory.set_byte(0x00, 0x02);   // To value od R0
        self.memory.set_byte(0x37, 0x03);   // Immediate Value
        self.memory.set_byte(0xfe, 0x04);   // Out
        self.memory.set_byte(0x00, 0x05);   // Value at R0
    }

    pub fn fetch_and_dispatch(&mut self) {
        loop {
            if self.pc == 0xFFFF {
                println!("Entered max memory limit at PC. Exiting");
                return;
            }

            let opcode: u16 = self.memory.get_byte(self.pc);
            
            self.execute_instruction(opcode);
            self.pc += 1;
        } 
    }

    fn execute_instruction(&mut self, opcode: u16) {
        match Opcodes::to_enum(opcode) {
            Opcodes::Nop => {},
            Opcodes::MovReg => {
                self.pc += 1;
                let destination = self.memory.get_byte(self.pc) as usize;
                self.pc += 1;
                let source = self.memory.get_byte(self.pc) as usize;
                self.registers[destination] = self.registers[source];
            },
            Opcodes::MovImm => {
                self.pc += 1;
                let destination = self.memory.get_byte(self.pc) as usize;
                self.pc += 1;
                let value: u16 = self.memory.get_byte(self.pc);
                self.registers[destination] = value;

            }

            Opcodes::Out => {
                self.pc += 1;
                let register = self.memory.get_byte(self.pc) as usize;
                print!("0x{:x}", self.registers[register]);

            }
            Opcodes::Non => {
                println!("Unimplemented opcode: {}", opcode);
            }
        }        
    }


}