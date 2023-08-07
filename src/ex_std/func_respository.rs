use std::collections::HashMap;

use crate::interp::ValueVariant;

pub type StdFuncArgs = Vec<ValueVariant>;
pub type StdFuncResult = Result<Option<ValueVariant>, String>;
pub type StdFunc = fn(&StdFuncArgs) -> StdFuncResult;
pub type StdFuncMap = HashMap<String, StdFunc>;

pub trait FunctionRepository {
    fn get_functions(&self) -> StdFuncMap;
}