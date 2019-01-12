Below VM
========

Virtual machine written in Rust. 

Instructions
-----------

|Opcode |Instruction |Argument |Argument|
|-------|------------|---------|--------|
|0x00   |NOP         |NULL     |NULL    |
|0x01   |EXT         |NULL     |NULL    |
|0x02   |MOVREG      |SRC      |REG     |
|0x03   |MOVIMM      |SRC      |IMM     |
|0x04   |MOVMEM      |SRC      |PTR     |
|0x05   |OR          |SRC      |DST     |
|0x06   |AND         |SRC      |DST     |
|0x07   |XOR         |SRC      |DST     |
|0x08   |NOT         |SRC      |NULL    |
|0x09   |ADDREG      |SRC      |REG     |
|0x0A   |ADDIMM      |SRC      |IMM     |
|0x0B   |ADDMEM      |SRC      |PTR     |
|0x0C   |SUBREG      |SRC      |REG     |
|0x0D   |SUBIMM      |SRC      |IMM     |
|0x0E   |SUBMEM      |SRC      |PTR     |
|0x0F   |MUL         |SRC      |DST     |
|0x10   |DIV         |SRC      |DST     |
|0x11   |      |      |     |
|0x12   |MMOVREG     |PTR      |REG     |
|0x13   |MMOVIMM     |PTR      |IMM     |
|0x14   |MMOVIMM     |PTR      |PTR     |
|0x15   |      |      |     |
|0x16   |      |      |     |
|0x17   |      |      |     |
|0x18   |      |      |     |
|0x19   |ADCREG      |SRC      |REG     |
|0x1A   |ADCIMM      |SRC      |IMM     |
|0x1B   |ADCMEM      |SRC      |PTR     |
|0x1C   |SBCREG      |SRC      |REG     |
|0x1D   |SBCIMM      |SRC      |IMM     |
|0x1E   |SBCMEM      |SRC      |PTR     |
