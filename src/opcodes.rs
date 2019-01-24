pub enum Opcodes {
    NOP,
    EXT,
    MOV,
    SET,
    LOAD,
    MMOV,
    MSET,
    MLOAD,
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
            0x02 => return Opcodes::MOV,
            0x03 => return Opcodes::SET,
            0x04 => return Opcodes::LOAD,
            0x05 => return Opcodes::MMOV,
            0x06 => return Opcodes::MSET,
            0x07 => return Opcodes::MLOAD,
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