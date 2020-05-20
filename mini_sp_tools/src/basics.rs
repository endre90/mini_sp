#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Parameter {
    pub name: String,
    pub value: bool
}

#[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct EnumVariable {
    pub name: String,
    pub r#type: String,
    pub domain: Vec<String>,
    pub param: Parameter,
}

impl Parameter {
    pub fn new(name: &str, value: &bool) -> Parameter {
        match name == "TRUE" {
            true => panic!("Error 5b376941-3c6e-4b52-bec3-49eb8d9991bb: Parameter name 'TRUE' is reserved."),
            false => {
                Parameter {
                    name: name.to_string(),
                    value: *value
                }
            }
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            name: "TRUE".to_string(),
            value: true
        }
    }
}

impl EnumVariable{
    pub fn new(name: &str, r#type: &str, domain: &Vec<&str>, param: Option<&Parameter>) -> EnumVariable {
        EnumVariable { 
            param: match param {
                Some(x) => x.clone(),
                None => Parameter::default()
            },
            name: name.to_string(),
            r#type: r#type.to_string(),
            domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()
        }
    }
}

#[test]
fn test_new_enum_variable(){
    let param = Parameter::new("param1", &false);
    assert_eq!("EnumVariable { name: \"z\", type: \"letters\", domain: [\"a\", \"b\", \"c\", \"d\"], param: Parameter { name: \"TRUE\", value: true } }", 
        &format!("{:?}", EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None)));
    assert_eq!("EnumVariable { name: \"z\", type: \"letters\", domain: [\"a\", \"b\", \"c\", \"d\"], param: Parameter { name: \"param1\", value: false } }", 
        &format!("{:?}", EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), Some(&param))));

}

// #[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct ParamBoolVariable {
//     pub param: Parameter,
//     pub name: String,
//     pub domain: Vec<bool>
// }

// #[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct ParamIntVariable {
//     pub param: Parameter,
//     pub name: String,
//     pub domain: Vec<i32>
// }

// impl ParamBoolVariable {
//     /// Creates a new boolean type parameterized variable
//     pub fn new(name: &str, param: &Parameter) -> ParamBoolVariable {
//         ParamBoolVariable { 
//             param: param.clone(),
//             name: name.to_string(), 
//             domain: vec!(true, false)}
//     }
// }

// impl ParamIntVariable {
//     /// Creates a new integer type parameterized variable
//     pub fn new(name: &str, domain: &Vec<&i32>, param: &Parameter) -> ParamIntVariable {
//         ParamIntVariable { 
//             param: param.clone(),
//             name: name.to_string(), 
//             domain: domain.iter().map(|x| **x).collect::<Vec<i32>>()
//         }
//     }
// }


// #[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct BoolVariable {
//     pub name: String,
//     pub domain: Vec<bool>
// }

// #[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct IntVariable {
//     pub name: String,
//     pub domain: Vec<i32>
// }

// #[derive(Hash, Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct BasicEnumVariable {
//     pub name: String,
//     pub r#type: String,
//     pub domain: Vec<String>
// }

// #[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct ParamEnumVariable {
//     pub param: Option<Parameter>,
//     pub name: String,
//     pub r#type: String,
//     pub domain: Vec<String>
// }

// #[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct EnumVariable {
//     pub basic: BasicEnumVariable,
//     pub param: ParamEnumVariable
// }

// impl BoolVariable {
//     /// Creates a new boolean type variable
//     pub fn new(name: &str) -> BoolVariable {
//         BoolVariable { 
//             name: name.to_string(), 
//             domain: vec!(true, false)
//         }
//     }
// }

// impl IntVariable {
//     /// Creates a new integer type variable
//     pub fn new(name: &str, domain: &Vec<&i32>) -> IntVariable {
//         IntVariable { 
//             name: name.to_string(), 
//             domain: domain.iter().map(|x| **x).collect::<Vec<i32>>()
//         }
//     }
// }

// impl BasicEnumVariable{
//     /// Creates a new enum variable with a defined type
//     pub fn new(name: &str, r#type: &str, domain: &Vec<&str>) -> BasicEnumVariable {
//         BasicEnumVariable { 
//             name: name.to_string(),
//             r#type: r#type.to_string(),
//             domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()
//         }
//     }
// }

// impl ParamEnumVariable{
//     /// Creates a new enum parameterized variable with a defined type
//     pub fn new(name: &str, r#type: &str, domain: &Vec<&str>, param: &Parameter) -> ParamEnumVariable {
//         ParamEnumVariable { 
//             param: param.clone(),
//             name: name.to_string(),
//             r#type: r#type.to_string(),
//             domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()
//         }
//     }
// }

// impl EnumVariable{
//     /// Creates both basic and param enum varisbles so that predicates are easier to handle
//     pub fn new(name: &str, r#type: &str, domain: &Vec<&str>, param: Option<&Parameter>) -> EnumVariable {
//         match 

//         ParamEnumVariable { 
//             param: param.clone(),
//             name: name.to_string(),
//             r#type: r#type.to_string(),
//             domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()
//         }
//     }
// }


// #[test]
// fn test_variables(){
//     assert_eq!("BoolVariable { name: \"x\", domain: [true, false] }", 
//         &format!("{:?}", BoolVariable::new("x")));
//     assert_eq!("IntVariable { name: \"y\", domain: [1, 2, 4, 5] }", 
//         &format!("{:?}", IntVariable::new("y", &vec!(&1, &2, &4, &5))));
//     assert_eq!("EnumVariable { name: \"z\", type: \"letters\", domain: [\"a\", \"b\", \"c\", \"d\"] }", 
//         &format!("{:?}", BasicEnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"))));
// }
