//! Z3 sorts for SP

use std::ffi::{CStr, CString};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use z3_sys::*;
use super::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Variable {
    n: String,    
    t: String,     
    d: Vec<String>  
}

#[derive(PartialEq, Clone, Debug)]
pub struct Transition {
    n: String,
    v: Vec<Variable>,
    g: Predicate,
    u: Predicate
}

#[derive(Debug)]
pub struct PlanningProblem {
    name: String,
    vars: Vec<Variable>,
    initial: Predicate,
    goal: Predicate,
    trans: Vec<Transition>,
    ltl_specs: Predicate,
    max_steps: u32
}

#[derive(Debug)]
pub struct PlanningFrame {
    state: Vec<String>,
    trans: String,
}

pub struct GetSPPlanningResultZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub model: Z3_model,
    pub nr_steps: u32,
    pub frames: PlanningResult
}

pub struct PlanningResult {
    pub plan_found: bool,
    pub plan_length: u32,
    pub trace: Vec<PlanningFrame>,
    pub time_to_solve: std::time::Duration,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Predicate {
    AND(Vec<Predicate>),
    OR(Vec<Predicate>),
    NOT(Vec<Predicate>),
    EQVAL(Variable, String),
    NEQVAL(Variable, String),
    EQVAR(Variable, Variable),
    NEQVAR(Variable, Variable),
    NEXT(Vec<Predicate>, Vec<Predicate>),
    AFTER(Vec<Predicate>, Vec<Predicate>, u32),
    GLOB(Vec<Predicate>),
    PBEQ(Vec<Predicate>, i32),
    TRUE,
    FALSE
}

pub struct KeepVariableValues<'ctx> {
    pub ctx: &'ctx ContextZ3
}

pub struct PredicateToAstZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub pred: Predicate,
    pub step: u32,
    pub r: Z3_ast
}

pub struct UpdatePredicateToAstZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub pred: Predicate,
    pub step: u32,
    pub r: Z3_ast
}

pub struct AssignmentToAstZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub init: Predicate,
    pub step: u32,
    pub r: Z3_ast
}

pub struct Sequential {}

pub struct Sequential2 {}

pub struct GenerateProblems {}

pub struct StateToPredicate {}

pub struct LowLevelSequential<'ctx> {
pub ctx: &'ctx ContextZ3
}

pub struct Compositional {}

pub struct Abstract<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub r: Z3_ast
}

pub struct AbstractHard<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub r: Z3_ast
}

pub struct Refine {}

pub trait IterOps<T, I>: IntoIterator<Item = T>
    where I: IntoIterator<Item = T>,
          T: PartialEq {
    fn intersect(self, other: I) -> Vec<T>;
    fn difference(self, other: I) -> Vec<T>;
}

impl<T, I> IterOps<T, I> for I
    where I: IntoIterator<Item = T>,
          T: PartialEq
{
    fn intersect(self, other: I) -> Vec<T> {
        let mut common = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                common.push(e1);
                v_other.remove(pos);
            }
        }

        common
    }

    fn difference(self, other: I) -> Vec<T> {
        let mut diff = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                v_other.remove(pos);
            } else {
                diff.push(e1);
            }
        }

        diff.append(&mut v_other);
        diff
    }
}

impl Variable {
    /// Creates a new Variable
    pub fn new(n: &str, t: &str, d: Vec<&str>) -> Variable {
        Variable { n: n.to_string(), 
                   t: t.to_string(),
                   d: d.iter().map(|x| x.to_string()).collect::<Vec<String>>()}
    }
}

impl Transition {
    pub fn new(n: &str, v: Vec<Variable>, g: &Predicate, u: &Predicate) -> Transition {
        Transition { n: n.to_string(),
                     v: v.clone(),
                     g: g.clone(),
                     u: u.clone() }
    }
}

impl PlanningProblem {
    pub fn new(name: String,
               vars: Vec<Variable>,
               initial: Predicate,
               goal: Predicate,
               trans: Vec<Transition>,
               ltl_specs: Predicate,
               max_steps: u32) -> PlanningProblem {
        PlanningProblem {
            name: name.to_string(),
            vars: vars,
            initial: initial,
            goal: goal,
            trans: trans,
            ltl_specs: ltl_specs,
            max_steps: max_steps
        }
    }
}

impl PlanningFrame {
    pub fn new(state: Vec<&str>, trans: &str) -> PlanningFrame {
        PlanningFrame {
            state: state.iter().map(|x| x.to_string()).collect(),
            trans: trans.to_string()
        }
    }
}

impl <'ctx> PredicateToAstZ3<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, pred: &Predicate, step: u32) -> Z3_ast {
        match pred {
            Predicate::TRUE => BoolZ3::new(&ctx, true),
            Predicate::FALSE => BoolZ3::new(&ctx, false),
            // Predicate::NOT(p) => NOTZ3::new(&ctx, PredicateToAstZ3::new(&ctx, p, step)),
            Predicate::NOT(p) => ANDZ3::new(&ctx, p.iter().map(|x| NOTZ3::new(&ctx, PredicateToAstZ3::new(&ctx, x, step))).collect()),
            Predicate::AND(p) => ANDZ3::new(&ctx, p.iter().map(|x| PredicateToAstZ3::new(&ctx, x, step)).collect()),
            Predicate::OR(p) => ORZ3::new(&ctx, p.iter().map(|x| PredicateToAstZ3::new(&ctx, x, step)).collect()),
            Predicate::EQVAL(x, y) => {
                let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let elems = &sort.enum_asts;
                let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
                EQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
            },
            Predicate::EQVAR(x, y) => {
                // TODO: check sorts before passing to Z3
                let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step).as_str());
                EQZ3::new(&ctx, v_1, v_2)
            },
            Predicate::NEQVAL(x, y) => {
                let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let elems = &sort.enum_asts;
                let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
                NEQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
            },
            Predicate::NEQVAR(x, y) => {
                // TODO: check sorts before passing to Z3
                let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step).as_str());
                NEQZ3::new(&ctx, v_1, v_2)
            },
            Predicate::NEXT(x, y) => NextZ3::new(&ctx, &x, &y, step),
            Predicate::AFTER(x, y, m) => AfterZ3::new(&ctx, &x, &y, *m, step),
            Predicate::GLOB(x) => GloballyZ3::new(&ctx, &x, step),
            Predicate::PBEQ(x, k) => PBEQZ3::new(&ctx, x.iter().map(|z| PredicateToAstZ3::new(&ctx, z, step)).collect(), *k)
        }
    }
}

impl <'ctx> UpdatePredicateToAstZ3<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, pred: &Predicate, step: u32) -> Z3_ast {
        match pred {
            Predicate::TRUE => BoolZ3::new(&ctx, true),
            Predicate::FALSE => BoolZ3::new(&ctx, false),
            Predicate::NOT(p) => ANDZ3::new(&ctx, p.iter().map(|x| NOTZ3::new(&ctx, UpdatePredicateToAstZ3::new(&ctx, x, step))).collect()),
            Predicate::AND(p) => ANDZ3::new(&ctx, p.iter().map(|x| UpdatePredicateToAstZ3::new(&ctx, x, step)).collect()),
            Predicate::OR(p) => ORZ3::new(&ctx, p.iter().map(|x| UpdatePredicateToAstZ3::new(&ctx, x, step)).collect()),
            Predicate::EQVAL(x, y) => {
                let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let elems = &sort.enum_asts;
                let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
                EQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
            },
            Predicate::EQVAR(x, y) => {
                // TODO: check sorts before passing to Z3
                let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step - 1).as_str());
                EQZ3::new(&ctx, v_1, v_2)
            },
            Predicate::NEQVAL(x, y) => {
                let sort = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let elems = &sort.enum_asts;
                let index = x.d.iter().position(|r| *r == y.to_string()).unwrap();
                NEQZ3::new(&ctx, EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", x.n.to_string(), step).as_str()), elems[index])
            },
            Predicate::NEQVAR(x, y) => {
                // TODO: check sorts before passing to Z3
                let sort_1 = EnumSortZ3::new(&ctx, &x.t, x.d.iter().map(|x| x.as_str()).collect());
                let sort_2 = EnumSortZ3::new(&ctx, &y.t, y.d.iter().map(|y| y.as_str()).collect());
                let v_1 = EnumVarZ3::new(&ctx, sort_1.r, format!("{}_s{}", x.n.to_string(), step).as_str());
                let v_2 = EnumVarZ3::new(&ctx, sort_2.r, format!("{}_s{}", y.n.to_string(), step - 1).as_str());
                NEQZ3::new(&ctx, v_1, v_2)
            },
            _ => panic!("implement")
        }
    }
}

impl <'ctx> KeepVariableValues<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, vars: &Vec<Variable>, trans: &Transition, step: u32) -> Z3_ast {

        let unchanged = IterOps::difference(vars, &trans.v);
        let mut assert_vec = vec!();
        for u in unchanged {
            let sort = EnumSortZ3::new(&ctx, &u.t, u.d.iter().map(|x| x.as_str()).collect());
            let v_1 = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", u.n.to_string(), step).as_str());
            let v_2 = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", u.n.to_string(), step - 1).as_str());
            assert_vec.push(EQZ3::new(&ctx, v_1, v_2));
        }
        ANDZ3::new(&ctx, assert_vec)
    }
}

impl Sequential {
    pub fn new(p: &PlanningProblem, vars: &Vec<Variable>) -> PlanningResult {

        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);
    
        slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.initial, 0)));

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.ltl_specs, 0)));
        slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.goal, 0)));

        let now = Instant::now();
        let mut plan_found: bool = false;

        let mut step: u32 = 0;

        while step < p.max_steps + 1 {
            step = step + 1;
            if SlvCheckZ3::new(&ctx, &slv) != 1 {
                SlvPopZ3::new(&ctx, &slv, 1);

                let mut all_trans = vec!();
                for t in &p.trans {
                    let name = format!("{}_t{}", &t.n, step);
                    let guard = Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &t.g, step - 1));
                    let updates = Abstract::new(&ctx, &vars, UpdatePredicateToAstZ3::new(&ctx, &t.u, step));
                    let keeps = Abstract::new(&ctx, &vars, KeepVariableValues::new(&ctx, &p.vars, t, step));

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, updates, keeps)));
                }

                slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
                SlvPushZ3::new(&ctx, &slv);
                slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step)));
                // slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
                slv_assert_z3!(&ctx, &slv, Abstract::new(&ctx, &vars, PredicateToAstZ3::new(&ctx, &p.goal, step)));
        
            } else {
                plan_found = true;
                break;
            }
        }

        let planning_time = now.elapsed();

        let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
        let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
        let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
        // for a in cnf {
        //     println!("{}", ast_to_string_z3!(&ctx, a))
        // }
        
        if plan_found == true {
            let model = SlvGetModelZ3::new(&ctx, &slv);
            let result = GetSPPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
            result
        } else {
            let model = FreshModelZ3::new(&ctx);
            let result = GetSPPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
            result
        }              
    }   
}

impl Sequential2 {
    pub fn new(p: &PlanningProblem, vars: &Vec<Variable>) -> PlanningResult {

        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);
    
        let rem_v =  IterOps::difference(p.vars.clone(), vars.clone());

        slv_assert_z3!(&ctx, &slv, AbstractHard::new(&ctx, &rem_v, PredicateToAstZ3::new(&ctx, &p.initial, 0)));

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        slv_assert_z3!(&ctx, &slv, AbstractHard::new(&ctx, &rem_v, PredicateToAstZ3::new(&ctx, &p.goal, 0)));

        let now = Instant::now();
        let mut plan_found: bool = false;

        let mut step: u32 = 0;

        while step < p.max_steps + 1 {
            step = step + 1;
            if SlvCheckZ3::new(&ctx, &slv) != 1 {
                SlvPopZ3::new(&ctx, &slv, 1);

                let mut all_trans = vec!();
                for t in &p.trans {
                    let name = format!("{}_t{}", &t.n, step);
                    let guard = AbstractHard::new(&ctx, &rem_v, PredicateToAstZ3::new(&ctx, &t.g, step - 1));
                    let updates = AbstractHard::new(&ctx, &rem_v, UpdatePredicateToAstZ3::new(&ctx, &t.u, step));
                    let keeps = AbstractHard::new(&ctx, &rem_v, KeepVariableValues::new(&ctx, &p.vars, t, step));

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, updates, keeps)));
                }

                slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
                SlvPushZ3::new(&ctx, &slv);
                // slv_assert_z3!(&ctx, &slv, AbstractHard::new(&ctx, &rem_v, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step)));
                // slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
                slv_assert_z3!(&ctx, &slv, AbstractHard::new(&ctx, &rem_v, PredicateToAstZ3::new(&ctx, &p.goal, step)));
        
            } else {
                plan_found = true;
                break;
            }
        }

        let planning_time = now.elapsed();

        // let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
        // let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
        // let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
        // for a in cnf {
        //     println!("{}", ast_to_string_z3!(&ctx, a))
        // }
        
        if plan_found == true {
            let model = SlvGetModelZ3::new(&ctx, &slv);
            let result = GetSPPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
            result
        } else {
            let model = FreshModelZ3::new(&ctx);
            let result = GetSPPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
            result
        }              
    }   
}

// convert an ast formula (predicate) to cnf and remove clauses that don't contain any variable from the filtering vector

impl <'ctx> Abstract<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, keep_v: &Vec<Variable>, p: Z3_ast) -> Z3_ast {

        if keep_v.len() != 0 {
            let cnf = GetCnfVectorZ3::new(&ctx, vec!(p));
            let mut filtered: Vec<Z3_ast> = vec!();

            for a in cnf {
                if keep_v.iter().any(|x| ast_to_string_z3!(&ctx, a).contains(&x.n)) {
                    filtered.push(a)
                }
            }

            // for a in cnf {
            //     for var in keep_v {
            //         if ast_to_string_z3!(&ctx, a).contains(&var.n){
            //             filtered.push(a)
            //         }
            //     }
            // }
            filtered.sort();
            filtered.dedup();
            ANDZ3::new(&ctx, filtered)
        } else {
            p
        }
    }
}

// convert an ast formula (predicate) to cnf and remove clauses that contain variables in the filtering vector
impl <'ctx> AbstractHard<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, rem_v: &Vec<Variable>, p: Z3_ast) -> Z3_ast {

        if rem_v.len() != 0 {
            let cnf = GetCnfVectorZ3::new(&ctx, vec!(p));
            let mut filtered: Vec<Z3_ast> = vec!();

            for a in cnf {
                if !rem_v.iter().any(|x| ast_to_string_z3!(&ctx, a).contains(&x.n)) {
                    filtered.push(a)
                }
            }

            filtered.sort();
            filtered.dedup();
            ANDZ3::new(&ctx, filtered)
        } else {
            p
        }
    }
}

impl StateToPredicate {
    pub fn new(state: &Vec<&str>, p: &PlanningProblem) -> Predicate {
        let mut and_vec: Vec<Predicate> = vec!();
        for s in state {
            let sep: Vec<&str> = s.split(" -> ").collect();
            let mut d: Vec<&str> = vec!();
            let mut t: &str = "";
            let mut n: &str = "";
            for v in &p.vars {
                if v.n == sep[0] {
                    n = sep[0];
                    d = v.d.iter().map(|x| x.as_str()).collect();
                    t = &v.t;
                }
            }

            let var = Variable::new(n, t, d);
            let val = sep[1];

            and_vec.push(Predicate::EQVAL(var, String::from(val)));
        }
        Predicate::AND(and_vec)
    }
}

// generates unrefined problems that are trivial to solve
// refinement comes later in the lower levels
impl GenerateProblems {
    pub fn new(r: &PlanningResult, p: &PlanningProblem, uv: &Vec<Variable>) -> Vec<PlanningProblem> {
        let mut new_problems: Vec<PlanningProblem> = vec!();

        match r.plan_found {
            false => panic!("No plan at GenerateProblems::new"),
            true => match r.plan_length == 0 {
                false => {
                    for i in 0..r.trace.len() - 1 {
                        new_problems.push(
                            PlanningProblem::new(
                                String::from("some"), 
                                uv.clone(), 
                                StateToPredicate::new(&r.trace[i].state.iter().map(|x| x.as_str()).collect(), &p),
                                StateToPredicate::new(&r.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &p),
                                p.trans.clone(),
                                p.ltl_specs.clone(),
                                p.max_steps)
                        )
                    }
                },
                true => new_problems.push(
                    PlanningProblem::new(
                        String::from("some"), 
                        uv.clone(), 
                        StateToPredicate::new(&r.trace[0].state.iter().map(|x| x.as_str()).collect(), &p),
                        StateToPredicate::new(&r.trace[0].state.iter().map(|x| x.as_str()).collect(), &p),
                        p.trans.clone(),
                        p.ltl_specs.clone(),
                        p.max_steps)
                )
            }
        }
        new_problems
    }
}

// impl Compositional {
//     pub fn new(p: &PlanningProblem) -> PlanningResult {

//         let prob_vars = p.vars;
//         let mut used_vars = vec!();
//         let mut concat_var = 0;

//         // for now just choose the next one, maybe will have to choose a group
//         fn choose_var(pv: Vec<Variable>, uv: Vec<Variable>) -> Variable {
//             let mut v: Variable;
//             for var in pv {
//                 if !uv.contains(&var) {
//                     break
//                 }
//                 v = var;
//             }
//             v
//         }

//         used_vars.push(choose_var(prob_vars, used_vars));
//         let level0_plan0 = Sequential::new(&p, &used_vars);
//         if level0_plan0.plan_found == true {
//             if p.vars.len() != used_vars.len() {
//                 used_vars.push(choose_var(prob_vars, used_vars));

//             } else {
//                 level0_plan0
//             }
//         } else {
//             level0_plan0
//         }

//         // fn aprc(p: &PlanningProblem, uv: Vec<Variable>) -> PlanningResult {
//         //     let subres = Sequential2::new(&p, &uv);
//         //     if p.vars.len() != uv.len() {
//         //         if subres.plan_found == false {
//         //             used_vars.push(choose_var(prob_vars, used_vars));
//         //             aprc(&p, &used_vars)
//         //         } else {
//         //             used_vars.push(choose_var(prob_vars, used_vars));
//         //             let new_problems: Vec<PlanningProblem> = vec!();
                    
//         //         }

//         //     }
//         // }
    
//     }
// }

impl <'ctx> GetSPPlanningResultZ3<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, model: Z3_model, nr_steps: u32, 
    planning_time: std::time::Duration, plan_found: bool) -> PlanningResult {
        let model_str = ModelToStringZ3::new(&ctx, model);
        let mut model_vec = vec!();

        let num = ModelGetNumConstsZ3::new(&ctx, model);
        let mut lines = model_str.lines();
        let mut i: u32 = 0;

        while i < num {
            model_vec.push(lines.next().unwrap_or(""));
            i = i + 1;
        }

        println!("{:#?}", model_vec);

        let mut trace: Vec<PlanningFrame> = vec!();
        
        for i in 0..nr_steps {
            let mut frame: PlanningFrame = PlanningFrame::new(vec!(), "");
            for j in &model_vec {
                let sep: Vec<&str> = j.split(" -> ").collect();
                if sep[0].ends_with(&format!("_s{}", i)){
                    let trimmed_state = sep[0].trim_end_matches(&format!("_s{}", i));
                    match sep[1] {
                        "false" => frame.state.push(sep[0].to_string()),
                        "true" => frame.state.push(sep[0].to_string()),
                        _ => frame.state.push(format!("{} -> {}", trimmed_state, sep[1])),
                    }
                } else if sep[0].ends_with(&format!("_t{}", i)) && sep[1] == "true" {
                    let trimmed_trans = sep[0].trim_end_matches(&format!("_t{}", i));
                    frame.trans = trimmed_trans.to_string();
                }
            }
            if model_vec.len() != 0 {
                trace.push(frame);
            }
        }

        PlanningResult {
            plan_found: plan_found,
            plan_length: nr_steps - 1,
            trace: trace,
            time_to_solve: planning_time,
        }
    }
}

// idea 0: model everything as usuall, then to cnf and filter out what is not needed. However, we still have
//         residual stuff in the clauses that we can not remove. This is the filtering bottleneck here.
//         When using an agressive filter, more stuff is removed than necessary and no plan is found.
// idea 1: plan in independent variable group state spaces and then join groups somehow. Have a boolean activation
//         variable for each variable group that goes on if the varisbles are used.
// idea 2: a pbeq constraint on predicates in the guard of transitions. maybe we can filter that way
// idea 3: afterprocesing the plan to filter out the needed stuff for the next step (worst plan, defeats the purpose) 

#[test]
fn test_idea_0(){
    let pose_domain = vec!("buffer", "home", "table");
    let stat_domain = vec!("active", "idle");
    let buffer_domain = vec!("cube", "empty");
    let gripper_domain = vec!("cube", "empty");
    let table_domain = vec!("cube", "empty");

    // var group 
    let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
    let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

    // var group
    let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
    let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

    // var group
    let buffer = Variable::new("buffer", "buffer", buffer_domain.clone());

    // var group
    let gripper = Variable::new("gripper", "gripper", gripper_domain.clone());

    // var group
    let table = Variable::new("table", "table", table_domain.clone());

    let all_vars = vec!(act_pos.clone(), ref_pos.clone(), act_stat.clone(), ref_stat.clone(),
        buffer.clone(), gripper.clone(), table.clone());

    let at_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
    let at_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));

    let set_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
    let set_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));

    let at_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
    let at_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
    let at_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));

    let set_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
    let set_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
    let set_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));

    let move_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
    let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
    let move_transient = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
    let stat_transient = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

    let cube_at_buffer = Predicate::EQVAL(buffer.clone(), String::from("cube"));
    let cube_at_gripper = Predicate::EQVAL(gripper.clone(), String::from("cube"));
    let cube_at_table = Predicate::EQVAL(table.clone(), String::from("cube"));

    let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
    let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
    let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));

    // status change
    let t1 = Transition::new(
        "start_activate", 
        vec!(ref_stat.clone()), 
        &Predicate::AND(
            vec!(
                Predicate::NOT(vec!(at_active.clone())),
                Predicate::NOT(vec!(set_active.clone()))
            )
        ),
        &set_active
    );

    let t2 = Transition::new(
        "finish_activate", 
        vec!(act_stat.clone()), 
        &Predicate::AND(
            vec!(
                set_active.clone(),
                Predicate::NOT(vec!(at_active.clone()))
            )
        ),
        &at_active
    );

    let t3 = Transition::new(
        "start_deactivate", 
        vec!(ref_stat.clone()), 
        &Predicate::AND(
            vec!(
                Predicate::NOT(vec!(at_idle.clone())),
                Predicate::NOT(vec!(set_idle.clone()))
            )
        ),
        &set_idle
    );
    
    let t4 = Transition::new(
        "finish_deactivate", 
        vec!(act_stat.clone()), 
        &Predicate::AND(
            vec!(
                set_idle.clone(),
                Predicate::NOT(vec!(at_idle.clone()))
            )
        ),
        &at_idle
    );

    // moving the robot
    let t5 = Transition::new(
        "start_move_to_buffer",
        vec!(ref_pos.clone()),
        &Predicate::AND(
            vec!(
                set_active.clone(),
                at_active.clone(),
                move_stable.clone(),
                Predicate::NOT(vec!(at_buffer.clone())),
                Predicate::NOT(vec!(set_buffer.clone()))
            )
        ),
        &Predicate::AND(
            vec!(
                set_buffer.clone()
            )
        )
    );

    let t6 = Transition::new(
        "finish_move_to_buffer",
        vec!(act_pos.clone()),
        &Predicate::AND(
            vec!(
                set_active.clone(),
                at_active.clone(),
                set_buffer.clone(),
                Predicate::NOT(vec!(at_buffer.clone()))
            )
        ),
        &Predicate::AND(
            vec!(
                at_buffer.clone()
            )
        )
    );

    let t7 = Transition::new(
        "start_move_to_table",
        vec!(ref_pos.clone()),
        &Predicate::AND(
            vec!(
                set_active.clone(),
                at_active.clone(),
                move_stable.clone(),
                Predicate::NOT(vec!(at_table.clone())),
                Predicate::NOT(vec!(set_table.clone()))
            )
        ),
        &Predicate::AND(
            vec!(
                set_table.clone()
            )
        )
    );

    let t8 = Transition::new(
        "finish_move_to_table",
        vec!(act_pos.clone()),
        &Predicate::AND(
            vec!(
                set_active.clone(),
                at_active.clone(),
                set_table.clone(),
                Predicate::NOT(vec!(at_table.clone()))
            )
        ),
        &Predicate::AND(
            vec!(
                at_table.clone()
            )
        )
    );

    let t9 = Transition::new(
        "start_move_to_home",
        vec!(ref_pos.clone()),
        &Predicate::AND(
            vec!(
                set_active.clone(),
                at_active.clone(),
                move_stable.clone(),
                Predicate::NOT(vec!(at_home.clone())),
                Predicate::NOT(vec!(set_home.clone()))
            )
        ),
        &Predicate::AND(
            vec!(
                set_home.clone()
            )
        )
    );

    let t10 = Transition::new(
        "finish_move_to_home",
        vec!(act_pos.clone()),
        &Predicate::AND(
            vec!(
                set_active.clone(),
                at_active.clone(),
                set_home.clone(),
                Predicate::NOT(vec!(at_home.clone()))
            )
        ),
        &Predicate::AND(
            vec!(
                at_home.clone()
            )
        )
    );

    let t11 = Transition::new(
        "take_cube_from_buffer", 
        vec!(gripper.clone(), buffer.clone()), 
        &Predicate::AND(
            vec!(
                gripper_empty.clone(), 
                cube_at_buffer.clone(), 
                stat_stable.clone(), 
                move_stable.clone(),
                at_buffer.clone()
            )
        ), 
        &Predicate::AND(
            vec!(
                buffer_empty.clone(), 
                cube_at_gripper.clone()
            )
        )
    );

    let t12 = Transition::new(
        "take_cube_from_table", 
        vec!(gripper.clone(), table.clone()), 
        &Predicate::AND(
            vec!(
                gripper_empty.clone(), 
                cube_at_table.clone(), 
                stat_stable.clone(), 
                move_stable.clone(),
                at_table.clone()
            )
        ), 
        &Predicate::AND(
            vec!(
                table_empty.clone(), 
                cube_at_gripper.clone()
            )
        )
    );

    let t13 = Transition::new(
        "leave_cube_at_buffer", 
        vec!(gripper.clone(), buffer.clone()), 
        &Predicate::AND(
            vec!(
                buffer_empty.clone(), 
                cube_at_gripper.clone(),
                stat_stable.clone(), 
                move_stable.clone(),
                at_buffer.clone()
            )
        ), 
        &Predicate::AND(
            vec!(
                gripper_empty.clone(), 
                cube_at_buffer.clone()
            )
        )
    );

    let t14 = Transition::new(
        "leave_cube_at_table", 
        vec!(gripper.clone(), table.clone()), 
        &Predicate::AND(
            vec!(
                table_empty.clone(), 
                cube_at_gripper.clone(),
                stat_stable.clone(), 
                move_stable.clone(),
                at_table.clone()
            )
        ), 
        &Predicate::AND(
            vec!(
                gripper_empty.clone(), 
                cube_at_table.clone()
            )
        )
    );

    let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);

    // 1. have to go through the "home" pose:
    let s1 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("table"))
                        ),
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    let s2 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                        ),
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("table"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 2. one cube in the system:
    // let s3 = Predicate::GLOB(
    //     vec!(
    //         Predicate::PBEQ(
    //             vec!(
    //                 Predicate::AND(vec!(cube_at_buffer.clone(), table_empty.clone(), gripper_empty.clone())),
    //                 Predicate::AND(vec!(cube_at_table.clone(), gripper_empty.clone(), buffer_empty.clone())),
    //                 Predicate::AND(vec!(cube_at_gripper.clone(), table_empty.clone(), buffer_empty.clone())),
    //             ),
    //             1
    //         )
    //     )
    // );
    let s3 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    Predicate::AND(
                        vec!(
                            cube_at_buffer.clone(), 
                            Predicate::NOT(
                                vec!(
                                    cube_at_gripper.clone()
                                )
                            ), 
                            Predicate::NOT(
                                vec!(
                                    cube_at_table.clone()
                                )
                            )
                        )
                    ),
                    Predicate::AND(
                        vec!(
                            cube_at_table.clone(), 
                            Predicate::NOT(
                                vec!(
                                    cube_at_gripper.clone()
                                )
                            ), 
                            Predicate::NOT(
                                vec!(
                                    cube_at_buffer.clone()
                                )
                            )
                        )
                    ),
                    Predicate::AND(
                        vec!(
                            cube_at_gripper.clone(), 
                            Predicate::NOT(
                                vec!(
                                    cube_at_table.clone()
                                )
                            ), 
                            Predicate::NOT(
                                vec!(
                                    cube_at_buffer.clone()
                                )
                            )
                        )
                    )
                ),
                1
            )
        )
    );

    let specs = Predicate::AND(vec!(s1, s2, s3));

    // initial:
    let initial = Predicate::AND(
        vec!(
            // move_stable.clone(),
            // stat_stable.clone(),
            // at_active.clone(),
            // at_buffer.clone(),
            cube_at_table.clone()
        )
    );

    // goal:
    let goal = Predicate::AND(
        vec!(
            // at_table.clone(),
            // at_idle.clone(),
            cube_at_buffer.clone()
        )
    );

    let problem = PlanningProblem::new(String::from("robot1"), all_vars, initial, goal, trans, specs, 30);
    // let result = Sequential::new(&problem, &vec!());

    let pos_vars = vec!(act_pos.clone(), ref_pos.clone());
    let stat_vars = vec!(act_stat.clone(), ref_stat.clone());
    let cube_vars = vec!(buffer.clone(), gripper.clone(), table.clone());
    let soft_result_full = Sequential::new(&problem, &vec!());
    let soft_result_pos = Sequential::new(&problem, &pos_vars);
    let soft_result_stat = Sequential::new(&problem, &stat_vars);
    let soft_result_cube = Sequential::new(&problem, &cube_vars);
    let hard_result_full = Sequential2::new(&problem, &vec!());
    let hard_result_pos = Sequential2::new(&problem, &pos_vars);
    let hard_result_stat = Sequential2::new(&problem, &stat_vars);
    let hard_result_cube = Sequential2::new(&problem, &cube_vars);

    println!("soft_plan_found_full: {:?}", soft_result_full.plan_found);
    println!("soft_plan_lenght_full: {:?}", soft_result_full.plan_length);
    println!("soft_time_to_solve_full: {:?}", soft_result_full.time_to_solve);
    println!("soft_trace_full: ");
    
    for t in &soft_result_full.trace{
        
        println!("soft_state_full: {:?}", t.state);
        println!("soft_trans_full: {:?}", t.trans);
        println!("=========================");
    }

    println!("hard_plan_found_full: {:?}", hard_result_full.plan_found);
    println!("hard_plan_lenght_full: {:?}", hard_result_full.plan_length);
    println!("hard_time_to_solve_full: {:?}", hard_result_full.time_to_solve);
    println!("hard_trace_full: ");
    
    for t in &hard_result_full.trace{
        
        println!("hard_state_full: {:?}", t.state);
        println!("hard_trans_full: {:?}", t.trans);
        println!("=========================");
    }

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    println!("soft_plan_found_pos: {:?}", soft_result_pos.plan_found);
    println!("soft_plan_lenght_pos: {:?}", soft_result_pos.plan_length);
    println!("soft_time_to_solve_pos: {:?}", soft_result_pos.time_to_solve);
    println!("soft_trace_pos: ");
    
    for t in &soft_result_pos.trace{
        
        println!("soft_state_pos: {:?}", t.state);
        println!("soft_trans_pos: {:?}", t.trans);
        println!("=========================");
    }

    println!("hard_plan_found_pos: {:?}", hard_result_pos.plan_found);
    println!("hard_plan_lenght_pos: {:?}", hard_result_pos.plan_length);
    println!("hard_time_to_solve_pos: {:?}", hard_result_pos.time_to_solve);
    println!("hard_trace_pos: ");
    
    for t in &hard_result_pos.trace{
        
        println!("hard_state_pos: {:?}", t.state);
        println!("hard_trans_pos: {:?}", t.trans);
        println!("=========================");
    }

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    println!("soft_plan_found_stat: {:?}", soft_result_stat.plan_found);
    println!("soft_plan_lenght_stat: {:?}", soft_result_stat.plan_length);
    println!("soft_time_to_solve_stat: {:?}", soft_result_stat.time_to_solve);
    println!("soft_trace_stat: ");
    
    for t in &soft_result_stat.trace{
        
        println!("soft_state_stat: {:?}", t.state);
        println!("soft_trans_stat: {:?}", t.trans);
        println!("=========================");
    }

    println!("hard_plan_found_stat: {:?}", hard_result_stat.plan_found);
    println!("hard_plan_lenght_stat: {:?}", hard_result_stat.plan_length);
    println!("hard_time_to_solve_stat: {:?}", hard_result_stat.time_to_solve);
    println!("hard_trace_stat: ");
    
    for t in &hard_result_stat.trace{
        
        println!("hard_state_stat: {:?}", t.state);
        println!("hard_trans_stat: {:?}", t.trans);
        println!("=========================");
    }

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    println!("soft_plan_found_cube: {:?}", soft_result_cube.plan_found);
    println!("soft_plan_lenght_cube: {:?}", soft_result_cube.plan_length);
    println!("soft_time_to_solve_cube: {:?}", soft_result_cube.time_to_solve);
    println!("soft_trace_cube: ");
    
    for t in &soft_result_cube.trace{
        
        println!("soft_state_cube: {:?}", t.state);
        println!("soft_trans_cube: {:?}", t.trans);
        println!("=========================");
    }

    println!("hard_plan_found_cube: {:?}", hard_result_cube.plan_found);
    println!("hard_plan_lenght_cube: {:?}", hard_result_cube.plan_length);
    println!("hard_time_to_solve_cube: {:?}", hard_result_cube.time_to_solve);
    println!("hard_trace_cube: ");
    
    for t in &hard_result_cube.trace{
        
        println!("hard_state_cube: {:?}", t.state);
        println!("hard_trans_cube: {:?}", t.trans);
        println!("=========================");
    }

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
}

#[test]
fn test_idea_1(){

    let pos: bool = false; // activation var
    let stat: bool = false; // activation var
    let cube: bool = true; // activation var

    let pose_domain = vec!("buffer", "home", "table");
    let stat_domain = vec!("active", "idle");
    let buffer_domain = vec!("cube", "ball", "empty");
    let gripper_domain = vec!("cube", "ball", "empty");
    let table_domain = vec!("cube", "ball", "empty");

    // var group pos
    let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());
    let ref_pos = Variable::new("ref_pos", "pose", pose_domain.clone());

    // var group stat
    let act_stat = Variable::new("act_stat", "status", stat_domain.clone());
    let ref_stat = Variable::new("ref_stat", "status", stat_domain.clone());

    // var group cube
    let buffer = Variable::new("buffer", "buffer", buffer_domain.clone());
    let gripper = Variable::new("gripper", "gripper", gripper_domain.clone());
    let table = Variable::new("table", "table", table_domain.clone());

    // basic predicates
    let t = Predicate::TRUE;
    let f = Predicate::FALSE;

    // act stat predicates
    let stat_active = Predicate::EQVAL(act_stat.clone(), String::from("active"));
    let stat_idle = Predicate::EQVAL(act_stat.clone(), String::from("idle"));
    let not_stat_active = Predicate::NOT(vec!(stat_active.clone()));
    let not_stat_idle = Predicate::NOT(vec!(stat_idle.clone()));

    // ref stat predicates
    let set_stat_active = Predicate::EQVAL(ref_stat.clone(), String::from("active"));
    let set_stat_idle = Predicate::EQVAL(ref_stat.clone(), String::from("idle"));
    let not_set_stat_active = Predicate::NOT(vec!(set_stat_active.clone()));
    let not_set_stat_idle = Predicate::NOT(vec!(set_stat_idle.clone()));

    // act pos predicates
    let pos_buffer = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
    let pos_table = Predicate::EQVAL(act_pos.clone(), String::from("table"));
    let pos_home = Predicate::EQVAL(act_pos.clone(), String::from("home"));
    let not_pos_buffer = Predicate::NOT(vec!(pos_buffer.clone()));
    let not_pos_table = Predicate::NOT(vec!(pos_table.clone()));
    let not_pos_home = Predicate::NOT(vec!(pos_home.clone()));

    // ref pos predicates
    let set_pos_buffer = Predicate::EQVAL(ref_pos.clone(), String::from("buffer"));
    let set_pos_table = Predicate::EQVAL(ref_pos.clone(), String::from("table"));
    let set_pos_home = Predicate::EQVAL(ref_pos.clone(), String::from("home"));
    let not_set_pos_buffer = Predicate::NOT(vec!(set_pos_buffer.clone()));
    let not_set_pos_table = Predicate::NOT(vec!(set_pos_table.clone()));
    let not_set_pos_home = Predicate::NOT(vec!(set_pos_home.clone()));

    // act buffer predicates
    let buffer_cube = Predicate::EQVAL(buffer.clone(), String::from("cube"));
    let buffer_ball = Predicate::EQVAL(buffer.clone(), String::from("ball"));
    let buffer_empty = Predicate::EQVAL(buffer.clone(), String::from("empty"));
    let not_buffer_cube = Predicate::NOT(vec!(buffer_cube.clone()));
    let not_buffer_ball = Predicate::NOT(vec!(buffer_ball.clone()));
    let not_buffer_empty = Predicate::NOT(vec!(buffer_empty.clone()));
    
    // act gripper predicates
    let gripper_cube = Predicate::EQVAL(gripper.clone(), String::from("cube"));
    let gripper_ball = Predicate::EQVAL(gripper.clone(), String::from("ball"));
    let gripper_empty = Predicate::EQVAL(gripper.clone(), String::from("empty"));
    let not_gripper_cube = Predicate::NOT(vec!(gripper_cube.clone()));
    let not_gripper_ball = Predicate::NOT(vec!(gripper_ball.clone()));
    let not_gripper_empty = Predicate::NOT(vec!(gripper_empty.clone()));

    // act table predicates
    let table_cube = Predicate::EQVAL(table.clone(), String::from("cube"));
    let table_ball = Predicate::EQVAL(table.clone(), String::from("ball"));
    let table_empty = Predicate::EQVAL(table.clone(), String::from("empty"));
    let not_table_cube = Predicate::NOT(vec!(table_cube.clone()));
    let not_table_ball = Predicate::NOT(vec!(table_ball.clone()));
    let not_table_empty = Predicate::NOT(vec!(table_empty.clone()));

    // are ref == act predicates
    let pos_stable = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
    let stat_stable = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
    let not_pos_stable = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
    let not_stat_stable = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

    // variables in the problem
    let all_vars = vec!(
        act_pos.clone(), 
        ref_pos.clone(), 
        act_stat.clone(), 
        ref_stat.clone(),
        buffer.clone(), 
        gripper.clone(), 
        table.clone()
    );

    // status change transitions
    let t1 = Transition::new(
        "start_activate", 
        vec!(ref_stat.clone()), 
        &Predicate::AND(
            vec!(
                if stat {not_stat_active.clone()} else {t.clone()},
                if stat {not_set_stat_active.clone()} else {t.clone()}
            )
        ),
        if stat {&set_stat_active} else {&t}
    );

    let t2 = Transition::new(
        "finish_activate", 
        vec!(act_stat.clone()), 
        &Predicate::AND(
            vec!(
                if stat {set_stat_active.clone()} else {t.clone()},
                if stat {not_stat_active.clone()} else {t.clone()}
            )
        ),
        if stat {&stat_active} else {&t}
    );

    let t3 = Transition::new(
        "start_deactivate", 
        vec!(ref_stat.clone()), 
        &Predicate::AND(
            vec!(
                if stat {not_stat_idle.clone()} else {t.clone()},
                if stat {not_set_stat_idle.clone()} else {t.clone()}
            )
        ),
        if stat {&set_stat_idle} else {&t}
    );
    
    let t4 = Transition::new(
        "finish_deactivate", 
        vec!(act_stat.clone()), 
        &Predicate::AND(
            vec!(
                if stat {not_stat_idle.clone()} else {t.clone()},
                if stat {set_stat_idle.clone()} else {t.clone()}
            )
        ),
        if stat {&stat_idle} else {&t}
    );

    let t5 = Transition::new(
        "start_move_to_buffer",
        vec!(ref_pos.clone()),
        &Predicate::AND(
            vec!(
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_stable.clone()} else {t.clone()},
                if pos {not_pos_buffer.clone()} else {t.clone()},
                if pos {not_set_pos_buffer.clone()} else {t.clone()}
            )
        ),
        if pos {&set_pos_buffer} else {&t}
    );
    
    let t6 = Transition::new(
        "finish_move_to_buffer",
        vec!(act_pos.clone()),
        &Predicate::AND(
            vec!(
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {not_pos_buffer.clone()} else {t.clone()},
                if pos {set_pos_buffer.clone()} else {t.clone()}
            )
        ),
        if pos {&pos_buffer} else {&t}
    );

    let t7 = Transition::new(
        "start_move_to_table",
        vec!(ref_pos.clone()),
        &Predicate::AND(
            vec!(
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_stable.clone()} else {t.clone()},
                if pos {not_pos_table.clone()} else {t.clone()},
                if pos {not_set_pos_table.clone()} else {t.clone()}
            )
        ),
        if pos {&set_pos_table} else {&t}
    );

    let t8 = Transition::new(
        "finish_move_to_table",
        vec!(act_pos.clone()),
        &Predicate::AND(
            vec!(
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {not_pos_table.clone()} else {t.clone()},
                if pos {set_pos_table.clone()} else {t.clone()}
            )
        ),
        if pos {&pos_table} else {&t}
    );

    let t9 = Transition::new(
        "start_move_to_home",
        vec!(ref_pos.clone()),
        &Predicate::AND(
            vec!(
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_stable.clone()} else {t.clone()},
                if pos {not_pos_home.clone()} else {t.clone()},
                if pos {not_set_pos_home.clone()} else {t.clone()}
            )
        ),
        if pos {&set_pos_home} else {&t}
    );

    let t10 = Transition::new(
        "finish_move_to_home",
        vec!(act_pos.clone()),
        &Predicate::AND(
            vec!(
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {not_pos_home.clone()} else {t.clone()},
                if pos {set_pos_home.clone()} else {t.clone()}
            )
        ),
        if pos {&pos_home} else {&t}
    );

    let t11 = Transition::new(
        "take_cube_from_buffer",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &Predicate::AND(
            vec!(
                if cube {buffer_cube.clone()} else {t.clone()},
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_buffer.clone()} else {t.clone()},
                if pos {set_pos_buffer.clone()} else {t.clone()}
            )
        ),
        if cube {&gripper_cube} else {&t}
    );

    let t12 = Transition::new(
        "take_cube_from_table",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &Predicate::AND(
            vec!(
                if cube {table_cube.clone()} else {t.clone()},
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_table.clone()} else {t.clone()},
                if pos {set_pos_table.clone()} else {t.clone()}
            )
        ),
        if cube {&gripper_cube} else {&t}
    );

    let t13 = Transition::new(
        "leave_cube_at_buffer",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &Predicate::AND(
            vec!(
                if cube {gripper_cube.clone()} else {t.clone()},
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_buffer.clone()} else {t.clone()},
                if pos {set_pos_buffer.clone()} else {t.clone()}
            )
        ),
        if cube {&buffer_cube} else {&t}
    );

    let t14 = Transition::new(
        "leave_cube_at_table",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &Predicate::AND(
            vec!(
                if cube {gripper_cube.clone()} else {t.clone()},
                if stat {stat_active.clone()} else {t.clone()},
                if stat {set_stat_active.clone()} else {t.clone()},
                if pos {pos_table.clone()} else {t.clone()},
                if pos {set_pos_table.clone()} else {t.clone()}
            )
        ),
        if cube {&table_cube} else {&t}
    );

    // 1. have to go through the "home" pose:
    let s1 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("table"))
                        ),
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 2. have to go through the "home" pose:
    let s2 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                        ),
                        vec!(
                            Predicate::EQVAL(act_pos.clone(), String::from("table"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 3. one cube in the system:
    let s3 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    Predicate::AND(
                        vec!(
                            buffer_cube.clone(), 
                            Predicate::NOT(
                                vec!(
                                    gripper_cube.clone()
                                )
                            ), 
                            Predicate::NOT(
                                vec!(
                                    table_cube.clone()
                                )
                            )
                        )
                    ),
                    Predicate::AND(
                        vec!(
                            table_cube.clone(), 
                            Predicate::NOT(
                                vec!(
                                    gripper_cube.clone()
                                )
                            ), 
                            Predicate::NOT(
                                vec!(
                                    buffer_cube.clone()
                                )
                            )
                        )
                    ),
                    Predicate::AND(
                        vec!(
                            gripper_cube.clone(), 
                            Predicate::NOT(
                                vec!(
                                    table_cube.clone()
                                )
                            ), 
                            Predicate::NOT(
                                vec!(
                                    buffer_cube.clone()
                                )
                            )
                        )
                    )
                ),
                1
            )
        )
    );

    let all_trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);

    let all_specs = Predicate::AND(vec!(s1, s2, s3));

    let initial = Predicate::AND(
        vec!(
            if pos {pos_stable.clone()} else {t.clone()},
            if pos {pos_buffer.clone()} else {t.clone()},
            if stat {stat_stable.clone()} else {t.clone()},
            if stat {stat_idle.clone()} else {t.clone()},
            if cube {table_cube.clone()} else {t.clone()}
        )
    );

    let goal = Predicate::AND(
        vec!(
            if pos {pos_table.clone()} else {t.clone()},
            if stat {stat_idle.clone()} else {t.clone()},
            if cube {buffer_cube.clone()} else {t.clone()}
        )
    );

    let problem = PlanningProblem::new(String::from("robot1"), all_vars, initial, goal, all_trans, all_specs, 30);
    
    let soft_result_full = Sequential::new(&problem, &vec!());

    println!("soft_plan_found_full: {:?}", soft_result_full.plan_found);
    println!("soft_plan_lenght_full: {:?}", soft_result_full.plan_length);
    println!("soft_time_to_solve_full: {:?}", soft_result_full.time_to_solve);
    println!("soft_trace_full: ");
    
    for t in &soft_result_full.trace{
        
        println!("soft_state_full: {:?}", t.state);
        println!("soft_trans_full: {:?}", t.trans);
        println!("=========================");
    }
}