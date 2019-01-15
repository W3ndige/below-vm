pub enum Opcodes {
    NOP,
    EXT,
    MOVREG,
    MOVIMM,
    MOVMEM,
    MMOVREG,
    MMOVIMM,
    MMOVMEM,
    OR,
    AND,
    XOR,
    NOT,
    ADD,
    ADC,

    OUT,
    NON
}

impl Opcodes {
    pub fn to_enum(opcode: u8) -> Self {
        match opcode {
            0x00 => return Opcodes::NOP,
            0x01 => return Opcodes::EXT,
            0x02 => return Opcodes::MOVREG,
            0x03 => return Opcodes::MOVIMM,
            0x04 => return Opcodes::MOVMEM,
            0x05 => return Opcodes::MMOVREG,
            0x06 => return Opcodes::MMOVIMM,
            0x07 => return Opcodes::MMOVMEM,
            0x08 => return Opcodes::OR,
            0x09 => return Opcodes::AND,
            0x0A => return Opcodes::XOR,
            0x0B => return Opcodes::NOT,
            0x0C => return Opcodes::ADD,
            0x0D => return Opcodes::ADC,
            0x16 => return Opcodes::OUT,
            _ =>    return Opcodes::NON
        } 
    }
}