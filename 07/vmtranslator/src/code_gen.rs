use super::parser::*;
use std::io::{ Write };

fn write_get_num<F: Write>(fname: &str, target: &mut F, loc: &Location, num: &u16) {
    match loc {
        Location::Local => {
            write!(target, "@LCL\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = M\n").unwrap();
        },
        Location::Argument => {
            write!(target, "@ARG\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = M\n").unwrap();
        },
        Location::This => {
            write!(target, "@THIS\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = M\n").unwrap();
        },
        Location::That => {
            write!(target, "@THAT\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = M\n").unwrap();
        },

        Location::Constant => {
            write!(target, "@{}\n", num).unwrap();
            write!(target, "D = A\n").unwrap();
        },

        Location::Static => {
            write!(target, "@{}.{}\n", fname, num).unwrap();
            write!(target, "D = M\n").unwrap();
        },
        Location::Pointer => {
            if *num == 1 {
                write!(target, "@THAT\n").unwrap();
                write!(target, "D = M\n").unwrap();
            } else if *num == 0 {
                write!(target, "@THIS\n").unwrap();
                write!(target, "D = M\n").unwrap();
            } else {
                panic!("Pointer should be 0/1");
            }
        },
        Location::Temp => {
            if *num > 7u16 {
                println!("Warning: temp is {} > 7", num);
            }
            write!(target, "@{}\n", num + 5).unwrap();
            write!(target, "D = M\n").unwrap();
        },
    };
}

fn write_get_addr<F: Write>(fname: &str, target: &mut F, loc: &Location, num: &u16) {
    match loc {
        Location::Local => {
            write!(target, "@LCL\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = A\n").unwrap();
        },
        Location::Argument => {
            write!(target, "@ARG\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = A\n").unwrap();
        },
        Location::This => {
            write!(target, "@THIS\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = A\n").unwrap();
        },
        Location::That => {
            write!(target, "@THAT\n").unwrap();
            write!(target, "D = M\n").unwrap();
            write!(target, "@{}\n", num).unwrap();
            write!(target, "A = D + A\n").unwrap();
            write!(target, "D = A\n").unwrap();
        },

        Location::Constant => {
            println!("Constant cannot be used in pop");
        },

        Location::Static => {
            write!(target, "@{}.{}\n", fname, num).unwrap();
            write!(target, "D = A\n").unwrap();
        },
        Location::Pointer => {
            if *num == 1 {
                write!(target, "@THAT\n").unwrap();
                write!(target, "D = A\n").unwrap();
            } else if *num == 0 {
                write!(target, "@THIS\n").unwrap();
                write!(target, "D = A\n").unwrap();
            } else {
                panic!("Pointer should be 0/1");
            }
        },
        Location::Temp => {
            if *num > 7u16 {
                println!("Warning: temp is {} > 7", num);
            }
            write!(target, "@{}\n", num + 5).unwrap();
            write!(target, "D = A\n").unwrap();
        },
    };
}

macro_rules! write_branch {
    ($target: expr, $asm:expr, $label: expr) => {
        write!($target, "D = A - D\n").unwrap();
        write!($target, "@TRANS_LABEL${}\n", $label).unwrap();
        write!($target, "D;{}\n", $asm).unwrap();
        write!($target, "D = 0\n").unwrap();
        write!($target, "@TRANS_LABEL_END${}\n", $label).unwrap();
        write!($target, "0;JMP\n").unwrap();
        write!($target, "(TRANS_LABEL${})\n", $label).unwrap();
        write!($target, "D = -1\n").unwrap();
        write!($target, "(TRANS_LABEL_END${})\n", $label).unwrap();
        $label += 1;
    };
}

pub fn code_gen<F: Write>(ast: &Vec<AST>, fname: &str, target: &mut F) {
    let mut label: usize = 0;
    for line in ast {
        write!(target, "// {:?}\n", line).unwrap();
        match line {
            AST::StackOp(StackOperator::Push, loc, num) => {
                write_get_num(fname, target, &loc, &num);
                write!(target, "@SP\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "M = D\n").unwrap();
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M + 1\n").unwrap();
            },
            AST::StackOp(StackOperator::Pop, loc, num) => {
                write_get_addr(fname, target, &loc, &num);
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M - 1\n").unwrap();
                // write!(target, "@SP\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "D = D + A\n").unwrap(); // swap A and D
                write!(target, "A = D - A\n").unwrap();
                write!(target, "M = D - A\n").unwrap();
            },
            AST::SingleOp(SingleOperator::Not) => {
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M - 1\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "D = !M\n").unwrap();

                write!(target, "@SP\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "M = D\n").unwrap();
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M + 1\n").unwrap();
            },
            AST::SingleOp(SingleOperator::Neg) => {
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M - 1\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "D = -M\n").unwrap();

                write!(target, "@SP\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "M = D\n").unwrap();
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M + 1\n").unwrap();
            },
            AST::SingleOp(op) => {
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M - 1\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "D = M\n").unwrap();
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M - 1\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "A = M\n").unwrap();

                match op {
                    SingleOperator::Add => {
                        write!(target, "D = A + D\n").unwrap();
                    },
                    SingleOperator::Sub => {
                        write!(target, "D = A - D\n").unwrap();
                    },
                    SingleOperator::And => {
                        write!(target, "D = A & D\n").unwrap();
                    },
                    SingleOperator::Or => {
                        write!(target, "D = A | D\n").unwrap();
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
                write!(target, "@SP\n").unwrap();
                write!(target, "A = M\n").unwrap();
                write!(target, "M = D\n").unwrap();
                write!(target, "@SP\n").unwrap();
                write!(target, "M = M + 1\n").unwrap();
            }
        }
    }
}
