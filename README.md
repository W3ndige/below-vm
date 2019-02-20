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
|0x09   |SUB         |REG      |REG     |
|0x0A   |MUL         |REG      |REG     |
|0x0B   |DIV         |REG      |REG     |
|0x0C   |SHL         |REG      |REG     |
|0x0D   |SHR         |REG      |REG     |
|0x0E   |INC         |REG      |NULL    |
|0x0F   |DEC         |REG      |NULL    |
|0x10   |LOAD        |REG      |PTR     |
|0x11   |STORE       |PTR      |REG     |
|0x12   |PUSH        |REG      |NULL    |
|0x13   |POP         |REG      |NULL    |
|0x20   |CMP         |REG      |REG     |
|0x30   |OUT         |REG      |NULL    |


