%include "syntax.inc"

SET R1, 0x1337
STORE 0xFFFC, R1
LOAD R2, 0xFFFC

OUT R1
OUT R2
EXT