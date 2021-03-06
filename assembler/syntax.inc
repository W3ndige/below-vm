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

%macro OR 2
db 0x04, (%1 << 4) | %2
%endmacro

%macro AND 2
db 0x05, (%1 << 4) | %2
%endmacro

%macro XOR 2
db 0x06, (%1 << 4) | %2
%endmacro

%macro NOT 1
db 0x07, (%1 << 4)
%endmacro

%macro ADD 2
db 0x08, (%1 << 4) | %2
%endmacro

%macro SUB 2
db 0x09, (%1 << 4) | %2
%endmacro

%macro MUL 2
db 0x0A, (%1 << 4) | %2
%endmacro

%macro DIV 2
db 0x0B, (%1 << 4) | %2
%endmacro

%macro SHL 2
db 0x0D, (%1 << 4) | %2
%endmacro

%macro SHR 2
db 0x0D, (%1 << 4) | %2
%endmacro

%macro INC 1
db 0x0E, %1 << 4
%endmacro

%macro DEC 1
db 0x0F, %1 << 4
%endmacro

%macro LOAD 2
db 0x10, %1 << 4
dw %2
%endmacro

%macro STORE 2
db 0x11, %2 << 4
dw %1
%endmacro

%macro PUSH 1
db 0x12, %1 << 4
%endmacro

%macro POP 1
db 0x13, %1 << 4
%endmacro

%macro CMP 2
db 0x20, (%1 << 4) | %2
%endmacro

%macro OUT 1
db 0x30, %1 << 4
%endmacro