// StackOp(Push, Constant, 10)
@10
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
// StackOp(Push, Constant, 21)
@21
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Constant, 22)
@22
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Argument, 2)
@ARG
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
// StackOp(Pop, Argument, 1)
@ARG
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
// StackOp(Push, Constant, 36)
@36
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, This, 6)
@THIS
D = M
@6
A = D + A
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Constant, 42)
@42
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Constant, 45)
@45
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, That, 5)
@THAT
D = M
@5
A = D + A
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
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
// StackOp(Push, Constant, 510)
@510
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Temp, 6)
@11
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
// StackOp(Push, That, 5)
@THAT
D = M
@5
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
// StackOp(Push, This, 6)
@THIS
D = M
@6
A = D + A
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, This, 6)
@THIS
D = M
@6
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
// StackOp(Push, Temp, 6)
@11
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
