pub enum Opcode {
    NOP,
    LOAD,
    NON
}

impl From<u16> for Opcode {
    fn from(code: u16) -> Self {
        match code {
            0x00 => return Opcode::NOP,
            0x01 => return Opcode::LOAD,
            _    => return Opcode::NON
        }
    }
}
