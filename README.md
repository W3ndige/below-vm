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
|0x12   |SHL         |NULL     |NULL    | # NOT FINISHED
|0x13   |SHR         |NULL     |NULL    | # NOT FINISHED
|0x14   |SAR         |NULL     |NULL    | # NOT FINISHED
|0x15   |SAL         |NULL     |NULL    | # NOT FINISHED
|0x16   |OUT         |REG      |NULL    |
