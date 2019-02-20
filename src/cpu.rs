use memory::Memory;
use opcodes::Opcodes;

const NUM_REGS: usize = 16;   

pub enum FLAGS {
    ZERO,
    CARRY,
    EQUAL,
    GREATER,
    LOWER,
}

impl FLAGS {
    pub fn to_byte(flag: Self) -> u8 {
        match flag {
            FLAGS::ZERO     => return 0x00,
            FLAGS::CARRY    => return 0x02,
            FLAGS::EQUAL    => return 0x04,
            FLAGS::GREATER  => return 0x08,
            FLAGS::LOWER    => return 0x16,
        }
    }
}

pub struct CPU {
    pc:         usize,                  /* Program counter */
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
        self.memory.read_file("assembler/program");
 
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
            if self.pc >= self.memory.get_memory_size() {
                println!("Entered max memory limit at PC. Exiting");
                return;                
            }

            let opcode: u8 = self.memory.code_get_instruction(self.pc);
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
                let immediate = self.memory.code_get_immediate(self.pc);
                self.pc += 1;
                self.registers[dst] = immediate;
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
                if (self.registers[src] + self.registers[dst]) as u32 > 0xFFFF {
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
                    if (self.registers[src] + self.registers[dst] + 1) as u32 > 0xFFFF {
                        self.set_flag(FLAGS::CARRY);
                    }
                    self.registers[dst] += self.registers[src] + 1;
                } else {
                    self.registers[dst] += self.registers[src];
                }

                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::SUB => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                if ((self.registers[dst] - self.registers[src]) as i32) < 0x0 {
                    self.set_flag(FLAGS::CARRY)
                }

                self.registers[dst] -= self.registers[src];
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }

            }

            Opcodes::SBC => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);
                if self.is_flag(FLAGS::CARRY) {
                    if ((self.registers[dst] - self.registers[src]) as i32) < 0x0 {
                        self.set_flag(FLAGS::CARRY);
                    }
                    self.registers[dst] -= self.registers[src] + 1;
                } else {
                    self.registers[dst] -= self.registers[src];
                }

                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::MUL => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);

                self.registers[dst] *= self.registers[src];
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::DIV => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);

                self.registers[dst] /= self.registers[src];
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::SHL => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);

                self.registers[dst] <<= self.registers[src];
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::SHR => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);

                self.registers[dst] >>= self.registers[src];
                if self.registers[dst] == 0 {
                    self.set_flag(FLAGS::ZERO);
                }
            }

            Opcodes::INC => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] += 1;
            }

            Opcodes::DEC => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] -= 1;
            }

            Opcodes::LOAD => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                let address = self.memory.code_get_immediate(self.pc);
                self.pc += 1;
                self.registers[dst] = self.memory.data_get_word(address);
            }

            Opcodes::STORE => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.pc += 1;
                let address = self.memory.code_get_immediate(self.pc);
                self.pc += 1;
                self.memory.data_set_word(self.registers[dst], address);
            }

            Opcodes::PUSH => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.memory.stack_push(self.registers[dst]);
            }

            Opcodes::POP => {
                self.pc += 1;
                let (_src, dst) = self.memory.get_registers_index(self.pc);
                self.registers[dst] = self.memory.stack_pop();
            }

            Opcodes::CMP => {
                self.pc += 1;
                let (src, dst) = self.memory.get_registers_index(self.pc);

                if self.registers[dst] == self.registers[src] {
                    self.set_flag(FLAGS::EQUAL);
                } else if self.registers[dst] > self.registers[src] {
                    self.set_flag(FLAGS::GREATER);
                } else if self.registers[dst] < self.registers[src] {
                    self.set_flag(FLAGS::LOWER);
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