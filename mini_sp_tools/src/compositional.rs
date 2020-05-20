use std::time::{Duration, Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct Activate {
    pub params: Vec<Parameter>
}

pub struct StateToParamPredicate {
    pub state: Vec<String>,
    pub prob: ParamPlanningProblem,
    pub ppred: Vec<ParamPredicate>
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

// impl StateToParamPredicate {
//     pub fn new(state: &Vec<&str>, prob: &ParamPlanningProblem) -> Vec<ParamPredicate> {
//         for s in state {
//             let sep: Vec<&str> = s.split(" -> ").collect();
//             for v in GetParamProblemVars::new(prob) {
//                 let var = if v.name == sep[0] {
//                     v;
//                     break;
//                 } else {
//                     panic!("var not in model?")
//                 };
//             };
//         } 
//         vec!()
//     }
// }

    // impl ParamStateToPredicateNew {
    //     pub fn new(state: &Vec<&str>, p: &ParamPlanningProblemNew) -> Vec<(String, Predicate)> {
    //         let mut pred_vec: Vec<(String, Predicate)> = vec!();
    //         for s in state {
    //             // let pred: (&str, Predicate)
    //             let sep: Vec<&str> = s.split(" -> ").collect();
    //             let mut d: Vec<&str> = vec!();
    //             let mut t: &str = "";
    //             let mut n: &str = "";
    //             for v in &p.vars {
    //                 if v.1.n == sep[0] {
    //                     n = sep[0];
    //                     d = v.1.d.iter().map(|x| x.as_str()).collect();
    //                     t = &v.1.t;
    //                 }
    //             }
    
    //             let var = Variable::new(n, t, d);
    //             let val = sep[1];
    //             let mut activator: String = String::from("");
    //             for param in &p.params {
    //                 if var.n.ends_with(&param.0) {
    //                     activator = param.0.to_string()
    //                 }
    //             };
                
    //             pred_vec.push((activator, Predicate::EQVAL(var, String::from(val))));
    //         }
    //         pred_vec
    //     }
    // }






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