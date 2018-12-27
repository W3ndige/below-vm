pub enum Opcode {
    NOP,
    UNIMPL
}

impl From<u16> for Opcode {
    fn from(code: u16) -> Self {
        match code {
            0 => return Opcode::NOP,
            _ => return Opcode::UNIMPL
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
    size:   usize,
}

impl Instruction {
    pub fn new(opcode: Opcode, size: usize) -> Instruction {
        Instruction {
            opcode: opcode,
            size:   size
        }
    }

}