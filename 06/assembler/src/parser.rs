use super::lexer::{ self, TokenType, Token };
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Number(u16),
    Ident(String),
}

/// Instruction Type A
/// @value
#[derive(Debug)]
pub struct InsA {
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub enum Dest {
    NULL = 0b000,
    M = 0b001,
    D = 0b010,
    MD = 0b011,
    A = 0b100,
    AM = 0b101,
    AD = 0b110,
    AMD = 0b111,
}

#[derive(Debug)]
pub enum Source {
    D,
    A,
    M,
    Zero,
    One,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Not,
    Or,
    And,
}

#[derive(Debug)]
pub struct Comp {
    pub left: Option<Source>,
    pub op: Option<Op>,
    pub right: Source,
}

#[derive(Debug, Clone, Copy)]
pub enum Jump {
    NULL = 0b000,
    JGT = 0b001,
    JEQ = 0b010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

/// Instruction Type C
/// dest=comp
/// comp;Jump
#[derive(Debug)]
pub struct InsC {
    pub dest: Dest,
    pub comp: Comp,
    pub jump: Jump,
}

#[derive(Debug)]
pub enum AST {
    InsA(InsA),
    InsC(InsC),
}

fn print_error(token: &Token) {
    println!("Unexpected token {:?} near {}:{}", token.token, token.line, token.ch);
}

/// eat token until new line
/// if `err_bad_token` is `true`, will print an error
/// while token is not comment or new line
fn eat_line<T: std::io::Read>(token: &mut lexer::TokenStream<T>, err_bad_token: bool) {
    loop {
        match token.next() {
            Some(t) => {
                match t.token {
                    TokenType::Comment(_) => continue,
                    TokenType::NewLine => break,
                    _ => if err_bad_token {
                            print_error(&t);
                        }
                }
            },
            None => break,
        }
    }
}

fn error<T: std::io::Read>(token: &mut lexer::TokenStream<T>, cur: &Token) {
    print_error(&cur);
    eat_line(token, false);
}

fn error_eof() {
    println!("Unexpected token near end of file");
}

/// eat ;JUMP
fn eat_jump<T: std::io::Read>(token: &mut lexer::TokenStream<T>) -> Option<Jump> {
    match token.next() {
        Some(Token{ token: TokenType::Symbol(';'), line: _, ch: _ }) => {
            match token.next() {
                Some(Token{ token: TokenType::Ident(ident), line: l, ch: c }) => {
                    match ident.as_str() {
                        "JGT" => Some(Jump::JGT),
                        "JEQ" => Some(Jump::JEQ),
                        "JGE" => Some(Jump::JGE),
                        "JLT" => Some(Jump::JLT),
                        "JNE" => Some(Jump::JNE),
                        "JLE" => Some(Jump::JLE),
                        "JMP" => Some(Jump::JMP),
                        _ => {
                            println!("Unexpected jump type {:?} near {}:{}", ident, l, c);
                            None
                        }
                    }
                },
                Some(t) => {
                    error(token, &t);
                    None
                }
                None => {
                    error_eof();
                    None
                }
            }
        },
        Some(t) => {
            error(token, &t);
            None
        },
        None => {
            error_eof();
            None
        }
    }
}

fn eat_source<T: std::io::Read>(token: &mut lexer::TokenStream<T>, eat_number: bool) -> Option<Source> {
    match token.next() {
        Some(Token{ token: TokenType::Ident(id), line: l, ch: c }) => {
            match id.as_str() {
                "D" => Some(Source::D),
                "A" => Some(Source::A),
                "M" => Some(Source::M),
                _ => {
                    println!("Unexpected source {} near {}:{}", id, l, c);
                    None
                },
            }
        },
        Some(Token{ token: TokenType::Number(0), line: l, ch: c }) => {
            if eat_number {
                Some(Source::Zero)
            } else {
                println!("Unexpected source 0 near {}:{}", l, c);
                eat_line(token, false);
                None
            }
        },
        Some(Token{ token: TokenType::Number(1), line: l, ch: c }) => {
            if eat_number {
                Some(Source::One)
            } else {
                println!("Unexpected source 1 near {}:{}", l, c);
                eat_line(token, false);
                None
            }
        },
        Some(t) => {
            error(token, &t);
            None
        },
        None => {
            error_eof();
            None
        },
    }
}

fn parse_source(id: &String) -> Option<Source> {
    match id.as_str() {
        "D" => Some(Source::D),
        "A" => Some(Source::A),
        "M" => Some(Source::M),
        _ => None,
    }
}

fn parse_dest(id: &String) -> Option<Dest> {
    match id.as_str() {
        "D" => Some(Dest::D),
        "A" => Some(Dest::A),
        "M" => Some(Dest::M),
        "MD" => Some(Dest::MD),
        "AM" => Some(Dest::AM),
        "AD" => Some(Dest::AD),
        "AMD" => Some(Dest::AMD),
        _ => None,
    }
}

pub fn parser<T: std::io::Read>(token: &mut lexer::TokenStream<T>) -> Option<Vec<AST>> {
    let mut vec : Vec<AST> = Vec::new();
    let mut symbol_tbl = HashMap::<String, u16>::new();
    symbol_tbl.insert("R0".to_owned(), 0);
    symbol_tbl.insert("R1".to_owned(), 1);
    symbol_tbl.insert("R2".to_owned(), 2);
    symbol_tbl.insert("R3".to_owned(), 3);
    symbol_tbl.insert("R4".to_owned(), 4);
    symbol_tbl.insert("R5".to_owned(), 5);
    symbol_tbl.insert("R6".to_owned(), 6);
    symbol_tbl.insert("R7".to_owned(), 7);
    symbol_tbl.insert("R8".to_owned(), 8);
    symbol_tbl.insert("R9".to_owned(), 9);
    symbol_tbl.insert("R10".to_owned(), 10);
    symbol_tbl.insert("R11".to_owned(), 11);
    symbol_tbl.insert("R12".to_owned(), 12);
    symbol_tbl.insert("R13".to_owned(), 13);
    symbol_tbl.insert("R14".to_owned(), 14);
    symbol_tbl.insert("R15".to_owned(), 15);
    symbol_tbl.insert("SP".to_owned(), 0);
    symbol_tbl.insert("LCL".to_owned(), 1);
    symbol_tbl.insert("ARG".to_owned(), 2);
    symbol_tbl.insert("THIS".to_owned(), 3);
    symbol_tbl.insert("THAT".to_owned(), 4);
    symbol_tbl.insert("SCREEN".to_owned(), 16384);
    symbol_tbl.insert("KBD".to_owned(), 24576);

    let mut err = false;
    // First loop: parse and collect symbols
    loop {
        match token.next() {
            Some(t) => {
                match t.token {
                    TokenType::Comment(_) | TokenType::NewLine => continue,
                    TokenType::Symbol('@') => {
                        // Type A
                        // @Number / @Ident
                        match token.next() {
                            Some(Token{ token: TokenType::Ident(id), line: _, ch: _ }) => {
                                vec.push(AST::InsA(InsA{
                                    value: Value::Ident(id),
                                }));
                                eat_line(token, true);
                            },
                            Some(Token{ token: TokenType::Number(num), line: _, ch: _ }) => {
                                vec.push(AST::InsA(InsA{
                                    value: Value::Number(num),
                                }));
                                eat_line(token, true);
                            },
                            Some(t) => {
                                error(token, &t);
                            },
                            None => {
                                error_eof();
                            },
                        }
                    },
                    TokenType::Ident(ref id) => {
                        // Type C
                        match token.next() {
                            Some(Token{ token: TokenType::Symbol('='), line: _, ch: _ }) => {
                                // Dest=Comp
                                let dest = parse_dest(id);
                                if dest.is_none() {
                                    error(token, &t);
                                    continue;
                                }
                                match token.next() {
                                    Some(Token{ token: TokenType::Number(0), line: _, ch: _ }) => {
                                        // Dest=0
                                        vec.push(AST::InsC(InsC{
                                            dest: dest.unwrap(),
                                            comp: Comp{left: None, op: None, right: Source::Zero},
                                            jump: Jump::NULL,
                                        }));
                                        eat_line(token, true);
                                    },
                                    Some(Token{ token: TokenType::Number(1), line: _, ch: _ }) => {
                                        // Dest=1
                                        vec.push(AST::InsC(InsC{
                                            dest: dest.unwrap(),
                                            comp: Comp{left: None, op: None, right: Source::One},
                                            jump: Jump::NULL,
                                        }));
                                        eat_line(token, true);
                                    },
                                    Some(Token{ token: TokenType::Symbol('!'), line: _, ch: _ }) => {
                                        // Dest=!ident
                                        let right = eat_source(token, false);
                                        if right.is_some() {
                                            vec.push(AST::InsC(InsC{
                                                dest: dest.unwrap(),
                                                comp: Comp{left: None, op: Some(Op::Not), right: right.unwrap()},
                                                jump: Jump::NULL,
                                            }));
                                            eat_line(token, true);
                                        }
                                    },
                                    Some(Token{ token: TokenType::Symbol('-'), line: _, ch: _ }) => {
                                        // Dest=-ident
                                        let right = eat_source(token, true);
                                        if right.is_some() {
                                            vec.push(AST::InsC(InsC{
                                                dest: dest.unwrap(),
                                                comp: Comp{left: None, op: Some(Op::Sub), right: right.unwrap()},
                                                jump: Jump::NULL,
                                            }));
                                            eat_line(token, true);
                                        }
                                    },
                                    Some(Token{ token: TokenType::Ident(ref id), line: l, ch: c }) => {
                                        // Dest=a op b, where op is + - | &
                                        match parse_source(id) {
                                            Some(left) => {
                                                match token.next() {
                                                    Some(Token{ token: TokenType::Symbol('+'), line: _, ch: _ }) => {
                                                        let right = eat_source(token, true);
                                                        if right.is_some() {
                                                            vec.push(AST::InsC(InsC{
                                                                dest: dest.unwrap(),
                                                                comp: Comp{left: Some(left), op: Some(Op::Add), right: right.unwrap()},
                                                                jump: Jump::NULL,
                                                            }));
                                                            eat_line(token, true);
                                                        }
                                                    },
                                                    Some(Token{ token: TokenType::Symbol('-'), line: _, ch: _ }) => {
                                                        let right = eat_source(token, true);
                                                        if right.is_some() {
                                                            vec.push(AST::InsC(InsC{
                                                                dest: dest.unwrap(),
                                                                comp: Comp{left: Some(left), op: Some(Op::Sub), right: right.unwrap()},
                                                                jump: Jump::NULL,
                                                            }));
                                                            eat_line(token, true);
                                                        }
                                                    },
                                                    Some(Token{ token: TokenType::Symbol('|'), line: _, ch: _ }) => {
                                                        let right = eat_source(token, false);
                                                        if right.is_some() {
                                                            vec.push(AST::InsC(InsC{
                                                                dest: dest.unwrap(),
                                                                comp: Comp{left: Some(left), op: Some(Op::Or), right: right.unwrap()},
                                                                jump: Jump::NULL,
                                                            }));
                                                            eat_line(token, true);
                                                        }
                                                    },
                                                    Some(Token{ token: TokenType::Symbol('&'), line: _, ch: _ }) => {
                                                        let right = eat_source(token, false);
                                                        if right.is_some() {
                                                            vec.push(AST::InsC(InsC{
                                                                dest: dest.unwrap(),
                                                                comp: Comp{left: Some(left), op: Some(Op::And), right: right.unwrap()},
                                                                jump: Jump::NULL,
                                                            }));
                                                            eat_line(token, true);
                                                        }
                                                    },
                                                    Some(Token{ token: TokenType::NewLine, line: _, ch: _ }) | None => {
                                                        vec.push(AST::InsC(InsC{
                                                            dest: dest.unwrap(),
                                                            comp: Comp{left: None, op: None, right: left},
                                                            jump: Jump::NULL,
                                                        }));
                                                    },
                                                    Some(Token{ token: TokenType::Comment(_), line: _, ch: _ }) => {
                                                        vec.push(AST::InsC(InsC{
                                                            dest: dest.unwrap(),
                                                            comp: Comp{left: None, op: None, right: left},
                                                            jump: Jump::NULL,
                                                        }));
                                                        eat_line(token, true);
                                                    },
                                                    Some(t) => {
                                                        error(token, &t);
                                                    },
                                                }
                                            },
                                            None => {
                                                println!("Unexpected ident {} near {}:{}", id, l, c);
                                                eat_line(token, false);
                                            }
                                        }
                                    },
                                    Some(t) => {
                                        error(token, &t);
                                    },
                                    None => {
                                        error_eof();
                                    }
                                }
                            },
                            Some(Token{ token: TokenType::Symbol('+'), line: _, ch: _ }) => {
                                // A+B;JMP
                                match parse_source(&id) {
                                    Some(left) => {
                                        let right = eat_source(token, true);
                                        if right.is_some() {
                                            let jump = eat_jump(token);
                                            if jump.is_some() {
                                                vec.push(AST::InsC(InsC{
                                                    dest: Dest::NULL,
                                                    comp: Comp{left: Some(left), op: Some(Op::Add), right: right.unwrap()},
                                                    jump: jump.unwrap(),
                                                }));
                                                eat_line(token, true);
                                            }
                                        }
                                    },
                                    None => {
                                        error(token, &t);
                                    }
                                }
                            },
                            Some(Token{ token: TokenType::Symbol('-'), line: _, ch: _ }) => {
                                // A-B;JMP
                                match parse_source(&id) {
                                    Some(left) => {
                                        let right = eat_source(token, true);
                                        if right.is_some() {
                                            let jump = eat_jump(token);
                                            if jump.is_some() {
                                                vec.push(AST::InsC(InsC{
                                                    dest: Dest::NULL,
                                                    comp: Comp{left: Some(left), op: Some(Op::Sub), right: right.unwrap()},
                                                    jump: jump.unwrap(),
                                                }));
                                                eat_line(token, true);
                                            }
                                        }
                                    },
                                    None => {
                                        error(token, &t);
                                    }
                                }
                            },
                            Some(Token{ token: TokenType::Symbol('&'), line: _, ch: _ }) => {
                                // A&B;JMP
                                match parse_source(&id) {
                                    Some(left) => {
                                        let right = eat_source(token, false);
                                        if right.is_some() {
                                            let jump = eat_jump(token);
                                            if jump.is_some() {
                                                vec.push(AST::InsC(InsC{
                                                    dest: Dest::NULL,
                                                    comp: Comp{left: Some(left), op: Some(Op::And), right: right.unwrap()},
                                                    jump: jump.unwrap(),
                                                }));
                                                eat_line(token, true);
                                            }
                                        }
                                    },
                                    None => {
                                        error(token, &t);
                                    }
                                }
                            },
                            Some(Token{ token: TokenType::Symbol('|'), line: _, ch: _ }) => {
                                // A|B;JMP
                                match parse_source(&id) {
                                    Some(left) => {
                                        let right = eat_source(token, false);
                                        if right.is_some() {
                                            let jump = eat_jump(token);
                                            if jump.is_some() {
                                                vec.push(AST::InsC(InsC{
                                                    dest: Dest::NULL,
                                                    comp: Comp{left: Some(left), op: Some(Op::Or), right: right.unwrap()},
                                                    jump: jump.unwrap(),
                                                }));
                                                eat_line(token, true);
                                            }
                                        }
                                    },
                                    None => {
                                        error(token, &t);
                                    }
                                }
                            },
                            Some(Token{ token: TokenType::Symbol(';'), line: _, ch: _ }) => {
                                // Ident;JMP
                                match parse_source(&id) {
                                    Some(right) => {
                                        let jump = match token.next() {
                                            Some(Token{ token: TokenType::Ident(ident), line: l, ch: c }) => {
                                                match ident.as_str() {
                                                    "JGT" => Some(Jump::JGT),
                                                    "JEQ" => Some(Jump::JEQ),
                                                    "JGE" => Some(Jump::JGE),
                                                    "JLT" => Some(Jump::JLT),
                                                    "JNE" => Some(Jump::JNE),
                                                    "JLE" => Some(Jump::JLE),
                                                    "JMP" => Some(Jump::JMP),
                                                    _ => {
                                                        println!("Unexpected jump type {:?} near {}:{}", ident, l, c);
                                                        None
                                                    }
                                                }
                                            },
                                            Some(t) => {
                                                error(token, &t);
                                                None
                                            }
                                            None => {
                                                error_eof();
                                                None
                                            }
                                        };
                                        if jump.is_some() {
                                            vec.push(AST::InsC(InsC{
                                                dest: Dest::NULL,
                                                comp: Comp{left: None, op: None, right},
                                                jump: jump.unwrap(),
                                            }));
                                            eat_line(token, true);
                                        }
                                    },
                                    None => {
                                        error(token, &t);
                                    }
                                }
                            },
                            Some(t) => {
                                // error
                                error(token, &t);
                            }
                            None => {
                                error_eof();
                            },
                        }
                    },
                    TokenType::Symbol('!') => {
                        // \!Ident;JUMP
                        let right = eat_source(token, false);
                        if right.is_some() {
                            let jump = eat_jump(token);
                            if jump.is_some() {
                                vec.push(AST::InsC(InsC{
                                    dest: Dest::NULL,
                                    comp: Comp{left: None, op: Some(Op::Not), right: right.unwrap()},
                                    jump: jump.unwrap(),
                                }));
                                eat_line(token, true);
                            }
                        }
                    },
                    TokenType::Symbol('-') => {
                        // -Ident;JUMP
                        let right = eat_source(token, true);
                        if right.is_some() {
                            let jump = eat_jump(token);
                            if jump.is_some() {
                                vec.push(AST::InsC(InsC{
                                    dest: Dest::NULL,
                                    comp: Comp{left: None, op: Some(Op::Sub), right: right.unwrap()},
                                    jump: jump.unwrap(),
                                }));
                                eat_line(token, true);
                            }
                        }
                    },
                    TokenType::Number(0) => {
                        // 0;JUMP
                        match eat_jump(token) {
                            Some(jump) => {
                                vec.push(AST::InsC(InsC{
                                    dest: Dest::NULL,
                                    comp: Comp{left: None, op: None, right: Source::Zero},
                                    jump,
                                }));
                                eat_line(token, true);
                            },
                            None => continue,
                        };
                    },
                    TokenType::Number(1) => {
                        // 1;JUMP
                        match eat_jump(token) {
                            Some(jump) => {
                                vec.push(AST::InsC(InsC{
                                    dest: Dest::NULL,
                                    comp: Comp{left: None, op: None, right: Source::One},
                                    jump,
                                }));
                                eat_line(token, true);
                            },
                            None => continue,
                        };
                    },
                    TokenType::Symbol('(') => {
                        // (Ident)
                        match token.next() {
                            Some(Token{ token: TokenType::Ident(id), line: _, ch: _ }) => {
                                match token.next() {
                                    Some(Token{ token: TokenType::Symbol(')'), line: _, ch: _ }) => {
                                        if symbol_tbl.contains_key(&id) {
                                            println!("Label {} redefined", id);
                                        }
                                        symbol_tbl.insert(id, vec.len() as u16);
                                        eat_line(token, true);
                                    },
                                    Some(t) => {
                                        err = true;
                                        error(token, &t);
                                    },
                                    None => {
                                        err = true;
                                        error_eof();
                                    }
                                }
                            },
                            Some(t) => {
                                err = true;
                                error(token, &t);
                            },
                            None => {
                                err = true;
                                error_eof();
                            }
                        }
                    },
                    _ => {
                        // error
                        println!("Unexpected token {:?} near {}:{}", t.token, t.line, t.ch);
                        eat_line(token, false);
                    }
                }
            },
            None => break,
        }
    };

    if err {
        return None
    }

    // second loop: replace all symbol
    let mut reg: u16 = 16;

    for ast in &mut vec {
        match ast {
            AST::InsA(ref mut a) => {
                match a.value {
                    Value::Ident(ref id) => {
                        match symbol_tbl.get(id) {
                            Some(val) => {
                                a.value = Value::Number(*val);
                            },
                            None => {
                                symbol_tbl.insert(String::from(id), reg);
                                a.value = Value::Number(reg);
                                reg += 1;
                            }
                        }
                    },
                    _ => ()
                }
                
            },
            _ => (),
        }
    }

    println!("============= AST ================");
    for ast in &vec {
        println!("{:?}", ast);
    }
    println!("============= SYM ================");
    for it in &symbol_tbl {
        println!("{:?}", it);
    }
    Some(vec)
}
