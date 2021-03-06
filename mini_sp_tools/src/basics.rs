#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Parameter {
    pub name: String,
    pub value: bool
}

#[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct BoolVariable {
    pub name: String,
    pub domain: Vec<bool>,
    pub param: Parameter
}

#[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct IntVariable {
    pub name: String,
    pub domain: Vec<i32>,
    pub param: Parameter
}

#[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
pub struct EnumVariable {
    pub name: String,
    pub r#type: String,
    pub domain: Vec<String>,
    pub param: Parameter
}

// #[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct EnumAssignment {
//     pub var: EnumVariable,
//     pub val: String
// }

// for now, later have to figure out how to add bool and int
// #[derive(Eq, Debug, PartialEq, Clone, PartialOrd, Ord)]
// pub struct State {
//     pub variables: Vec<EnumVariable>,
//     pub asssignments: Vec<EnumAssignment>,
//     pub partial: bool
// }

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


impl BoolVariable {
    pub fn new(name: &str, param: Option<&Parameter>) -> BoolVariable {
        BoolVariable { 
            param: match param {
                Some(x) => x.to_owned(),
                None => Parameter::default()
            },
            name: name.to_string(), 
            domain: vec!(true, false)}
    }
}

impl IntVariable {
    pub fn new(name: &str, domain: &Vec<&i32>, param: Option<&Parameter>) -> IntVariable {
        IntVariable { 
            param: match param {
                Some(x) => x.to_owned(),
                None => Parameter::default()
            },
            name: name.to_string(), 
            domain: domain.iter().map(|x| **x).collect::<Vec<i32>>()
        }
    }
}

impl EnumVariable{
    pub fn new(name: &str, r#type: &str, domain: &Vec<&str>, param: Option<&Parameter>) -> EnumVariable {
        EnumVariable { 
            param: match param {
                Some(x) => x.to_owned(),
                None => Parameter::default()
            },
            name: match name == "EMPTY" {
                true => panic!("Error 69e2abf9-498b-4d5c-88c7-30ea70ed27fb: EnumVariable name 'EMPTY' is reserved."),
                false => name.to_string()
            },
            r#type: r#type.to_string(),
            domain: domain.iter().map(|x| x.to_string()).collect::<Vec<String>>()
        }
    }
}

impl Default for EnumVariable {
    fn default() -> Self {
        EnumVariable {
            param: Parameter::default(),
            name: "EMPTY".to_string(),
            r#type: "EMPTY".to_string(),
            domain: vec!()
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