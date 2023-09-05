use std::{collections::HashMap, mem::Discriminant};
use crate::ex_core::interp::ValueVariant;
pub type StdMethodArgs = Vec<ValueVariant>;
pub type StdMethodResult = Result<Option<ValueVariant>, String>;
pub type StdMethod = fn(&ValueVariant, &StdMethodArgs) -> StdMethodResult;
pub type StdMethodsMap = HashMap<String, StdMethod>;

pub trait StdMethodsRepository {
    fn get_diterminant(&self) -> Discriminant<ValueVariant>;
    fn get_methods(&self) -> StdMethodsMap;
}
