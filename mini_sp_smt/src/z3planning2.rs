use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};
use z3_sys::*;
use super::*;

#[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct BoolVariable {
    name: String,
    domain: Vec<bool>
}

#[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct IntVariable {
    name: String,
    domain: Vec<i32>
}

#[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct EnumVariable {
    name: String,
    r#type: String,
    domain: Vec<String>
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct BoolAssignment {
    var: BoolVariable,
    val: bool
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct IntAssignment {
    var: IntVariable,
    val: i32
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct EnumAssignment {
    var: EnumVariable,
    val: String
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub enum Thing {
    Value,
    Variable, 
    Predicate
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub enum Predicate {
    TRUE,
    FALSE,
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    NOT(Vec<Predicate>),
    NEXT(Vec<Predicate>, Vec<Predicate>),
    AFTER(Vec<Predicate>, Vec<Predicate>, u32),
    GLOB(Vec<Predicate>),
    PBEQ(Vec<Predicate>, i32),
    EQ(Thing, Thing),
    NEQ(Thing, Thing)
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParamTransition {
    name: String,
    guard: Vec<(String, Predicate)>,
    update: Vec<(String, Predicate)>
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

impl EnumVariable {
    /// Creates a new enum variable with a defined type
    pub fn new(name: &str, r#type: &str, domain: &Vec<&str>) -> EnumVariable {
        EnumVariable { name: name.to_string(),
                      r#type: r#type.to_string(),
                      domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()}
    }
}

impl BoolAssignment {
    pub fn new(var: &BoolVariable, val: &bool) -> BoolAssignment {
        BoolAssignment {
            var: var.clone(),
            val: *val
        }
    }
}

impl IntAssignment {
    pub fn new(var: &IntVariable, val: &i32) -> IntAssignment {
        match var.domain.contains(&val) {
            true => {
                IntAssignment {
                    var: var.clone(),
                    val: *val
                }
            },
            false => panic!("E001: Value not in the variable domain.")
        }
    }
}

impl EnumAssignment {
    pub fn new(var: &EnumVariable, val: &str) -> EnumAssignment {
        match var.domain.contains(&val.to_string()) {
            true => EnumAssignment {
                var: var.clone(),
                val: val.to_string()
            },
            false => panic!("E002: Value not in the variable domain.")
        }
    }
}

#[test]
fn test_variables(){
    println!("{:?}", BoolVariable::new("x"));
    println!("{:?}", IntVariable::new("y", &vec!(&1, &2, &4, &5)));
    println!("{:?}", EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d")));
}

#[test]
fn test_assignments(){
    println!("{:?}", BoolAssignment::new(&BoolVariable::new("x"), &true));
    println!("{:?}", IntAssignment::new(&IntVariable::new("y", &vec!(&1, &2, &4, &5)), &4));
    println!("{:?}", EnumAssignment::new(&EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d")), "c"));
}
