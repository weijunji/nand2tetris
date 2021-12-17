// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// i = R1
// sum = 0
// while(i != 0) {
// sum += R2; i--;
// }

// mov i, R0
// loop:
// jz i, end
// add sum, sum, R1
// sub i, i, 1
// j loop
// mov R2, sum
// end:

// mov i, R0
@0
D=M // mov D, R0
@i
M=D // mov i, D

// mov sum, 0
@sum
M=0

(LOOP)
// jz i, fin
@END
D; JEQ

// add sum, sum, R1
@1
D=M // mov D, R1
@sum
M=M+D // add sum, sum, D

// sub i, i, 1
@i
M=M-1
D=M

// j loop
@LOOP
0;JMP

(END)
// mov R2, sum
@sum
D=M
@2
M=D
