//! Z3 ltlf

use super::*;
use std::ffi::{CStr, CString};
use z3_sys::*;

pub struct NextZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast,
    pub y: Z3_ast
}

pub struct AfterZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast,
    pub y: Z3_ast
}

pub struct GloballyZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast
}

// pub struct UntilZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub x: Z3_ast,
//     pub y: Z3_ast
// }

// pub struct Until2Z3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub x: Z3_ast,
//     pub y: Z3_ast
// }

// pub struct AtLeastOnceZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub x: Z3_ast,
//     pub y: Z3_ast
// }

// chronological order
impl <'ctx> AfterZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Vec<Predicate>, y: &Vec<Predicate>, many: u32, step: u32) -> Z3_ast {
        let mut assert_vec = vec!();
        for m in 1..many + 1 {
            assert_vec.push(
                ANDZ3::new(&ctx, y.iter().map(|z| PredicateToAstZ3::new(&ctx, z, step + m)).collect())
            )
        }
        ANDZ3::new(&ctx, vec!(
            ANDZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, step)).collect()),
            ORZ3::new(&ctx, assert_vec)
            )
        )
    }
}

// chronological order
impl <'ctx> NextZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Vec<Predicate>, y: &Vec<Predicate>, step: u32) -> Z3_ast {
        let mut assert_vec = vec!();
        for s in 0..step + 1 {
            assert_vec.push(ANDZ3::new(&ctx, vec!(
                ANDZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, s)).collect()),
                ANDZ3::new(&ctx, y.iter().map(|z| PredicateToAstZ3::new(&ctx, z, s + 1)).collect())
            )))
        }
        ORZ3::new(&ctx, assert_vec)
    }
}

// impl <'ctx> AfterZ3<'ctx> {
//     pub fn new(ctx: &ContextZ3, x: &Vec<Predicate>, y: &Vec<Predicate>, step: u32) -> Z3_ast {
//         let mut assert_vec: Vec<Z3_ast> = vec!();
        
//         match step == 0 {
//             true => panic!("Can't have A after B in 0 steps"),
//             false => {
//                 for s in 0..step {
//                     let leader = ANDZ3::new(&ctx, y.iter().map(|z| PredicateToAstZ3::new(&ctx, z, s)).collect());
//                     let mut follower_vec: Vec<Z3_ast> = vec!();
//                     for f in s + 1..step + 1 {
//                         follower_vec.push(ANDZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, f)).collect()))
//                     }
//                     let follower = ORZ3::new(&ctx, follower_vec.clone());
//                     assert_vec.push(ANDZ3::new(&ctx, vec!(leader, follower)));
//                 }
//                 ORZ3::new(&ctx, assert_vec)
//             }
//         }
//     }
// }

impl <'ctx> GloballyZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Vec<Predicate>, step: u32) -> Z3_ast {
        let mut assert_vec = vec!();
        for s in 0..step + 1 {
            assert_vec.push(
                ANDZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, s)).collect())
            )
        }
        ANDZ3::new(&ctx, assert_vec)
    }
}


// Predicate::NEXT(x, y) => {
//     ANDZ3::new(&ctx, vec!(
//         ANDZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, step)).collect()),
//         ANDZ3::new(&ctx, y.iter().map(|z| PredicateToAstZ3::new(&ctx, z, step + 1)).collect())
//     ))
// }

// impl <'ctx> UntilZ3<'ctx> {
//     pub fn new(ctx: &ContextZ3, ts_model: &TransitionSystemModel,
//         x: &Predicate, y: &Vec<&Predicate>, mut from_step: u32, until_step: u32) -> Z3_ast {
//         let mut y_vec: Vec<Z3_ast> = vec!();
//         for sub_y in y {
//             y_vec.push(GetSPPredicateZ3::new(&ctx, ts_model, from_step, sub_y));
//         }        
//         // let y_vec = ANDZ3::new(&ctx, GetSPPredicateZ3::new(&ctx, ts_model, from_step - 1, y);
//         if from_step < until_step {
//             from_step = from_step + 1;
//             ORZ3::new(&ctx, vec!(
//                     ANDZ3::new(&ctx, y_vec),
//                     ANDZ3::new(&ctx, vec!(
//                             GetSPPredicateZ3::new(&ctx, ts_model, from_step - 1, x),
//                             UntilZ3::new(&ctx, ts_model, &x, &y, from_step, until_step)
//                         )
//                     )
//                 )
//             )
//         } else if from_step == until_step {
//             ANDZ3::new(&ctx, vec!(
//                 ANDZ3::new(&ctx, y_vec),
//                 // GetSPPredicateZ3::new(&ctx, ts_model, from_step, y_vec),
//                 GetSPPredicateZ3::new(&ctx, ts_model, from_step, x)
//                 )
//             )
//         } else {
//             panic! ("from_step > until_step")
//         }
//     }
// }

// impl <'ctx> Until2Z3<'ctx> {
//     pub fn new(ctx: &ContextZ3, ts_model: &TransitionSystemModel,
//         // x: Vec<&Predicate>, y: Vec<&Predicate>, from_step: u32, until_step: u32) -> Z3_ast {
//         x: &Predicate, y: &Predicate, from_step: u32, until_step: u32) -> Z3_ast {
//         let mut conj: Vec<Z3_ast> = vec!();
//         for s in from_step..=until_step {
//             let mut disj: Vec<Z3_ast> = vec!();
//             disj.push(GetSPPredicateZ3::new(&ctx, ts_model, s, x));
//             for i in 0..=s{
//                 disj.push(GetSPPredicateZ3::new(&ctx, ts_model, i, y));
//             }
//             conj.push(ORZ3::new(&ctx,disj));
//         }
//         ANDZ3::new(&ctx, conj)
//     }
// }


// // In a finite length trace, property has to hold at least in one step (a disjunction basically).
// // Property defined as a conjunction of predicates
// impl <'ctx> AtLeastOnceZ3<'ctx> {
//     pub fn new(ctx: &ContextZ3, ts_model: &TransitionSystemModel, 
//         x: Vec<&Predicate>, from_step: u32, until_step: u32) -> Z3_ast {
//         let mut disj: Vec<Z3_ast> = vec!();
//         for step in from_step..until_step + 1 {
//             let mut conj: Vec<Z3_ast> = vec!();
//             for pred in &x {
//                 conj.push(GetSPPredicateZ3::new(&ctx, ts_model, step, pred));
//             }
//             disj.push(ANDZ3::new(&ctx, conj));
//         }
//         ORZ3::new(&ctx, disj)
//     }
// }