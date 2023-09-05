use std::collections::HashMap;
use crate::ex_core::interp::ValueVariant;
use mem::discriminant;
pub type StdMethodArgs = Vec<ValueVariant>;
pub type StdMethodResult = Result<Option<ValueVariant>, String>;
pub type StdMethod = fn(&ValueVariant, &StdMethodArgs) -> StdMethodResult;
pub type StdMethodsMap = HashMap<String, StdMethod>;

pub trait StdMethod {
    fn get_diterminant() -> Discriminant<ValueVariant>;
    fn get_methods() -> StdMethodsMap;
}
