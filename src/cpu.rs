use memory::Memory;
use opcodes::Opcodes;

const NUM_REGS: usize = 16;   

pub enum FLAGS {
    ZERO,
    CARRY
}

impl FLAGS {
    pub fn to_byte(flag: Self) -> u8 {
        match flag {
            FLAGS::ZERO => return 0x00,
            FLAGS::CARRY => return 0x02
        }
    }
}

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
        self.memory.read_file();
 
    }

    pub fn set_flag(&mut self, flag: FLAGS) {
        self.flags |= FLAGS::to_byte(flag);        
    }

    pub fn is_flag(&mut self, flag: FLAGS) -> bool {
        if self.flags & FLAGS::to_byte(flag) == 1 {
            return true;
        }
        return false;
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

            Opcodes::MOV => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] = self.registers[src];
            }

            Opcodes::SET => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                let immediate = self.memory.get_word(self.pc);
                self.pc += 1;
                self.registers[dst] = immediate;
            }

            Opcodes::LOAD => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                let address = self.memory.get_word(self.pc);
                self.pc += 1;
                self.registers[dst] = self.memory.get_word(address);
            }

            Opcodes::MMOV => {
                self.pc += 1;
                let address = self.memory.get_word(self.pc);
                self.pc += 2;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                self.memory.set_word(self.registers[dst], address);
            }

            Opcodes::MSET => {
                self.pc += 1;
                let address = self.memory.get_word(self.pc);
                self.pc += 2;
                let immediate = self.memory.get_word(self.pc);
                self.pc += 1;
                self.memory.set_word(immediate, address);
            }

            Opcodes::MLOAD => {
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
                if (self.registers[src] + self.registers[dst]) as usize > 0xFFFF {
                    self.set_flag(FLAGS::CARRY);
                }

                self.registers[dst] += self.registers[src];
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::ADC => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                if self.is_flag(FLAGS::CARRY) {
                    if (self.registers[src] + self.registers[dst] + 1) as usize > 0xFFFF {
                        self.set_flag(FLAGS::CARRY);
                    }
                }

                self.registers[dst] += self.registers[src] + 1;
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::OUT => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                let register_value = self.registers[dst];
                println!("0x{:x}", register_value);
            }

            Opcodes::NON => {
                println!("Unimplemented opcode: 0x{:x} at PC 0x{:x}", opcode, self.pc);
            }
        }
        return true;
    }

}