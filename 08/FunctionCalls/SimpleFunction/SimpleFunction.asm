// FunctionOp(Func, "SimpleFunction.test", 2)
(SimpleFunction.test)
@2
D = A
(SimpleFunction.SimpleFunction.test$INIT_LOOP)
D = D - 1
@SimpleFunction.SimpleFunction.test$INIT_END
D;JLT
@SP
A = M
M = 0
@SP
M = M + 1
@SimpleFunction.SimpleFunction.test$INIT_LOOP
0;JMP
(SimpleFunction.SimpleFunction.test$INIT_END)
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
// SingleOp(Not)
@SP
M = M - 1
A = M
D = !M
@SP
A = M
M = D
@SP
M = M + 1
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
// SingleOp(Ret)
@ARG
D = M
@R15
M = D
@SP
M = M - 1
A = M
D = M
@ARG
A = M
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
@R14
M = D
@R15
D = M + 1
@SP
M = D
@R14
A = M
0;JMP
