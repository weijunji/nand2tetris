//! keyword: 'class' | 'constructor' | 'function' |
//!          'method' | 'field' | 'static' | 'var' | 'int' |
//!          'char' | 'boolean' | 'void' | 'true' | 'false' |
//!          'null' | 'this' | 'let' | 'do' | 'if' | 'else' |
//!          'while' | 'return’
//! 
//! symbol: '{' | '}' | '(' | ')' | '[' | ']' | '. ' | ', ' | '; ' | '+' | '-' | '*' |
//!         '/' | '&' | '|' | '<' | '>' | '=' | '~'
//! 
//! integerConstant: a decimal number in the range 0 ... 32767
//! 
//! StringConstant: '"' a sequence of Unicode characters,
//!                 not including double quote or newline '"'
//! 
//! identifier: a sequence of letters, digits, and
//!             underscore ( '_' ) not starting with a digit.

use logos::{Logos, Lexer};

fn process_str(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let trim: String = slice[1..slice.len() - 1].parse().ok()?;
    Some(trim.replace("\n", "").replace("\r", ""))
}

fn process_num(lex: &mut Lexer<Token>) -> Option<u16> {
    let num: u16 = lex.slice().parse().ok()?;
    if num > 32767 {
        None
    } else {
        Some(num)
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |id| id.slice().parse())]
    Ident(String),

    #[regex("\"[^\"]*\"", process_str)]
    Str(String),

    #[regex(r"0|[1-9][0-9]*", process_num)]
    Number(u16),

    #[regex(r"\{|\}|\(|\)|\[|\]|\.|,|;|\+|-|\*|/|&|\||<|>|=|~", |c| c.slice().parse())]
    Symbol(char),

    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    // Comments
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex("/\\*\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/", logos::skip)]
    Error,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ident() {
        let mut lex = Token::lexer("abc _a_b12c4");

        assert_eq!(lex.next(), Some(Token::Ident("abc".to_owned())));
        assert_eq!(lex.next(), Some(Token::Ident("_a_b12c4".to_owned())));
    }

    #[test]
    fn test_str() {
        let mut lex = Token::lexer("\"abc\"");

        assert_eq!(lex.next(), Some(Token::Str("abc".to_owned())));

        let mut lex = Token::lexer("\"abc\r\n123\"");

        assert_eq!(lex.next(), Some(Token::Str("abc123".to_owned())));
    }

    #[test]
    fn test_number() {
        let mut lex = Token::lexer("0 123 32767 32768");

        assert_eq!(lex.next(), Some(Token::Number(0)));
        assert_eq!(lex.next(), Some(Token::Number(123)));
        assert_eq!(lex.next(), Some(Token::Number(32767)));
        assert_eq!(lex.next(), Some(Token::Error));
    }

    #[test]
    fn test_symbol() {
        let mut lex = Token::lexer("{}()[].,;+-*/&|<>=~");

        assert_eq!(lex.next(), Some(Token::Symbol('{')));
        assert_eq!(lex.next(), Some(Token::Symbol('}')));
        assert_eq!(lex.next(), Some(Token::Symbol('(')));
        assert_eq!(lex.next(), Some(Token::Symbol(')')));
        assert_eq!(lex.next(), Some(Token::Symbol('[')));
        assert_eq!(lex.next(), Some(Token::Symbol(']')));
        assert_eq!(lex.next(), Some(Token::Symbol('.')));
        assert_eq!(lex.next(), Some(Token::Symbol(',')));
        assert_eq!(lex.next(), Some(Token::Symbol(';')));
        assert_eq!(lex.next(), Some(Token::Symbol('+')));
        assert_eq!(lex.next(), Some(Token::Symbol('-')));
        assert_eq!(lex.next(), Some(Token::Symbol('*')));
        assert_eq!(lex.next(), Some(Token::Symbol('/')));
        assert_eq!(lex.next(), Some(Token::Symbol('&')));
        assert_eq!(lex.next(), Some(Token::Symbol('|')));
        assert_eq!(lex.next(), Some(Token::Symbol('<')));
        assert_eq!(lex.next(), Some(Token::Symbol('>')));
        assert_eq!(lex.next(), Some(Token::Symbol('=')));
        assert_eq!(lex.next(), Some(Token::Symbol('~')));
    }
}
