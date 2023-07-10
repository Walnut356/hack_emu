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
// // File name: projects/07/StackArithmetic/StackTest/StackTest.vm
// 
// // Executes a sequence of arithmetic and logical operations
// // on the stack. 
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
AM=M+1
// eq
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@EQ_1
D;JEQ
@SP
A=M-1
M=0
(EQ_1)
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
AM=M+1
// eq
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@EQ_2
D;JEQ
@SP
A=M-1
M=0
(EQ_2)
// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
AM=M+1
// eq
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@EQ_3
D;JEQ
@SP
A=M-1
M=0
(EQ_3)
// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
AM=M+1
// lt
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@LT_1
D;JLT
@SP
A=M-1
M=0
(LT_1)
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
AM=M+1
// lt
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@LT_2
D;JLT
@SP
A=M-1
M=0
(LT_2)
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
AM=M+1
// lt
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@LT_3
D;JLT
@SP
A=M-1
M=0
(LT_3)
// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
AM=M+1
// gt
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@GT_1
D;JGT
@SP
A=M-1
M=0
(GT_1)
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
AM=M+1
// gt
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@GT_2
D;JGT
@SP
A=M-1
M=0
(GT_2)
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
AM=M+1
// gt
@SP
AM=M-1
D=M
A=A-1
D=M-D
M=-1
@GT_3
D;JGT
@SP
A=M-1
M=0
(GT_3)
// push constant 57
@57
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 31
@31
D=A
@SP
A=M
M=D
@SP
AM=M+1
// push constant 53
@53
D=A
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
// push constant 112
@112
D=A
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
// neg
@SP
A=M-1
M=-M
// and
@SP
AM=M-1
D=M
@SP
A=M-1
M=D&M
// push constant 82
@82
D=A
@SP
A=M
M=D
@SP
AM=M+1
// or
@SP
AM=M-1
D=M
@SP
A=M-1
M=D|M
// not
@SP
A=M-1
M=!M
