//init 'stack' pointer
@256
D=A
@SP
M=D
//call Sys.init
@Sys.init$ret0
D=A
@SP
A=M
M=D
@SP
AM=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
AM=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
AM=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
AM=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
AM=M+1
@SP
D=M
@5
D=D-A
@0
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.init
0;JMP
(Sys.init$ret0)
// // This file is part of www.nand2tetris.org
// // and the book "The Elements of Computing Systems"
// // by Nisan and Schocken, MIT Press.
// // File name: projects/07/MemoryAccess/PointerTest/PointerTest.vm
// 
// // Executes pop and push commands using the 
// // pointer, this, and that segments.
// push constant 3030
@3030
D=A
@SP
A=M
M=D
@SP
AM=M+1
// pop pointer 0
@SP
AM=M-1
D=M
@THIS
M=D
// push constant 3040
@3040
D=A
@SP
A=M
M=D
@SP
AM=M+1
// pop pointer 1
@SP
AM=M-1
D=M
@THAT
M=D
// push constant 32
@32
D=A
@SP
A=M
M=D
@SP
AM=M+1
// pop this 2
@THIS
D=M
@2
A=D+A
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// push constant 46
@46
D=A
@SP
A=M
M=D
@SP
AM=M+1
// pop that 6
@THAT
D=M
@6
A=D+A
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
// push pointer 0
@THIS
D=M
@SP
A=M
M=D
@SP
AM=M+1
// push pointer 1
@THAT
D=M
@SP
A=M
M=D
@SP
AM=M+1
// add
@SP
AM=M-1
D=M
@SP
A=M-1
M=D+M
// push this 2
@THIS
D=M
@2
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// sub
@SP
AM=M-1
D=M
@SP
A=M-1
M=M-D
// push that 6
@THAT
D=M
@6
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// add
@SP
AM=M-1
D=M
@SP
A=M-1
M=D+M
