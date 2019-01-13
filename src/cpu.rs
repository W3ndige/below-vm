use memory::Memory;
use opcodes::Opcodes;

const NUM_REGS:     usize = 16;   

pub struct CPU {
    pc:         u16,                    /* Program counter */
    flags:      u16,                    /* Flag register */
    registers:  [u16; NUM_REGS],        /* Sixteen 16-bit registers */
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
        /* Testing */

        self.memory.set_byte(0x03, 0x01);
        self.memory.set_byte(0x01, 0x02);
        self.memory.set_byte(0x01, 0x03);
        self.memory.set_byte(0x02, 0x04);
    }

    pub fn fetch_and_dispatch(&mut self) {
        let mut running = true;
        while running {
            if self.pc == 0xFFFF {
                println!("Entered max memory limit at PC. Exiting");
                return;
            }

            let opcode: u8 = self.memory.get_byte(self.pc);
            if !self.execute_instruction(opcode) {
                running = false;
            }

            self.pc += 1;
        } 
    }

    fn execute_instruction(&mut self, opcode: u8) -> bool {
        match Opcodes::to_enum(opcode) {
            Opcodes::NOP => {},
            Opcodes::EXT => {
                return false
            }
            Opcodes::MOVREG => {
                self.pc += 1;
                let registers: u8 = self.memory.get_byte(self.pc);
                let src = (registers & 0xF) as usize;
                let dst = (registers >> 0x4) as usize;
                
                self.registers[dst] = self.registers[src];
            }
            Opcodes::MOVIMM => {
                self.pc += 1;
                let registers: u8 = self.memory.get_byte(self.pc);
                self.pc += 1;
                let immediate_upper = self.memory.get_byte(self.pc) as u16;
                self.pc += 1;
                let immediate_lower = self.memory.get_byte(self.pc) as u16;
                let immediate = immediate_lower | immediate_upper << 8;
                let dst = (registers >> 0x4) as usize;
                self.registers[dst] = immediate;
            }
            Opcodes::MOVMEM => {
                self.pc += 1;
                let registers: u8 = self.memory.get_byte(self.pc);
                self.pc += 1;
                let address_upper = self.memory.get_byte(self.pc) as u16;
                self.pc += 1;
                let address_lower = self.memory.get_byte(self.pc) as u16;
                let address: u16 = address_lower | address_upper << 8;
                let dst = (registers >> 0x4) as usize;
                self.registers[dst] = self.memory.get_word(address);
            }
            Opcodes::NON => {
                println!("Unimplemented opcode: {}", opcode);
            }
        }
        return true;
    }


}