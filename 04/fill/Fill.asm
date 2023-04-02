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

// Put your code here.

(START)

@KBD
D=M

@SCREEN1
D;JGT

@SCREEN0
D;JEQ

(SCREEN1)
@SCREEN
D=A
@addr
M=D

@8192
D=A

(LOOP1)

@addr
A=M
M=-1

@addr
M=M+1
D=D-1

@LOOP1
D;JGT

@START
0;JMP

(SCREEN0)
@SCREEN
D=A
@addr
M=D

@8192
D=A

(LOOP2)

@addr
A=M
M=0

@addr
M=M+1
D=D-1

@LOOP2
D;JGT

@START
0;JMP