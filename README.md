Below VM
========

Virtual machine written in Rust. 

Instructions
-----------

|Opcode |Instruction |Argument |Argument|
|-------|------------|---------|--------|
|0x00   |NOP         |NULL     |NULL    |
|0x01   |EXT         |NULL     |NULL    |
|0x02   |MOVREG      |DST      |REG     |
|0x03   |MOVIMM      |DST      |IMM     |
|0x04   |MOVMEM      |DST      |PTR     |
|0x05   |MMOVREG     |PTR      |REG     |
|0x06   |MMOVIMM     |PTR      |IMM     |
|0x07   |MMOVMEM     |PTR      |PTR     |
|0x08   |OR          |DST      |SRC     |
|0x09   |AND         |DST      |SRC     |
|0x0A   |XOR         |DST      |SRC     |
|0x0B   |NOT         |DST      |NULL    |
|0x0C   |ADD         |DST      |REG     |
|0x0D   |ADC         |DST      |REG     |
|0x0E   |SUB         |DST      |REG     |
|0x0F   |SBC         |DST      |REG     |
|0x10   |MUL         |DST      |REG     |
|0x1A   |DIV         |DST      |REG     |
