// //! Z3 sorts for SP

// use std::ffi::{CStr, CString};
// use std::collections::HashMap;
// use std::time::{Duration, Instant};
// use z3_sys::*;
// use super::*;

// #[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
// pub struct Variable {
//     n: String,    
//     t: String,     
//     d: Vec<String>  
// }

// #[derive(PartialEq, Clone, Debug)]
// pub struct Transition {
//     n: String,
//     v: Vec<Variable>,
//     g: Predicate,
//     u: Predicate
// }

// #[derive(PartialEq, Clone, Debug)]
// pub struct ParamTransition {
//     n: String,
//     v: Vec<Variable>,
//     g: Vec<(String, Predicate)>,
//     u: Vec<(String, Predicate)>
// }

// #[derive(Debug)]
// pub struct PlanningProblem {
//     name: String,
//     vars: Vec<Variable>,
//     initial: Predicate,
//     goal: Predicate,
//     trans: Vec<Transition>,
//     ltl_specs: Predicate,
//     max_steps: u32
// }

// #[derive(Debug)]
// pub struct ParamPlanningProblem {
//     name: String,
//     vars: Vec<Variable>,
//     params: Vec<(String, bool)>,
//     initial: Vec<(String, Predicate)>,
//     goal: Vec<(String, Predicate)>,
//     trans: Vec<ParamTransition>,
//     ltl_specs: Predicate, // probably will be parameterized later as well
//     // level: u32,
//     // concat: u32,
//     max_steps: u32
// }

// #[derive(Debug)]
// pub struct ParamPlanningProblemNew {
//     name: String,
//     vars: Vec<(String, Variable)>,
//     params: Vec<(String, bool)>,
//     initial: Vec<(String, Predicate)>,
//     goal: Vec<(String, Predicate)>,
//     trans: Vec<ParamTransition>,
//     ltl_specs: Predicate, // probably will be parameterized later as well
//     // level: u32,
//     // concat: u32,
//     max_steps: u32
// }

// #[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
// pub struct PlanningFrame {
//     state: Vec<String>,
//     trans: String,
// }



// pub struct GetParamPlanningResultZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub model: Z3_model,
//     pub level: u32,
//     pub concat: u32,
//     pub nr_steps: u32,
//     pub frames: PlanningResult
// }


// pub struct GetPlanningResultZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub model: Z3_model,
//     pub nr_steps: u32,
//     pub frames: PlanningResult
// }

// pub struct PlanningResult {
//     pub plan_found: bool,
//     pub plan_length: u32,
//     pub trace: Vec<PlanningFrame>,
//     pub time_to_solve: std::time::Duration,
// }

// #[derive(PartialEq, Clone, Debug)]
// pub struct ParamPlanningResult {
//     pub plan_found: bool,
//     pub plan_length: u32,
//     pub level: u32,
//     pub concat: u32,
//     pub trace: Vec<PlanningFrame>,
//     pub time_to_solve: std::time::Duration,
// }

// #[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
// pub enum Predicate {
//     AND(Vec<Predicate>),
//     OR(Vec<Predicate>),
//     NOT(Vec<Predicate>),
//     EQVAL(Variable, String),
//     NEQVAL(Variable, String),
//     EQVAR(Variable, Variable),
//     NEQVAR(Variable, Variable),
//     NEXT(Vec<Predicate>, Vec<Predicate>),
//     AFTER(Vec<Predicate>, Vec<Predicate>, u32),
//     GLOB(Vec<Predicate>),
//     PBEQ(Vec<Predicate>, i32),
//     TRUE,
//     FALSE
// }

// pub struct KeepVariableValues<'ctx> {
//     pub ctx: &'ctx ContextZ3
// }

// pub struct PredicateToAstZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub pred: Predicate,
//     pub step: u32,
//     pub r: Z3_ast
// }

// pub struct UpdatePredicateToAstZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub pred: Predicate,
//     pub step: u32,
//     pub r: Z3_ast
// }

// pub struct AssignmentToAstZ3<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub init: Predicate,
//     pub step: u32,
//     pub r: Z3_ast
// }

// pub struct Sequential {}

// pub struct ParamSequential {}

// pub struct ParamSequentialNew {}

// pub struct Sequential2 {}

// pub struct GenerateProblems {}

// pub struct GenerateParamProblems {}

// pub struct GenerateAndSolveLevel {}

// pub struct RemoveLoops {}

// // pub struct GenerateProblem {}

// pub struct StateToPredicate {}

// pub struct ParamStateToPredicate {}

// pub struct ParamStateToPredicateNew {}

// pub struct LowLevelSequential<'ctx> {
// pub ctx: &'ctx ContextZ3
// }

// pub struct Compositional {}

// pub struct Compositional2 {}

// pub struct Compositional3 {}

// pub struct Concatenate {}

// pub struct ActivateNextParam {}

// pub struct Abstract<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub r: Z3_ast
// }

// pub struct AbstractHard<'ctx> {
//     pub ctx: &'ctx ContextZ3,
//     pub r: Z3_ast
// }

// pub struct Refine {}

// // pub trait IterOps<T, I>: IntoIterator<Item = T>
// //     where I: IntoIterator<Item = T>,
// //           T: PartialEq {
// //     fn intersect(self, other: I) -> Vec<T>;
// //     fn difference(self, other: I) -> Vec<T>;
// // }

// // impl<T, I> IterOps<T, I> for I
// //     where I: IntoIterator<Item = T>,
// //           T: PartialEq
// // {
// //     fn intersect(self, other: I) -> Vec<T> {
// //         let mut common = Vec::new();
// //         let mut v_other: Vec<_> = other.into_iter().collect();

// //         for e1 in self.into_iter() {
// //             if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
// //                 common.push(e1);
// //                 v_other.remove(pos);
// //             }
// //         }

// //         common
// //     }

// //     fn difference(self, other: I) -> Vec<T> {
// //         let mut diff = Vec::new();
// //         let mut v_other: Vec<_> = other.into_iter().collect();

// //         for e1 in self.into_iter() {
// //             if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
// //                 v_other.remove(pos);
// //             } else {
// //                 diff.push(e1);
// //             }
// //         }

// //         diff.append(&mut v_other);
// //         diff
// //     }
// // }

// impl Variable {
//     /// Creates a new Variable
//     pub fn new(n: &str, t: &str, d: Vec<&str>) -> Variable {
//         Variable { n: n.to_string(), 
//                    t: t.to_string(),
//                    d: d.iter().map(|x| x.to_string()).collect::<Vec<String>>()}
//     }
// }

// impl Transition {
//     pub fn new(n: &str, v: Vec<Variable>, g: &Predicate, u: &Predicate) -> Transition {
//         Transition { n: n.to_string(),
//                      v: v.clone(),
//                      g: g.clone(),
//                      u: u.clone() }
//     }
// }

// impl ParamTransition {
//     pub fn new(n: &str, v: Vec<Variable>, g: &Vec<(&str, Predicate)>, u: &Vec<(&str, Predicate)>) -> ParamTransition {
//         ParamTransition { n: n.to_string(),
//                      v: v.clone(),
//                      g: g.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
//                      u: u.iter().map(|x| (x.0.to_string(), x.1.clone())).collect()
//         }
//     }
// }

// impl PlanningProblem {
//     pub fn new(name: String,
//                vars: Vec<Variable>,
//                initial: Predicate,
//                goal: Predicate,
//                trans: Vec<Transition>,
//                ltl_specs: Predicate,
//                max_steps: u32) -> PlanningProblem {
//         PlanningProblem {
//             name: name.to_string(),
//             vars: vars,
//             initial: initial,
//             goal: goal,
//             trans: trans,
//             ltl_specs: ltl_specs,
//             max_steps: max_steps
//         }
//     }
// }

// impl ParamPlanningProblem {
//     pub fn new(name: String,
//                vars: Vec<Variable>,
//                params: Vec<(&str, bool)>,
//                initial: Vec<(&str, Predicate)>,
//                goal: Vec<(&str, Predicate)>,
//                trans: Vec<ParamTransition>,
//                ltl_specs: Predicate,
//             //    level: u32,
//             //    concat: u32,
//                max_steps: u32) -> ParamPlanningProblem {
//         ParamPlanningProblem {
//             name: name.to_string(),
//             vars: vars,
//             params: params.iter().map(|x| (x.0.to_string(), x.1)).collect(),
//             initial: initial.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
//             goal: goal.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
//             trans: trans,
//             ltl_specs: ltl_specs,
//             // level: level,
//             // concat: concat,
//             max_steps: max_steps
//         }
//     }
// }

// impl ParamPlanningProblemNew {
//     pub fn new(name: String,
//                vars: Vec<(&str, Variable)>,
//                params: Vec<(&str, bool)>,
//                initial: Vec<(&str, Predicate)>,
//                goal: Vec<(&str, Predicate)>,
//                trans: Vec<ParamTransition>,
//                ltl_specs: Predicate,
//             //    level: u32,
//             //    concat: u32,
//                max_steps: u32) -> ParamPlanningProblemNew {
//         ParamPlanningProblemNew {
//             name: name.to_string(),
//             vars: vars.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
//             params: params.iter().map(|x| (x.0.to_string(), x.1)).collect(),
//             initial: initial.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
//             goal: goal.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
//             trans: trans,
//             ltl_specs: ltl_specs,
//             // level: level,
//             // concat: concat,
//             max_steps: max_steps
//         }
//     }
// }

// impl PlanningFrame {
//     pub fn new(state: Vec<&str>, trans: &str) -> PlanningFrame {
//         PlanningFrame {
//             state: state.iter().map(|x| x.to_string()).collect(),
//             trans: trans.to_string()
//         }
//     }
// }

// impl <'ctx> PredicateToAstZ3<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, pred: &Predicate, step: u32) -> Z3_ast {
//         match pred {
//             Predicate::TRUE => BoolZ3::new(&ctx, true),
//             Predicate::FALSE => BoolZ3::new(&ctx, false),
//             // Predicate::NOT(p) => NOTZ3::new(&ctx, PredicateToAstZ3::new(&ctx, p, step)),
//             Predicate::NOT(p) => ANDZ3::new(&ctx, p.iter().map(|x| NOTZ3::new(&ctx, PredicateToAstZ3::new(&ctx, x, step))).collect()),
//             Predicate::AND(p) => ANDZ3::new(&ctx, p.iter().map(|x| PredicateToAstZ3::new(&ctx, x, step)).collect()),
//             Predicate::OR(p) => ORZ3::new(&ctx, p.iter().map(|x| PredicateToAstZ3::new(&ctx, x, step)).collect()),
//             Predicate::EQVAL(x, y) => {
//                 let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let elems = &sort.enum_asts;
//                 let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
//                 EQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
//             },
//             Predicate::EQVAR(x, y) => {
//                 // TODO: check sorts before passing to Z3
//                 let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
//                 let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
//                 let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step).as_str());
//                 EQZ3::new(&ctx, v_1, v_2)
//             },
//             Predicate::NEQVAL(x, y) => {
//                 let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let elems = &sort.enum_asts;
//                 let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
//                 NEQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
//             },
//             Predicate::NEQVAR(x, y) => {
//                 // TODO: check sorts before passing to Z3
//                 let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
//                 let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
//                 let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step).as_str());
//                 NEQZ3::new(&ctx, v_1, v_2)
//             },
//             Predicate::NEXT(x, y) => NextZ3::new(&ctx, &x, &y, step),
//             Predicate::AFTER(x, y, m) => AfterZ3::new(&ctx, &x, &y, *m, step),
//             Predicate::GLOB(x) => GloballyZ3::new(&ctx, &x, step),
//             Predicate::PBEQ(x, k) => PBEQZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, step)).collect(), *k)
//         }
//     }
// }

// impl <'ctx> UpdatePredicateToAstZ3<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, pred: &Predicate, step: u32) -> Z3_ast {
//         match pred {
//             Predicate::TRUE => BoolZ3::new(&ctx, true),
//             Predicate::FALSE => BoolZ3::new(&ctx, false),
//             Predicate::NOT(p) => ANDZ3::new(&ctx, p.iter().map(|x| NOTZ3::new(&ctx, UpdatePredicateToAstZ3::new(&ctx, x, step))).collect()),
//             Predicate::AND(p) => ANDZ3::new(&ctx, p.iter().map(|x| UpdatePredicateToAstZ3::new(&ctx, x, step)).collect()),
//             Predicate::OR(p) => ORZ3::new(&ctx, p.iter().map(|x| UpdatePredicateToAstZ3::new(&ctx, x, step)).collect()),
//             Predicate::EQVAL(x, y) => {
//                 let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let elems = &sort.enum_asts;
//                 let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
//                 EQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
//             },
//             Predicate::EQVAR(x, y) => {
//                 // TODO: check sorts before passing to Z3
//                 let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
//                 let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
//                 let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step - 1).as_str());
//                 EQZ3::new(&ctx, v_1, v_2)
//             },
//             Predicate::NEQVAL(x, y) => {
//                 let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let elems = &sort.enum_asts;
//                 let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
//                 NEQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
//             },
//             Predicate::NEQVAR(x, y) => {
//                 // TODO: check sorts before passing to Z3
//                 let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
//                 let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
//                 let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
//                 let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step - 1).as_str());
//                 NEQZ3::new(&ctx, v_1, v_2)
//             },
//             _ => panic!("implement")
//         }
//     }
// }

// impl <'ctx> KeepVariableValues<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, vars: &Vec<Variable>, trans: &Transition, step: u32) -> Z3_ast {

//         let unchanged = IterOps::difference(vars, &trans.v);
//         let mut assert_vec = vec!();
//         for u in unchanged {
//             let sort = EnumSortZ3::new(&ctx, &u.t, u.d.iter().map(|x| x.as_str()).collect());
//             let v_1 = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", u.n.to_string(), step).as_str());
//             let v_2 = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", u.n.to_string(), step - 1).as_str());
//             assert_vec.push(EQZ3::new(&ctx, v_1, v_2));
//         }
//         ANDZ3::new(&ctx, assert_vec)
//     }
// }

// impl Sequential {
//     pub fn new(p: &PlanningProblem, vars: &Vec<Variable>) -> PlanningResult {

//         let cfg = ConfigZ3::new();
//         let ctx = ContextZ3::new(&cfg);
//         let slv = SolverZ3::new(&ctx);
    
//         slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.initial, 0)));

//         SlvPushZ3::new(&ctx, &slv); // create backtracking point
//         slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.ltl_specs, 0)));
//         slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.goal, 0)));

//         let now = Instant::now();
//         let mut plan_found: bool = false;

//         let mut step: u32 = 0;

//         while step < p.max_steps + 1 {
//             step = step + 1;
//             if SlvCheckZ3::new(&ctx, &slv) != 1 {
//                 SlvPopZ3::new(&ctx, &slv, 1);

//                 let mut all_trans = vec!();
//                 for t in &p.trans {
//                     let name = format!("{}_t{}", &t.n, step);
//                     let guard = Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &t.g, step - 1));
//                     let updates = Abstract::new(&ctx, &vars, UpdatePredicateToAstZ3::new(&ctx, &t.u, step));
//                     let keeps = Abstract::new(&ctx, &vars, KeepVariableValues::new(&ctx, &p.vars, t, step));

//                     all_trans.push(ANDZ3::new(&ctx, 
//                         vec!(EQZ3::new(&ctx, 
//                             BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
//                             BoolZ3::new(&ctx, true)),
//                         guard, updates, keeps)));
//                 }

//                 slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
//                 SlvPushZ3::new(&ctx, &slv);
//                 slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step)));
//                 // slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
//                 slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.goal, step)));
        
//             } else {
//                 plan_found = true;
//                 break;
//             }
//         }

//         let planning_time = now.elapsed();

//         let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
//         let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
//         let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
//         // for a in cnf {
//         //     println!("{}", ast_to_string_z3!(&ctx, a))
//         // }
        
//         if plan_found == true {
//             let model = SlvGetModelZ3::new(&ctx, &slv);
//             let result = GetPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
//             result
//         } else {
//             let model = FreshModelZ3::new(&ctx);
//             let result = GetPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
//             result
//         }              
//     }   
// }

// // accepts param_planning_problem that accepts param transitions
// impl ParamSequential {
//     pub fn new(p: &ParamPlanningProblem, params: &Vec<(&str, bool)>, level: u32, concat: u32) -> ParamPlanningResult {

//         // println!("parameterized_trans: {:?}", p.trans[0]);

//         // resolve transitions based on parameters:
//         let mut resolved_trans = vec!();
//         for t in &p.trans {
//             let mut resolved_guard = vec!();
//             let mut resolved_update = vec!();
//             for param in params {
//                 for t_guard in &t.g {
//                     if t_guard.0 == param.0 && param.1 {
//                         resolved_guard.push(t_guard.1.clone())
//                     }
//                 };
//                 for t_update in &t.u {
//                     if t_update.0 == param.0 && param.1 {
//                         resolved_update.push(t_update.1.clone())
//                     }
//                 };
//             }

//             resolved_guard.sort();
//             resolved_guard.dedup();

//             resolved_update.sort();
//             resolved_update.dedup();

//             resolved_trans.push(
//                 Transition::new(
//                     t.n.as_str(), 
//                     t.v.clone(),
//                     &Predicate::AND(resolved_guard),
//                     &Predicate::AND(resolved_update)
//             )
//         )
//         }

//         // println!("resolved_trans: {:?}", resolved_trans[0]);

//         // resolve initial state:
//         let mut initial_pred_vec = vec!();
//         for pred in &p.initial {
//             for param in params {
//                 if pred.0 == param.0 && param.1 {
//                     initial_pred_vec.push(pred.1.clone())
//                 }
//             }
//         }

//         let initial = Predicate::AND(initial_pred_vec);

//         // resolve goal state:
//         let mut goal_pred_vec = vec!();
//         for pred in &p.goal {
//             for param in params {
//                 if pred.0 == param.0 && param.1 {
//                     goal_pred_vec.push(pred.1.clone())
//                 }
//             }
//         }

//         let goal = Predicate::AND(goal_pred_vec);

//         let cfg = ConfigZ3::new();
//         let ctx = ContextZ3::new(&cfg);
//         let slv = SolverZ3::new(&ctx);
    
//         slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &initial, 0));

//         SlvPushZ3::new(&ctx, &slv); // create backtracking point
//         slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, 0));
//         slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &goal, 0));

//         let now = Instant::now();
//         let mut plan_found: bool = false;

//         let mut step: u32 = 0;

//         while step < p.max_steps + 1 {
//             step = step + 1;
//             if SlvCheckZ3::new(&ctx, &slv) != 1 {
//                 SlvPopZ3::new(&ctx, &slv, 1);

//                 let mut all_trans = vec!();
//                 for t in &resolved_trans {
//                     let name = format!("{}_t{}", &t.n, step);
//                     let guard = PredicateToAstZ3::new(&ctx, &t.g, step - 1);
//                     let updates = UpdatePredicateToAstZ3::new(&ctx, &t.u, step);
//                     let keeps = KeepVariableValues::new(&ctx, &p.vars, t, step);

//                     all_trans.push(ANDZ3::new(&ctx, 
//                         vec!(EQZ3::new(&ctx, 
//                             BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
//                             BoolZ3::new(&ctx, true)),
//                         guard, updates, keeps)));
//                 }

//                 slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
//                 SlvPushZ3::new(&ctx, &slv);
//                 slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
//                 // slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
//                 slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &goal, step));
        
//             } else {
//                 plan_found = true;
//                 break;
//             }
//         }

//         let planning_time = now.elapsed();

//         let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
//         let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
//         let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
//         // for a in cnf {
//         //     println!("{}", ast_to_string_z3!(&ctx, a))
//         // }
        
//         if plan_found == true {
//             let model = SlvGetModelZ3::new(&ctx, &slv);
//             let result = GetParamPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found, level, concat);
//             result
//         } else {
//             let model = FreshModelZ3::new(&ctx);
//             let result = GetParamPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found, level, concat);
//             result
//         }              
//     }   
// }



// impl ParamSequentialNew {
//     pub fn new(p: &ParamPlanningProblemNew, params: &Vec<(&str, bool)>, level: u32, concat: u32) -> ParamPlanningResult {

//         // println!("parameterized_trans: {:?}", p.trans[0]);

//         //resolve variables based on parameters:
//         let mut res_vars = vec!();
//         for var in &p.vars {
//             for param in params {
//                 if var.0 == param.0 && param.1 {
//                     res_vars.push(var.1.clone())
//                 }
//             }
//         }


//         // resolve transitions based on parameters:
//         let mut resolved_trans = vec!();
//         for t in &p.trans {
//             let mut resolved_guard = vec!();
//             let mut resolved_update = vec!();
//             for param in params {
//                 for t_guard in &t.g {
//                     if t_guard.0 == param.0 && param.1 {
//                         resolved_guard.push(t_guard.1.clone())
//                     }
//                 };
//                 for t_update in &t.u {
//                     if t_update.0 == param.0 && param.1 {
//                         resolved_update.push(t_update.1.clone())
//                     }
//                 };
//             }

//             resolved_guard.sort();
//             resolved_guard.dedup();

//             resolved_update.sort();
//             resolved_update.dedup();

//             resolved_trans.push(
//                 Transition::new(
//                     t.n.as_str(), 
//                     t.v.clone(),
//                     &Predicate::AND(resolved_guard),
//                     &Predicate::AND(resolved_update)
//             )
//         )
//         }

//         // println!("resolved_trans: {:?}", resolved_trans[0]);

//         // resolve initial state:
//         let mut initial_pred_vec = vec!();
//         for pred in &p.initial {
//             for param in params {
//                 if pred.0 == param.0 && param.1 {
//                     initial_pred_vec.push(pred.1.clone())
//                 }
//             }
//         }

//         let initial = Predicate::AND(initial_pred_vec);

//         // resolve goal state:
//         let mut goal_pred_vec = vec!();
//         for pred in &p.goal {
//             for param in params {
//                 if pred.0 == param.0 && param.1 {
//                     goal_pred_vec.push(pred.1.clone())
//                 }
//             }
//         }

//         let goal = Predicate::AND(goal_pred_vec);

//         let cfg = ConfigZ3::new();
//         let ctx = ContextZ3::new(&cfg);
//         let slv = SolverZ3::new(&ctx);
    
//         slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &initial, 0));

//         SlvPushZ3::new(&ctx, &slv); // create backtracking point
//         slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, 0));
//         slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &goal, 0));

//         let now = Instant::now();
//         let mut plan_found: bool = false;

//         let mut step: u32 = 0;

//         while step < p.max_steps + 1 {
//             step = step + 1;
//             if SlvCheckZ3::new(&ctx, &slv) != 1 {
//                 SlvPopZ3::new(&ctx, &slv, 1);

//                 let mut all_trans = vec!();
//                 for t in &resolved_trans {
//                     let name = format!("{}_t{}", &t.n, step);
//                     let guard = PredicateToAstZ3::new(&ctx, &t.g, step - 1);
//                     let updates = UpdatePredicateToAstZ3::new(&ctx, &t.u, step);
//                     let keeps = KeepVariableValues::new(&ctx, &res_vars, t, step);

//                     all_trans.push(ANDZ3::new(&ctx, 
//                         vec!(EQZ3::new(&ctx, 
//                             BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
//                             BoolZ3::new(&ctx, true)),
//                         guard, updates, keeps)));
//                 }

//                 slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
//                 SlvPushZ3::new(&ctx, &slv);
//                 slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
//                 // slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
//                 slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &goal, step));
        
//             } else {
//                 plan_found = true;
//                 break;
//             }
//         }

//         let planning_time = now.elapsed();

//         let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
//         let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
//         let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
//         // for a in cnf {
//         //     println!("{}", ast_to_string_z3!(&ctx, a))
//         // }
        
//         if plan_found == true {
//             let model = SlvGetModelZ3::new(&ctx, &slv);
//             let result = GetParamPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found, level, concat);
//             result
//         } else {
//             let model = FreshModelZ3::new(&ctx);
//             let result = GetParamPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found, level, concat);
//             result
//         }              
//     }   
// }




// impl ActivateNextParam {
//     pub fn new(params: &Vec<(String, bool)>, order: &Vec<&str>) -> Vec<(String, bool)> {
//         let mut index_to_update = 0;
//         match params.iter().all(|x| x.1) {
//             true => panic!("ActivateNextParam: all parameters activated!"),
//             false => {
//                 for ord in order {
//                     for p in params {
//                         if &p.0 == ord && !p.1 {
//                             match params.iter().position(|x| x == p) {
//                                 Some(x) => index_to_update = x,
//                                 None => panic!("ActivateNextParam: element fail.")
//                             }
//                             break;
//                         }
//                     }
//                 };
//                 let mut pclone = params.clone();
//                 let mut to_update = pclone.remove(index_to_update);
//                 to_update.1 = true;
//                 pclone.push((to_update.0, to_update.1));
//                 pclone.iter().map(|x| (x.0.to_string(), x.1)).collect()
//             }
//         }
//     }
// }

// impl <'ctx> Abstract<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, keep_v: &Vec<Variable>, p: Z3_ast) -> Z3_ast {

//         if keep_v.len() != 0 {
//             let cnf = GetCnfVectorZ3::new(&ctx, vec!(p));
//             let mut filtered: Vec<Z3_ast> = vec!();

//             for a in cnf {
//                 if keep_v.iter().any(|x| ast_to_string_z3!(&ctx, a).contains(&x.n)) {
//                     filtered.push(a)
//                 }
//             }

//             // for a in cnf {
//             //     for var in keep_v {
//             //         if ast_to_string_z3!(&ctx, a).contains(&var.n){
//             //             filtered.push(a)
//             //         }
//             //     }
//             // }
//             filtered.sort();
//             filtered.dedup();
//             ANDZ3::new(&ctx, filtered)
//         } else {
//             p
//         }
//     }
// }

// impl StateToPredicate {
//     pub fn new(state: &Vec<&str>, p: &PlanningProblem) -> Predicate {
//         let mut and_vec: Vec<Predicate> = vec!();
//         for s in state {
//             let sep: Vec<&str> = s.split(" -> ").collect();
//             let mut d: Vec<&str> = vec!();
//             let mut t: &str = "";
//             let mut n: &str = "";
//             for v in &p.vars {
//                 if v.n == sep[0] {
//                     n = sep[0];
//                     d = v.d.iter().map(|x| x.as_str()).collect();
//                     t = &v.t;
//                 }
//             }

//             let var = Variable::new(n, t, d);
//             let val = sep[1];

//             and_vec.push(Predicate::EQVAL(var, String::from(val)));
//         }
//         Predicate::AND(and_vec)
//     }
// }

// impl ParamStateToPredicate {
//     pub fn new(state: &Vec<&str>, p: &ParamPlanningProblem) -> Vec<(String, Predicate)> {
//         let mut pred_vec: Vec<(String, Predicate)> = vec!();
//         for s in state {
//             // let pred: (&str, Predicate)
//             let sep: Vec<&str> = s.split(" -> ").collect();
//             let mut d: Vec<&str> = vec!();
//             let mut t: &str = "";
//             let mut n: &str = "";
//             for v in &p.vars {
//                 if v.n == sep[0] {
//                     n = sep[0];
//                     d = v.d.iter().map(|x| x.as_str()).collect();
//                     t = &v.t;
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

// impl Concatenate {
//     pub fn new(results: &Vec<ParamPlanningResult>) -> ParamPlanningResult {
//         let conc_plan_found = match results.iter().all(|x| x.plan_found) {
//             true => true,
//             false => false
//         };
//         let conc_plan_lenght = results.iter().map(|x| x.plan_length).sum();

//         let conc_plan_level = match results.len() != 0 {
//             true => results[0].level,
//             false => 666
//         };
        
//         let conc_plan_concat:u32 = 123456789;
//         let mut conc_plan_trace: Vec<PlanningFrame> = vec!();

//         for res in results {
//             if results.iter().position(|x| x == res).unwrap() == 0 {
//                 for tr in res.trace.clone() {
//                     conc_plan_trace.push(tr)
//                 }
//             } else {
//                 for tr in res.trace.clone() {
//                     if res.trace.iter().position(|x| *x == tr).unwrap() != 0 {
                        
//                         conc_plan_trace.push(tr)
//                     }
//                 }
//             }
//         };
//         let conc_plan_duration = results.iter().map(|x| x.time_to_solve).sum();

//         ParamPlanningResult {
//             plan_found: conc_plan_found,
//             plan_length: conc_plan_lenght,
//             level: conc_plan_level,
//             concat: conc_plan_concat,
//             trace: conc_plan_trace,
//             time_to_solve: conc_plan_duration
//         }
//     }
// }

// // add the duration of this process as well
// impl RemoveLoops {
//     pub fn new(result: &ParamPlanningResult) -> ParamPlanningResult { 
//         let mut duplicates: Vec<(Vec<String>, usize, usize)> = vec!();
//         // let mut sorted_result = result.clone();
//         let mut sorted_trace: Vec<PlanningFrame> = vec!();
//         let mut has_duplicates: Vec<(PlanningFrame, usize, usize)> = vec!();

//         for mut r in &result.trace {
//             let mut sorted_state = r.state.clone();
//             sorted_state.sort();
//             let frame: PlanningFrame = PlanningFrame::new(sorted_state.iter().map(|x| x.as_str()).collect(), &r.trans);
//             sorted_trace.push(frame);
//         }

//         for tr1 in &sorted_trace {
//             for tr2 in &sorted_trace {
//                 if tr1.state == tr2.state {
//                     let start = match sorted_trace.iter().position(|x| x.state == tr1.state) {
//                         Some(y) => y as usize,
//                         None => 666
//                     };
//                     let finish = match sorted_trace.iter().rposition(|x| x.state == tr1.state) {
//                         Some(y) => y as usize,
//                         None => 666
//                     };
//                     if start != finish {
//                         if !has_duplicates.iter().any(|x| x.0.state == tr1.state) {
//                             has_duplicates.push((tr1.clone(), start, finish))
//                         }
                        
//                     }
//                 }
//             }
//         }

//         has_duplicates.sort();
//         has_duplicates.dedup();

//         let mut fixed: Vec<PlanningFrame> = vec!();

        

//         while has_duplicates.len() != 0 {
//             println!("DUPLICATES: {:?}\n", has_duplicates);
//             // println!("SORTED and FIXED: {:?}\n", sorted_trace);
//             // println!("====================================");
//             if has_duplicates[0].1 != 123456789 {
//                 fixed = sorted_trace.drain(has_duplicates[0].1 + 1..has_duplicates[0].2 + 1).collect();
//                 has_duplicates.remove(0);
//                 if has_duplicates.len() != 0 {
//                     has_duplicates[0].1 = match sorted_trace.iter().position(|x| x.state == has_duplicates[0].0.state) {
//                         Some(y) => y as usize,
//                         None => 123456789
//                     };
//                     has_duplicates[0].2 = match sorted_trace.iter().rposition(|x| x.state == has_duplicates[0].0.state) {
//                         Some(y) => y as usize,
//                         None => 123456789
//                     };
//                 }
//             } else {
//                 has_duplicates.remove(0);
//                 if has_duplicates.len() != 0 {
//                     has_duplicates[0].1 = match sorted_trace.iter().position(|x| x.state == has_duplicates[0].0.state) {
//                         Some(y) => y as usize,
//                         None => 123456789
//                     };
//                     has_duplicates[0].2 = match sorted_trace.iter().rposition(|x| x.state == has_duplicates[0].0.state) {
//                         Some(y) => y as usize,
//                         None => 123456789
//                     };
//                 }
//             }
//         }

//         ParamPlanningResult {
//             plan_found: result.plan_found,
//             plan_length: sorted_trace.len() as u32 - 1,
//             level: result.level,
//             concat: result.concat,
//             trace: sorted_trace,
//             time_to_solve: result.time_to_solve
//         }        
//     }
// }

// impl Compositional2 {
//     pub fn new(result: &ParamPlanningResult,
//                problem: &ParamPlanningProblem,
//                params: &Vec<(String, bool)>, 
//                order: &Vec<&str>, 
//                all_results: &Vec<ParamPlanningResult>,
//                level: u32) -> ParamPlanningResult {
    
//         let all_results: Vec<ParamPlanningResult> = vec!();
//         let mut final_result: ParamPlanningResult = result.clone();

//         let current_level = level + 1;
//         if !params.iter().all(|x| x.1) {
            
//             if result.plan_found {
//                 let mut inheritance: Vec<String> = vec!() ;
//                 let mut level_results = vec!();
//                 let activated_params = &ActivateNextParam::new(&params, &order);
//                 let mut concat = 0;
//                 if result.plan_length != 0 {
//                     for i in 0..=result.trace.len() - 1 {
//                         if i == 0 {
//                             // println!("DDDDDDDDDDDDDDDDDDDDD i == 0");
//                             let new_problem = ParamPlanningProblem::new(
//                                 format!("problem_l{:?}_c{:?}", current_level, concat), 
//                                 problem.vars.clone(),
//                                 activated_params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 ParamStateToPredicate::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.trans.clone(),
//                                 problem.ltl_specs.clone(),
//                                 problem.max_steps);
//                             let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
                            
//                             if new_result.plan_found {
//                                 level_results.push(new_result.clone());
//                                 match new_result.trace.last() {
//                                     Some(x) => inheritance = x.state.clone(),
//                                     None => panic!("No tail in the plan! 1")
//                                 }
//                             } else {
//                                 panic!("NO PLAN FOUND 1 !")
//                             }

//                             concat = concat + 1;                         
//                         } else if i == result.trace.len() - 1 {
//                             // println!("DDDDDDDDDDDDDDDDDDDDD i == result.trace.len() - 1");
//                             let new_problem = ParamPlanningProblem::new(
//                                 format!("problem_l{:?}_c{:?}", current_level, concat), 
//                                 problem.vars.clone(),
//                                 activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                                 ParamStateToPredicate::new(&inheritance.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.trans.clone(),
//                                 problem.ltl_specs.clone(),
//                                 problem.max_steps);
//                             let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
                            
//                             if new_result.plan_found {
//                                 level_results.push(new_result.clone());
//                             } else {
//                                 panic!("NO PLAN FOUND 2 !")
//                             }
//                             concat = concat + 1;
//                         } else {
//                             // println!("DDDDDDDDDDDDDDDDDDDDD i == else");
//                             let new_problem = ParamPlanningProblem::new(
//                                 format!("problem_l{:?}_c{:?}", current_level, concat), 
//                                 problem.vars.clone(),
//                                 activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                                 ParamStateToPredicate::new(&inheritance.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 ParamStateToPredicate::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.trans.clone(),
//                                 problem.ltl_specs.clone(),
//                                 problem.max_steps);
//                             let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);

//                             if new_result.plan_found {
//                                 level_results.push(new_result.clone());
//                                 match new_result.trace.last() {
//                                     Some(x) => inheritance = x.state.clone(),
//                                     None => panic!("No tail in the plan! 2")
//                                 }
//                             } else {
//                                 panic!("NO PLAN FOUND 3 ! {}")
//                             }
//                             concat = concat + 1;   
//                         }
//                     }
//                 } else {

//                     // have to handle this case somehow this is one of the bottlenecks
//                     // println!("DDDDDDDDDDDDDDDDDDDDD lenght == 0");
//                     let activated_params = &ActivateNextParam::new(&params, &order);
//                     let new_problem = ParamPlanningProblem::new(
//                         String::from("some"), 
//                         problem.vars.clone(),
//                         activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                         problem.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         problem.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         // ParamStateToPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         // ParamStateToPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         problem.trans.clone(),
//                         problem.ltl_specs.clone(),
//                         problem.max_steps);
//                     let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
//                     if new_result.plan_found {
//                         level_results.push(new_result.clone());
//                         match new_result.trace.last() {
//                             Some(x) => inheritance = x.state.clone(),
//                             None => panic!("No tail in the plan! 3")
//                         }
//                     } else {
//                         panic!("NO PLAN FOUND 4 !")
//                     }
//                         concat = concat + 1;   
//                 }

//                 final_result = Concatenate::new(&level_results);
//                 let delooped_and_sorted = RemoveLoops::new(&final_result);
                

//                 println!("inlevel: {:?}", delooped_and_sorted.level);
//                 println!("inconcat: {:?}", delooped_and_sorted.concat);
//                 println!("inplan_found: {:?}", delooped_and_sorted.plan_found);
//                 println!("inplan_lenght: {:?}", delooped_and_sorted.plan_length);
//                 println!("intime_to_solve: {:?}", delooped_and_sorted.time_to_solve);
//                 println!("intrace: ");
            
//                 for t in &delooped_and_sorted.trace{
                
//                     println!("state: {:?}", t.state);
//                     println!("trans: {:?}", t.trans);
//                     println!("=========================");
//                 }
            
                

//                 final_result = Compositional2::new(&delooped_and_sorted, &problem, &activated_params, &order, &all_results, current_level);   
//             }
//         }
//         final_result
//     }
// }



// impl Compositional3 {
//     pub fn new(result: &ParamPlanningResult,
//                problem: &ParamPlanningProblemNew,
//                params: &Vec<(String, bool)>, 
//                order: &Vec<&str>, 
//                all_results: &Vec<ParamPlanningResult>,
//                level: u32) -> ParamPlanningResult {
    
//         let all_results: Vec<ParamPlanningResult> = vec!();
//         let mut final_result: ParamPlanningResult = result.clone();

//         let current_level = level + 1;
//         if !params.iter().all(|x| x.1) {
            
//             if result.plan_found {
//                 let mut inheritance: Vec<String> = vec!() ;
//                 let mut level_results = vec!();
//                 let activated_params = &ActivateNextParam::new(&params, &order);
//                 let mut concat = 0;
//                 if result.plan_length != 0 {
//                     for i in 0..=result.trace.len() - 1 {
//                         if i == 0 {
//                             // println!("DDDDDDDDDDDDDDDDDDDDD i == 0");
//                             let new_problem = ParamPlanningProblemNew::new(
//                                 format!("problem_l{:?}_c{:?}", current_level, concat), 
//                                 problem.vars.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 activated_params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 ParamStateToPredicateNew::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.trans.clone(),
//                                 problem.ltl_specs.clone(),
//                                 problem.max_steps);
//                             let new_result = ParamSequentialNew::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
                            
//                             if new_result.plan_found {
//                                 level_results.push(new_result.clone());
//                                 match new_result.trace.last() {
//                                     Some(x) => inheritance = x.state.clone(),
//                                     None => panic!("No tail in the plan! 1")
//                                 }
//                             } else {
//                                 panic!("NO PLAN FOUND 1 !")
//                             }

//                             concat = concat + 1;                         
//                         } else if i == result.trace.len() - 1 {
//                             // println!("DDDDDDDDDDDDDDDDDDDDD i == result.trace.len() - 1");
//                             let new_problem = ParamPlanningProblemNew::new(
//                                 format!("problem_l{:?}_c{:?}", current_level, concat), 
//                                 problem.vars.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                                 ParamStateToPredicateNew::new(&inheritance.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.trans.clone(),
//                                 problem.ltl_specs.clone(),
//                                 problem.max_steps);
//                             let new_result = ParamSequentialNew::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
                            
//                             if new_result.plan_found {
//                                 level_results.push(new_result.clone());
//                             } else {
//                                 panic!("NO PLAN FOUND 2 !")
//                             }
//                             concat = concat + 1;
//                         } else {
//                             // println!("DDDDDDDDDDDDDDDDDDDDD i == else");
//                             let new_problem = ParamPlanningProblemNew::new(
//                                 format!("problem_l{:?}_c{:?}", current_level, concat), 
//                                 problem.vars.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                                 ParamStateToPredicateNew::new(&inheritance.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 ParamStateToPredicateNew::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 problem.trans.clone(),
//                                 problem.ltl_specs.clone(),
//                                 problem.max_steps);
//                             let new_result = ParamSequentialNew::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);

//                             if new_result.plan_found {
//                                 level_results.push(new_result.clone());

//                                 println!("problem_nlevel: {:?}", new_result.level);
//                                 println!("problem_nconcat: {:?}", new_result.concat);
//                                 println!("problem_nplan_found: {:?}", new_result.plan_found);
//                                 println!("problem_nplan_lenght: {:?}", new_result.plan_length);
//                                 println!("problem_ntime_to_solve: {:?}", new_result.time_to_solve);
//                                 println!("problem_ntrace: ");

//                                 for t in &new_result.trace{
                                
//                                     println!("state: {:?}", t.state);
//                                     println!("trans: {:?}", t.trans);
//                                     println!("=========================");
//                                 }

//                                 match new_result.trace.last() {
//                                     Some(x) => inheritance = x.state.clone(),
//                                     None => panic!("No tail in the plan! 2")
//                                 }
//                             } else {
//                                 panic!("NO PLAN FOUND 3 !")
//                             }
//                             concat = concat + 1;   
//                         }
//                     }
//                 } else {

//                     // have to handle this case somehow this is one of the bottlenecks
//                     // println!("DDDDDDDDDDDDDDDDDDDDD lenght == 0");
//                     let activated_params = &ActivateNextParam::new(&params, &order);
//                     let new_problem = ParamPlanningProblemNew::new(
//                         String::from("some"), 
//                         problem.vars.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                         problem.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         problem.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         // ParamStateToPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         // ParamStateToPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                         problem.trans.clone(),
//                         problem.ltl_specs.clone(),
//                         problem.max_steps);
//                     let new_result = ParamSequentialNew::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
//                     if new_result.plan_found {
//                         level_results.push(new_result.clone());
//                         match new_result.trace.last() {
//                             Some(x) => inheritance = x.state.clone(),
//                             None => panic!("No tail in the plan! 3")
//                         }
//                     } else {
//                         panic!("NO PLAN FOUND 4 !")
//                     }
//                         concat = concat + 1;   
//                 }

//                 final_result = Concatenate::new(&level_results);
//                 let delooped_and_sorted = RemoveLoops::new(&final_result);
                

//                 println!("inlevel: {:?}", delooped_and_sorted.level);
//                 println!("inconcat: {:?}", delooped_and_sorted.concat);
//                 println!("inplan_found: {:?}", delooped_and_sorted.plan_found);
//                 println!("inplan_lenght: {:?}", delooped_and_sorted.plan_length);
//                 println!("intime_to_solve: {:?}", delooped_and_sorted.time_to_solve);
//                 println!("intrace: ");
            
//                 for t in &delooped_and_sorted.trace{
                
//                     println!("state: {:?}", t.state);
//                     println!("trans: {:?}", t.trans);
//                     println!("=========================");
//                 }
            
                

//                 final_result = Compositional3::new(&delooped_and_sorted, &problem, &activated_params, &order, &all_results, current_level);   
//             }
//         }
//         final_result
//     }
// }




// impl <'ctx> GetPlanningResultZ3<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, model: Z3_model, nr_steps: u32, 
//     planning_time: std::time::Duration, plan_found: bool) -> PlanningResult {
//         let model_str = ModelToStringZ3::new(&ctx, model);
//         let mut model_vec = vec!();

//         let num = ModelGetNumConstsZ3::new(&ctx, model);
//         let mut lines = model_str.lines();
//         let mut i: u32 = 0;

//         while i < num {
//             model_vec.push(lines.next().unwrap_or(""));
//             i = i + 1;
//         }

//         // println!("{:#?}", model_vec);

//         let mut trace: Vec<PlanningFrame> = vec!();
        
//         for i in 0..nr_steps {
//             let mut frame: PlanningFrame = PlanningFrame::new(vec!(), "");
//             for j in &model_vec {
//                 let sep: Vec<&str> = j.split(" -> ").collect();
//                 if sep[0].ends_with(&format!("_s{}", i)){
//                     let trimmed_state = sep[0].trim_end_matches(&format!("_s{}", i));
//                     match sep[1] {
//                         "false" => frame.state.push(sep[0].to_string()),
//                         "true" => frame.state.push(sep[0].to_string()),
//                         _ => frame.state.push(format!("{} -> {}", trimmed_state, sep[1])),
//                     }
//                 } else if sep[0].ends_with(&format!("_t{}", i)) && sep[1] == "true" {
//                     let trimmed_trans = sep[0].trim_end_matches(&format!("_t{}", i));
//                     frame.trans = trimmed_trans.to_string();
//                 }
//             }
//             if model_vec.len() != 0 {
//                 trace.push(frame);
//             }
//         }

//         PlanningResult {
//             plan_found: plan_found,
//             plan_length: nr_steps - 1,
//             trace: trace,
//             time_to_solve: planning_time,
//         }
//     }
// }

// impl <'ctx> GetParamPlanningResultZ3<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, model: Z3_model, nr_steps: u32, 
//     planning_time: std::time::Duration, plan_found: bool, level: u32, concat: u32) -> ParamPlanningResult {
//         let model_str = ModelToStringZ3::new(&ctx, model);
//         let mut model_vec = vec!();

//         let num = ModelGetNumConstsZ3::new(&ctx, model);
//         let mut lines = model_str.lines();
//         let mut i: u32 = 0;

//         while i < num {
//             model_vec.push(lines.next().unwrap_or(""));
//             i = i + 1;
//         }

//         // println!("{:#?}", model_vec);

//         let mut trace: Vec<PlanningFrame> = vec!();
        
//         for i in 0..nr_steps {
//             let mut frame: PlanningFrame = PlanningFrame::new(vec!(), "");
//             for j in &model_vec {
//                 let sep: Vec<&str> = j.split(" -> ").collect();
//                 if sep[0].ends_with(&format!("_s{}", i)){
//                     let trimmed_state = sep[0].trim_end_matches(&format!("_s{}", i));
//                     match sep[1] {
//                         "false" => frame.state.push(sep[0].to_string()),
//                         "true" => frame.state.push(sep[0].to_string()),
//                         _ => frame.state.push(format!("{} -> {}", trimmed_state, sep[1])),
//                     }
//                 } else if sep[0].ends_with(&format!("_t{}", i)) && sep[1] == "true" {
//                     let trimmed_trans = sep[0].trim_end_matches(&format!("_t{}", i));
//                     frame.trans = trimmed_trans.to_string();
//                 }
//             }
//             if model_vec.len() != 0 {
//                 trace.push(frame);
//             }
//         }

//         ParamPlanningResult {
//             plan_found: plan_found,
//             plan_length: nr_steps - 1,
//             level: level,
//             concat:concat,
//             trace: trace,
//             time_to_solve: planning_time,
//         }
//     }
// }

// #[test]
// fn test_idea_1_iteration_5(){
    
//     let pose_domain = vec!("buffer", "home", "table");
//     let stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group cube, have to think of something better
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));
    
//     // act gripper predicates
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         act_pos.clone(), 
//         ref_pos.clone(), 
//         act_stat.clone(), 
//         ref_stat.clone(),
//         buffer.clone(), 
//         gripper.clone(), 
//         table.clone()
//     );

//     let t1 = ParamTransition::new(
//         "start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     // 1. have to go through the "home" pose:
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 2. have to go through the "home" pose:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 3. one cube in the system:
//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );

//     let now = Instant::now();

//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     let seq_planning_time = now.elapsed();

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

//     // 

//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), true), ("cube".to_string(), false));
//     let refining_order: Vec<&str> = vec!("pos", "cube", "stat"); // opposite for some reason? fix this
//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

//     let now = Instant::now();
        
//     let solution = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

//     // RemoveLoops::new(&solution);
//     let sorted_result = RemoveLoops::new(&solution);

//     let comp_planning_time = now.elapsed();

//     println!("comp_level: {:?}", solution.level);
//     println!("comp_concat: {:?}", solution.concat);
//     println!("comp_plan_found: {:?}", solution.plan_found);
//     println!("comp_plan_lenght: {:?}", solution.plan_length);
//     println!("comp_time_to_solve: {:?}", solution.time_to_solve);
//     println!("comp_trace: ");

//     for t in &solution.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

    

//     println!("sorted_level: {:?}", sorted_result.level);
//     println!("sorted_concat: {:?}", sorted_result.concat);
//     println!("sorted_plan_found: {:?}", sorted_result.plan_found);
//     println!("sorted_plan_lenght: {:?}", sorted_result.plan_length);
//     println!("sorted_time_to_solve: {:?}", sorted_result.time_to_solve);
//     println!("sorted_trace: ");

//     for t in &sorted_result.trace{
    
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

//     println!("TOTAL SEQUENTIAL TIME: {:?}", seq_planning_time);
//     println!("TOTAL COMPOSITIONAL TIME: {:?}", comp_planning_time);
// }

// #[test]
// fn test_idea_1_iteration_6(){

//     // a global constraint should be made that changea in the system are one at a time
//     // so for instance, if you're activating a robot and a gripper, you have to finish activating
//     // one first and then actvate another. This maybe makes sense in general
    
//     let pose_domain = vec!("buffer", "home", "table");
//     let gripper_pose_domain = vec!("cube", "ball", "closed", "open");
//     let stat_domain = vec!("active", "idle");
//     let gripper_stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group gripper pos
//     let act_grip_pos = Variable::new("act_grip_pos", "grip_pose", gripper_pose_domain.clone());
//     let ref_grip_pos = Variable::new("ref_grip_pos", "grip_pose", gripper_pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group grip stat
//     let act_grip_stat = Variable::new("act_grip_stat", "grip_status", gripper_stat_domain.clone());
//     let ref_grip_stat = Variable::new("ref_grip_stat", "grip_status", gripper_stat_domain.clone());

//     // var group cube, have to think of something better
//     // well ok we have sensors in the gripper beside opening for some reason
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // act grip stat predicates
//     let grip_stat_active = Predicate::EQVAL(act_grip_stat.clone(), String::from("active"));
//     let grip_stat_idle = Predicate::EQVAL(act_grip_stat.clone(), String::from("idle"));
//     let not_grip_stat_active = Predicate::NOT(vec!(grip_stat_active.clone()));
//     let not_grip_stat_idle = Predicate::NOT(vec!(grip_stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // ref grip stat predicates
//     let set_grip_stat_active = Predicate::EQVAL(ref_grip_stat.clone(), String::from("active"));
//     let set_grip_stat_idle = Predicate::EQVAL(ref_grip_stat.clone(), String::from("idle"));
//     let not_set_grip_stat_active = Predicate::NOT(vec!(set_grip_stat_active.clone()));
//     let not_set_grip_stat_idle = Predicate::NOT(vec!(set_grip_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));

//     // act gripper predicates, just for keeping track of the cube
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act grip pos predicates
//     let grip_pos_open = Predicate::EQVAL(act_grip_pos.clone(), String::from("open"));
//     let grip_pos_closed = Predicate::EQVAL(act_grip_pos.clone(), String::from("closed"));
//     let grip_pos_cube = Predicate::EQVAL(act_grip_pos.clone(), String::from("cube"));
//     let grip_pos_ball = Predicate::EQVAL(act_grip_pos.clone(), String::from("ball"));
//     let not_grip_pos_open = Predicate::NOT(vec!(grip_pos_open.clone()));
//     let not_grip_pos_closed = Predicate::NOT(vec!(grip_pos_closed.clone()));
//     let not_grip_pos_cube = Predicate::NOT(vec!(grip_pos_cube.clone()));
//     let not_grip_pos_ball = Predicate::NOT(vec!(grip_pos_ball.clone()));
    
//     // ref grip pos predicates
//     let set_grip_pos_open = Predicate::EQVAL(ref_grip_pos.clone(), String::from("open"));
//     let set_grip_pos_closed = Predicate::EQVAL(ref_grip_pos.clone(), String::from("closed"));
//     let set_grip_pos_cube = Predicate::EQVAL(ref_grip_pos.clone(), String::from("cube"));
//     let set_grip_pos_ball = Predicate::EQVAL(ref_grip_pos.clone(), String::from("ball"));
//     let not_set_grip_pos_open = Predicate::NOT(vec!(set_grip_pos_open.clone()));
//     let not_set_grip_pos_closed = Predicate::NOT(vec!(set_grip_pos_closed.clone()));
//     let not_set_grip_pos_cube = Predicate::NOT(vec!(set_grip_pos_cube.clone()));
//     let not_set_grip_pos_ball = Predicate::NOT(vec!(set_grip_pos_ball.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are robot ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // are gripper ref == act predicates
//     let grip_pos_stable = Predicate::EQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let grip_stat_stable = Predicate::EQVAR(act_grip_stat.clone(), ref_grip_stat.clone());
//     let not_grip_pos_stable = Predicate::NEQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let not_grip_stat_stable = Predicate::NEQVAR(act_grip_stat.clone(), ref_grip_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         act_pos.clone(), 
//         ref_pos.clone(), 
//         act_stat.clone(), 
//         ref_stat.clone(),
//         act_grip_pos.clone(),
//         act_grip_stat.clone(),
//         ref_grip_pos.clone(),
//         ref_grip_stat.clone(),
//         buffer.clone(),
//         gripper.clone(),
//         table.clone()
//     );

//     let t1 = ParamTransition::new(
//         "robot_start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "robot_finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "robot_start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "robot_finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     let t15 = ParamTransition::new(
//         "gripper_start_activate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_active.clone()),
//             ("grip_stat", not_set_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone())
//         )
//     );

//     let t16 = ParamTransition::new(
//         "gripper_finish_activate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_stat", not_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone())
//         )
//     );

//     let t17 = ParamTransition::new(
//         "gripper_start_deactivate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", not_set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_idle.clone())
//         )
//     );

//     let t18 = ParamTransition::new(
//         "gripper_finish_deactivate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_idle.clone())
//         )
//     );

//     let t19 = ParamTransition::new(
//         "gripper_start_move_to_closed",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", not_set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_closed.clone())
//         )
//     );

//     let t20 = ParamTransition::new(
//         "gripper_finish_move_to_closed",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_closed.clone())
//         )
//     );

//     let t21 = ParamTransition::new(
//         "gripper_start_move_to_open",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", not_set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_open.clone())
//         )
//     );

//     let t22 = ParamTransition::new(
//         "gripper_finish_move_to_open",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_open.clone())
//         )
//     );

//     let t23 = ParamTransition::new(
//         "gripper_start_move_to_cube",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", not_set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_cube.clone())
//         )
//     );

//     let t24 = ParamTransition::new(
//         "gripper_finish_move_to_cube",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_cube.clone())
//         )
//     );

//     let t25 = ParamTransition::new(
//         "gripper_start_move_to_ball",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", not_set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_ball.clone())
//         )
//     );

//     let t26 = ParamTransition::new(
//         "gripper_finish_move_to_ball",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_ball.clone())
//         )
//     );

//     // 1. have to go through the "home" pose:
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 2. have to go through the "home" pose:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 3. one cube in the system:
//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // 4. gripper not moving, so...:
//     let s5 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             grip_pos_ball.clone(),
//                             grip_pos_closed.clone(),
//                             grip_pos_open.clone(),
//                             set_grip_pos_ball.clone(),
//                             set_grip_pos_closed.clone(),
//                             set_grip_pos_open.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // one operation is executing at a time:
//     let s6 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_grip_stat_stable.clone(),
//                             not_stat_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );


//     // additional constraint to make the plans be the same:
//     // robot has to be active before we activate the gripper... how can we avoid adding this constraint and still have the same plan?
//     let s7 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             grip_stat_active.clone(),
//                             not_stat_active.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_stable.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true),
//     ("grip_pos".to_string(), true), ("grip_stat".to_string(), true));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14,
//         t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s6, s7));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );

//     let now = Instant::now();

//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     let seq_planning_time = now.elapsed();

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), false), ("grip_stat".to_string(), false), 
//  ("cube".to_string(), true));
//     let refining_order: Vec<&str> = vec!(/*"grip_pos",*/ "pos", "grip_stat", "stat", "cube"); // opposite for some reason? fix this
//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     // println!("level: {:?}", result.level);
//     // println!("concat: {:?}", result.concat);
//     // println!("plan_found: {:?}", result.plan_found);
//     // println!("plan_lenght: {:?}", result.plan_length);
//     // println!("time_to_solve: {:?}", result.time_to_solve);
//     // println!("trace: ");

//     // for t in &result.trace{
 
//     //     println!("state: {:?}", t.state);
//     //     println!("trans: {:?}", t.trans);
//     //     println!("=========================");
//     // }

//     let now = Instant::now();
        
//     let solution = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

//     let sorted_result = RemoveLoops::new(&solution);

//     let comp_planning_time = now.elapsed();

//     println!("comp_level: {:?}", solution.level);
//     println!("comp_concat: {:?}", solution.concat);
//     println!("comp_plan_found: {:?}", solution.plan_found);
//     println!("comp_plan_lenght: {:?}", solution.plan_length);
//     println!("comp_time_to_solve: {:?}", solution.time_to_solve);
//     println!("comp_trace: ");

//     for t in &solution.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

    

//     println!("sorted_level: {:?}", sorted_result.level);
//     println!("sorted_concat: {:?}", sorted_result.concat);
//     println!("sorted_plan_found: {:?}", sorted_result.plan_found);
//     println!("sorted_plan_lenght: {:?}", sorted_result.plan_length);
//     println!("sorted_time_to_solve: {:?}", sorted_result.time_to_solve);
//     println!("sorted_trace: ");

//     for t in &sorted_result.trace{
    
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

//     println!("TOTAL SEQUENTIAL TIME: {:?}", seq_planning_time);
//     println!("TOTAL COMPOSITIONAL TIME: {:?}", comp_planning_time);
// }

// // #[test]
// // fn test_order_delivery_1() {
// //     // Maybe we have to consider replanning in this example, what if the scanner scans a color that we don't need?
// //     // An agv comes in with an order to deliver somewhere else for example 2 green cubes, one blue cube, and one red ball
// //     // There is one dispenser of random items, the gripper knows which shape it has picked up but not which color
// //     // The scanner tells which color we have picked. If either the shape or the color doesn't fit in our order, we return it
// //     // Otherwise we leave it on the agv. When the order is received, the agv moves on

// //     let robot_pose_domain = vec!("dispenser", "return", "scanner", "home", "agv"); // always has to go through home
// //     let gripper_pose_domain = vec!("open", "closed", "cube", "ball"); // we assume we know what we are grabbing
// //     let robot_stat_domain = vec!("active", "idle");
// //     let gripper_stat_domain = vec!("active", "idle");
// //     // let dispenser_domain = vec!("cube", "ball"); // actually, the dispenser doesn't know what it dispenses
// //     let scanner_domain = vec!("blue", "red", "green", "none"); // the scanner continously scans, and gives one random color when object is there
// //     // maybe we can use this to generate random data and maybe give back to the buffer if wrong color
// //     // if the agv doesn;t have slots, just a box, maybe we can fake it but letting it have like 10 slots and fill them with stuff in order?
// //     // or use smt array logic?

// //     // var group pos
// //     let act_rob_pos = Variable::new("act_pos", "pose", pose_domain.clone());
// //     let ref_rob_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

// //     // var group stat
// //     let act_rob_stat = Variable::new("act_stat", "status", stat_domain.clone());
// //     let ref_rob_stat = Variable::new("ref_stat", "status", stat_domain.clone());

// //     // var grpup grip
// //     let act_grip_pos = Variable::new("act_pos", "pose", pose_domain.clone());
// //     let ref_grip_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

// //     // var group cube, have to think of something better
// //     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
// //     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
// //     let table = Variable::new("table_cube", "table", table_domain.clone());

// //     // act stat predicates
// //     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
// //     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
// //     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
// //     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

// //     // ref stat predicates
// //     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
// //     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
// //     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
// //     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

// //     // act pos predicates
// //     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
// //     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
// //     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
// //     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
// //     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
// //     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

// //     // ref pos predicates
// //     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
// //     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
// //     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
// //     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
// //     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
// //     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

// //     // act buffer predicates
// //     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
// //     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
// //     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
// //     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
// //     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
// //     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));
    
// //     // act gripper predicates
// //     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
// //     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
// //     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
// //     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
// //     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
// //     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

// //     // act table predicates
// //     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
// //     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
// //     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
// //     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
// //     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
// //     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

// //     // are ref == act predicates
// //     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
// //     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
// //     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
// //     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

// //     // variables in the problem
// //     let all_vars = vec!(
// //         act_pos.clone(), 
// //         ref_pos.clone(), 
// //         act_stat.clone(), 
// //         ref_stat.clone(),
// //         buffer.clone(), 
// //         gripper.clone(), 
// //         table.clone()
// //     );

// //     let t1 = ParamTransition::new(
// //         "start_activate",
// //         vec!(ref_stat.clone()),
// //         &vec!(
// //             ("stat", not_stat_active.clone()),
// //             ("stat", not_set_stat_active.clone())
// //         ),
// //         &vec!(
// //             ("stat", set_stat_active.clone())
// //         )
// //     );

// //     let t2 = ParamTransition::new(
// //         "finish_activate",
// //         vec!(act_stat.clone()),
// //         &vec!(
// //             ("stat", set_stat_active.clone()),
// //             ("stat", not_stat_active.clone())
// //         ),
// //         &vec!(
// //             ("stat", stat_active.clone())
// //         )
// //     );

// //     let t3 = ParamTransition::new(
// //         "start_deactivate",
// //         vec!(ref_stat.clone()),
// //         &vec!(
// //             ("stat", not_stat_idle.clone()),
// //             ("stat", not_set_stat_idle.clone())
// //         ),
// //         &vec!(
// //             ("stat", set_stat_idle.clone())
// //         )
// //     );

// //     let t4 = ParamTransition::new(
// //         "finish_deactivate",
// //         vec!(act_stat.clone()),
// //         &vec!(
// //             ("stat", not_stat_idle.clone()),
// //             ("stat", set_stat_idle.clone())
// //         ),
// //         &vec!(
// //             ("stat", stat_idle.clone())
// //         )
// //     );

// //     let t5 = ParamTransition::new(
// //         "start_move_to_buffer",
// //         vec!(ref_pos.clone()),
// //         &vec!(
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_stable.clone()),
// //             ("pos", not_pos_buffer.clone()),
// //             ("pos", not_set_pos_buffer.clone())
// //         ),
// //         &vec!(
// //             ("pos", set_pos_buffer.clone())
// //         )
// //     );

// //     let t6 = ParamTransition::new(
// //         "finish_move_to_buffer",
// //         vec!(act_pos.clone()),
// //         &vec!(
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", not_pos_buffer.clone()),
// //             ("pos", set_pos_buffer.clone())
// //         ),
// //         &vec!(
// //             ("pos", pos_buffer.clone())
// //         )
// //     );

// //     let t7 = ParamTransition::new(
// //         "start_move_to_table",
// //         vec!(ref_pos.clone()),
// //         &vec!(
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_stable.clone()),
// //             ("pos", not_pos_table.clone()),
// //             ("pos", not_set_pos_table.clone())
// //         ),
// //         &vec!(
// //             ("pos", set_pos_table.clone())
// //         )
// //     );

// //     let t8 = ParamTransition::new(
// //         "finish_move_to_table",
// //         vec!(act_pos.clone()),
// //         &vec!(
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", not_pos_table.clone()),
// //             ("pos", set_pos_table.clone())
// //         ),
// //         &vec!(
// //             ("pos", pos_table.clone())
// //         )
// //     );

// //     let t9 = ParamTransition::new(
// //         "start_move_to_home",
// //         vec!(ref_pos.clone()),
// //         &vec!(
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_stable.clone()),
// //             ("pos", not_pos_home.clone()),
// //             ("pos", not_set_pos_home.clone())
// //         ),
// //         &vec!(
// //             ("pos", set_pos_home.clone())
// //         )
// //     );

// //     let t10 = ParamTransition::new(
// //         "finish_move_to_home",
// //         vec!(act_pos.clone()),
// //         &vec!(
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", not_pos_home.clone()),
// //             ("pos", set_pos_home.clone())
// //         ),
// //         &vec!(
// //             ("pos", pos_home.clone())
// //         )
// //     );

// //     let t11 = ParamTransition::new(
// //         "take_cube_from_buffer",
// //         vec!(gripper.clone(), buffer.clone(), table.clone()),
// //         &vec!(
// //             ("cube", buffer_cube.clone()),
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_buffer.clone()),
// //             ("pos", set_pos_buffer.clone())
// //         ),
// //         &vec!(
// //             ("cube", gripper_cube.clone())
// //         )
// //     );

// //     let t12 = ParamTransition::new(
// //         "take_cube_from_table",
// //         vec!(gripper.clone(), buffer.clone(), table.clone()),
// //         &vec!(
// //             ("cube", table_cube.clone()),
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_table.clone()),
// //             ("pos", set_pos_table.clone())
// //         ),
// //         &vec!(
// //             ("cube", gripper_cube.clone())
// //         )
// //     );

// //     let t13 = ParamTransition::new(
// //         "leave_cube_at_buffer",
// //         vec!(gripper.clone(), buffer.clone(), table.clone()),
// //         &vec!(
// //             ("cube", gripper_cube.clone()),
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_buffer.clone()),
// //             ("pos", set_pos_buffer.clone())
// //         ),
// //         &vec!(
// //             ("cube", buffer_cube.clone())
// //         )
// //     );

// //     let t14 = ParamTransition::new(
// //         "leave_cube_at_table",
// //         vec!(gripper.clone(), buffer.clone(), table.clone()),
// //         &vec!(
// //             ("cube", gripper_cube.clone()),
// //             ("stat", stat_active.clone()),
// //             ("stat", set_stat_active.clone()),
// //             ("pos", pos_table.clone()),
// //             ("pos", set_pos_table.clone())
// //         ),
// //         &vec!(
// //             ("cube", table_cube.clone())
// //         )
// //     );

// //     // 1. have to go through the "home" pose:
// //     let s1 = Predicate::GLOB(
// //         vec!(
// //             Predicate::NOT(
// //                 vec!(
// //                     Predicate::AFTER(
// //                         vec!(
// //                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
// //                         ),
// //                         vec!(
// //                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
// //                         ),
// //                         2 // how can this be improved so that the plan also holds for bigger a number
// //                     )
// //                 )
// //             )
// //         )
// //     );
    
// //     // 2. have to go through the "home" pose:
// //     let s2 = Predicate::GLOB(
// //         vec!(
// //             Predicate::NOT(
// //                 vec!(
// //                     Predicate::AFTER(
// //                         vec!(
// //                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
// //                         ),
// //                         vec!(
// //                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
// //                         ),
// //                         2 // how can this be improved so that the plan also holds for bigger a number
// //                     )
// //                 )
// //             )
// //         )
// //     );
    
// //     // 3. one cube in the system:
// //     let s3 = Predicate::GLOB(
// //         vec!(
// //             Predicate::PBEQ(
// //                 vec!(
// //                     Predicate::AND(
// //                         vec!(
// //                             buffer_cube.clone(), 
// //                             Predicate::NOT(
// //                                 vec!(
// //                                     gripper_cube.clone()
// //                                 )
// //                             ), 
// //                             Predicate::NOT(
// //                                 vec!(
// //                                     table_cube.clone()
// //                                 )
// //                             )
// //                         )
// //                     ),
// //                     Predicate::AND(
// //                         vec!(
// //                             table_cube.clone(), 
// //                             Predicate::NOT(
// //                                 vec!(
// //                                     gripper_cube.clone()
// //                                 )
// //                             ), 
// //                             Predicate::NOT(
// //                                 vec!(
// //                                     buffer_cube.clone()
// //                                 )
// //                             )
// //                         )
// //                     ),
// //                     Predicate::AND(
// //                         vec!(
// //                             gripper_cube.clone(), 
// //                             Predicate::NOT(
// //                                 vec!(
// //                                     table_cube.clone()
// //                                 )
// //                             ), 
// //                             Predicate::NOT(
// //                                 vec!(
// //                                     buffer_cube.clone()
// //                                 )
// //                             )
// //                         )
// //                     )
// //                 ),
// //                 1
// //             )
// //         )
// //     );
    
// //     // 4. no ball in the system:
// //     let s4 = Predicate::GLOB(
// //         vec!(
// //             Predicate::NOT(
// //                 vec!(
// //                     Predicate::OR(
// //                         vec!(
// //                             buffer_ball.clone(),
// //                             table_ball.clone(),
// //                             gripper_ball.clone()
// //                         )
// //                     )
// //                 )
// //             )
// //         )
// //     );
    
// //     let initial = vec!(
// //         ("pos", pos_stable.clone()),
// //         ("pos", pos_buffer.clone()),
// //         ("stat", stat_stable.clone()),
// //         ("stat", stat_idle.clone()),
// //         ("cube", table_cube.clone())
// //     );

// //     let goal = vec!(
// //         ("pos", pos_table.clone()),
// //         ("stat", stat_idle.clone()),
// //         ("cube", buffer_cube.clone())
// //     );
    
// //     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true));
// //     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);
// //     let specs = Predicate::AND(vec!(s1, s2, s3, s4));

// //     let mut concat: u32 = 0;
// //     let mut level: u32 = 0;

// //     let problem = ParamPlanningProblem::new(
// //         String::from("param_prob_1"), 
// //         all_vars.clone(),
// //         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
// //         initial.clone(),
// //         goal.clone(),
// //         trans.clone(), 
// //         specs.clone(),
// //         30
// //     );

// //     let now = Instant::now();

// //     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

// //     let seq_planning_time = now.elapsed();

// //     println!("level: {:?}", result.level);
// //     println!("concat: {:?}", result.concat);
// //     println!("plan_found: {:?}", result.plan_found);
// //     println!("plan_lenght: {:?}", result.plan_length);
// //     println!("time_to_solve: {:?}", result.time_to_solve);
// //     println!("trace: ");

// //     for t in &result.trace{
 
// //         println!("state: {:?}", t.state);
// //         println!("trans: {:?}", t.trans);
// //         println!("=========================");
// //     }

// //     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), false), ("cube".to_string(), true));
// //     let refining_order: Vec<&str> = vec!("pos", "stat", "cube"); // opposite for some reason? fix this
// //     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

// //     println!("level: {:?}", result.level);
// //     println!("concat: {:?}", result.concat);
// //     println!("plan_found: {:?}", result.plan_found);
// //     println!("plan_lenght: {:?}", result.plan_length);
// //     println!("time_to_solve: {:?}", result.time_to_solve);
// //     println!("trace: ");

// //     for t in &result.trace{
 
// //         println!("state: {:?}", t.state);
// //         println!("trans: {:?}", t.trans);
// //         println!("=========================");
// //     }

// //     let now = Instant::now();
        
// //     let solutions = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

// //     let comp_planning_time = now.elapsed();

// //     println!("TOTAL SEQUENTIAL TIME: {:?}", seq_planning_time);
// //     println!("TOTAL COMPOSITIONAL TIME: {:?}", comp_planning_time);
// // }



// // TESTS FOR THE ETFA2020 PAPER AFTER THIS POINT
// #[test]
// fn example_1_sequential(){
    
//     let pose_domain = vec!("buffer", "home", "table");
//     let stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group cube, have to think of something better
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));
    
//     // act gripper predicates
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         act_pos.clone(), 
//         ref_pos.clone(), 
//         act_stat.clone(), 
//         ref_stat.clone(),
//         buffer.clone(), 
//         gripper.clone(), 
//         table.clone()
//     );

//     let t1 = ParamTransition::new(
//         "start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     // 1. have to go through the "home" pose:
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 2. have to go through the "home" pose:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 3. one cube in the system:
//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );

//     let now = Instant::now();

//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     let seq_planning_time = now.elapsed();

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

//     // // 

//     // let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), true), ("cube".to_string(), false));
//     // let refining_order: Vec<&str> = vec!("pos", "cube", "stat"); // opposite for some reason? fix this
//     // let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     // println!("level: {:?}", result.level);
//     // println!("concat: {:?}", result.concat);
//     // println!("plan_found: {:?}", result.plan_found);
//     // println!("plan_lenght: {:?}", result.plan_length);
//     // println!("time_to_solve: {:?}", result.time_to_solve);
//     // println!("trace: ");

//     // for t in &result.trace{
 
//     //     println!("state: {:?}", t.state);
//     //     println!("trans: {:?}", t.trans);
//     //     println!("=========================");
//     // }

//     // let now = Instant::now();
        
//     // let solution = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

//     // // RemoveLoops::new(&solution);
//     // let sorted_result = RemoveLoops::new(&solution);

//     // let comp_planning_time = now.elapsed();

//     // println!("comp_level: {:?}", solution.level);
//     // println!("comp_concat: {:?}", solution.concat);
//     // println!("comp_plan_found: {:?}", solution.plan_found);
//     // println!("comp_plan_lenght: {:?}", solution.plan_length);
//     // println!("comp_time_to_solve: {:?}", solution.time_to_solve);
//     // println!("comp_trace: ");

//     // for t in &solution.trace{
 
//     //     println!("state: {:?}", t.state);
//     //     println!("trans: {:?}", t.trans);
//     //     println!("=========================");
//     // }

    

//     // println!("sorted_level: {:?}", sorted_result.level);
//     // println!("sorted_concat: {:?}", sorted_result.concat);
//     // println!("sorted_plan_found: {:?}", sorted_result.plan_found);
//     // println!("sorted_plan_lenght: {:?}", sorted_result.plan_length);
//     // println!("sorted_time_to_solve: {:?}", sorted_result.time_to_solve);
//     // println!("sorted_trace: ");

//     // for t in &sorted_result.trace{
    
//     //     println!("state: {:?}", t.state);
//     //     println!("trans: {:?}", t.trans);
//     //     println!("=========================");
//     // }

//     println!("TOTAL SEQUENTIAL TIME: {:?}", seq_planning_time);
//     // println!("TOTAL COMPOSITIONAL TIME: {:?}", comp_planning_time);
// }

// #[test]
// fn example_1_compositional(){
    
//     let pose_domain = vec!("buffer", "home", "table");
//     let stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group cube, have to think of something better
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));
    
//     // act gripper predicates
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         ("pos", act_pos.clone()), 
//         ("pos", ref_pos.clone()), 
//         ("stat", act_stat.clone()), 
//         ("stat", ref_stat.clone()),
//         ("cube", buffer.clone()), 
//         ("cube", gripper.clone()), 
//         ("cube", table.clone())
//     );

//     let t1 = ParamTransition::new(
//         "start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     // 1. have to go through the "home" pose:
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 2. have to go through the "home" pose:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 3. one cube in the system:
//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), false), ("cube".to_string(), true));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblemNew::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );


//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), false), ("cube".to_string(), true));
//     let refining_order: Vec<&str> = vec!("pos", "stat", "cube"); // opposite for some reason? fix this
//     let result = ParamSequentialNew::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     println!("comp_level: {:?}", result.level);
//     println!("comp_concat: {:?}", result.concat);
//     println!("comp_plan_found: {:?}", result.plan_found);
//     println!("comp_plan_lenght: {:?}", result.plan_length);
//     println!("comp_time_to_solve: {:?}", result.time_to_solve);
//     println!("comp_trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }
 
//     let solution = Compositional3::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

//     println!("comp_level: {:?}", solution.level);
//     println!("comp_concat: {:?}", solution.concat);
//     println!("comp_plan_found: {:?}", solution.plan_found);
//     println!("comp_plan_lenght: {:?}", solution.plan_length);
//     println!("comp_time_to_solve: {:?}", solution.time_to_solve);
//     println!("comp_trace: ");

//     for t in &solution.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }

// }

// #[test]
// fn example_2_sequential(){
    
//     let r1_pose_domain = vec!("11", "12", "13", "14", "15");
//     let r2_pose_domain = vec!("21", "22", "23", "24", "25");

//     // r1 var group
//     let r1_act_pos = Variable::new("r1_act_pos", "r1_pose", r1_pose_domain.clone());
//     let r1_ref_pos = Variable::new("r1_ref_pos", "r1_pose", r1_pose_domain.clone());

//     // r2 var group
//     let r2_act_pos = Variable::new("r2_act_pos", "r2_pose", r2_pose_domain.clone());
//     let r2_ref_pos = Variable::new("r2_ref_pos", "r2_pose", r2_pose_domain.clone());

//     // r1 act pos predicates
//     let r1_pos_1 = Predicate::EQVAL(r1_act_pos.clone(), String::from("11"));
//     let r1_pos_2 = Predicate::EQVAL(r1_act_pos.clone(), String::from("12"));
//     let r1_pos_3 = Predicate::EQVAL(r1_act_pos.clone(), String::from("13"));
//     let r1_pos_4 = Predicate::EQVAL(r1_act_pos.clone(), String::from("14"));
//     let r1_pos_5 = Predicate::EQVAL(r1_act_pos.clone(), String::from("15"));
//     let not_r1_pos_1 = Predicate::NOT(vec!(r1_pos_1.clone()));
//     let not_r1_pos_2 = Predicate::NOT(vec!(r1_pos_2.clone()));
//     let not_r1_pos_3 = Predicate::NOT(vec!(r1_pos_3.clone()));
//     let not_r1_pos_4 = Predicate::NOT(vec!(r1_pos_4.clone()));
//     let not_r1_pos_5 = Predicate::NOT(vec!(r1_pos_5.clone()));

//     // r2 act pos predicates
//     let r2_pos_1 = Predicate::EQVAL(r2_act_pos.clone(), String::from("21"));
//     let r2_pos_2 = Predicate::EQVAL(r2_act_pos.clone(), String::from("22"));
//     let r2_pos_3 = Predicate::EQVAL(r2_act_pos.clone(), String::from("23"));
//     let r2_pos_4 = Predicate::EQVAL(r2_act_pos.clone(), String::from("24"));
//     let r2_pos_5 = Predicate::EQVAL(r2_act_pos.clone(), String::from("25"));
//     let not_r2_pos_1 = Predicate::NOT(vec!(r2_pos_1.clone()));
//     let not_r2_pos_2 = Predicate::NOT(vec!(r2_pos_2.clone()));
//     let not_r2_pos_3 = Predicate::NOT(vec!(r2_pos_3.clone()));
//     let not_r2_pos_4 = Predicate::NOT(vec!(r2_pos_4.clone()));
//     let not_r2_pos_5 = Predicate::NOT(vec!(r2_pos_5.clone()));

//     // r1 ref pos predicates
//     let r1_set_pos_1 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("11"));
//     let r1_set_pos_2 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("12"));
//     let r1_set_pos_3 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("13"));
//     let r1_set_pos_4 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("14"));
//     let r1_set_pos_5 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("15"));
//     let not_r1_set_pos_1 = Predicate::NOT(vec!(r1_set_pos_1.clone()));
//     let not_r1_set_pos_2 = Predicate::NOT(vec!(r1_set_pos_2.clone()));
//     let not_r1_set_pos_3 = Predicate::NOT(vec!(r1_set_pos_3.clone()));
//     let not_r1_set_pos_4 = Predicate::NOT(vec!(r1_set_pos_4.clone()));
//     let not_r1_set_pos_5 = Predicate::NOT(vec!(r1_set_pos_5.clone()));

//     // r2 act pos predicates
//     let r2_set_pos_1 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("21"));
//     let r2_set_pos_2 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("22"));
//     let r2_set_pos_3 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("23"));
//     let r2_set_pos_4 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("24"));
//     let r2_set_pos_5 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("25"));
//     let not_r2_set_pos_1 = Predicate::NOT(vec!(r2_set_pos_1.clone()));
//     let not_r2_set_pos_2 = Predicate::NOT(vec!(r2_set_pos_2.clone()));
//     let not_r2_set_pos_3 = Predicate::NOT(vec!(r2_set_pos_3.clone()));
//     let not_r2_set_pos_4 = Predicate::NOT(vec!(r2_set_pos_4.clone()));
//     let not_r2_set_pos_5 = Predicate::NOT(vec!(r2_set_pos_5.clone()));

//     // are ref == act predicates
//     let r1_pos_stable = Predicate::EQVAR(r1_act_pos.clone(), r1_ref_pos.clone());
//     let r2_pos_stable = Predicate::EQVAR(r2_act_pos.clone(), r2_ref_pos.clone());
//     let not_r1_pos_stable = Predicate::NEQVAR(r1_act_pos.clone(), r1_ref_pos.clone());
//     let not_r2_pos_stable = Predicate::NEQVAR(r2_act_pos.clone(), r2_ref_pos.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         r1_act_pos.clone(), 
//         r1_ref_pos.clone(), 
//         r2_act_pos.clone(), 
//         r2_ref_pos.clone()
//     );

//     let t1 = ParamTransition::new(
//         "r1_start_move_to_11",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", r1_pos_2.clone()),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_1.clone()),
//             ("r1_pos", not_r1_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_1.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "r1_finish_move_to_11",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_1.clone()),
//             ("r1_pos", r1_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_1.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "r1_start_move_to_12",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", Predicate::OR(vec!(r1_pos_1.clone(), r1_pos_3.clone()))),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_2.clone()),
//             ("r1_pos", not_r1_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_2.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "r1_finish_move_to_12",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_2.clone()),
//             ("r1_pos", r1_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_2.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "r1_start_move_to_13",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", Predicate::OR(vec!(r1_pos_2.clone(), r1_pos_4.clone()))),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_3.clone()),
//             ("r1_pos", not_r1_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_3.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "r1_finish_move_to_13",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_3.clone()),
//             ("r1_pos", r1_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_3.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "r1_start_move_to_14",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", Predicate::OR(vec!(r1_pos_3.clone(), r1_pos_5.clone()))),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_4.clone()),
//             ("r1_pos", not_r1_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_4.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "r1_finish_move_to_14",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_4.clone()),
//             ("r1_pos", r1_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_4.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "r1_start_move_to_15",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", r1_pos_4.clone()),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_5.clone()),
//             ("r1_pos", not_r1_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_5.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "r1_finish_move_to_15",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_5.clone()),
//             ("r1_pos", r1_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_5.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "r2_start_move_to_21",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", r2_pos_2.clone()),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_1.clone()),
//             ("r2_pos", not_r2_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_1.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "r2_finish_move_to_21",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_1.clone()),
//             ("r2_pos", r2_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_1.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "r2_start_move_to_22",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", Predicate::OR(vec!(r2_pos_1.clone(), r2_pos_3.clone()))),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_2.clone()),
//             ("r2_pos", not_r2_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_2.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "r2_finish_move_to_22",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_2.clone()),
//             ("r2_pos", r2_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_2.clone())
//         )
//     );

//     let t15 = ParamTransition::new(
//         "r2_start_move_to_23",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", Predicate::OR(vec!(r2_pos_2.clone(), r2_pos_4.clone()))),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_3.clone()),
//             ("r2_pos", not_r2_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_3.clone())
//         )
//     );

//     let t16 = ParamTransition::new(
//         "r2_finish_move_to_23",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_3.clone()),
//             ("r2_pos", r2_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_3.clone())
//         )
//     );

//     let t17 = ParamTransition::new(
//         "r2_start_move_to_24",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", Predicate::OR(vec!(r2_pos_3.clone(), r2_pos_5.clone()))),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_4.clone()),
//             ("r2_pos", not_r2_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_4.clone())
//         )
//     );

//     let t18 = ParamTransition::new(
//         "r2_finish_move_to_24",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_4.clone()),
//             ("r2_pos", r2_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_4.clone())
//         )
//     );

//     let t19 = ParamTransition::new(
//         "r2_start_move_to_25",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", r2_pos_4.clone()),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_5.clone()),
//             ("r2_pos", not_r2_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_5.clone())
//         )
//     );

//     let t20 = ParamTransition::new(
//         "r2_finish_move_to_25",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_5.clone()),
//             ("r2_pos", r2_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_5.clone())
//         )
//     );

//     // only one robot moves at a time (a good csrt to test the compositional algorithm):
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_r1_pos_stable.clone(),
//                             not_r2_pos_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // they can't be at the cross section or move towards or away from the cross section at the same time:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             r1_pos_3.clone(),
//                             r2_pos_3.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     let initial = vec!(
//         ("r1_pos", r1_pos_stable.clone()),
//         ("r1_pos", r1_pos_1.clone()),
//         ("r1_pos", r1_set_pos_1.clone()),
//         ("r2_pos", r2_pos_stable.clone()),
//         ("r2_pos", r2_pos_1.clone()),
//         ("r2_pos", r2_set_pos_1.clone())
//     );

//     let goal = vec!(
//         ("r1_pos", r1_pos_stable.clone()),
//         ("r1_pos", r1_pos_5.clone()),
//         ("r1_pos", r1_set_pos_5.clone()),
//         ("r2_pos", r2_pos_stable.clone()),
//         ("r2_pos", r2_pos_5.clone()),
//         ("r2_pos", r2_set_pos_5.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("r1_pos".to_string(), true), ("r2_pos".to_string(), false));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20);
//     let specs = Predicate::AND(vec!(s1, s2));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("agvs_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );

//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }


// }

// #[test]
// fn example_2_compositional(){
    
//     let r1_pose_domain = vec!("11", "12", "13", "14", "15");
//     let r2_pose_domain = vec!("21", "22", "23", "24", "25");

//     // r1 var group
//     let r1_act_pos = Variable::new("r1_act_pos", "r1_pose", r1_pose_domain.clone());
//     let r1_ref_pos = Variable::new("r1_ref_pos", "r1_pose", r1_pose_domain.clone());

//     // r2 var group
//     let r2_act_pos = Variable::new("r2_act_pos", "r2_pose", r2_pose_domain.clone());
//     let r2_ref_pos = Variable::new("r2_ref_pos", "r2_pose", r2_pose_domain.clone());

//     // r1 act pos predicates
//     let r1_pos_1 = Predicate::EQVAL(r1_act_pos.clone(), String::from("11"));
//     let r1_pos_2 = Predicate::EQVAL(r1_act_pos.clone(), String::from("12"));
//     let r1_pos_3 = Predicate::EQVAL(r1_act_pos.clone(), String::from("13"));
//     let r1_pos_4 = Predicate::EQVAL(r1_act_pos.clone(), String::from("14"));
//     let r1_pos_5 = Predicate::EQVAL(r1_act_pos.clone(), String::from("15"));
//     let not_r1_pos_1 = Predicate::NOT(vec!(r1_pos_1.clone()));
//     let not_r1_pos_2 = Predicate::NOT(vec!(r1_pos_2.clone()));
//     let not_r1_pos_3 = Predicate::NOT(vec!(r1_pos_3.clone()));
//     let not_r1_pos_4 = Predicate::NOT(vec!(r1_pos_4.clone()));
//     let not_r1_pos_5 = Predicate::NOT(vec!(r1_pos_5.clone()));

//     // r2 act pos predicates
//     let r2_pos_1 = Predicate::EQVAL(r2_act_pos.clone(), String::from("21"));
//     let r2_pos_2 = Predicate::EQVAL(r2_act_pos.clone(), String::from("22"));
//     let r2_pos_3 = Predicate::EQVAL(r2_act_pos.clone(), String::from("23"));
//     let r2_pos_4 = Predicate::EQVAL(r2_act_pos.clone(), String::from("24"));
//     let r2_pos_5 = Predicate::EQVAL(r2_act_pos.clone(), String::from("25"));
//     let not_r2_pos_1 = Predicate::NOT(vec!(r2_pos_1.clone()));
//     let not_r2_pos_2 = Predicate::NOT(vec!(r2_pos_2.clone()));
//     let not_r2_pos_3 = Predicate::NOT(vec!(r2_pos_3.clone()));
//     let not_r2_pos_4 = Predicate::NOT(vec!(r2_pos_4.clone()));
//     let not_r2_pos_5 = Predicate::NOT(vec!(r2_pos_5.clone()));

//     // r1 ref pos predicates
//     let r1_set_pos_1 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("11"));
//     let r1_set_pos_2 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("12"));
//     let r1_set_pos_3 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("13"));
//     let r1_set_pos_4 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("14"));
//     let r1_set_pos_5 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("15"));
//     let not_r1_set_pos_1 = Predicate::NOT(vec!(r1_set_pos_1.clone()));
//     let not_r1_set_pos_2 = Predicate::NOT(vec!(r1_set_pos_2.clone()));
//     let not_r1_set_pos_3 = Predicate::NOT(vec!(r1_set_pos_3.clone()));
//     let not_r1_set_pos_4 = Predicate::NOT(vec!(r1_set_pos_4.clone()));
//     let not_r1_set_pos_5 = Predicate::NOT(vec!(r1_set_pos_5.clone()));

//     // r2 act pos predicates
//     let r2_set_pos_1 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("21"));
//     let r2_set_pos_2 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("22"));
//     let r2_set_pos_3 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("23"));
//     let r2_set_pos_4 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("24"));
//     let r2_set_pos_5 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("25"));
//     let not_r2_set_pos_1 = Predicate::NOT(vec!(r2_set_pos_1.clone()));
//     let not_r2_set_pos_2 = Predicate::NOT(vec!(r2_set_pos_2.clone()));
//     let not_r2_set_pos_3 = Predicate::NOT(vec!(r2_set_pos_3.clone()));
//     let not_r2_set_pos_4 = Predicate::NOT(vec!(r2_set_pos_4.clone()));
//     let not_r2_set_pos_5 = Predicate::NOT(vec!(r2_set_pos_5.clone()));

//     // are ref == act predicates
//     let r1_pos_stable = Predicate::EQVAR(r1_act_pos.clone(), r1_ref_pos.clone());
//     let r2_pos_stable = Predicate::EQVAR(r2_act_pos.clone(), r2_ref_pos.clone());
//     let not_r1_pos_stable = Predicate::NEQVAR(r1_act_pos.clone(), r1_ref_pos.clone());
//     let not_r2_pos_stable = Predicate::NEQVAR(r2_act_pos.clone(), r2_ref_pos.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         ("r1_pos", r1_act_pos.clone()), 
//         ("r1_pos", r1_ref_pos.clone()), 
//         ("r2_pos", r2_act_pos.clone()), 
//         ("r2_pos", r2_ref_pos.clone())
//     );

//     let t1 = ParamTransition::new(
//         "r1_start_move_to_11",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", r1_pos_2.clone()),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_1.clone()),
//             ("r1_pos", not_r1_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_1.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "r1_finish_move_to_11",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_1.clone()),
//             ("r1_pos", r1_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_1.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "r1_start_move_to_12",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", Predicate::OR(vec!(r1_pos_1.clone(), r1_pos_3.clone()))),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_2.clone()),
//             ("r1_pos", not_r1_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_2.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "r1_finish_move_to_12",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_2.clone()),
//             ("r1_pos", r1_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_2.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "r1_start_move_to_13",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", Predicate::OR(vec!(r1_pos_2.clone(), r1_pos_4.clone()))),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_3.clone()),
//             ("r1_pos", not_r1_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_3.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "r1_finish_move_to_13",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_3.clone()),
//             ("r1_pos", r1_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_3.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "r1_start_move_to_14",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", Predicate::OR(vec!(r1_pos_3.clone(), r1_pos_5.clone()))),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_4.clone()),
//             ("r1_pos", not_r1_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_4.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "r1_finish_move_to_14",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_4.clone()),
//             ("r1_pos", r1_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_4.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "r1_start_move_to_15",
//         vec!(r1_ref_pos.clone()),
//         &vec!(
//             ("r1_pos", r1_pos_4.clone()),
//             ("r1_pos", r1_pos_stable.clone()),
//             ("r1_pos", not_r1_pos_5.clone()),
//             ("r1_pos", not_r1_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_set_pos_5.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "r1_finish_move_to_15",
//         vec!(r1_act_pos.clone()),
//         &vec!(
//             ("r1_pos", not_r1_pos_5.clone()),
//             ("r1_pos", r1_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r1_pos", r1_pos_5.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "r2_start_move_to_21",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", r2_pos_2.clone()),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_1.clone()),
//             ("r2_pos", not_r2_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_1.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "r2_finish_move_to_21",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_1.clone()),
//             ("r2_pos", r2_set_pos_1.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_1.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "r2_start_move_to_22",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", Predicate::OR(vec!(r2_pos_1.clone(), r2_pos_3.clone()))),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_2.clone()),
//             ("r2_pos", not_r2_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_2.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "r2_finish_move_to_22",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_2.clone()),
//             ("r2_pos", r2_set_pos_2.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_2.clone())
//         )
//     );

//     let t15 = ParamTransition::new(
//         "r2_start_move_to_23",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", Predicate::OR(vec!(r2_pos_2.clone(), r2_pos_4.clone()))),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_3.clone()),
//             ("r2_pos", not_r2_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_3.clone())
//         )
//     );

//     let t16 = ParamTransition::new(
//         "r2_finish_move_to_23",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_3.clone()),
//             ("r2_pos", r2_set_pos_3.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_3.clone())
//         )
//     );

//     let t17 = ParamTransition::new(
//         "r2_start_move_to_24",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", Predicate::OR(vec!(r2_pos_3.clone(), r2_pos_5.clone()))),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_4.clone()),
//             ("r2_pos", not_r2_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_4.clone())
//         )
//     );

//     let t18 = ParamTransition::new(
//         "r2_finish_move_to_24",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_4.clone()),
//             ("r2_pos", r2_set_pos_4.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_4.clone())
//         )
//     );

//     let t19 = ParamTransition::new(
//         "r2_start_move_to_25",
//         vec!(r2_ref_pos.clone()),
//         &vec!(
//             ("r2_pos", r2_pos_4.clone()),
//             ("r2_pos", r2_pos_stable.clone()),
//             ("r2_pos", not_r2_pos_5.clone()),
//             ("r2_pos", not_r2_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_set_pos_5.clone())
//         )
//     );

//     let t20 = ParamTransition::new(
//         "r2_finish_move_to_25",
//         vec!(r2_act_pos.clone()),
//         &vec!(
//             ("r2_pos", not_r2_pos_5.clone()),
//             ("r2_pos", r2_set_pos_5.clone())
//         ),
//         &vec!(
//             ("r2_pos", r2_pos_5.clone())
//         )
//     );

//     // only one robot moves at a time (a good csrt to test the compositional algorithm):
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_r1_pos_stable.clone(),
//                             not_r2_pos_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // they can't be at the cross section or move towards or away from the cross section at the same time:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             r1_pos_3.clone(),
//                             r2_pos_3.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     let initial = vec!(
//         ("r1_pos", r1_pos_stable.clone()),
//         ("r1_pos", r1_pos_1.clone()),
//         ("r1_pos", r1_set_pos_1.clone()),
//         ("r2_pos", r2_pos_stable.clone()),
//         ("r2_pos", r2_pos_1.clone()),
//         ("r2_pos", r2_set_pos_1.clone())
//     );

//     let goal = vec!(
//         ("r1_pos", r1_pos_stable.clone()),
//         ("r1_pos", r1_pos_5.clone()),
//         ("r1_pos", r1_set_pos_5.clone()),
//         ("r2_pos", r2_pos_stable.clone()),
//         ("r2_pos", r2_pos_5.clone()),
//         ("r2_pos", r2_set_pos_5.clone())
//     );

//     let mut act: Vec<(String, bool)> = vec!(("r1_pos".to_string(), true), ("r2_pos".to_string(), false));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10);//, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20);
//     let specs = Predicate::AND(vec!());

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblemNew::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );


//     // let mut act: Vec<(String, bool)> = vec!(("r1_pos".to_string(), true), ("r2_pos".to_string(), false));
//     let refining_order: Vec<&str> = vec!("r2_pos", "r1_pos"); // opposite for some reason? fix this
//     let result = ParamSequentialNew::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     println!("comp_level: {:?}", result.level);
//     println!("comp_concat: {:?}", result.concat);
//     println!("comp_plan_found: {:?}", result.plan_found);
//     println!("comp_plan_lenght: {:?}", result.plan_length);
//     println!("comp_time_to_solve: {:?}", result.time_to_solve);
//     println!("comp_trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }
 
//     let solution = Compositional3::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

//     println!("comp_level: {:?}", solution.level);
//     println!("comp_concat: {:?}", solution.concat);
//     println!("comp_plan_found: {:?}", solution.plan_found);
//     println!("comp_plan_lenght: {:?}", solution.plan_length);
//     println!("comp_time_to_solve: {:?}", solution.time_to_solve);
//     println!("comp_trace: ");

//     for t in &solution.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }
// }

// #[test]
// fn example_3_sequential(){

//     // a global constraint should be made that changea in the system are one at a time
//     // so for instance, if you're activating a robot and a gripper, you have to finish activating
//     // one first and then actvate another. This maybe makes sense in general
    
//     let pose_domain = vec!("buffer", "home", "table");
//     let gripper_pose_domain = vec!("cube", "ball", "closed", "open");
//     let stat_domain = vec!("active", "idle");
//     let gripper_stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group gripper pos
//     let act_grip_pos = Variable::new("act_grip_pos", "grip_pose", gripper_pose_domain.clone());
//     let ref_grip_pos = Variable::new("ref_grip_pos", "grip_pose", gripper_pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group grip stat
//     let act_grip_stat = Variable::new("act_grip_stat", "grip_status", gripper_stat_domain.clone());
//     let ref_grip_stat = Variable::new("ref_grip_stat", "grip_status", gripper_stat_domain.clone());

//     // var group cube, have to think of something better
//     // well ok we have sensors in the gripper beside opening for some reason
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // act grip stat predicates
//     let grip_stat_active = Predicate::EQVAL(act_grip_stat.clone(), String::from("active"));
//     let grip_stat_idle = Predicate::EQVAL(act_grip_stat.clone(), String::from("idle"));
//     let not_grip_stat_active = Predicate::NOT(vec!(grip_stat_active.clone()));
//     let not_grip_stat_idle = Predicate::NOT(vec!(grip_stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // ref grip stat predicates
//     let set_grip_stat_active = Predicate::EQVAL(ref_grip_stat.clone(), String::from("active"));
//     let set_grip_stat_idle = Predicate::EQVAL(ref_grip_stat.clone(), String::from("idle"));
//     let not_set_grip_stat_active = Predicate::NOT(vec!(set_grip_stat_active.clone()));
//     let not_set_grip_stat_idle = Predicate::NOT(vec!(set_grip_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));

//     // act gripper predicates, just for keeping track of the cube
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act grip pos predicates
//     let grip_pos_open = Predicate::EQVAL(act_grip_pos.clone(), String::from("open"));
//     let grip_pos_closed = Predicate::EQVAL(act_grip_pos.clone(), String::from("closed"));
//     let grip_pos_cube = Predicate::EQVAL(act_grip_pos.clone(), String::from("cube"));
//     let grip_pos_ball = Predicate::EQVAL(act_grip_pos.clone(), String::from("ball"));
//     let not_grip_pos_open = Predicate::NOT(vec!(grip_pos_open.clone()));
//     let not_grip_pos_closed = Predicate::NOT(vec!(grip_pos_closed.clone()));
//     let not_grip_pos_cube = Predicate::NOT(vec!(grip_pos_cube.clone()));
//     let not_grip_pos_ball = Predicate::NOT(vec!(grip_pos_ball.clone()));
    
//     // ref grip pos predicates
//     let set_grip_pos_open = Predicate::EQVAL(ref_grip_pos.clone(), String::from("open"));
//     let set_grip_pos_closed = Predicate::EQVAL(ref_grip_pos.clone(), String::from("closed"));
//     let set_grip_pos_cube = Predicate::EQVAL(ref_grip_pos.clone(), String::from("cube"));
//     let set_grip_pos_ball = Predicate::EQVAL(ref_grip_pos.clone(), String::from("ball"));
//     let not_set_grip_pos_open = Predicate::NOT(vec!(set_grip_pos_open.clone()));
//     let not_set_grip_pos_closed = Predicate::NOT(vec!(set_grip_pos_closed.clone()));
//     let not_set_grip_pos_cube = Predicate::NOT(vec!(set_grip_pos_cube.clone()));
//     let not_set_grip_pos_ball = Predicate::NOT(vec!(set_grip_pos_ball.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are robot ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // are gripper ref == act predicates
//     let grip_pos_stable = Predicate::EQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let grip_stat_stable = Predicate::EQVAR(act_grip_stat.clone(), ref_grip_stat.clone());
//     let not_grip_pos_stable = Predicate::NEQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let not_grip_stat_stable = Predicate::NEQVAR(act_grip_stat.clone(), ref_grip_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         act_pos.clone(), 
//         ref_pos.clone(), 
//         act_stat.clone(), 
//         ref_stat.clone(),
//         act_grip_pos.clone(),
//         act_grip_stat.clone(),
//         ref_grip_pos.clone(),
//         ref_grip_stat.clone(),
//         buffer.clone(),
//         gripper.clone(),
//         table.clone()
//     );

//     let t1 = ParamTransition::new(
//         "robot_start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "robot_finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "robot_start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "robot_finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     let t15 = ParamTransition::new(
//         "gripper_start_activate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_active.clone()),
//             ("grip_stat", not_set_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone())
//         )
//     );

//     let t16 = ParamTransition::new(
//         "gripper_finish_activate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_stat", not_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone())
//         )
//     );

//     let t17 = ParamTransition::new(
//         "gripper_start_deactivate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", not_set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_idle.clone())
//         )
//     );

//     let t18 = ParamTransition::new(
//         "gripper_finish_deactivate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_idle.clone())
//         )
//     );

//     let t19 = ParamTransition::new(
//         "gripper_start_move_to_closed",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", not_set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_closed.clone())
//         )
//     );

//     let t20 = ParamTransition::new(
//         "gripper_finish_move_to_closed",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_closed.clone())
//         )
//     );

//     let t21 = ParamTransition::new(
//         "gripper_start_move_to_open",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", not_set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_open.clone())
//         )
//     );

//     let t22 = ParamTransition::new(
//         "gripper_finish_move_to_open",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_open.clone())
//         )
//     );

//     let t23 = ParamTransition::new(
//         "gripper_start_move_to_cube",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", not_set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_cube.clone())
//         )
//     );

//     let t24 = ParamTransition::new(
//         "gripper_finish_move_to_cube",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_cube.clone())
//         )
//     );

//     let t25 = ParamTransition::new(
//         "gripper_start_move_to_ball",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", not_set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_ball.clone())
//         )
//     );

//     let t26 = ParamTransition::new(
//         "gripper_finish_move_to_ball",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_ball.clone())
//         )
//     );

//     // 1. have to go through the "home" pose:
//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 2. have to go through the "home" pose:
//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 3. one cube in the system:
//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // 4. gripper can only be open on cube, so...:
//     let s5 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             grip_pos_ball.clone(),
//                             grip_pos_closed.clone(),
//                             // grip_pos_open.clone(),
//                             set_grip_pos_ball.clone(),
//                             set_grip_pos_closed.clone(),
//                             // set_grip_pos_open.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // one operation is executing at a time:
//     let s6 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_grip_stat_stable.clone(),
//                             not_stat_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // cant move to the table of buffer if the cube is not in the gripper and it is not open
//     let s8 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             set_pos_buffer.clone(),
//                             buffer_cube.clone(),
//                             grip_pos_cube.clone()
//                             // not_grip_stat_stable.clone(),
//                             // not_stat_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     let s9 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             set_pos_table.clone(),
//                             table_cube.clone(),
//                             grip_pos_cube.clone()
//                             // not_grip_stat_stable.clone(),
//                             // not_stat_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );


//     // additional constraint to make the plans be the same:
//     // robot has to be active before we activate the gripper... how can we avoid adding this constraint and still have the same plan?
//     let s7 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             grip_stat_active.clone(),
//                             not_stat_active.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_stable.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("grip_pos", grip_pos_open.clone()),
//         ("grip_pos", grip_pos_stable.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("grip_pos", grip_pos_open.clone()),
//         ("grip_pos", grip_pos_stable.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true),
//     ("grip_pos".to_string(), true), ("grip_stat".to_string(), true));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14,
//         t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s6, s7));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         30
//     );

//     let now = Instant::now();

//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     let seq_planning_time = now.elapsed();

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }
// }


// //three home poses, has to go through all. gripper also not active in home2
// #[test]
// fn example_4_sequential(){

//     // a global constraint should be made that changea in the system are one at a time
//     // so for instance, if you're activating a robot and a gripper, you have to finish activating
//     // one first and then actvate another. This maybe makes sense in general
    
//     let pose_domain = vec!("buffer", "home", "home2", "home3", "table");
//     let gripper_pose_domain = vec!("cube", "ball", "closed", "open");
//     let stat_domain = vec!("active", "idle");
//     let gripper_stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group gripper pos
//     let act_grip_pos = Variable::new("act_grip_pos", "grip_pose", gripper_pose_domain.clone());
//     let ref_grip_pos = Variable::new("ref_grip_pos", "grip_pose", gripper_pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group grip stat
//     let act_grip_stat = Variable::new("act_grip_stat", "grip_status", gripper_stat_domain.clone());
//     let ref_grip_stat = Variable::new("ref_grip_stat", "grip_status", gripper_stat_domain.clone());

//     // var group cube, have to think of something better
//     // well ok we have sensors in the gripper beside opening for some reason
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // act grip stat predicates
//     let grip_stat_active = Predicate::EQVAL(act_grip_stat.clone(), String::from("active"));
//     let grip_stat_idle = Predicate::EQVAL(act_grip_stat.clone(), String::from("idle"));
//     let not_grip_stat_active = Predicate::NOT(vec!(grip_stat_active.clone()));
//     let not_grip_stat_idle = Predicate::NOT(vec!(grip_stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // ref grip stat predicates
//     let set_grip_stat_active = Predicate::EQVAL(ref_grip_stat.clone(), String::from("active"));
//     let set_grip_stat_idle = Predicate::EQVAL(ref_grip_stat.clone(), String::from("idle"));
//     let not_set_grip_stat_active = Predicate::NOT(vec!(set_grip_stat_active.clone()));
//     let not_set_grip_stat_idle = Predicate::NOT(vec!(set_grip_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let pos_home2 = Predicate::EQVAL(act_pos.clone(), String::from("home2"));
//     let pos_home3 = Predicate::EQVAL(act_pos.clone(), String::from("home3"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));
//     let not_pos_home2 = Predicate::NOT(vec!(pos_home2.clone()));
//     let not_pos_home3 = Predicate::NOT(vec!(pos_home3.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let set_pos_home2 = Predicate::EQVAL(ref_pos.clone(), String::from("home2"));
//     let set_pos_home3 = Predicate::EQVAL(ref_pos.clone(), String::from("home3"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));
//     let not_set_pos_home2 = Predicate::NOT(vec!(set_pos_home2.clone()));
//     let not_set_pos_home3 = Predicate::NOT(vec!(set_pos_home3.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));

//     // act gripper predicates, just for keeping track of the cube
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act grip pos predicates
//     let grip_pos_open = Predicate::EQVAL(act_grip_pos.clone(), String::from("open"));
//     let grip_pos_closed = Predicate::EQVAL(act_grip_pos.clone(), String::from("closed"));
//     let grip_pos_cube = Predicate::EQVAL(act_grip_pos.clone(), String::from("cube"));
//     let grip_pos_ball = Predicate::EQVAL(act_grip_pos.clone(), String::from("ball"));
//     let not_grip_pos_open = Predicate::NOT(vec!(grip_pos_open.clone()));
//     let not_grip_pos_closed = Predicate::NOT(vec!(grip_pos_closed.clone()));
//     let not_grip_pos_cube = Predicate::NOT(vec!(grip_pos_cube.clone()));
//     let not_grip_pos_ball = Predicate::NOT(vec!(grip_pos_ball.clone()));
    
//     // ref grip pos predicates
//     let set_grip_pos_open = Predicate::EQVAL(ref_grip_pos.clone(), String::from("open"));
//     let set_grip_pos_closed = Predicate::EQVAL(ref_grip_pos.clone(), String::from("closed"));
//     let set_grip_pos_cube = Predicate::EQVAL(ref_grip_pos.clone(), String::from("cube"));
//     let set_grip_pos_ball = Predicate::EQVAL(ref_grip_pos.clone(), String::from("ball"));
//     let not_set_grip_pos_open = Predicate::NOT(vec!(set_grip_pos_open.clone()));
//     let not_set_grip_pos_closed = Predicate::NOT(vec!(set_grip_pos_closed.clone()));
//     let not_set_grip_pos_cube = Predicate::NOT(vec!(set_grip_pos_cube.clone()));
//     let not_set_grip_pos_ball = Predicate::NOT(vec!(set_grip_pos_ball.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are robot ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // are gripper ref == act predicates
//     let grip_pos_stable = Predicate::EQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let grip_stat_stable = Predicate::EQVAR(act_grip_stat.clone(), ref_grip_stat.clone());
//     let not_grip_pos_stable = Predicate::NEQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let not_grip_stat_stable = Predicate::NEQVAR(act_grip_stat.clone(), ref_grip_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         act_pos.clone(), 
//         ref_pos.clone(), 
//         act_stat.clone(), 
//         ref_stat.clone(),
//         act_grip_pos.clone(),
//         act_grip_stat.clone(),
//         ref_grip_pos.clone(),
//         ref_grip_stat.clone(),
//         buffer.clone(),
//         gripper.clone(),
//         table.clone()
//     );

//     let t1 = ParamTransition::new(
//         "robot_start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "robot_finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "robot_start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "robot_finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     let t15 = ParamTransition::new(
//         "gripper_start_activate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_active.clone()),
//             ("grip_stat", not_set_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone())
//         )
//     );

//     let t16 = ParamTransition::new(
//         "gripper_finish_activate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_stat", not_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone())
//         )
//     );

//     let t17 = ParamTransition::new(
//         "gripper_start_deactivate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", not_set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_idle.clone())
//         )
//     );

//     let t18 = ParamTransition::new(
//         "gripper_finish_deactivate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_idle.clone())
//         )
//     );

//     let t19 = ParamTransition::new(
//         "gripper_start_move_to_closed",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", not_set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_closed.clone())
//         )
//     );

//     let t20 = ParamTransition::new(
//         "gripper_finish_move_to_closed",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_closed.clone())
//         )
//     );

//     let t21 = ParamTransition::new(
//         "gripper_start_move_to_open",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", not_set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_open.clone())
//         )
//     );

//     let t22 = ParamTransition::new(
//         "gripper_finish_move_to_open",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_open.clone())
//         )
//     );

//     let t23 = ParamTransition::new(
//         "gripper_start_move_to_cube",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", not_set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_cube.clone())
//         )
//     );

//     let t24 = ParamTransition::new(
//         "gripper_finish_move_to_cube",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_cube.clone())
//         )
//     );

//     let t25 = ParamTransition::new(
//         "gripper_start_move_to_ball",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", not_set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_ball.clone())
//         )
//     );

//     let t26 = ParamTransition::new(
//         "gripper_finish_move_to_ball",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_ball.clone())
//         )
//     );

//     let t27 = ParamTransition::new(
//         "start_move_to_home2",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home2.clone()),
//             ("pos", not_set_pos_home2.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home2.clone())
//         )
//     );

//     let t28 = ParamTransition::new(
//         "finish_move_to_home2",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home2.clone()),
//             ("pos", set_pos_home2.clone())
//         ),
//         &vec!(
//             ("pos", pos_home2.clone())
//         )
//     );

//     let t29 = ParamTransition::new(
//         "start_move_to_home3",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home3.clone()),
//             ("pos", not_set_pos_home3.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home3.clone())
//         )
//     );

//     let t30 = ParamTransition::new(
//         "finish_move_to_home3",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home3.clone()),
//             ("pos", set_pos_home3.clone())
//         ),
//         &vec!(
//             ("pos", pos_home3.clone())
//         )
//     );

//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home2")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home3")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("home"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home3")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("table")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("home3"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("table")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s5 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("table")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     // // 1. have to go through the "home" pose:
//     // let s1 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s2 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s3 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s4 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s5 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );
    
//     // // 2. have to go through the "home" pose:
//     // let s6 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );
    
//     // 3. one cube in the system:
//     let s7 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s8 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // 4. gripper can only be open on cube, so...:
//     let s9 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             grip_pos_ball.clone(),
//                             grip_pos_closed.clone(),
//                             // grip_pos_open.clone(),
//                             set_grip_pos_ball.clone(),
//                             set_grip_pos_closed.clone(),
//                             // set_grip_pos_open.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // one operation is executing at a time:
//     let s10 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_grip_stat_stable.clone(),
//                             not_stat_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // cant move to the table of buffer if the cube is not in the gripper and it is not open
//     // let s8 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AND(
//     //                     vec!(
//     //                         set_pos_buffer.clone(),
//     //                         buffer_cube.clone(),
//     //                         grip_pos_cube.clone()
//     //                         // not_grip_stat_stable.clone(),
//     //                         // not_stat_stable.clone()
//     //                     )
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s9 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AND(
//     //                     vec!(
//     //                         set_pos_table.clone(),
//     //                         table_cube.clone(),
//     //                         grip_pos_cube.clone()
//     //                         // not_grip_stat_stable.clone(),
//     //                         // not_stat_stable.clone()
//     //                     )
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );


//     // additional constraint to make the plans be the same:
//     // robot has to be active before we activate the gripper... how can we avoid adding this constraint and still have the same plan?
//     let s11 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             grip_stat_active.clone(),
//                             not_stat_active.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // need another one, namely not to move gripper until moving:
//     let s12 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_grip_stat_stable.clone(),
//                             not_pos_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // gripper not active in "home2"
//     let s13 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             pos_home2.clone(),
//                             grip_stat_active.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );


//     // let s11
    
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_stable.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("grip_pos", grip_pos_open.clone()),
//         ("grip_pos", grip_pos_stable.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("grip_pos", grip_pos_open.clone()),
//         ("grip_pos", grip_pos_stable.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true),
//     ("grip_pos".to_string(), true), ("grip_stat".to_string(), true));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14,
//         t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s7, s8, s9, s10, s11, s12));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         50
//     );

//     let now = Instant::now();

//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     let seq_planning_time = now.elapsed();

//     println!("level: {:?}", result.level);
//     println!("concat: {:?}", result.concat);
//     println!("plan_found: {:?}", result.plan_found);
//     println!("plan_lenght: {:?}", result.plan_length);
//     println!("time_to_solve: {:?}", result.time_to_solve);
//     println!("trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }
// }

// // three home poses, has to go through all. in home2 the gripper has to be deactivated
// #[test]
// fn example_4_compositional(){

//     // a global constraint should be made that changea in the system are one at a time
//     // so for instance, if you're activating a robot and a gripper, you have to finish activating
//     // one first and then actvate another. This maybe makes sense in general
    
//     let pose_domain = vec!("buffer", "home", "home2", "home3", "table");
//     let gripper_pose_domain = vec!("cube", "ball", "closed", "open");
//     let stat_domain = vec!("active", "idle");
//     let gripper_stat_domain = vec!("active", "idle");
//     let buffer_domain = vec!("cube", "ball", "empty");
//     let gripper_domain = vec!("cube", "ball", "empty");
//     let table_domain = vec!("cube", "ball", "empty");

//     // var group pos
//     let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
//     let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

//     // var group gripper pos
//     let act_grip_pos = Variable::new("act_grip_pos", "grip_pose", gripper_pose_domain.clone());
//     let ref_grip_pos = Variable::new("ref_grip_pos", "grip_pose", gripper_pose_domain.clone());

//     // var group stat
//     let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
//     let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

//     // var group grip stat
//     let act_grip_stat = Variable::new("act_grip_stat", "grip_status", gripper_stat_domain.clone());
//     let ref_grip_stat = Variable::new("ref_grip_stat", "grip_status", gripper_stat_domain.clone());

//     // var group cube, have to think of something better
//     // well ok we have sensors in the gripper beside opening for some reason
//     let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
//     let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
//     let table = Variable::new("table_cube", "table", table_domain.clone());

//     // act stat predicates
//     let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
//     let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
//     let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
//     let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

//     // act grip stat predicates
//     let grip_stat_active = Predicate::EQVAL(act_grip_stat.clone(), String::from("active"));
//     let grip_stat_idle = Predicate::EQVAL(act_grip_stat.clone(), String::from("idle"));
//     let not_grip_stat_active = Predicate::NOT(vec!(grip_stat_active.clone()));
//     let not_grip_stat_idle = Predicate::NOT(vec!(grip_stat_idle.clone()));

//     // ref stat predicates
//     let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
//     let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
//     let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
//     let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

//     // ref grip stat predicates
//     let set_grip_stat_active = Predicate::EQVAL(ref_grip_stat.clone(), String::from("active"));
//     let set_grip_stat_idle = Predicate::EQVAL(ref_grip_stat.clone(), String::from("idle"));
//     let not_set_grip_stat_active = Predicate::NOT(vec!(set_grip_stat_active.clone()));
//     let not_set_grip_stat_idle = Predicate::NOT(vec!(set_grip_stat_idle.clone()));

//     // act pos predicates
//     let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
//     let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
//     let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
//     let pos_home2 = Predicate::EQVAL(act_pos.clone(), String::from("home2"));
//     let pos_home3 = Predicate::EQVAL(act_pos.clone(), String::from("home3"));
//     let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
//     let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
//     let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));
//     let not_pos_home2 = Predicate::NOT(vec!(pos_home2.clone()));
//     let not_pos_home3 = Predicate::NOT(vec!(pos_home3.clone()));

//     // ref pos predicates
//     let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
//     let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
//     let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
//     let set_pos_home2 = Predicate::EQVAL(ref_pos.clone(), String::from("home2"));
//     let set_pos_home3 = Predicate::EQVAL(ref_pos.clone(), String::from("home3"));
//     let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
//     let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
//     let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));
//     let not_set_pos_home2 = Predicate::NOT(vec!(set_pos_home2.clone()));
//     let not_set_pos_home3 = Predicate::NOT(vec!(set_pos_home3.clone()));

//     // act buffer predicates
//     let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
//     let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
//     let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
//     let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
//     let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
//     let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));

//     // act gripper predicates, just for keeping track of the cube
//     let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
//     let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
//     let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
//     let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
//     let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
//     let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

//     // act grip pos predicates
//     let grip_pos_open = Predicate::EQVAL(act_grip_pos.clone(), String::from("open"));
//     let grip_pos_closed = Predicate::EQVAL(act_grip_pos.clone(), String::from("closed"));
//     let grip_pos_cube = Predicate::EQVAL(act_grip_pos.clone(), String::from("cube"));
//     let grip_pos_ball = Predicate::EQVAL(act_grip_pos.clone(), String::from("ball"));
//     let not_grip_pos_open = Predicate::NOT(vec!(grip_pos_open.clone()));
//     let not_grip_pos_closed = Predicate::NOT(vec!(grip_pos_closed.clone()));
//     let not_grip_pos_cube = Predicate::NOT(vec!(grip_pos_cube.clone()));
//     let not_grip_pos_ball = Predicate::NOT(vec!(grip_pos_ball.clone()));
    
//     // ref grip pos predicates
//     let set_grip_pos_open = Predicate::EQVAL(ref_grip_pos.clone(), String::from("open"));
//     let set_grip_pos_closed = Predicate::EQVAL(ref_grip_pos.clone(), String::from("closed"));
//     let set_grip_pos_cube = Predicate::EQVAL(ref_grip_pos.clone(), String::from("cube"));
//     let set_grip_pos_ball = Predicate::EQVAL(ref_grip_pos.clone(), String::from("ball"));
//     let not_set_grip_pos_open = Predicate::NOT(vec!(set_grip_pos_open.clone()));
//     let not_set_grip_pos_closed = Predicate::NOT(vec!(set_grip_pos_closed.clone()));
//     let not_set_grip_pos_cube = Predicate::NOT(vec!(set_grip_pos_cube.clone()));
//     let not_set_grip_pos_ball = Predicate::NOT(vec!(set_grip_pos_ball.clone()));

//     // act table predicates
//     let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
//     let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
//     let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
//     let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
//     let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
//     let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

//     // are robot ref == act predicates
//     let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
//     let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
//     let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
//     let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

//     // are gripper ref == act predicates
//     let grip_pos_stable = Predicate::EQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let grip_stat_stable = Predicate::EQVAR(act_grip_stat.clone(), ref_grip_stat.clone());
//     let not_grip_pos_stable = Predicate::NEQVAR(act_grip_pos.clone(), ref_grip_pos.clone());
//     let not_grip_stat_stable = Predicate::NEQVAR(act_grip_stat.clone(), ref_grip_stat.clone());

//     // variables in the problem
//     let all_vars = vec!(
//         act_pos.clone(), 
//         ref_pos.clone(), 
//         act_stat.clone(), 
//         ref_stat.clone(),
//         act_grip_pos.clone(),
//         act_grip_stat.clone(),
//         ref_grip_pos.clone(),
//         ref_grip_stat.clone(),
//         buffer.clone(),
//         gripper.clone(),
//         table.clone()
//     );

//     let t1 = ParamTransition::new(
//         "robot_start_activate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_active.clone()),
//             ("stat", not_set_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_active.clone())
//         )
//     );

//     let t2 = ParamTransition::new(
//         "robot_finish_activate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", set_stat_active.clone()),
//             ("stat", not_stat_active.clone())
//         ),
//         &vec!(
//             ("stat", stat_active.clone())
//         )
//     );

//     let t3 = ParamTransition::new(
//         "robot_start_deactivate",
//         vec!(ref_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", not_set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", set_stat_idle.clone())
//         )
//     );

//     let t4 = ParamTransition::new(
//         "robot_finish_deactivate",
//         vec!(act_stat.clone()),
//         &vec!(
//             ("stat", not_stat_idle.clone()),
//             ("stat", set_stat_idle.clone())
//         ),
//         &vec!(
//             ("stat", stat_idle.clone())
//         )
//     );

//     let t5 = ParamTransition::new(
//         "start_move_to_buffer",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", not_set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_buffer.clone())
//         )
//     );

//     let t6 = ParamTransition::new(
//         "finish_move_to_buffer",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone())
//         ),
//         &vec!(
//             ("pos", pos_buffer.clone())
//         )
//     );

//     let t7 = ParamTransition::new(
//         "start_move_to_table",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", not_set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_table.clone())
//         )
//     );

//     let t8 = ParamTransition::new(
//         "finish_move_to_table",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_table.clone()),
//             ("pos", set_pos_table.clone())
//         ),
//         &vec!(
//             ("pos", pos_table.clone())
//         )
//     );

//     let t9 = ParamTransition::new(
//         "start_move_to_home",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", not_set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home.clone())
//         )
//     );

//     let t10 = ParamTransition::new(
//         "finish_move_to_home",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home.clone()),
//             ("pos", set_pos_home.clone())
//         ),
//         &vec!(
//             ("pos", pos_home.clone())
//         )
//     );

//     let t11 = ParamTransition::new(
//         "take_cube_from_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", buffer_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t12 = ParamTransition::new(
//         "take_cube_from_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", table_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", gripper_cube.clone())
//         )
//     );

//     let t13 = ParamTransition::new(
//         "leave_cube_at_buffer",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_buffer.clone()),
//             ("pos", set_pos_buffer.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", buffer_cube.clone())
//         )
//     );

//     let t14 = ParamTransition::new(
//         "leave_cube_at_table",
//         vec!(gripper.clone(), buffer.clone(), table.clone()),
//         &vec!(
//             ("cube", gripper_cube.clone()),
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_table.clone()),
//             ("pos", set_pos_table.clone()),
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone()),
//         ),
//         &vec!(
//             ("cube", table_cube.clone())
//         )
//     );

//     let t15 = ParamTransition::new(
//         "gripper_start_activate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_active.clone()),
//             ("grip_stat", not_set_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone())
//         )
//     );

//     let t16 = ParamTransition::new(
//         "gripper_finish_activate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_stat", not_grip_stat_active.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone())
//         )
//     );

//     let t17 = ParamTransition::new(
//         "gripper_start_deactivate",
//         vec!(ref_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", not_set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", set_grip_stat_idle.clone())
//         )
//     );

//     let t18 = ParamTransition::new(
//         "gripper_finish_deactivate",
//         vec!(act_grip_stat.clone()),
//         &vec!(
//             ("grip_stat", not_grip_stat_idle.clone()),
//             ("grip_stat", set_grip_stat_idle.clone())
//         ),
//         &vec!(
//             ("grip_stat", grip_stat_idle.clone())
//         )
//     );

//     let t19 = ParamTransition::new(
//         "gripper_start_move_to_closed",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", not_set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_closed.clone())
//         )
//     );

//     let t20 = ParamTransition::new(
//         "gripper_finish_move_to_closed",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_closed.clone()),
//             ("grip_pos", set_grip_pos_closed.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_closed.clone())
//         )
//     );

//     let t21 = ParamTransition::new(
//         "gripper_start_move_to_open",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", not_set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_open.clone())
//         )
//     );

//     let t22 = ParamTransition::new(
//         "gripper_finish_move_to_open",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_open.clone()),
//             ("grip_pos", set_grip_pos_open.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_open.clone())
//         )
//     );

//     let t23 = ParamTransition::new(
//         "gripper_start_move_to_cube",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", not_set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_cube.clone())
//         )
//     );

//     let t24 = ParamTransition::new(
//         "gripper_finish_move_to_cube",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_cube.clone()),
//             ("grip_pos", set_grip_pos_cube.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_cube.clone())
//         )
//     );

//     let t25 = ParamTransition::new(
//         "gripper_start_move_to_ball",
//         vec!(ref_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", grip_pos_stable.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", not_set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", set_grip_pos_ball.clone())
//         )
//     );

//     let t26 = ParamTransition::new(
//         "gripper_finish_move_to_ball",
//         vec!(act_grip_pos.clone()),
//         &vec!(
//             ("grip_stat", grip_stat_active.clone()),
//             ("grip_stat", set_grip_stat_active.clone()),
//             ("grip_pos", not_grip_pos_ball.clone()),
//             ("grip_pos", set_grip_pos_ball.clone())
//         ),
//         &vec!(
//             ("grip_pos", grip_pos_ball.clone())
//         )
//     );

//     let t27 = ParamTransition::new(
//         "start_move_to_home2",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home2.clone()),
//             ("pos", not_set_pos_home2.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home2.clone())
//         )
//     );

//     let t28 = ParamTransition::new(
//         "finish_move_to_home2",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home2.clone()),
//             ("pos", set_pos_home2.clone())
//         ),
//         &vec!(
//             ("pos", pos_home2.clone())
//         )
//     );

//     let t29 = ParamTransition::new(
//         "start_move_to_home3",
//         vec!(ref_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", pos_stable.clone()),
//             ("pos", not_pos_home3.clone()),
//             ("pos", not_set_pos_home3.clone())
//         ),
//         &vec!(
//             ("pos", set_pos_home3.clone())
//         )
//     );

//     let t30 = ParamTransition::new(
//         "finish_move_to_home3",
//         vec!(act_pos.clone()),
//         &vec!(
//             ("stat", stat_active.clone()),
//             ("stat", set_stat_active.clone()),
//             ("pos", not_pos_home3.clone()),
//             ("pos", set_pos_home3.clone())
//         ),
//         &vec!(
//             ("pos", pos_home3.clone())
//         )
//     );

//     // // 1. have to go through the "home" pose:
//     // let s1 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s2 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s3 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s4 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s5 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );
    
//     // // 2. have to go through the "home" pose:
//     // let s6 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AFTER(
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//     //                     ),
//     //                     vec!(
//     //                         Predicate::EQVAL(act_pos.clone(), String::from("table"))
//     //                     ),
//     //                     2 // how can this be improved so that the plan also holds for bigger a number
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     let s1 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("table"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home2")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home3")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s2 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("home"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home3")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s3 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("table")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s4 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("home3"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("table")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );

//     let s5 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AFTER(
//                         vec!(
//                             Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
//                         ),
//                         vec!(
//                             Predicate::OR(
//                                 vec!(
//                                     Predicate::EQVAL(act_pos.clone(), String::from("table")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home")),
//                                     Predicate::EQVAL(act_pos.clone(), String::from("home2"))
//                                 )   
//                             )
//                         ),
//                         2 // how can this be improved so that the plan also holds for bigger a number
//                     )
//                 )
//             )
//         )
//     );
    
//     // 3. one cube in the system:
//     let s7 = Predicate::GLOB(
//         vec!(
//             Predicate::PBEQ(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             buffer_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             table_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     gripper_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     ),
//                     Predicate::AND(
//                         vec!(
//                             gripper_cube.clone(), 
//                             Predicate::NOT(
//                                 vec!(
//                                     table_cube.clone()
//                                 )
//                             ), 
//                             Predicate::NOT(
//                                 vec!(
//                                     buffer_cube.clone()
//                                 )
//                             )
//                         )
//                     )
//                 ),
//                 1
//             )
//         )
//     );
    
//     // 4. no ball in the system:
//     let s8 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             buffer_ball.clone(),
//                             table_ball.clone(),
//                             gripper_ball.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // 4. gripper can only be open on cube, so...:
//     let s9 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::OR(
//                         vec!(
//                             grip_pos_ball.clone(),
//                             grip_pos_closed.clone(),
//                             // grip_pos_open.clone(),
//                             set_grip_pos_ball.clone(),
//                             set_grip_pos_closed.clone(),
//                             // set_grip_pos_open.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // one operation is executing at a time:
//     let s10 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_grip_stat_stable.clone(),
//                             not_stat_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // cant move to the table of buffer if the cube is not in the gripper and it is not open
//     // let s8 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AND(
//     //                     vec!(
//     //                         set_pos_buffer.clone(),
//     //                         buffer_cube.clone(),
//     //                         grip_pos_cube.clone()
//     //                         // not_grip_stat_stable.clone(),
//     //                         // not_stat_stable.clone()
//     //                     )
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );

//     // let s9 = Predicate::GLOB(
//     //     vec!(
//     //         Predicate::NOT(
//     //             vec!(
//     //                 Predicate::AND(
//     //                     vec!(
//     //                         set_pos_table.clone(),
//     //                         table_cube.clone(),
//     //                         grip_pos_cube.clone()
//     //                         // not_grip_stat_stable.clone(),
//     //                         // not_stat_stable.clone()
//     //                     )
//     //                 )
//     //             )
//     //         )
//     //     )
//     // );


//     // additional constraint to make the plans be the same:
//     // robot has to be active before we activate the gripper... how can we avoid adding this constraint and still have the same plan?
//     let s11 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             grip_stat_active.clone(),
//                             not_stat_active.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
//     // need another one, namely not to move gripper until moving:
//     let s12 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             not_grip_stat_stable.clone(),
//                             not_pos_stable.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );

//     // gripper not active in "home2"
//     let s13 = Predicate::GLOB(
//         vec!(
//             Predicate::NOT(
//                 vec!(
//                     Predicate::AND(
//                         vec!(
//                             pos_home2.clone(),
//                             grip_stat_active.clone()
//                         )
//                     )
//                 )
//             )
//         )
//     );
    
//     let initial = vec!(
//         ("pos", pos_stable.clone()),
//         ("pos", pos_buffer.clone()),
//         ("stat", stat_stable.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_stable.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("grip_pos", grip_pos_open.clone()),
//         ("grip_pos", grip_pos_stable.clone()),
//         ("cube", table_cube.clone())
//     );

//     let goal = vec!(
//         ("pos", pos_table.clone()),
//         ("stat", stat_idle.clone()),
//         ("grip_stat", grip_stat_idle.clone()),
//         ("grip_pos", grip_pos_open.clone()),
//         ("grip_pos", grip_pos_stable.clone()),
//         ("cube", buffer_cube.clone())
//     );
    
//     // let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true),
//     // ("grip_pos".to_string(), true), ("grip_stat".to_string(), true));
//     // let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14,
//     //     t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30);
//     // let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s6, s7, s8, s9, s10));

//     // let mut concat: u32 = 0;
//     // let mut level: u32 = 0;

//     // let problem = ParamPlanningProblem::new(
//     //     String::from("param_prob_1"), 
//     //     all_vars.clone(),
//     //     act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//     //     initial.clone(),
//     //     goal.clone(),
//     //     trans.clone(), 
//     //     specs.clone(),
//     //     40
//     // );

//     // let now = Instant::now();

//     // let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     // let seq_planning_time = now.elapsed();

//     // println!("level: {:?}", result.level);
//     // println!("concat: {:?}", result.concat);
//     // println!("plan_found: {:?}", result.plan_found);
//     // println!("plan_lenght: {:?}", result.plan_length);
//     // println!("time_to_solve: {:?}", result.time_to_solve);
//     // println!("trace: ");

//     // for t in &result.trace{
 
//     //     println!("state: {:?}", t.state);
//     //     println!("trans: {:?}", t.trans);
//     //     println!("=========================");
//     // }

//     // ===========================

//     let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), false), ("cube".to_string(), false),
//     ("grip_pos".to_string(), true), ("grip_stat".to_string(), false));
//     let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14,
//         t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30);
//     let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s7, s8, s9, s10, s11, s12));

//     let mut concat: u32 = 0;
//     let mut level: u32 = 0;

//     let problem = ParamPlanningProblem::new(
//         String::from("param_prob_1"), 
//         all_vars.clone(),
//         act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
//         initial.clone(),
//         goal.clone(),
//         trans.clone(), 
//         specs.clone(),
//         100
//     );


//     // let mut act: Vec<(String, bool)> = vec!(("r1_pos".to_string(), true), ("r2_pos".to_string(), false));
//     let refining_order: Vec<&str> = vec!("pos", "stat", "cube", "grip_stat", "grip_pos"); // opposite for some reason? fix this
//     let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

//     println!("comp_level: {:?}", result.level);
//     println!("comp_concat: {:?}", result.concat);
//     println!("comp_plan_found: {:?}", result.plan_found);
//     println!("comp_plan_lenght: {:?}", result.plan_length);
//     println!("comp_time_to_solve: {:?}", result.time_to_solve);
//     println!("comp_trace: ");

//     for t in &result.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }
 
//     let solution = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

//     println!("comp_level: {:?}", solution.level);
//     println!("comp_concat: {:?}", solution.concat);
//     println!("comp_plan_found: {:?}", solution.plan_found);
//     println!("comp_plan_lenght: {:?}", solution.plan_length);
//     println!("comp_time_to_solve: {:?}", solution.time_to_solve);
//     println!("comp_trace: ");

//     for t in &solution.trace{
 
//         println!("state: {:?}", t.state);
//         println!("trans: {:?}", t.trans);
//         println!("=========================");
//     }



// }