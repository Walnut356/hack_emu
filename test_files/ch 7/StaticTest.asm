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
// // File name: projects/07/MemoryAccess/StaticTest/StaticTest.vm
// 
// // Executes pop and push commands using the static segment.
// push constant 111
@111
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 333
@333
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 888
@888
D=A
@SP
A=M
M=D
@SP
AM=M+1
// pop static 8
@static
D=M
@8
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
// pop static 3
@static
D=M
@3
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
// pop static 1
@static
D=M
@1
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
// push static 3
@static
D=M
@3
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// push static 1
@static
D=M
@1
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
// push static 8
@static
D=M
@8
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
