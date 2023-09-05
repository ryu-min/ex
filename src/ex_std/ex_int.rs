use crate::ex_core::ValueVariant;

use super::method_repository::{StdMethodsMap, StdMethodsRepository, StdMethodArgs, StdMethodResult};


pub struct IntMethods {
    methods: StdMethodsMap
}

impl IntMethods {
    pub fn new() -> Self {
        let mut methods = StdMethodsMap::new();
        methods.insert("test".to_string(), IntMethods::test);
        return Self {
            methods : methods
        }
    }

    fn test(me: &ValueVariant, args: &StdMethodArgs) -> StdMethodResult {
        println!("call test function with this {}", me.to_string());
        println!("arg count is {}", args.len());
        for arg in args.iter() {
            println!("{}",arg.to_string());
        }
        return Ok(Some(ValueVariant::Integer(69)));
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
