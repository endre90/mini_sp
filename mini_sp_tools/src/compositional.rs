use std::time::{Duration, Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct Activate {
    pub params: Vec<Parameter>
}

impl Activate {
    pub fn new(params: &Vec<Parameter>) -> Vec<Parameter> {
        let mut new_params: Vec<Parameter> = vec!();
        let mut activated_ff: bool = false;
        match params.iter().all(|x| x.value) {
            true => panic!("Error 830d4128-68b0-42f2-9ec6-64717fd17b74: All parameters active in Activate."),
            false => {
                for param in params {
                    if param.value | activated_ff {
                        new_params.push(param.clone())
                    } else {
                        let activated = Parameter::new(&param.name, &true);
                        activated_ff = true;
                        new_params.push(activated)
                    }
                }
            }
        }
        new_params
    }
}

#[test]
fn test_activate() {
    let param_a = Parameter::new("a", &true);
    let param_b = Parameter::new("b", &false);
    let param_c = Parameter::new("c", &false);
    let params = vec!(param_a, param_b, param_c);
    assert_eq!("[Parameter { name: \"a\", value: true }, Parameter { name: \"b\", value: false }, Parameter { name: \"c\", value: false }]", format!("{:?}", params));
    let new_params = Activate::new(&params);
    assert_eq!("[Parameter { name: \"a\", value: true }, Parameter { name: \"b\", value: true }, Parameter { name: \"c\", value: false }]", format!("{:?}", new_params));
}

#[test]
#[should_panic(expected = "Error 830d4128-68b0-42f2-9ec6-64717fd17b74: All parameters active in Activate.")]
fn test_activate_panic() {
    let param_a = Parameter::new("a", &true);
    let param_b = Parameter::new("b", &true);
    let param_c = Parameter::new("c", &true);
    let params = vec!(param_a, param_b, param_c);
    Activate::new(&params);
}