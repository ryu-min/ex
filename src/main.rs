mod tokenizer;
mod parser;
mod interp;
use ex_std::FunctionRepository;

use crate::tokenizer::tokenize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Write};
use std::fs;
use crate::parser::Parser;
use crate::interp::{Interpreter, ValueVariant};
use crate::ex_std::IOFunctionRepo;

mod ex_std;

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
            } else if let Err(err_msg) = parse_result {
                println!("Parse error: {}", err_msg);
            }
        } else {
            println!("can't parse input");
        }
    }
}

fn interp_file(path: &String) -> io::Result<()> {
    let file_content = fs::read_to_string(path)?;
    let tokens = tokenize(&file_content);
    let mut parser = Parser::new(&tokens);
    let parse_res = parser.parse();
    if let Ok(expr) = parse_res {
        let mut interp = Interpreter::new();
        let interp_res = interp.parse(expr);
        if let Err(err_msg) = interp_res {
            println!("Interpreter error: {}", err_msg);
        }
    } else if let Err(err_msg) = parse_res {
        println!("Parsing error: {}", err_msg);
    }
    Ok(())
}

fn print_usage() {
    println!("USAGE: ");
    println!("To run the command line interpreter: ");
    println!("    ex.exe");
    println!("To interpret a file: ");
    println!("    ex.exe <path_to_file>");
    println!("To show this message: ");
    println!("    ex.exe --help or ex.exe -h");
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_usage();
    } else if args.len() == 1 {
        run_cl_interp();
    } else if args.len() == 2 {
        interp_file(&args[1])?;
    } else {
        println!("Error: not valid count of args");
        print_usage();
    }
    Ok(())    
}
