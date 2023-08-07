use std::collections::HashMap;


use super::func_respository::{FunctionRepository, StdFuncResult, StdFuncArgs, StdFuncMap, StdFunc};
pub struct IOFunctionRepo {
    funcs : StdFuncMap,
}
impl IOFunctionRepo {
    pub fn new() -> Self {
        let mut funcs : HashMap<String, StdFunc> = HashMap::new();
        funcs.insert("write".to_string(), IOFunctionRepo::write);
        Self {
            funcs : funcs
        }
    }
    
    pub fn write(args: &StdFuncArgs) -> StdFuncResult {
        for arg in args.iter() {
            println!("{}", arg.to_string());
        }
        return Ok(None);
    }
}

impl FunctionRepository for IOFunctionRepo {
    fn get_functions(&self) -> super::func_respository::StdFuncMap {
        self.funcs.clone()
    }
}

