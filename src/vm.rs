use cpu::CPU;

pub struct VM {
    cpu:        CPU,                /* CPU structure */
}

impl VM {
    pub fn new() -> VM {
        VM {
            cpu:        CPU::new(),
        }
    }

    pub fn run(&mut self) {
        self.cpu.init();
        self.cpu.fetch_and_dispatch();
    }

}