use super::parser::*;
use std::io::{ Write };

fn write_number(target: &mut [u8], mut num: u16) {
    let mut p = target.len();
    if num > ((1 << p) - 1) {
        panic!("Buf Overflow");
    }
    while num != 0 {
        p -= 1;
        target[p] = if num & 1 == 0 { '0' as u8 } else { '1' as u8 };
        num >>= 1;
    }
}

pub fn code_gen<F: Write>(ast: &Vec<AST>, target: &mut F) {
    // first scan: collect labels
    for line in ast {
        let mut code: [u8; 17] = ['0' as u8; 17];
        match line {
            AST::InsA(InsA{ value: Value::Number(n) }) => {
                code[0] = '0' as u8;
                write_number(&mut code[1..16], *n);
            },
            AST::InsA(InsA{ value: Value::Ident(id) }) => {
                panic!("Unsolved ident {}", id);
            },
            AST::InsC(InsC{ dest, comp, jump}) => {
                code[0] = '1' as u8;
                code[1] = '1' as u8;
                code[2] = '1' as u8;
                write_number(&mut code[10..13], *dest as u16);
                write_number(&mut code[13..16], *jump as u16);
                match comp {
                    Comp{left: None, op: None, right: Source::Zero} => {
                        write_number(&mut code[3..10], 0b0101010);
                    },
                    Comp{left: None, op: None, right: Source::One} => {
                        write_number(&mut code[3..10], 0b0111111);
                    },
                    Comp{left: None, op: Some(Op::Sub), right: Source::One} => {
                        write_number(&mut code[3..10], 0b0111010);
                    },
                    Comp{left: None, op: None, right: Source::D} => {
                        write_number(&mut code[3..10], 0b0001100);
                    },
                    Comp{left: None, op: None, right: Source::A} => {
                        write_number(&mut code[3..10], 0b0110000);
                    },
                    Comp{left: None, op: Some(Op::Not), right: Source::D} => {
                        write_number(&mut code[3..10], 0b0001101);
                    },
                    Comp{left: None, op: Some(Op::Not), right: Source::A} => {
                        write_number(&mut code[3..10], 0b0110001);
                    },
                    Comp{left: None, op: Some(Op::Sub), right: Source::D} => {
                        write_number(&mut code[3..10], 0b0001111);
                    },
                    Comp{left: None, op: Some(Op::Sub), right: Source::A} => {
                        write_number(&mut code[3..10], 0b0110011);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Add), right: Source::One} => {
                        write_number(&mut code[3..10], 0b0011111);
                    },
                    Comp{left: Some(Source::A), op: Some(Op::Add), right: Source::One} => {
                        write_number(&mut code[3..10], 0b0110111);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Sub), right: Source::One} => {
                        write_number(&mut code[3..10], 0b0001110);
                    },
                    Comp{left: Some(Source::A), op: Some(Op::Sub), right: Source::One} => {
                        write_number(&mut code[3..10], 0b0110010);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Add), right: Source::A} => {
                        write_number(&mut code[3..10], 0b0000010);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Sub), right: Source::A} => {
                        write_number(&mut code[3..10], 0b0010011);
                    },
                    Comp{left: Some(Source::A), op: Some(Op::Sub), right: Source::D} => {
                        write_number(&mut code[3..10], 0b0000111);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::And), right: Source::A} => {
                        write_number(&mut code[3..10], 0b0000000);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Or), right: Source::A} => {
                        write_number(&mut code[3..10], 0b0010101);
                    },
                    
                    Comp{left: None, op: None, right: Source::M} => {
                        write_number(&mut code[3..10], 0b1110000);
                    },
                    Comp{left: None, op: Some(Op::Not), right: Source::M} => {
                        write_number(&mut code[3..10], 0b1110001);
                    },
                    Comp{left: None, op: Some(Op::Sub), right: Source::M} => {
                        write_number(&mut code[3..10], 0b1110011);
                    },
                    Comp{left: Some(Source::M), op: Some(Op::Add), right: Source::One} => {
                        write_number(&mut code[3..10], 0b1110111);
                    },
                    Comp{left: Some(Source::M), op: Some(Op::Sub), right: Source::One} => {
                        write_number(&mut code[3..10], 0b1110010);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Add), right: Source::M} => {
                        write_number(&mut code[3..10], 0b1000010);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Sub), right: Source::M} => {
                        write_number(&mut code[3..10], 0b1010011);
                    },
                    Comp{left: Some(Source::M), op: Some(Op::Sub), right: Source::D} => {
                        write_number(&mut code[3..10], 0b1000111);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::And), right: Source::M} => {
                        write_number(&mut code[3..10], 0b1000000);
                    },
                    Comp{left: Some(Source::D), op: Some(Op::Or), right: Source::M} => {
                        write_number(&mut code[3..10], 0b1010101);
                    },
                    _ => { panic!("Invalid ins {:?}", comp); },
                }
                // 0    0b0101010
                // 1    0b0111111
                // -1   0b0111010
                // D    0b0001100
                // A    0b0110000
                // !D   0b0001101
                // !A   0b0110001
                // -D   0b0001111
                // -A   0b0110011
                // D+1  0b0011111
                // A+1  0b0110111
                // D-1  0b0001110
                // A-1  0b0110010
                // D+A  0b0000010
                // D-A  0b0010011
                // A-D  0b0000111
                // D&A  0b0000000
                // D|A  0b0010101
                
                // 0b1110000 M
                // 0b1110001 !M
                // 0b1110011 -M
                // 0b1110111 M+1
                // 0b1110010 M-1
                // 0b1000010 D+M
                // 0b1010011 D-M
                // 0b1000111 M-D
                // 0b1000000 D&M
                // 0b1010101 D|M
            },
        };
        code[16] = '\n' as u8;
        target.write(&code).unwrap();
    }
}
