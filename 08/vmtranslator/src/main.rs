use std::fs::File;
use std::fs::OpenOptions;
use std::str;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::{ metadata, read_dir };

mod lexer;
mod parser;
mod code_gen;

fn main() {
    println!("VM Translator (Rust Version 0.1)");
    let args: Vec<String> = env::args().collect();
    
    for fname in &args[1..] {
        let p = Path::new(&fname);
        let md = metadata(p).unwrap();
        if md.is_file() {
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
        } else {
            let mut target = Vec::<u8>::new();

            for entry in read_dir(p).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let metadata = metadata(&path).unwrap();
        
                if metadata.is_file() && path.extension().unwrap() == "vm" {
                    let mut f = File::open(&path).unwrap();
                    let mut tokens = lexer::lexer(&mut f).unwrap();
                    let ast = parser::parser(&mut tokens);
                    code_gen::code_gen(&ast, path.file_stem().unwrap().to_str().unwrap(), &mut target);
                }
            }

            let mut tf = p.join(p.file_name().unwrap());
            tf.set_extension("asm");
            let mut ft = OpenOptions::new().write(true)
                            .truncate(true)
                            .create(true)
                            .open(tf)
                            .unwrap();
            write!(ft, "@Sys.init\n").unwrap();
            write!(ft, "0;JMP\n").unwrap();
            ft.write_all(&target).unwrap();
        }
    }
}
