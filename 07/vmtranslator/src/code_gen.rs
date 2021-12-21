use super::parser::*;
use std::io::{ Write };

fn write_get_num<F: Write>(fname: &str, target: &mut F, loc: &Location, num: &u16) {
    match loc {
        Location::Local => {
            write!(target, "@LCL\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = M\n");
        },
        Location::Argument => {
            write!(target, "@ARG\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = M\n");
        },
        Location::This => {
            write!(target, "@THIS\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = M\n");
        },
        Location::That => {
            write!(target, "@THAT\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = M\n");
        },

        Location::Constant => {
            write!(target, "@{}\n", num);
            write!(target, "D = A\n");
        },

        Location::Static => {
            write!(target, "@{}.{}\n", fname, num);
            write!(target, "D = M\n");
        },
        Location::Pointer => {
            if *num == 1 {
                write!(target, "@THAT\n");
                write!(target, "D = M\n");
            } else if *num == 0 {
                write!(target, "@THIS\n");
                write!(target, "D = M\n");
            } else {
                panic!("Pointer should be 0/1");
            }
        },
        Location::Temp => {
            if *num > 7u16 {
                println!("Warning: temp is {} > 7", num);
            }
            write!(target, "@{}\n", num + 5);
            write!(target, "D = M\n");
        },
    };
}

fn write_get_addr<F: Write>(fname: &str, target: &mut F, loc: &Location, num: &u16) {
    match loc {
        Location::Local => {
            write!(target, "@LCL\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = A\n");
        },
        Location::Argument => {
            write!(target, "@ARG\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = A\n");
        },
        Location::This => {
            write!(target, "@THIS\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = A\n");
        },
        Location::That => {
            write!(target, "@THAT\n");
            write!(target, "D = M\n");
            write!(target, "@{}\n", num);
            write!(target, "A = D + A\n");
            write!(target, "D = A\n");
        },

        Location::Constant => {
            println!("Constant cannot be used in pop");
        },

        Location::Static => {
            write!(target, "@{}.{}\n", fname, num);
            write!(target, "D = A\n");
        },
        Location::Pointer => {
            if *num == 1 {
                write!(target, "@THAT\n");
                write!(target, "D = A\n");
            } else if *num == 0 {
                write!(target, "@THIS\n");
                write!(target, "D = A\n");
            } else {
                panic!("Pointer should be 0/1");
            }
        },
        Location::Temp => {
            if *num > 7u16 {
                println!("Warning: temp is {} > 7", num);
            }
            write!(target, "@{}\n", num + 5);
            write!(target, "D = A\n");
        },
    };
}

macro_rules! write_branch {
    ($target: expr, $asm:expr, $label: expr) => {
        write!($target, "D = A - D\n");
        write!($target, "@TRANS_LABEL${}\n", $label);
        write!($target, "D;{}\n", $asm);
        write!($target, "D = 0\n");
        write!($target, "@TRANS_LABEL_END${}\n", $label);
        write!($target, "0;JMP\n");
        write!($target, "(TRANS_LABEL${})\n", $label);
        write!($target, "D = -1\n");
        write!($target, "(TRANS_LABEL_END${})\n", $label);
        $label += 1;
    };
}

pub fn code_gen<F: Write>(ast: &Vec<AST>, fname: &str, target: &mut F) {
    let mut label: usize = 0;
    for line in ast {
        write!(target, "// {:?}\n", line);
        match line {
            AST::StackOp(StackOperator::Push, loc, num) => {
                write_get_num(fname, target, &loc, &num);
                write!(target, "@SP\n");
                write!(target, "A = M\n");
                write!(target, "M = D\n");
                write!(target, "@SP\n");
                write!(target, "M = M + 1\n");
            },
            AST::StackOp(StackOperator::Pop, loc, num) => {
                write_get_addr(fname, target, &loc, &num);
                write!(target, "@SP\n");
                write!(target, "M = M - 1\n");
                // write!(target, "@SP\n");
                write!(target, "A = M\n");
                write!(target, "A = M\n");
                write!(target, "D = D + A\n"); // swap A and D
                write!(target, "A = D - A\n");
                write!(target, "M = D - A\n");
            },
            AST::SingleOp(SingleOperator::Not) => {
                write!(target, "@SP\n");
                write!(target, "M = M - 1\n");
                write!(target, "A = M\n");
                write!(target, "D = !M\n");

                write!(target, "@SP\n");
                write!(target, "A = M\n");
                write!(target, "M = D\n");
                write!(target, "@SP\n");
                write!(target, "M = M + 1\n");
            },
            AST::SingleOp(SingleOperator::Neg) => {
                write!(target, "@SP\n");
                write!(target, "M = M - 1\n");
                write!(target, "A = M\n");
                write!(target, "D = -M\n");

                write!(target, "@SP\n");
                write!(target, "A = M\n");
                write!(target, "M = D\n");
                write!(target, "@SP\n");
                write!(target, "M = M + 1\n");
            },
            AST::SingleOp(op) => {
                write!(target, "@SP\n");
                write!(target, "M = M - 1\n");
                write!(target, "A = M\n");
                write!(target, "D = M\n");
                write!(target, "@SP\n");
                write!(target, "M = M - 1\n");
                write!(target, "A = M\n");
                write!(target, "A = M\n");

                match op {
                    SingleOperator::Add => {
                        write!(target, "D = A + D\n");
                    },
                    SingleOperator::Sub => {
                        write!(target, "D = A - D\n");
                    },
                    SingleOperator::And => {
                        write!(target, "D = A & D\n");
                    },
                    SingleOperator::Or => {
                        write!(target, "D = A | D\n");
                    },
                    SingleOperator::Eq => {
                        write_branch!(target, "JEQ", label);
                    },
                    SingleOperator::Lt => {
                        write_branch!(target, "JLT", label);
                    },
                    SingleOperator::Gt => {
                        write_branch!(target, "JGT", label);
                    },
                    _ => {
                        panic!("Should not reach here");
                    }
                };
                write!(target, "@SP\n");
                write!(target, "A = M\n");
                write!(target, "M = D\n");
                write!(target, "@SP\n");
                write!(target, "M = M + 1\n");
            }
        }
    }
}
