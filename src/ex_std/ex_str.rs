use crate::ex_core::ValueVariant;

use super::method_repository::{StdMethodsMap, StdMethodsRepository, StdMethodArgs, StdMethodResult};


pub struct StringMethods {
    methods: StdMethodsMap
}

impl StringMethods {
    pub fn new() -> Self {
        let mut methods = StdMethodsMap::new();
        methods.insert("to_int".to_string(), StringMethods::to_int);
        methods.insert("to_float".to_string(), StringMethods::to_float);
        return Self {
            methods : methods
        }
    }
    
    fn get_str(v: &ValueVariant) -> Result<String, String> {
        match v {
            ValueVariant::String(s) => {
                return Ok(s.clone());
            }
            _ => {
                return Err(format!("exptected int, find {}", v.to_string()));
            }
        }
    }

    fn to_int(this: &ValueVariant, args:&StdMethodArgs) -> StdMethodResult {
        if args.len() != 0 {
            return Err(String::from("method arg expected 0 arguments"));
        }
        let this_s = Self::get_str(this)?;
        if let Ok(i) = this_s.parse::<i64>() {
            return Ok(Some(ValueVariant::Integer(i)));
        } else {
            return Err(format!("can't convert {} to int", this_s.to_string()));    
        }
    }

    fn to_float(this: &ValueVariant, args:&StdMethodArgs) -> StdMethodResult {
        if args.len() != 0 {
            return Err(String::from("method arg expected 0 arguments"));
        }
        let this_s = Self::get_str(this)?;
        if let Ok(f) = this_s.parse::<f64>() {
            return Ok(Some(ValueVariant::Float(f)));
        } else {
            return Err(format!("can't convert {} to int", this_s.to_string()));    
        }
    }

}

impl StdMethodsRepository for StringMethods {
    fn get_diterminant(&self) -> std::mem::Discriminant<crate::ex_core::ValueVariant> {
        return std::mem::discriminant(&&crate::ex_core::ValueVariant::String("".to_string()));
    }

    fn get_methods(&self) -> StdMethodsMap {
        return self.methods.clone();
    }
}
