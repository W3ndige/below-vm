Below VM
========

Virtual machine written in Rust. 

Memory
------

# 0x0000 -0x7FFF
 
 Executable memory region reserved only for the program. Readble but not writable. 

# 0x7FFF to 0x

Instructions
-----------

|Opcode |Instruction |Argument |Argument|
|-------|------------|---------|--------|
|0x00   |NOP         |NULL     |NULL    |
|0x01   |EXT         |NULL     |NULL    |
|0x02   |MOV         |REG      |REG     |
|0x03   |SET         |REG      |IMM     |
|0x04   |LOAD        |REG      |PTR     |
|0x05   |MMOV        |PTR      |REG     |
|0x06   |MSET        |PTR      |IMM     |
|0x07   |MLOAD       |PTR      |PTR     |
|0x08   |OR          |REG      |REG     |
|0x09   |AND         |REG      |REG     |
|0x0A   |XOR         |REG      |REG     |
|0x0B   |NOT         |REG      |NULL    |
|0x0C   |ADD         |REG      |REG     |
|0x0D   |ADC         |REG      |REG     |
|0x0E   |SUB         |REG      |REG     |
|0x0F   |SBC         |REG      |REG     |
|0x10   |MUL         |REG      |REG     |
|0x11   |DIV         |REG      |REG     |
|0x12   |SHL         |REG      |REG     |
|0x13   |SHR         |REG      |REG     |
|0x14   |OUT         |REG      |NULL    |
|0x15   |CMP         |REG      |REG     |
|0x16   |JMP         |IMM      |NULL    |
|0x17   |JMP_GR      |IMM      |NULL    |
|0x18   |JMP_LO      |IMM      |NULL    |
|0x19   |JMP_EQ      |IMM      |NULL    |

