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
@SP
AM=M-1
D=M
@StaticTest.8
M=D
// pop static 3
@SP
AM=M-1
D=M
@StaticTest.3
M=D
// pop static 1
@SP
AM=M-1
D=M
@StaticTest.1
M=D
// push static 3
@StaticTest.3
D=M
@SP
A=M
M=D
@SP
AM=M+1
// push static 1
@StaticTest.1
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
@StaticTest.8
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
