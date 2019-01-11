pub enum Opcodes {
    Nop,
    MovReg,
    MovImm,

    Out,
    Non
}

impl Opcodes {
    pub fn to_enum(opcode: u16) -> Self {
        match opcode {
            0x00 => return Opcodes::Nop,
            0x01 => return Opcodes::MovReg,
            0x02 => return Opcodes::MovImm,

            
            0xfe => return Opcodes::Out,
            _ =>    return Opcodes::Non
        } 
    }
}