@Sys.init
0;JMP
// FunctionOp(Func, "Sys.init", 0)
(Sys.init)
// StackOp(Push, Constant, 4000)
@4000
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Pointer, 0)
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 5000)
@5000
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Pointer, 1)
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// FunctionOp(Call, "Sys.main", 0)
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
@5
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Sys.main
0;JMP
(Sys$retAddr0)
// StackOp(Pop, Temp, 1)
@6
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// BranchOp(Label, "LOOP")
(Sys.LABEL$LOOP)
// BranchOp(Goto, "LOOP")
@Sys.LABEL$LOOP
0;JMP
// FunctionOp(Func, "Sys.main", 5)
(Sys.main)
@5
D = A
(Sys.Sys.main$INIT_LOOP)
D = D - 1
@Sys.Sys.main$INIT_END
D;JLT
@SP
A = M
M = 0
@SP
M = M + 1
@Sys.Sys.main$INIT_LOOP
0;JMP
(Sys.Sys.main$INIT_END)
// StackOp(Push, Constant, 4001)
@4001
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Pointer, 0)
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 5001)
@5001
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Pointer, 1)
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 200)
@200
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Local, 1)
@LCL
D = M
@1
A = D + A
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 40)
@40
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Local, 2)
@LCL
D = M
@2
A = D + A
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 6)
@6
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Local, 3)
@LCL
D = M
@3
A = D + A
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 123)
@123
D = A
@SP
A = M
M = D
@SP
M = M + 1
// FunctionOp(Call, "Sys.add12", 1)
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
@6
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Sys.add12
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
// StackOp(Push, Local, 0)
@LCL
D = M
@0
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Local, 1)
@LCL
D = M
@1
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Local, 2)
@LCL
D = M
@2
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Local, 3)
@LCL
D = M
@3
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Local, 4)
@LCL
D = M
@4
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Add)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A + D
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Add)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A + D
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Add)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A + D
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Add)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A + D
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
// FunctionOp(Func, "Sys.add12", 0)
(Sys.add12)
// StackOp(Push, Constant, 4002)
@4002
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Pointer, 0)
@THIS
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 5002)
@5002
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Pointer, 1)
@THAT
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
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
// StackOp(Push, Constant, 12)
@12
D = A
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Add)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A + D
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
