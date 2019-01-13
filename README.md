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
|0x05   |OR          |DST      |SRC     |
|0x06   |AND         |DST      |SRC     |
|0x07   |XOR         |DST      |SRC     |
|0x08   |NOT         |DST      |NULL    |
|0x09   |ADDREG      |DST      |REG     |
|0x0A   |ADDIMM      |DST      |IMM     |
|0x0B   |ADDMEM      |DST      |PTR     |
|0x0C   |SUBREG      |DST      |REG     |
|0x0D   |SUBIMM      |DST      |IMM     |
|0x0E   |SUBMEM      |DST      |PTR     |
|0x0F   |MUL         |DST      |REG     |
|0x10   |DIV         |DST      |REG     |
|0x11   |      |      |     |
|0x12   |MMOVREG     |PTR      |REG     |
|0x13   |MMOVIMM     |PTR      |IMM     |
|0x14   |MMOVIMM     |PTR      |PTR     |
|0x15   |      |      |     |
|0x16   |      |      |     |
|0x17   |      |      |     |
|0x18   |      |      |     |
|0x19   |ADCREG      |DST      |REG     |
|0x1A   |ADCIMM      |DST      |IMM     |
|0x1B   |ADCMEM      |DST      |PTR     |
|0x1C   |SBCREG      |DST      |REG     |
|0x1D   |SBCIMM      |DST      |IMM     |
|0x1E   |SBCMEM      |DST      |PTR     |
