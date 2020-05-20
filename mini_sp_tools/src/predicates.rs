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
    PBEQ(Vec<Predicate>, i32), // exactly n true predicates in a step
    NEXT(Box<Predicate>), // in the next step
    ALWAYS(Box<Predicate>), // in every step of the trace
    NEVER(Box<Predicate>), // neve in the trace
    EVENTUALLY(Box<Predicate>), // at least once in the trace
    UNTIL(Box<Predicate>, Box<Predicate>), // first one true until the second
    RELEASE(Box<Predicate>, Box<Predicate>), // first one releases the second of the duty to be true when it becomes true
    AFTER(Box<Predicate>, Box<Predicate>), // a predicate should hold in the step after the other
    // SAFTER(Box<Predicate>, Box<Predicate>, u32), // a predicate should hold somewhen after the other, good for sequences
    TPBEQ(Box<Predicate>, u32) // exactly n times true in a trace
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct ParamPredicate {
    pub preds: Vec<Predicate>
}

pub struct PredicateToAstZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub pred: Predicate,
    pub step: u32,
    pub r: Z3_ast
}

impl ParamPredicate {
    pub fn new(preds: &Vec<&Predicate>) -> ParamPredicate {
        ParamPredicate {
            preds: preds.iter().map(|&x| x.clone()).collect()
        }
    }
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
                    false => panic!("Error 6f789b86-7f6c-4426-ab0f-6b5b72dd2c55: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::EQRR(x, y) => {
                match x.r#type == y.r#type {
                    true => {
                        match r#type {
                            "guard" | "state" | "specs" => {
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
                            _ => panic!("Error 53b0fd14-1ddd-4bf0-8dc7-d372d6ad8c99: Predicate type '{}' is not allowed.", r#type)
                        }
                    },
                    false => panic!("Error c8022e33-ed30-43af-8e45-8cfdaf09e8a5: Sorts '{}' and '{}' are incompatible.", x.r#type, y.r#type)
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
                    false => panic!("Error c8250dfd-6d3c-4371-8fee-813ba5100d80: Value '{}' not in the domain of variable '{}'.", y, x.name)
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
                    false => panic!("Error 82ffe471-c922-43b0-bfa0-98c27626408e: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::NEQRR(x, y) => {
                match x.r#type == y.r#type {
                    true => {
                        match r#type {
                            "guard" | "state" | "specs" => {
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
                            _ => panic!("Error 9ed281a9-173c-44ab-9226-cdbcdd13cc83: Predicate type '{}' is not allowed.", r#type)
                        }
                    },
                    false => panic!("Error 708e3200-2c6c-4a2b-a283-1e0b5b87b2cc: Sorts '{}' and '{}' are incompatible.", x.r#type, y.r#type)
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
                    false => panic!("Error 44d999a5-4dac-4957-ac02-7806ce8f0ea8: Value '{}' not in the domain of variable '{}'.", y, x.name)
                }
            },
            Predicate::NEQPP(x, y) => NEQZ3::new(&ctx, PredicateToAstZ3::new(&ctx, x, r#type, step), PredicateToAstZ3::new(&ctx, y, r#type, step)),
            Predicate::PBEQ(x, k) => PBEQZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, r#type, step)).collect(), *k),
            Predicate::NEXT(x) => NextZ3::new(&ctx, &x, r#type, step),
            Predicate::ALWAYS(x) => AlwaysZ3::new(&ctx, &x, r#type, step),
            Predicate::NEVER(x) => AlwaysZ3::new(&ctx, &Predicate::NOT(x.clone()), r#type, step),
            Predicate::EVENTUALLY(x) => EventuallyZ3::new(&ctx, &x, r#type, step),
            Predicate::UNTIL(x, y) => UntilZ3::new(&ctx, &x, &y, r#type, step),
            Predicate::RELEASE(x, y) => ReleaseZ3::new(&ctx, &x, &y, r#type, step),
            Predicate::AFTER(x, y) => AfterZ3::new(&ctx, &x, &y, r#type, step),
            // Predicate::SAFTER(x, y, z) => SomewhenAfterZ3::new(&ctx, &x, &y, r#type, step, &z),
            Predicate::TPBEQ(x, y) => TracePBEQZ3::new(&ctx, &x, r#type, &y, step)
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

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::NOT(Box::new(Predicate::EQRR(x, y)));
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(not (= x_s3 y_s3))", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_and_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None);

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::AND(vec!(Predicate::EQRR(x, y.clone()), Predicate::EQRR(y, z)));
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(and (= x_s3 y_s3) (= y_s3 z_s3))", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_or_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None);

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::OR(vec!(Predicate::EQRR(x, y.clone()), Predicate::EQRR(y, z)));
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(or (= x_s3 y_s3) (= y_s3 z_s3))", ast_to_string_z3!(&ctx, pred));
}

#[test]
fn test_eqrl_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRL(x, b);
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(= x_s3 b)", ast_to_string_z3!(&ctx, pred));
}

#[test]
#[should_panic(expected = "Error 6f789b86-7f6c-4426-ab0f-6b5b72dd2c55: Value 'e' not in the domain of variable 'x'.")]
fn test_eqrl_predicate_panic(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let e = "e".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRL(x, e);
    PredicateToAstZ3::new(&ctx, &n, "guard", &3);
}

#[test]
fn test_eqrr_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None);

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRR(x, y.clone());
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(= x_s3 y_s3)", ast_to_string_z3!(&ctx, pred));
}

#[test]
#[should_panic(expected = "Error 53b0fd14-1ddd-4bf0-8dc7-d372d6ad8c99: Predicate type 'other' is not allowed.")]
fn test_eqrr_predicate_panic_1(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRR(x, y.clone());
    PredicateToAstZ3::new(&ctx, &n, "other", &3);
}

#[test]
#[should_panic(expected = "Error c8022e33-ed30-43af-8e45-8cfdaf09e8a5: Sorts 'letters' and 'numbers' are incompatible.")]
fn test_eqrr_predicate_panic_2(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "numbers", &vec!("1", "2", "3", "4"), None);

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQRR(x, y.clone());
    PredicateToAstZ3::new(&ctx, &n, "state", &3);
}

#[test]
fn test_eqlr_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQLR(b, x);
    let pred = PredicateToAstZ3::new(&ctx, &n, "guard", &3);
    assert_eq!("(= x_s3 b)", ast_to_string_z3!(&ctx, pred));
}

#[test]
#[should_panic(expected = "Error c8250dfd-6d3c-4371-8fee-813ba5100d80: Value 'e' not in the domain of variable 'x'.")]
fn test_eqlr_predicate_panic(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let e = "e".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let n = Predicate::EQLR(e, x);
    PredicateToAstZ3::new(&ctx, &n, "guard", &3);
}

#[test]
fn test_pbeq_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();
    let d = "d".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let nb = Predicate::EQLR(b, x);
    let nc = Predicate::EQLR(c, y);
    let nd = Predicate::EQLR(d, z);
    let pbeq = Predicate::PBEQ(vec!(nb, nc, nd), 2);
    let pred = PredicateToAstZ3::new(&ctx, &pbeq, "guard", &4);
    // assert_eq!("(= x_s3 b)", ast_to_string_z3!(&ctx, pred));

    slv_assert_z3!(&ctx, &slv, pred);
    slv_check_z3!(&ctx, &slv);

    let model = slv_get_model_z3!(&ctx, &slv);
    assert_eq!("z_s4 -> b\nx_s4 -> b\ny_s4 -> c\n", model_to_string_z3!(&ctx, model));
}

#[test]
fn test_always_pbeq_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();
    let d = "d".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let nb = Predicate::EQLR(b, x);
    let nc = Predicate::EQLR(c, y);
    let nd = Predicate::EQLR(d, z);
    let pbeq = Predicate::PBEQ(vec!(nb, nc, nd), 2);
    let pred = Predicate::ALWAYS(Box::new(pbeq));
    let glob_pred = PredicateToAstZ3::new(&ctx, &pred, "guard", &2);
    
    // assert_eq!("(= x_s3 b)", ast_to_string_z3!(&ctx, pred));

    slv_assert_z3!(&ctx, &slv, glob_pred);
    slv_check_z3!(&ctx, &slv);

    let model = slv_get_model_z3!(&ctx, &slv);
    assert_eq!("x_s2 -> b\nx_s0 -> b\ny_s1 -> b\nz_s2 -> d\ny_s2 -> b\nz_s1 -> d\ny_s0 -> b\nz_s0 -> d\nx_s1 -> b\n", model_to_string_z3!(&ctx, model));
}

#[test]
fn test_next_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let prev = Predicate::EQRL(x.clone(), b);

    let pred = Predicate::NEXT(Box::new(prev));
    let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
    assert_eq!("(= x_s4 b)", ast_to_string_z3!(&ctx, ast));
}

#[test]
fn test_after_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let prev = Predicate::EQRL(x.clone(), b);
    let next = Predicate::EQRL(x.clone(), c);
    let pred = Predicate::AFTER(Box::new(prev), Box::new(next));
    let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
    assert_eq!("(and (= x_s3 b) (= x_s4 c))", ast_to_string_z3!(&ctx, ast));
}

// #[test]
// fn test_safter_predicate(){

//     let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
//     let b = "b".to_string();
//     let c = "c".to_string();

//     let cfg = ConfigZ3::new();
//     let ctx = ContextZ3::new(&cfg);
//     let prev = Predicate::EQRL(x.clone(), b);
//     let next = Predicate::EQRL(x.clone(), c);
//     let pred = Predicate::SAFTER(Box::new(prev), Box::new(next), 6);
//     let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
//     assert_eq!("(and (= x_s3 b) (or (= x_s4 c) (= x_s5 c) (= x_s6 c)))", ast_to_string_z3!(&ctx, ast));
// }

// #[test]
// fn test_eventually_safter_predicate(){

//     let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
//     let b = "b".to_string();
//     let c = "c".to_string();

//     let cfg = ConfigZ3::new();
//     let ctx = ContextZ3::new(&cfg);
//     let prev = Predicate::EQRL(x.clone(), b);
//     let next = Predicate::EQRL(x.clone(), c);
//     let pred = Predicate::SAFTER(Box::new(prev), Box::new(next), 6);
//     let alwys = Predicate::EVENTUALLY(Box::new(pred));
//     let ast = PredicateToAstZ3::new(&ctx, &alwys, "guard", &3);
//     assert_eq!("(or (and (= x_s0 b)\n         (or (= x_s1 c) (= x_s2 c) (= x_s3 c) (= x_s4 c) (= x_s5 c) (= x_s6 c)))\n    (and (= x_s1 b) (or (= x_s2 c) (= x_s3 c) (= x_s4 c) (= x_s5 c) (= x_s6 c)))\n    (and (= x_s2 b) (or (= x_s3 c) (= x_s4 c) (= x_s5 c) (= x_s6 c)))\n    (and (= x_s3 b) (or (= x_s4 c) (= x_s5 c) (= x_s6 c))))", ast_to_string_z3!(&ctx, ast));
// }

#[test]
fn test_always_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let prev = Predicate::EQRL(x.clone(), b.clone());
    let pred = Predicate::ALWAYS(Box::new(prev));
    let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
    assert_eq!("(and (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b))", ast_to_string_z3!(&ctx, ast));
}

#[test]
fn test_always_after_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let prev = Predicate::EQRL(x.clone(), b.clone());
    let next = Predicate::EQRL(y.clone(), c.clone());
    let pred = Predicate::AFTER(Box::new(prev), Box::new(next));
    let glob_pred = Predicate::ALWAYS(Box::new(pred));
    let ast = PredicateToAstZ3::new(&ctx, &glob_pred, "guard", &3);
    assert_eq!("(and (= x_s0 b)\n     (= y_s1 c)\n     (= x_s1 b)\n     (= y_s2 c)\n     (= x_s2 b)\n     (= y_s3 c)\n     (= x_s3 b)\n     (= y_s4 c))", ast_to_string_z3!(&ctx, ast));
}

#[test]
fn test_eventually_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let prev = Predicate::EQRL(x.clone(), b.clone());
    let pred = Predicate::EVENTUALLY(Box::new(prev));
    let ast = PredicateToAstZ3::new(&ctx, &pred, "guard", &3);
    assert_eq!("(or (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b))", ast_to_string_z3!(&ctx, ast));
}

#[test]
fn test_eventually_after_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let left = Predicate::EQLR(b, x);
    let right = Predicate::EQLR(c, y);
    let next = Predicate::AFTER(Box::new(left), Box::new(right));
    let alonce = Predicate::EVENTUALLY(Box::new(next));
    let glob_pred = PredicateToAstZ3::new(&ctx, &alonce, "guard", &3);

    slv_assert_z3!(&ctx, &slv, glob_pred);
    slv_check_z3!(&ctx, &slv);

    let model = slv_get_model_z3!(&ctx, &slv);
    assert_eq!("x_s2 -> a\nx_s0 -> b\ny_s1 -> c\ny_s4 -> b\ny_s3 -> b\ny_s2 -> b\nx_s3 -> b\nx_s1 -> a\n", model_to_string_z3!(&ctx, model));
}

#[test]
fn test_trace_pbeq_predicate(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let pred = Predicate::EQRL(x.clone(), b);

    let trace_pbeq = Predicate::TPBEQ(Box::new(pred), 2);
    let trace_pbeq_pred = PredicateToAstZ3::new(&ctx, &trace_pbeq, "guard", &4);

    assert_eq!("((_ pbeq 2 1 1 1 1 1) (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b) (= x_s4 b))", ast_to_string_z3!(&ctx, trace_pbeq_pred));

    slv_assert_z3!(&ctx, &slv, trace_pbeq_pred);
    slv_check_z3!(&ctx, &slv);

    let model = slv_get_model_z3!(&ctx, &slv);
    assert_eq!("x_s2 -> a\nx_s0 -> b\nx_s3 -> a\nx_s1 -> b\nx_s4 -> a\n", model_to_string_z3!(&ctx, model));
}