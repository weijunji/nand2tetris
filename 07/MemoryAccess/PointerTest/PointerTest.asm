// StackOp(Push, Constant, 3030)
@3030
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
// StackOp(Push, Constant, 3040)
@3040
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
// StackOp(Push, Constant, 32)
@32
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, This, 2)
@THIS
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
// StackOp(Push, Constant, 46)
@46
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, That, 6)
@THAT
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
// StackOp(Push, Pointer, 0)
@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Pointer, 1)
@THAT
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
// StackOp(Push, This, 2)
@THIS
D = M
@2
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
// StackOp(Push, That, 6)
@THAT
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
