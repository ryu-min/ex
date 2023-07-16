mod tokenizer;
mod parser;
mod interp;
use crate::tokenizer::tokenize;
use std::collections::HashMap;
use std::io;
use crate::parser::Parser;
use crate::interp::Interpreter;


fn main() -> io::Result<()>{
    let mut test_map = HashMap::new();
    test_map.insert("(2 + 2) * 2", 8.);
    test_map.insert("2 + 2 * 2 * 2", 10.);
    test_map.insert("-2 + 2", 0.);
    test_map.insert("-(2 + 2 * 2)", -6.);
    for (prog, _) in test_map.iter() {
        let prog = prog.to_string();
        let expr = Parser::new(&tokenize(&prog)).parse();
        let res = Interpreter::new(expr).eval();
        println!("program {} result is {}", prog, res);
    }
    Ok(())    
}
