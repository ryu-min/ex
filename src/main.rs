use std::io;
mod tokenizer;
use crate::tokenizer::tokenize;

fn main() -> io::Result<()>{
    let program = "222 + (\"ddd\"  ) + ddd";
    let tokens = tokenize(&program.to_string());
    println!("program is {program}");
    for token in tokens {
        println!("token is {}", token.to_string());
    }
    Ok(())    
}
