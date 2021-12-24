// StackOp(Push, Constant, 0)
@0
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Local, 0)
@LCL
D = M
@0
A = D + A
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// BranchOp(Label, "LOOP_START")
(BasicLoop.LABEL$LOOP_START)
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
// StackOp(Pop, Local, 0)
@LCL
D = M
@0
A = D + A
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
// StackOp(Pop, Argument, 0)
@ARG
D = M
@0
A = D + A
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
// BranchOp(If, "LOOP_START")
@SP
M = M - 1
A = M
D = M
@BasicLoop.LABEL$LOOP_START
D;JNE
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
