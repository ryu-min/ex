use crate::ex_core::ValueVariant;

use super::method_repository::{StdMethodsMap, StdMethodsRepository, StdMethodArgs, StdMethodResult};


pub struct IntMethods {
    methods: StdMethodsMap
}

impl IntMethods {
    pub fn new() -> Self {
        let mut methods = StdMethodsMap::new();
        methods.insert("pow".to_string(), IntMethods::pow);
        return Self {
            methods : methods
        }
    }
    
    fn to_int(v: &ValueVariant) -> Result<i64, String> {
        match v {
            ValueVariant::Integer(i) => {
                return Ok(i.clone());
            }
            _ => {
                return Err(format!("exptected int, find {}", v.to_string()));
            }
        }
    }

    fn pow(this: &ValueVariant, args:&StdMethodArgs) -> StdMethodResult {
        if args.len() != 1 {
            return Err(format!("method arg expected 1 argument, find {}", args.len()));
        }
        let this_i = Self::to_int(this)?;
        let pow = Self::to_int(&args[0])?;
        return Ok(Some(ValueVariant::Integer(this_i.pow(pow as u32))));
    }

}

impl StdMethodsRepository for IntMethods {
    fn get_diterminant(&self) -> std::mem::Discriminant<crate::ex_core::ValueVariant> {
        return std::mem::discriminant(&crate::ex_core::ValueVariant::Integer(0));
    }

    fn get_methods(&self) -> StdMethodsMap {
        return self.methods.clone();
    }
}
