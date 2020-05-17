use std::ffi::{CStr, CString};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct NextZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast
}

pub struct AlwaysZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast
}

pub struct EventuallyZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast
}

pub struct UntilZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast,
    pub y: Z3_ast
}

pub struct ReleaseZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast,
    pub y: Z3_ast
}

impl <'ctx> NextZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        PredicateToAstZ3::new(&ctx, x, r#type, &(step + 1))
    } 
}

impl <'ctx> AlwaysZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        let mut assert_vec = vec!();
        for s in 0..step + 1 {
            assert_vec.push(
                PredicateToAstZ3::new(&ctx, x, r#type, &s)
            )
        }
        ANDZ3::new(&ctx, assert_vec)
    }
}

impl <'ctx> EventuallyZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        let mut assert_vec = vec!();
        for s in 0..step + 1 {
            assert_vec.push(
                PredicateToAstZ3::new(&ctx, x, r#type, &s)
            )
        }
        ORZ3::new(&ctx, assert_vec)
    }
}

impl <'ctx> UntilZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, y: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        let from_step: u32 = 0;
        fn recursive_subfn(ctx: &ContextZ3, x: &Predicate, y: &Predicate, 
            r#type: &str, mut from_step: u32, until_step: u32) -> Z3_ast {
            if from_step < until_step {
                from_step = from_step + 1;
                ORZ3::new(&ctx, 
                    vec!(
                        PredicateToAstZ3::new(&ctx, y, r#type, &(from_step - 1)),
                        ANDZ3::new(&ctx, 
                            vec!(
                                PredicateToAstZ3::new(&ctx, x, r#type, &(from_step - 1)),
                                recursive_subfn(&ctx, &x, &y, r#type, from_step, until_step)
                            )
                        )
                    )
                )
            } else if from_step == until_step{
                ANDZ3::new(&ctx, 
                    vec!(
                        PredicateToAstZ3::new(&ctx, y, r#type, &(from_step))
                    )
                )
            } else {
                panic! ("Error 76f32414-b307-4c41-a497-86746c009e56: from_step > until_step in Until ltlf.")
            }
        }
        recursive_subfn(&ctx, &x, &y, r#type, from_step, *step)
    }
}

impl <'ctx> ReleaseZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, y: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        let from_step: u32 = 0;
        fn recursive_subfn(ctx: &ContextZ3, x: &Predicate, y: &Predicate, 
            r#type: &str, mut from_step: u32, until_step: u32) -> Z3_ast {
            if from_step < until_step {
                from_step = from_step + 1;
                ANDZ3::new(&ctx, 
                    vec!(
                        PredicateToAstZ3::new(&ctx, y, r#type, &(from_step - 1)),
                        ORZ3::new(&ctx, 
                            vec!(
                                PredicateToAstZ3::new(&ctx, x, r#type, &(from_step - 1)),
                                recursive_subfn(&ctx, &x, &y, r#type, from_step, until_step)
                            )
                        )
                    )
                )
            } else if from_step == until_step{
                ORZ3::new(&ctx, 
                    vec!(
                        PredicateToAstZ3::new(&ctx, y, r#type, &(from_step))
                    )
                )
            } else {
                panic! ("Error cbf10fd3-6845-4786-bdaa-fa5ee564b4f8: from_step > until_step in Release ltlf.")
            }
        }
        recursive_subfn(&ctx, &x, &y, r#type, from_step, *step)
    }
}

#[test]
fn test_next_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let pred = Predicate::EQRL(x.clone(), b);

    let next_ltlf = NextZ3::new(&ctx, &pred, "guard", &4);

    assert_eq!("(= x_s5 b)", ast_to_string_z3!(&ctx, next_ltlf));
}

#[test]
fn test_always_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let pred = Predicate::EQRL(x.clone(), b);

    let glob_ltlf = AlwaysZ3::new(&ctx, &pred, "guard", &4);

    assert_eq!("(and (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b) (= x_s4 b))", ast_to_string_z3!(&ctx, glob_ltlf));
}

#[test]
fn test_eventually_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let pred = Predicate::EQRL(x.clone(), b);

    let at_least_once_ltlf = EventuallyZ3::new(&ctx, &pred, "guard", &4);

    assert_eq!("(or (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b) (= x_s4 b))", ast_to_string_z3!(&ctx, at_least_once_ltlf));
}

#[test]
fn test_until_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();
    let c = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let p1 = Predicate::EQRL(x.clone(), b);
    let p2 = Predicate::EQRL(y.clone(), c);

    let until_ltlf = UntilZ3::new(&ctx, &p1, &p2, "guard", &2);

    assert_eq!("(or (= y_s0 b) (and (= x_s0 b) (or (= y_s1 b) (and (= x_s1 b) (= y_s2 b)))))", ast_to_string_z3!(&ctx, until_ltlf));
}

#[test]
fn test_release_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();
    let c = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let p1 = Predicate::EQRL(x.clone(), b);
    let p2 = Predicate::EQRL(y.clone(), c);

    let release_ltlf = ReleaseZ3::new(&ctx, &p1, &p2, "guard", &2);

    assert_eq!("(and (= y_s0 b) (or (= x_s0 b) (and (= y_s1 b) (or (= x_s1 b) (= y_s2 b)))))", ast_to_string_z3!(&ctx, release_ltlf));
}