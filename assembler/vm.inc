[org 0x0]

; CPU REGISTERS
%define R0  0
%define R1  1
%define R2  2
%define R3  3
%define R4  4
%define R5  5
%define R6  6
%define R7  7
%define R8  8
%define R9  9
%define R10 10
%define R11 11
%define R12 12
%define R13 13
%define R14 14
%define R15 15

; Instructions.
%macro NOP 0
db 0x00
%endmacro

%macro EXT 0
db 0x01
%endmacro

%macro MOV 2
db 0x02, (%1 << 4) | %2
%endmacro

%macro SET 2
db 0x03, %1 << 4
dw %2
%endmacro

%macro LOAD 2
db 0x04, %1 << 4
dw %2
%endmacro

%macro MMOV 2
db 0x05
dw %1
db %2 << 4
%endmacro

%macro MSET 2
db 0x06
dw %1, %2
%endmacro

%macro MLOAD 2
db 0x07
dw %1, %2
%endmacro

%macro OR 2
db 0x08, (%1 << 4) | %2
%endmacro

%macro AND 2
db 0x09, (%1 << 4) | %2
%endmacro

%macro XOR 2
db 0x0A, (%1 << 4) | %2
%endmacro

%macro NOT 1
db 0x0B, (%1 << 4)
%endmacro

%macro ADD 2
db 0x0C, (%1 << 4) | %2
%endmacro

%macro ADC 2
db 0x0D, (%1 << 4) | %2
%endmacro

%macro SUB 2
db 0x0E, (%1 << 4) | %2
%endmacro

%macro SBC 2
db 0x0F, (%1 << 4) | %2
%endmacro

%macro MUL 2
db 0x10, (%1 << 4) | %2
%endmacro

%macro DIV 2
db 0x11, (%1 << 4) | %2
%endmacro

%macro OUT 1
db 0x16, %1 << 4
%endmacro