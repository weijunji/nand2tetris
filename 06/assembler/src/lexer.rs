use std::iter::Iterator;
use std::io::{ BufReader, Read, BufRead };

/// Tokens for Hack lang
/// Symbol can be `@` `=` `+` `-` `!` `&` `|` `;` `(` and `)`
/// Ident can include letter digital `_` `.` `$` and `:`
/// Number cannot be negative, and must be decimal
/// Comment is start with `//` and end with newline
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Symbol(char),
    Ident(String),
    Number(u16),
    Comment(String),
    NewLine,
    Invalid(String),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub line: u32,
    pub ch: u32,
}

#[derive(Debug)]
pub struct TokenStream<'a, T> {
    reader: BufReader<&'a mut T>,
    ch: Option<char>,
    cur_line: u32,
    cur_ch: u32,
}

impl<'a, T: Read> TokenStream<'a, T> {
    fn next_ch(&mut self) -> Option<char> {
        if self.ch.is_some() {
            return self.ch;
        }

        let mut buf : [u8; 1] = [0; 1];
        let n = self.reader.read(&mut buf).ok()?;
        if n == 0 {
            self.ch = None;
            None
        } else {
            match buf[0] as char {
                '\n' => {
                    self.cur_line += 1;
                    self.cur_ch = 0;
                },
                _ => {
                    self.cur_ch += 1;
                }
            };
            self.ch = Some(buf[0] as char);
            self.ch
        }
    }

    fn eat(&mut self) {
        self.ch = None;
    }

    fn eat_number(&mut self) -> Option<u16> {
        // max 15-bit number is 32767
        let mut num: u16 = self.next_ch().unwrap() as u16 - '0' as u16;
        let mut overflow = false;
        self.eat();
        loop {
            let ch = self.next_ch();
            match ch {
                Some('0'..='9') => {
                    let dig = ch.unwrap() as u16 - '0' as u16;
                    if num > 3276 {
                        overflow = true;
                    }
                    if !overflow {
                        num *= 10;
                        num += dig;
                    }
                    if num > 32767 {
                        overflow = true;
                    }
                    self.eat();
                },
                _ => break,
            }
        }
        if overflow {
            None
        } else {
            Some(num)
        }
    }

    fn eat_ident(&mut self) -> String {
        let mut s = String::new();
        loop {
            let ch = self.next_ch();
            match ch {
                Some(c @ ('a'..='z'| 'A'..='Z' | '_' | '.' | '$' | ':' | '0'..='9')) => {
                    s.push(c);
                    self.eat();
                },
                _ => break,
            }
        };
        s
    }

    fn eat_comment(&mut self) -> Option<String> {
        self.eat();
        let ch = self.next_ch();
        match ch {
            Some('/') => {
                self.eat();
                let mut s = String::new();
                self.reader.read_line(&mut s).ok()?;
                if s.chars().last().unwrap() == '\n' {
                    self.ch = Some('\n');
                    self.cur_line += 1;
                    self.cur_ch = 0;
                    s.pop();
                }
                if s.chars().last().unwrap() == '\r' {
                    s.pop();
                }
                Some(s)
            },
            _ => {
                None
            }
        }
    }
}

impl<'a, T: Read> Iterator for TokenStream<'a, T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_ch = self.next_ch()?;
        let ch = self.cur_ch;
        let line = self.cur_line;
        let token = match next_ch {
            '0' ..= '9' => {
                match self.eat_number() {
                    Some(n) => TokenType::Number(n),
                    None => TokenType::Invalid("Overflow".to_owned()),
                }
            },
            '@' | '=' | '+' | '-' | '!' | '&' | '|' | ';' | '(' | ')' => {
                self.eat();
                TokenType::Symbol(next_ch)
            },
            '/' => {
                match self.eat_comment() {
                    Some(s) => TokenType::Comment(s),
                    None => TokenType::Invalid("Invalid Character".to_owned()),
                }
            },
            '\n' => {
                self.eat();
                TokenType::NewLine
            },
            '\r' => {
                self.eat();
                let c = self.next_ch();
                self.eat();
                match c {
                    Some('\n') => TokenType::NewLine,
                    _ => TokenType::Invalid("Invalid Character".to_owned()),
                }
            },
            ' ' => {
                self.eat();
                return self.next();
            },
            'a'..='z'| 'A'..='Z' | '_' | '.' | '$' | ':' => {
                TokenType::Ident(self.eat_ident())
            },
            _ => {
                self.eat();
                TokenType::Invalid("Invalid Character".to_owned())
            }
        };
        Some(Token {
            token,
            ch,
            line,
        })
    }
}

pub fn lexer<T: Read>(f: &mut T) -> Option<TokenStream<T>> {
    let reader = BufReader::new(f);
    Some(TokenStream {
        reader,
        ch: None,
        cur_line: 1,
        cur_ch: 0,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number() {
        let mut s = "000032767".as_bytes();
        let mut l = lexer(&mut s).unwrap();
        assert_eq!(l.next().unwrap().token, TokenType::Number(32767));

        let mut s = "0000".as_bytes();
        let mut l = lexer(&mut s).unwrap();
        assert_eq!(l.next().unwrap().token, TokenType::Number(0));

        let mut s = "000032768".as_bytes();
        let mut l = lexer(&mut s).unwrap();
        assert_eq!(l.next().unwrap().token, TokenType::Invalid("Overflow".to_owned()));
    }

    #[test]
    fn test_comment() {
        let mut s = "//comment\n@ 123//comment\n/123".as_bytes();
        let mut l = lexer(&mut s).unwrap();
        assert_eq!(l.next().unwrap().token, TokenType::Comment("comment".to_owned()));
        assert_eq!(l.next().unwrap().token, TokenType::NewLine);
        assert_eq!(l.next().unwrap().token, TokenType::Symbol('@'));
        assert_eq!(l.next().unwrap().token, TokenType::Number(123));
        assert_eq!(l.next().unwrap().token, TokenType::Comment("comment".to_owned()));
        assert_eq!(l.next().unwrap().token, TokenType::NewLine);
        assert_eq!(l.next().unwrap().token, TokenType::Invalid("Invalid Character".to_owned()));
        assert_eq!(l.next().unwrap().token, TokenType::Number(123));
    }

    #[test]
    fn test_ident() {
        let mut s = "@A\na_dasd13:d.1$1=123".as_bytes();
        let mut l = lexer(&mut s).unwrap();
        assert_eq!(l.next().unwrap().token, TokenType::Symbol('@'));
        assert_eq!(l.next().unwrap().token, TokenType::Ident("A".to_owned()));
        assert_eq!(l.next().unwrap().token, TokenType::NewLine);
        assert_eq!(l.next().unwrap().token, TokenType::Ident("a_dasd13:d.1$1".to_owned()));
        assert_eq!(l.next().unwrap().token, TokenType::Symbol('='));
        assert_eq!(l.next().unwrap().token, TokenType::Number(123));
        assert_eq!(l.next(), None);
    }
}
