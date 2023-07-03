//init 'stack' pointer
@256
D=A
@SP
M=D
//init 'local' pointer
@300
D=A
@LCL
M=D
//init 'argument' pointer
@400
D=A
@ARG
M=D
//init 'this' pointer
@3000
D=A
@THIS
M=D
//init 'that' pointer
@3010
D=A
@THAT
M=D
//push constant 7
@7
D=A
@SP
A=M
M=D
@SP
AM=M+1
//push constant 8
@8
D=A
@SP
A=M
M=D
@SP
AM=M+1
//add
@SP
AM=M-1
D=M
@SP
AM=M-1
M=D+M
@SP
AM=M+1
(INFINITE_LOOP)
@INFINITE_LOOP
0;JMP