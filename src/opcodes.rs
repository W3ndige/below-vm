pub enum Opcodes {
    NOP,
    EXT,
    MOV,
    SET,
    OR,
    AND,
    XOR,
    NOT,
    ADD,
    ADC,
    SUB,
    SBC,
    MUL,
    DIV,
    SHL,
    SHR,
    INC,
    DEC,
    LOAD,
    STORE,

    CMP,

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
            0x04 => return Opcodes::OR,
            0x05 => return Opcodes::AND,
            0x06 => return Opcodes::XOR,
            0x07 => return Opcodes::NOT,
            0x08 => return Opcodes::ADD,
            0x09 => return Opcodes::SUB,
            0x0A => return Opcodes::MUL,
            0x0B => return Opcodes::DIV,
            0x0C => return Opcodes::SHL,
            0x0D => return Opcodes::SHR,
            0x0E => return Opcodes::INC,
            0x0F => return Opcodes::DEC,
            0x10 => return Opcodes::LOAD,
            0x11 => return Opcodes::STORE,

            0x20 => return Opcodes::CMP,

            0x30 => return Opcodes::OUT,


            _ =>    return Opcodes::NON
        } 
    }
}