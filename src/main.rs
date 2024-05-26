use std::fs;
pub mod lexer;
use lexer::*;
pub mod parser;
use parser::*;
use std::env;
// pub mod transpiler;
// use transpiler::*;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{} {}\n", "ARGS:".green(), args.len());

    let mut path = String::new();
    match args.len() {
        2 => path = args[1].clone(),
        _ => panic!("TOO MANY/LITTLE ARGS JBJFHNFKDB!!!!!!!!")
    }

    let contents = fs::read_to_string(path)
        .expect("file not found");
    println!("{}\n{}\n", "CONTENTS:".green(), contents);
    
    let tokenized = lex(contents);
    println!("{}", "TOKENS:".green());
    for token in tokenized.iter(){
        print!("{:?},  ", token);
        if let &Token::Symbol(s) = &token{
            if s == "\n" {
                println!()
            }
        }
    }
    
    let parsed = parse(tokenized);
    println!("{}", "\n\nNODES:".green());
    for node in parsed.iter(){
        println!("{:?}", node);
    }

    // let result = transpile(parsed);
    // println!("TRANSPILED:\n{result}");
}

