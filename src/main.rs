mod tokenizer;
mod parser;
mod interp;
use crate::tokenizer::tokenize;
use std::collections::{HashMap, HashSet};
use std::io;
use crate::parser::Parser;
use crate::interp::Interpreter;


fn main() -> io::Result<()>{
    let mut test_map = HashSet::new();
    test_map.insert("var b  = \"aa\" + \"bb\" \n\
                     write b \n\
                     write b + b \n\
                     write b + b + \"cc\"
                     write b, \"cc\"
                     ");
    for prog in test_map.iter() {
        let prog = prog.to_string();
        let expr = Parser::new(&tokenize(&prog)).parse().unwrap();
        let mut interp = Interpreter::new(); 
        interp.parse(expr);
    }
    Ok(())    
}
