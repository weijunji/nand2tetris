// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// init sbase & stop
@SCREEN
D=A
@sbase
M=D
@8191
D=D+A
@stop
M=D

(LOOP)
// mov D, keyboard
@KBD
D=M
// mov pset, -1
@pset
M=-1 // pix set to -1
// jgt D, FILL
@FILL
D; JGT
// mov pset, 0
@pset
M=0 // pix set to 0
// fallthrough to FILL

(FILL)
// mov i, sbase
@sbase
D=M // mov D, sbase
@i
M=D // mov i, D

(FILL_LOOP)
@stop
D=M-D // stop - i
@FILL_END
D; JLT // if stop < i

// mov M[i], pset
@pset
D=M // mov D, pset
@i
A=M // mov A, i
M=D // mov M[i], D

// add i, i, 1
@i
M=M+1
// mov D, i
D=M

@FILL_LOOP
0;JMP
(FILL_END)
@LOOP
0;JMP
