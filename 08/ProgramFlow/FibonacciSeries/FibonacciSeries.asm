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
// StackOp(Push, Constant, 0)
@0
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, That, 0)
@THAT
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
// StackOp(Push, Constant, 1)
@1
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, That, 1)
@THAT
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
// BranchOp(Label, "MAIN_LOOP_START")
(FibonacciSeries.LABEL$MAIN_LOOP_START)
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
// BranchOp(If, "COMPUTE_ELEMENT")
@SP
M = M - 1
A = M
D = M
@FibonacciSeries.LABEL$COMPUTE_ELEMENT
D;JNE
// BranchOp(Goto, "END_PROGRAM")
@FibonacciSeries.LABEL$END_PROGRAM
0;JMP
// BranchOp(Label, "COMPUTE_ELEMENT")
(FibonacciSeries.LABEL$COMPUTE_ELEMENT)
// StackOp(Push, That, 0)
@THAT
D = M
@0
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, That, 1)
@THAT
D = M
@1
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
// StackOp(Pop, That, 2)
@THAT
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
// StackOp(Push, Pointer, 1)
@THAT
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
// BranchOp(Goto, "MAIN_LOOP_START")
@FibonacciSeries.LABEL$MAIN_LOOP_START
0;JMP
// BranchOp(Label, "END_PROGRAM")
(FibonacciSeries.LABEL$END_PROGRAM)
