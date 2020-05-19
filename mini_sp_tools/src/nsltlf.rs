use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct AfterZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast,
    pub y: Z3_ast
}

// pub struct SomewhenAfterZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub x: Z3_ast,
//     pub y: Z3_ast
// }

pub struct TracePBEQZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub x: Z3_ast
}

// chronological order
impl <'ctx> AfterZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, y: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
        ANDZ3::new(&ctx, vec!(
            PredicateToAstZ3::new(&ctx, x, r#type, &step),
            PredicateToAstZ3::new(&ctx, y, r#type, &(step + 1))))
    } 
}

// // chronological order
// impl <'ctx> SomewhenAfterZ3<'ctx> {
//     pub fn new(ctx: &ContextZ3, x: &Predicate, y: &Predicate, r#type: &str, step: &u32) -> Z3_ast {
//         let mut after = vec!();
//         for s in *step + 1..*max_step + 1{
//             after.push(PredicateToAstZ3::new(&ctx, y, r#type, &s))
//         }
//         ANDZ3::new(&ctx, 
//             vec!(
//                 PredicateToAstZ3::new(&ctx, x, r#type, &step),
//                 ORZ3::new(&ctx, after)
//             )
//         )
//     } 
// }

// Exactly n times true in a trace
impl <'ctx> TracePBEQZ3<'ctx> {
    pub fn new(ctx: &ContextZ3, x: &Predicate, r#type: &str, be_true: &u32, step: &u32) -> Z3_ast {
        let mut assert_vec = vec!();
        for s in 0..step + 1 {
            assert_vec.push(
                PredicateToAstZ3::new(&ctx, x, r#type, &s)
            )
        }
        PBEQZ3::new(&ctx, assert_vec, *be_true as i32)
    }
}

#[test]
fn test_after_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();
    let c = "c".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);

    let prev = Predicate::EQRL(x.clone(), b);
    let next = Predicate::EQRL(x.clone(), c);

    let next_ltlf = AfterZ3::new(&ctx, &prev, &next, "guard", &5);

    assert_eq!("(and (= x_s5 b) (= x_s6 c))", ast_to_string_z3!(&ctx, next_ltlf));
}

// #[test]
// fn test_safter_ltlf(){

//     let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
//     let b = "b".to_string();
//     let c = "c".to_string();

//     let cfg = ConfigZ3::new();
//     let ctx = ContextZ3::new(&cfg);

//     let prev = Predicate::EQRL(x.clone(), b);
//     let next = Predicate::EQRL(x.clone(), c);

//     let next_ltlf = SomewhenAfterZ3::new(&ctx, &prev, &next, "guard", &3, &6);

//     assert_eq!("(and (= x_s3 b) (or (= x_s4 c) (= x_s5 c) (= x_s6 c)))", ast_to_string_z3!(&ctx, next_ltlf));
// }

#[test]
fn test_trace_pbeq_ltlf(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"));
    let b = "b".to_string();

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let pred = Predicate::EQRL(x.clone(), b);

    let trace_pbeq_ltlf = TracePBEQZ3::new(&ctx, &pred, "guard", &2, &4);

    assert_eq!("((_ pbeq 2 1 1 1 1 1) (= x_s0 b) (= x_s1 b) (= x_s2 b) (= x_s3 b) (= x_s4 b))", ast_to_string_z3!(&ctx, trace_pbeq_ltlf));

    slv_assert_z3!(&ctx, &slv, trace_pbeq_ltlf);
    slv_check_z3!(&ctx, &slv);

    let model = slv_get_model_z3!(&ctx, &slv);
    assert_eq!("x_s2 -> a\nx_s0 -> b\nx_s3 -> a\nx_s1 -> b\nx_s4 -> a\n", model_to_string_z3!(&ctx, model));
}