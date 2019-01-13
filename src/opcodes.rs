pub enum Opcodes {
    NOP,
    EXT,
    MOVREG,
    MOVIMM,
    MOVMEM,
    /*OR,
    AND,
    XOR,
    NOT,
    ADDREG,
    ADDIMM,
    ADDMEM,
    SUBREG,
    SUBIMM,
    SUBMEM,
    MUL,
    DIV,*/


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
            _ =>    return Opcodes::NON
        } 
    }
}