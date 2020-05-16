use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub enum Predicate {
    TRUE,
    FALSE,
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    NOT(Box<Predicate>),
    EQRL(EnumVariable, String),
    EQRR(EnumVariable, EnumVariable),
    EQLR(String, EnumVariable),
    EQPP(Box<Predicate>, Box<Predicate>),
    NEQRL(EnumVariable, String),
    NEQRR(EnumVariable, EnumVariable),
    NEQLR(String, EnumVariable),
    NEQPP(Box<Predicate>, Box<Predicate>),
    NEXT(Box<Predicate>, Box<Predicate>),
    GLOB(Box<Predicate>),
    // AFTER(Predicate, Predicate, u32),
    
    // PBEQ(Vec<Predicate>, i32),
    // EQ(Assignment),
    // NEQ(Thing, Thing)
}

pub struct PredicateToAstZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub pred: Predicate,
    pub step: u32,
    pub r: Z3_ast
}

impl <'ctx> PredicateToAstZ3<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, pred: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        match pred {
            Predicate::TRUE => BoolZ3::new(&ctx, true),
            Predicate::FALSE => BoolZ3::new(&ctx, false),
            Predicate::NOT(p) => NOTZ3::new(&ctx, PredicateToAstZ3::new(&ctx, p, r#type, step)),
            Predicate::AND(p) => ANDZ3::new(&ctx, p.iter().map(|x| PredicateToAstZ3::new(&ctx, x, r#type, step)).collect()),
            Predicate::OR(p) => ORZ3::new(&ctx, p.iter().map(|x| PredicateToAstZ3::new(&ctx, x, r#type, step)).collect()),
            Predicate::EQRL(x, y) => {
                match x.domain.contains(&y) {
                    true => {
                        let sort = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                        let elems = &sort.enum_asts;
                        let index = x.domain.iter().position(|r| *r == y.to_string()).unwrap();
                        EQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.name.to_string(), step).as_str()), elems[index])      
                    },
                    false => panic!("Error: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::EQRR(x, y) => {
                match x.r#type == y.r#type {
                    true => {
                        match r#type {
                            "guard" | "state" => {
                                let sort_1 = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                                let sort_2 = EnumSortZ3::new(&ctx, &y.r#type, y.domain.iter().map(|y| y.as_str()).collect());
                                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.name.to_string(), step).as_str());
                                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.name.to_string(), step).as_str());
                                EQZ3::new(&ctx, v_1, v_2)
                            },
                            "update" => {
                                let sort_1 = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                                let sort_2 = EnumSortZ3::new(&ctx, &y.r#type, y.domain.iter().map(|y| y.as_str()).collect());
                                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.name.to_string(), step).as_str());
                                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.name.to_string(), step - 1).as_str());
                                EQZ3::new(&ctx, v_1, v_2)
                            },
                            _ => panic!("Error: Predicate type '{}' is not allowed.", r#type)
                        }
                    },
                    false => panic!("Error: Sorts '{}' and '{}' are incompatible.", x.r#type, y.r#type)
                }
            },
            Predicate::EQLR(y, x) => {
                match x.domain.contains(&y) {
                    true => {
                        let sort = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                        let elems = &sort.enum_asts;
                        let index = x.domain.iter().position(|r| *r == y.to_string()).unwrap();
                        EQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.name.to_string(), step).as_str()), elems[index])      
                    },
                    false => panic!("Error: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::EQPP(x, y) => EQZ3::new(&ctx, PredicateToAstZ3::new(&ctx, x, r#type, step), PredicateToAstZ3::new(&ctx, y, r#type, step)),
            Predicate::NEQRL(x, y) => {
                match x.domain.contains(&y) {
                    true => {
                        let sort = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                        let elems = &sort.enum_asts;
                        let index = x.domain.iter().position(|r| *r == y.to_string()).unwrap();
                        NEQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.name.to_string(), step).as_str()), elems[index])      
                    },
                    false => panic!("Error: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::NEQRR(x, y) => {
                match x.r#type == y.r#type {
                    true => {
                        match r#type {
                            "guard" | "state" => {
                                let sort_1 = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                                let sort_2 = EnumSortZ3::new(&ctx, &y.r#type, y.domain.iter().map(|y| y.as_str()).collect());
                                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.name.to_string(), step).as_str());
                                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.name.to_string(), step).as_str());
                                NEQZ3::new(&ctx, v_1, v_2)
                            },
                            "update" => {
                                let sort_1 = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                                let sort_2 = EnumSortZ3::new(&ctx, &y.r#type, y.domain.iter().map(|y| y.as_str()).collect());
                                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.name.to_string(), step).as_str());
                                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.name.to_string(), step - 1).as_str());
                                NEQZ3::new(&ctx, v_1, v_2)
                            },
                            _ => panic!("Error: Predicate type '{}' is not allowed.", r#type)
                        }
                    },
                    false => panic!("Error: Sorts '{}' and '{}' are incompatible.", x.r#type, y.r#type)
                }
            },
            Predicate::NEQLR(y, x) => {
                match x.domain.contains(&y) {
                    true => {
                        let sort = EnumSortZ3::new(&ctx, &x.r#type, x.domain.iter().map(|x| x.as_str()).collect());
                        let elems = &sort.enum_asts;
                        let index = x.domain.iter().position(|r| *r == y.to_string()).unwrap();
                        NEQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.name.to_string(), step).as_str()), elems[index])      
                    },
                    false => panic!("Error: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::NEQPP(x, y) => NEQZ3::new(&ctx, PredicateToAstZ3::new(&ctx, x, r#type, step), PredicateToAstZ3::new(&ctx, y, r#type, step)),
            Predicate::NEXT(x, y) => NextZ3::new(&ctx, &x, &y, r#type, step),
            Predicate::GLOB(x) => GloballyZ3::new(&ctx, &x, r#type, step)
        }
    }
}

#[test]
fn test_true_predicate(){

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let t = Predicate::TRUE;
    let pred = PredicateToAstZ3::new(&ctx, &t, "guard", &3);
    assert_eq!("true", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_false_predicate(){

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let t = Predicate::FALSE;
    let pred = PredicateToAstZ3::new(&ctx, &t, "guard", &3);
    assert_eq!("false", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_not_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::NOT(Box::new(Predicate::EQRR(x, y)));
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(not (= x_s3 y_s3))", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_and_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::AND(vec!(Predicate::EQRR(x, y.clone()), Predicate::EQRR(y, z)));
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(and (= x_s3 y_s3) (= y_s3 z_s3))", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_or_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::OR(vec!(Predicate::EQRR(x, y.clone()), Predicate::EQRR(y, z)));
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(or (= x_s3 y_s3) (= y_s3 z_s3))", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_eqrl_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRL(x, b);
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(= x_s3 b)", ast_to_string_z3!(&ctx, pred));
}

#[test]
#[should_panic(expected = "Error: Value 'e' not in the domain of variable 'x'.")]
fn test_eqrl_predicate_panic(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let e = "e".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRL(x, e);
    PredicateToAstZ3::new(&ctx, &n, "guard", &3);
}

#[test]
fn test_eqrr_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRR(x, y.clone());
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(= x_s3 y_s3)", ast_to_string_z3!(&ctx, pred));
}

#[test]
#[should_panic(expected = "Error: Predicate type 'other' is not allowed.")]
fn test_eqrr_predicate_panic_1(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRR(x, y.clone());
    PredicateToAstZ3::new(&ctx, &n, "other", &3);
}

#[test]
#[should_panic(expected = "Error: Sorts 'letters' and 'numbers' are incompatible.")]
fn test_eqrr_predicate_panic_2(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "numbers", &vec!("1", "2", "3", "4"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRR(x, y.clone());
    PredicateToAstZ3::new(&ctx, &n, "state", &3);
}

#[test]
fn test_eqlr_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQLR(b, x);
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(= x_s3 b)", ast_to_string_z3!(&ctx, pred));
}

#[test]
#[should_panic(expected = "Error: Value 'e' not in the domain of variable 'x'.")]
fn test_eqlr_predicate_panic(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let e = "e".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQLR(e, x);
    PredicateToAstZ3::new(&ctx, &n, "guard", &3);
}

#[test]
fn test_next_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let prev = Predicate::EQRL(x.clone(), b);
    let next = Predicate::EQRL(x.clone(), c);
    let pred = Predicate::NEXT(Box::new(prev), Box::new(next));
    let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
    assert_eq!("(and (= x_s3 b) (= x_s4 c))", ast_to_string_z3!(&ctx, ast));
}

#[test]
fn test_glob_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let prev = Predicate::EQRL(x.clone(), b.clone());
    let pred = Predicate::GLOB(Box::new(prev));
    let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
    assert_eq!("(and (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b))", ast_to_string_z3!(&ctx, ast));
}

#[test]
fn test_glob_next_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let prev = Predicate::EQRL(x.clone(), b.clone());
    let next = Predicate::EQRL(y.clone(), b.clone());
    let pred = Predicate::NEXT(Box::new(prev), Box::new(next));
    let glob_pred = Predicate::GLOB(Box::new(pred));
    let ast = PredicateToAstZ3::new(&ctx, &glob_pred, "guard", &3);
    assert_eq!("(and (= x_s3 b) (= x_s4 c))", ast_to_string_z3!(&ctx, ast));

}