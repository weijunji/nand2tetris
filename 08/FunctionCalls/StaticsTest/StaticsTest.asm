@Sys.init
0;JMP
// FunctionOp(Func, "Class1.set", 0)
(Class1.set)
// StackOp(Push, Argument, 0)
@ARG
D = M
@0
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Static, 0)
@Class1.0
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Argument, 1)
@ARG
D = M
@1
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Static, 1)
@Class1.1
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 0)
@0
D = A
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Ret)
@ARG
D = M
@R15
M = D
@SP
M = M - 1
A = M
D = M
@R14
M = D
@LCL
D = M
@SP
M = D
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@ARG
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@LCL
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@SP
M = M - 1
A = M
D = M
@R13
M = D
@R14
D = M
@R15
A = M
M = D
@R15
D = M + 1
@SP
M = D
@R13
A = M
0;JMP
// FunctionOp(Func, "Class1.get", 0)
(Class1.get)
// StackOp(Push, Static, 0)
@Class1.0
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Static, 1)
@Class1.1
D = M
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Sub)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A - D
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Ret)
@ARG
D = M
@R15
M = D
@SP
M = M - 1
A = M
D = M
@R14
M = D
@LCL
D = M
@SP
M = D
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@ARG
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@LCL
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@SP
M = M - 1
A = M
D = M
@R13
M = D
@R14
D = M
@R15
A = M
M = D
@R15
D = M + 1
@SP
M = D
@R13
A = M
0;JMP
// FunctionOp(Func, "Class2.set", 0)
(Class2.set)
// StackOp(Push, Argument, 0)
@ARG
D = M
@0
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Static, 0)
@Class2.0
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Argument, 1)
@ARG
D = M
@1
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Static, 1)
@Class2.1
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 0)
@0
D = A
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Ret)
@ARG
D = M
@R15
M = D
@SP
M = M - 1
A = M
D = M
@R14
M = D
@LCL
D = M
@SP
M = D
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@ARG
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@LCL
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@SP
M = M - 1
A = M
D = M
@R13
M = D
@R14
D = M
@R15
A = M
M = D
@R15
D = M + 1
@SP
M = D
@R13
A = M
0;JMP
// FunctionOp(Func, "Class2.get", 0)
(Class2.get)
// StackOp(Push, Static, 0)
@Class2.0
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Static, 1)
@Class2.1
D = M
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Sub)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A - D
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Ret)
@ARG
D = M
@R15
M = D
@SP
M = M - 1
A = M
D = M
@R14
M = D
@LCL
D = M
@SP
M = D
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@ARG
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@LCL
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
@SP
M = M - 1
A = M
D = M
@R13
M = D
@R14
D = M
@R15
A = M
M = D
@R15
D = M + 1
@SP
M = D
@R13
A = M
0;JMP
// FunctionOp(Func, "Sys.init", 0)
(Sys.init)
// StackOp(Push, Constant, 6)
@6
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Constant, 8)
@8
D = A
@SP
A = M
M = D
@SP
M = M + 1
// FunctionOp(Call, "Class1.set", 2)
@Sys$retAddr0
D = A
@SP
A = M
M = D
@SP
M = M + 1
@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1
@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1
@SP
D = M
@7
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Class1.set
0;JMP
(Sys$retAddr0)
// StackOp(Pop, Temp, 0)
@5
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 23)
@23
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Constant, 15)
@15
D = A
@SP
A = M
M = D
@SP
M = M + 1
// FunctionOp(Call, "Class2.set", 2)
@Sys$retAddr1
D = A
@SP
A = M
M = D
@SP
M = M + 1
@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1
@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1
@SP
D = M
@7
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Class2.set
0;JMP
(Sys$retAddr1)
// StackOp(Pop, Temp, 0)
@5
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// FunctionOp(Call, "Class1.get", 0)
@Sys$retAddr2
D = A
@SP
A = M
M = D
@SP
M = M + 1
@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1
@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1
@SP
D = M
@5
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Class1.get
0;JMP
(Sys$retAddr2)
// FunctionOp(Call, "Class2.get", 0)
@Sys$retAddr3
D = A
@SP
A = M
M = D
@SP
M = M + 1
@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1
@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1
@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1
@SP
D = M
@5
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Class2.get
0;JMP
(Sys$retAddr3)
// BranchOp(Label, "WHILE")
(Sys.LABEL$WHILE)
// BranchOp(Goto, "WHILE")
@Sys.LABEL$WHILE
0;JMP
