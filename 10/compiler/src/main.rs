#![allow(dead_code)]

use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::str;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::{ metadata, read_dir };

use logos::Logos;

mod lexer;
mod parser;

fn process_file(path: &Path) {
    let mut f = fs::read_to_string(&path).unwrap();
    let mut tokens = lexer::Token::lexer(&mut f);
    //for t in tokens {
    //    println!("{:?}", t);
    //}
    let ast = parser::parser(&mut tokens);
    println!("{:#?}", ast);
}

fn main() {
    println!("Jack Lang Compiler (Rust Version 0.1)");
    let args: Vec<String> = env::args().collect();
    for fname in &args[1..] {
        let p = Path::new(&fname);
        let md = metadata(p).unwrap();
        if md.is_file() {
            process_file(&p);
        } else {
            for entry in read_dir(p).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let metadata = metadata(&path).unwrap();
        
                if metadata.is_file() && path.extension().unwrap() == "jack" {
                    process_file(&path);
                }
            }
        }
    }
}
