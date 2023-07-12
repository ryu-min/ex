use std::io;
mod tokenizer;
mod parser;
use crate::tokenizer::tokenize;

fn main() -> io::Result<()>{
    let program = "var s = \"str\" x = x * ( x + y )";
    let tokens = tokenize(&program.to_string());
    println!("program is {program}");
    for token in tokens {
        println!("token is {}", token.to_string());
    }
    Ok(())    
}
