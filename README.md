Below VM
========

Virtual machine written in Rust. 

Instructions
-----------

|Opcode |Instruction |Argument |Argument|
|-------|------------|---------|--------|
|0x00   |NOP         |NULL     |NULL    |
|0x01   |EXT         |NULL     |NULL    |
|0x02   |MOV         |REG      |REG     |
|0x03   |SET         |REG      |IMM     |
|0x04   |OR          |REG      |REG     |
|0x05   |AND         |REG      |REG     |
|0x06   |XOR         |REG      |REG     |
|0x07   |NOT         |REG      |NULL    |
|0x08   |ADD         |REG      |REG     |
|0x09   |ADC         |REG      |REG     |
|0x0A   |SUB         |REG      |REG     |
|0x0B   |SBC         |REG      |REG     |
|0x0C   |MUL         |REG      |REG     |
|0x0D   |DIV         |REG      |REG     |
|0x0E   |SHL         |REG      |REG     |
|0x0F   |SHR         |REG      |REG     |
|0x10   |CMP         |REG      |REG     |
|....   |...         |REG      |REG     |
|0x20   |OUT         |REG      |NULL    |

