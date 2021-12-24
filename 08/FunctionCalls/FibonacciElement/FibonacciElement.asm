@Sys.init
0;JMP
// FunctionOp(Func, "Main.fibonacci", 0)
(Main.fibonacci)
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
// StackOp(Push, Constant, 2)
@2
D = A
@SP
A = M
M = D
@SP
M = M + 1
// SingleOp(Lt)
@SP
M = M - 1
A = M
D = M
@SP
M = M - 1
A = M
A = M
D = A - D
@Main.TRANS_LABEL$0
D;JLT
D = 0
@Main.TRANS_LABEL_END$0
0;JMP
(Main.TRANS_LABEL$0)
D = -1
(Main.TRANS_LABEL_END$0)
@SP
A = M
M = D
@SP
M = M + 1
// BranchOp(If, "IF_TRUE")
@SP
M = M - 1
A = M
D = M
@Main.LABEL$IF_TRUE
D;JNE
// BranchOp(Goto, "IF_FALSE")
@Main.LABEL$IF_FALSE
0;JMP
// BranchOp(Label, "IF_TRUE")
(Main.LABEL$IF_TRUE)
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
// BranchOp(Label, "IF_FALSE")
(Main.LABEL$IF_FALSE)
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
// StackOp(Push, Constant, 2)
@2
D = A
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
// FunctionOp(Call, "Main.fibonacci", 1)
@Main$retAddr1
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
@Main.fibonacci
0;JMP
(Main$retAddr1)
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
// StackOp(Push, Constant, 1)
@1
D = A
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
// FunctionOp(Call, "Main.fibonacci", 1)
@Main$retAddr2
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
@Main.fibonacci
0;JMP
(Main$retAddr2)
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
// FunctionOp(Func, "Sys.init", 0)
(Sys.init)
// StackOp(Push, Constant, 4)
@4
D = A
@SP
A = M
M = D
@SP
M = M + 1
// FunctionOp(Call, "Main.fibonacci", 1)
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
@6
D = D - A
@ARG
M = D
@SP
D = M
@LCL
M = D
@Main.fibonacci
0;JMP
(Sys$retAddr0)
// BranchOp(Label, "WHILE")
(Sys.LABEL$WHILE)
// BranchOp(Goto, "WHILE")
@Sys.LABEL$WHILE
0;JMP
