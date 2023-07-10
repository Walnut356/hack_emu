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
// // File name: projects/08/FunctionCalls/FibonacciElement/Main.vm
// 
// // Computes the n'th element of the Fibonacci series, recursively.
// // n is given in argument[0].  Called by the Sys.init function 
// // (part of the Sys.vm file), which also pushes the argument[0] 
// // parameter before this code starts running.
// 
// function Main.fibonacci 0
(Main.fibonacci)
// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
AM=M+1
// lt                     // checks if n<2
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
// if-goto IF_TRUE
@SP
AM=M-1
D=M
@IF_TRUE
D;JNE
// goto IF_FALSE
@IF_FALSE
0;JMP

// label IF_TRUE          // if n<2, return n
(IF_TRUE)
// push argument 0        
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// return
@LCL
D=M
@R15
M=D
@5
A=D-A
D=M
@R14
M=D
@ARG
D=M
@0
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
@ARG
D=M+1
@SP
M=D
@R15
D=M
@1
A=D-A
D=M
@THAT
M=D
@R15
D=M
@2
A=D-A
D=M
@THIS
M=D
@R15
D=M
@3
A=D-A
D=M
@ARG
M=D
@R15
D=M
@4
A=D-A
D=M
@LCL
M=D
@R14
A=M
0;JMP
// label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1)
(IF_FALSE)
// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// push constant 2
@2
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
// call Main.fibonacci 1  // computes fib(n-2)
@Main.fibonacci$ret0
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
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$ret0)
// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
AM=M+1
// push constant 1
@1
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
// call Main.fibonacci 1  // computes fib(n-1)
@Main.fibonacci$ret1
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
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$ret1)
// add                    // returns fib(n-1) + fib(n-2)
@SP
AM=M-1
D=M
@SP
A=M-1
M=D+M
// return
@LCL
D=M
@R15
M=D
@5
A=D-A
D=M
@R14
M=D
@ARG
D=M
@0
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
@ARG
D=M+1
@SP
M=D
@R15
D=M
@1
A=D-A
D=M
@THAT
M=D
@R15
D=M
@2
A=D-A
D=M
@THIS
M=D
@R15
D=M
@3
A=D-A
D=M
@ARG
M=D
@R15
D=M
@4
A=D-A
D=M
@LCL
M=D
@R14
A=M
0;JMP
// // This file is part of www.nand2tetris.org
// // and the book "The Elements of Computing Systems"
// // by Nisan and Schocken, MIT Press.
// // File name: projects/08/FunctionCalls/FibonacciElement/Sys.vm
// 
// // Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
// // function, which computes the n'th element of the Fibonacci series.
// // Note that by convention, the Sys.init function is called "automatically" 
// // by the bootstrap code.
// 
// function Sys.init 0
(Sys.init)
// push constant 4
@4
D=A
@SP
A=M
M=D
@SP
AM=M+1
// call Main.fibonacci 1   // computes the 4'th fibonacci element
@Main.fibonacci$ret2
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
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci$ret2)
// label WHILE
(WHILE)
// goto WHILE              // loops infinitely
@WHILE
0;JMP
