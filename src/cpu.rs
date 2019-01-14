use memory::Memory;
use opcodes::Opcodes;

const NUM_REGS: usize = 16;   

pub struct CPU {
    pc:         u16,                    /* Program counter */
    flags:      u8,                     /* Flag register */
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

        self.memory.set_byte(0x05, 0x00);
        self.memory.set_byte(0x13, 0x01);
        self.memory.set_byte(0x37, 0x02);
        self.memory.set_byte(0x12, 0x03);
        self.memory.set_byte(0x01, 0x04);
        self.memory.set_byte(0x01, 0x05);
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;        
    }

    pub fn is_flag(&mut self, flag: u8) -> bool {
        if self.flags & flag == 1 {
            return true;
        }
        return false;
    }


    pub fn fetch_and_dispatch(&mut self) {
        let mut running = true;
        self.memory.set_word(0x1234, 0x11);
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
                let (src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] = self.registers[src];
            }
            Opcodes::MOVIMM => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                let immediate = self.memory.get_word(self.pc);
                self.pc += 1;
                self.registers[dst] = immediate;
            }
            Opcodes::MOVMEM => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                let address = self.memory.get_word(self.pc);
                self.pc += 1;
                self.registers[dst] = self.memory.get_word(address);
            }
            Opcodes::MMOVREG => {
                self.pc += 1;
                let address = self.memory.get_word(self.pc);
                self.pc += 2;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                self.memory.set_word(self.registers[dst], address);
            }
            Opcodes::MMOVIMM => {
                self.pc += 1;
                let address = self.memory.get_word(self.pc);
                self.pc += 2;
                let immediate = self.memory.get_word(self.pc);
                self.pc += 1;
                self.memory.set_word(immediate, address);
            }
            Opcodes::MMOVMEM => {
                self.pc += 1;
                let dst_address = self.memory.get_word(self.pc);
                self.pc += 2;
                let src_address = self.memory.get_word(self.pc);
                self.pc += 1;
                let value = self.memory.get_word(src_address);
                self.memory.set_word(value, dst_address);
            }
            Opcodes::OR => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] |= self.registers[src];
            }
            Opcodes::AND => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] &= self.registers[src];
            }
            Opcodes::XOR => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] ^= self.registers[src];
            }
            Opcodes::NOT => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] = !self.registers[dst];
            }
            Opcodes::ADD => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                if src + dst > 0xFFFF {

                }
            }
            Opcodes::NON => {
                println!("Unimplemented opcode: 0x{:x}", opcode);
            }
        }
        return true;
    }

}