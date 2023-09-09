use std::{collections::HashMap, io::{self, Write}};
use crate::ex_core::ValueVariant;

use super::func_respository::{FunctionRepository, StdFuncResult, StdFuncArgs, StdFuncMap, StdFunc};
//use crate::core::interp::ValueVariant;
pub struct IOFunctionRepo {
    funcs : StdFuncMap,
}
impl IOFunctionRepo {
    pub fn new() -> Self {
        let mut funcs : HashMap<String, StdFunc> = HashMap::new();
        funcs.insert("write".to_string(), IOFunctionRepo::write);
        funcs.insert("writeln".to_string(), IOFunctionRepo::writeln);
        funcs.insert("read".to_string(), IOFunctionRepo::read);
        Self {
            funcs : funcs
        }
    }
    
    fn write(args: &StdFuncArgs) -> StdFuncResult {
        for arg in args.iter() {
            print!("{}", arg.to_string());
        }
        io::stdout().flush().expect("Failed to flush stdout");
        return Ok(None);
    }
    
    fn writeln(args: &StdFuncArgs) -> StdFuncResult {
        for arg in args.iter() {
            println!("{}", arg.to_string());
        }
        if args.len() == 0 {
            println!();
        }
        return Ok(None);
    }

    fn read(_: &StdFuncArgs) -> StdFuncResult {
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            return Err(err.to_string());
        }
        input = input.strip_suffix("\r\n").or(input.strip_suffix("\n")).unwrap_or(&input).to_owned();
        return Ok(Some(ValueVariant::String(input.clone())));
    }


}

impl FunctionRepository for IOFunctionRepo {
    fn get_functions(&self) -> super::func_respository::StdFuncMap {
        self.funcs.clone()
    }
}

