use std::fs;
pub mod lexer;
use lexer::*;
pub mod parser;
use parser::*;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("ARGS: {}\n", args.len());

    let mut path = "/Users/vladyslav/Documents/code/rust/mochalang/testfile.mch".to_string();
    match args.len() {
        1 => {},
        2 => path = args[1].clone(),
        _ => panic!("TOO MANY ARGS JBJFHNFKDB")
    }

    let contents = fs::read_to_string(path)
        .expect("file not found");
    println!("CONTENTS:\n{}\n", contents);
    
    let tokenized = lex(contents);
    println!("TOKENS:\n{:?}\n", tokenized);
    
    let parsed = parse(tokenized);
    println!("NODES:");
    for node in parsed.iter(){
        println!("{:?}", node);
    }
}

