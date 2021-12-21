use std::iter::Iterator;
use std::io::{ BufReader, Read, BufRead };

#[derive(Debug, PartialEq)]
pub enum TokenType {
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
                Some(c @ ('a'..='z'| 'A'..='Z' | '_' | '.' | '$' | ':' | '0'..='9' | '-')) => {
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
            'a'..='z'| 'A'..='Z' | '_' | '.' | '$' | ':' | '-' => {
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
