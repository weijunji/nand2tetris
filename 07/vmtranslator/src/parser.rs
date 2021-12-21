use super::lexer::{ self, Token, TokenType };

#[derive(Debug)]
pub enum SingleOperator {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum StackOperator {
    Push,
    Pop,
}

#[derive(Debug)]
pub enum Location {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

#[derive(Debug)]
pub enum AST {
    StackOp(StackOperator, Location, u16),
    SingleOp(SingleOperator),
}

fn print_error(token: &Token) {
    println!("Unexpected token {:?} near {}:{}", token.token, token.line, token.ch);
}

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

macro_rules! single_op {
    ($vec:expr, $token:expr, $t:expr) => {{
        $vec.push(AST::SingleOp($t));
        eat_line($token, true);
    }}
}


pub fn eat_loc_num<T: std::io::Read>(token: &mut lexer::TokenStream<T>) -> Option<(Location, u16)> {
    match token.next() {
        Some(t) => {
            let loc = match t.token {
                TokenType::Ident(id) => {
                    match id.as_str() {
                        "local" => Location::Local,
                        "argument" => Location::Argument,
                        "this" => Location::This,
                        "that" => Location::That,
                        "constant" => Location::Constant,
                        "static" => Location::Static,
                        "pointer" => Location::Pointer,
                        "temp" => Location::Temp,
                        _ => {
                            println!("Unexpected location {:?} near {}:{}", id, t.line, t.ch);
                            eat_line(token, false);
                            return None;
                        }
                    }
                },
                _ => {
                    error(token, &t);
                    return None;
                }
            };
            let num = match token.next() {
                Some(Token{ token: TokenType::Number(n), .. }) => n,
                Some(t) => {
                    error(token, &t);
                    return None
                },
                None => {
                    error_eof();
                    return None
                },
            };
            return Some((loc, num));
        },
        None => error_eof(),
    }
    None
}

pub fn parser<T: std::io::Read>(token: &mut lexer::TokenStream<T>) -> Vec<AST> {
    let mut vec : Vec<AST> = Vec::new();
    loop {
        match token.next() {
            Some(t) => {
                match t.token {
                    TokenType::Comment(_) => continue,
                    TokenType::NewLine => continue,
                    TokenType::Ident(id) => {
                        match id.as_str() {
                            "push" => {
                                match eat_loc_num(token) {
                                    Some((loc, num)) => {
                                        vec.push(AST::StackOp(StackOperator::Push, loc, num));
                                        eat_line(token, true);
                                    },
                                    None => (),
                                }
                            },
                            "pop" => {
                                match eat_loc_num(token) {
                                    Some((loc, num)) => {
                                        vec.push(AST::StackOp(StackOperator::Pop, loc, num));
                                        eat_line(token, true);
                                    },
                                    None => (),
                                }
                            },
                            "add" => single_op!(vec, token, SingleOperator::Add),
                            "sub" => single_op!(vec, token, SingleOperator::Sub),
                            "neg" => single_op!(vec, token, SingleOperator::Neg),
                            "eq" => single_op!(vec, token, SingleOperator::Eq),
                            "gt" => single_op!(vec, token, SingleOperator::Gt),
                            "lt" => single_op!(vec, token, SingleOperator::Lt),
                            "and" => single_op!(vec, token, SingleOperator::And),
                            "or" => single_op!(vec, token, SingleOperator::Or),
                            "not" => single_op!(vec, token, SingleOperator::Not),
                            _ => {
                                println!("Unexpected operator {:?} near {}:{}", id, t.line, t.ch);
                            },
                        }
                    },
                    TokenType::Number(_) | TokenType::Invalid(_) => error(token, &t),
                }
            },
            None => break,
        }
    }
    for ast in &vec {
        println!("{:?}", ast);
    }
    vec
}
