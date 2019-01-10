use memory::Memory;
use cpu::CPU;

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

    pub fn fetch_and_dispatch(&mut self) {
        let op: u16 = self.memory.get_byte(self.cpu.get_pc());
        println!("{}", op);
        self.cpu.step_pc(1);
    }

}