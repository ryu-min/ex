mod tokenizer;
mod parser;
mod interp;
use crate::tokenizer::tokenize;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use crate::parser::Parser;
use crate::interp::Interpreter;

fn run_cl_interp() {
    println!("Run commnand line Ex interpreter");
    let mut interp = Interpreter::new();
    loop {
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let tokens = tokenize(&input);
            let mut parser = Parser::new(&tokens);
            let parse_result = parser.parse();
            if let Ok(expr) = parse_result {
                if let Err(err_msg) = interp.parse(expr) {
                    println!("Interpreter error: {}", err_msg);
                }
            }
        } else {
            println!("can't parse input");
        }
    }
}

fn main() -> io::Result<()>{
    run_cl_interp();
    // let mut test_map = HashSet::new();
    //  test_map.insert("var b  = \"aa\" + \"bb\" \n\
    //                  write (b ) \n \
    //                  write (b + b)\n \
    //                  write (b + b + \"cc\") \n \
    //                  write (b, \"cc\")");         
    // for prog in test_map.iter() {
    //     let prog = prog.to_string();
    //     let expr = Parser::new(&tokenize(&prog)).parse().unwrap();
        
    //     let mut interp = Interpreter::new(); 
    //     interp.parse(expr);
    // }
    Ok(())    
}
