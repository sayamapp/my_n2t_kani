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

(CHECK_INPUT)
    @KBD
    D=M 
    @FILL_WHITE
    D;JEQ
    @FILL_BLACK
    0;JMP

(FILL_WHITE)
    @SCREEN
    D=A 
    (FW_LOOP)
        A=D
        M=0
        @KBD
        A=A-1
        D=D-A 
        @CHECK_INPUT 
        D; JGE 
        @KBD
        A=A-1
        D=D+A 
        D=D+1 
        @FW_LOOP
        0; JMP


(FILL_BLACK)
    @SCREEN
    D=A
    (FB_LOOP)
        A=D
        M=-1
        @KBD
        A=A-1
        D=D-A 
        @CHECK_INPUT
        D;JGE
        @KBD
        A=A-1
        D=D+A
        D=D+1
        @FB_LOOP
        0; JMP



