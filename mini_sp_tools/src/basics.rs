use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};
use super::*;

#[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct BoolVariable {
    pub name: String,
    pub domain: Vec<bool>
}

#[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct IntVariable {
    pub name: String,
    pub domain: Vec<i32>
}

#[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct EnumVariable {
    pub name: String,
    pub r#type: String,
    pub domain: Vec<String>
}

impl BoolVariable {
    /// Creates a new boolean type variable
    pub fn new(name: &str) -> BoolVariable {
        BoolVariable { name: name.to_string(), 
                       domain: vec!(true, false)}
    }
}

impl IntVariable {
    /// Creates a new integer type variable
    pub fn new(name: &str, domain: &Vec<&i32>) -> IntVariable {
        IntVariable { name: name.to_string(), 
                      domain: domain.iter().map(|x| **x).collect::<Vec<i32>>()}
    }
}

impl EnumVariable{
    /// Creates a new enum variable with a defined type
    pub fn new(name: &str, r#type: &str, domain: &Vec<&str>) -> EnumVariable {
        EnumVariable { name: name.to_string(),
                      r#type: r#type.to_string(),
                      domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()}
    }
}

#[test]
fn test_variables(){
    assert_eq!("BoolVariable { name: \"x\", domain: [true, false] }", 
        &format!("{:?}", BoolVariable::new("x")));
    assert_eq!("IntVariable { name: \"y\", domain: [1, 2, 4, 5] }", 
        &format!("{:?}", IntVariable::new("y", &vec!(&1, &2, &4, &5))));
    assert_eq!("EnumVariable { name: \"z\", type: \"letters\", domain: [\"a\", \"b\", \"c\", \"d\"] }", 
        &format!("{:?}", EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"))));
}
