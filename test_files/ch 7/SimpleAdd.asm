//init 'stack' pointer
@256
D=A
@SP
M=D
//call Sys.init
@Sys.init
0;JMP
(Sys.init$ret0)
(INFINITE_LOOP)
@INFINITE_LOOP
0;JMP
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
