// StackOp(Push, Constant, 7)
@7
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
