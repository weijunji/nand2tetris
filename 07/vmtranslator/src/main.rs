use std::fs::File;
use std::fs::OpenOptions;
use std::str;
use std::env;
use std::path::Path;
use std::io::Write;

mod lexer;
mod parser;
mod code_gen;

fn main() {
    println!("VM Translator (Rust Version 0.1)");
    let args: Vec<String> = env::args().collect();
    
    for fname in &args[1..] {
        let p = Path::new(&fname);
        let mut f = File::open(p).unwrap();
        let mut tokens = lexer::lexer(&mut f).unwrap();
        //for t in tokens {
        //    println!("{:?}", t);
        //}
        let ast = parser::parser(&mut tokens);
        let mut target = Vec::<u8>::new();
        code_gen::code_gen(&ast, p.file_stem().unwrap().to_str().unwrap(), &mut target);
        println!("=========== machine lang ===========");
        println!("{}", str::from_utf8(&target).unwrap());

        let mut ft = OpenOptions::new().write(true)
                           .truncate(true)
                           .create(true)
                           .open(p.with_extension("asm"))
                           .unwrap();
        ft.write_all(&target).unwrap();
    }
}
