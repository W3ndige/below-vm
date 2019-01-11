use memory::Memory;
use cpu::CPU;
use opcodes::Opcodes;

pub struct VM {
    cpu:        CPU,                /* CPU structure */
    memory:     Memory,             /* Memury structure */
}

impl VM {
    pub fn new() -> VM {
        VM {
            cpu:        CPU::new(),
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
            if self.cpu.get_pc() == 0xFFFF {
                println!("Entered max memory limit at PC. Exiting");
                return;
            }

            let opcode: u16 = self.memory.get_byte(self.cpu.get_pc());
            
            self.execute_instruction(opcode);
            self.cpu.step_pc(1);
        } 
    }

    fn execute_instruction(&mut self, opcode: u16) {
        match Opcodes::to_enum(opcode) {
            Opcodes::Nop => {},
            Opcodes::MovReg => {
                self.cpu.step_pc(1);
                let destination = self.memory.get_byte(self.cpu.get_pc()) as usize;
                self.cpu.step_pc(1);
                let source = self.memory.get_byte(self.cpu.get_pc()) as usize;
                self.cpu.registers[destination] = self.cpu.registers[source];
            },
            Opcodes::MovImm => {
                self.cpu.step_pc(1);
                let destination = self.memory.get_byte(self.cpu.get_pc()) as usize;
                self.cpu.step_pc(1);
                let value: u16 = self.memory.get_byte(self.cpu.get_pc());
                self.cpu.registers[destination] = value;

            }

            Opcodes::Out => {
                self.cpu.step_pc(1);
                let register = self.memory.get_byte(self.cpu.get_pc()) as usize;
                print!("0x{:x}", self.cpu.registers[register]);

            }
            Opcodes::Non => {
                println!("Unimplemented opcode: {}", opcode);
            }
        }        
    }
}