// StackOp(Push, Constant, 111)
@111
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Constant, 333)
@333
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Constant, 888)
@888
D = A
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Pop, Static, 8)
@StaticTest.8
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Pop, Static, 3)
@StaticTest.3
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Pop, Static, 1)
@StaticTest.1
D = A
@SP
M = M - 1
A = M
A = M
D = D + A
A = D - A
M = D - A
// StackOp(Push, Static, 3)
@StaticTest.3
D = M
@SP
A = M
M = D
@SP
M = M + 1
// StackOp(Push, Static, 1)
@StaticTest.1
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
// StackOp(Push, Static, 8)
@StaticTest.8
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
