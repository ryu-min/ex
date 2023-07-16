mod tokenizer;
mod parser;
mod interp;
use crate::tokenizer::tokenize;
use std::io;
use crate::parser::Parser;
use crate::interp::Interpreter;


fn main() -> io::Result<()>{
    let program1 = "(2 + 2) * 2".to_string();
    let program2 = "2 + 2 * 2 * 2".to_string();
    let expr1 = Parser::new(&tokenize(&program1)).parse();
    let expr2 = Parser::new(&tokenize(&program2)).parse();
    let res1 = Interpreter::new(expr1).eval();
    let res2 = Interpreter::new(expr2).eval();
    println!("program {} result is {}", program1, res1);
    println!("program {} result is {}", program2, res2);
    Ok(())    
}
