//! Z3 sorts for SP

use std::ffi::{CStr, CString};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use z3_sys::*;
use super::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
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

#[derive(PartialEq, Clone, Debug)]
pub struct ParamTransition {
    n: String,
    v: Vec<Variable>,
    g: Vec<(String, Predicate)>,
    u: Vec<(String, Predicate)>
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
pub struct ParamPlanningProblem {
    name: String,
    vars: Vec<Variable>,
    params: Vec<(String, bool)>,
    initial: Vec<(String, Predicate)>,
    goal: Vec<(String, Predicate)>,
    trans: Vec<ParamTransition>,
    ltl_specs: Predicate, // probably will be parameterized later as well
    // level: u32,
    // concat: u32,
    max_steps: u32
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlanningFrame {
    state: Vec<String>,
    trans: String,
}

pub struct GetPlanningResultZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub model: Z3_model,
    pub nr_steps: u32,
    pub frames: PlanningResult
}

pub struct GetParamPlanningResultZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub model: Z3_model,
    pub level: u32,
    pub concat: u32,
    pub nr_steps: u32,
    pub frames: PlanningResult
}

pub struct PlanningResult {
    pub plan_found: bool,
    pub plan_length: u32,
    pub trace: Vec<PlanningFrame>,
    pub time_to_solve: std::time::Duration,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParamPlanningResult {
    pub plan_found: bool,
    pub plan_length: u32,
    pub level: u32,
    pub concat: u32,
    pub trace: Vec<PlanningFrame>,
    pub time_to_solve: std::time::Duration,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
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

pub struct ParamSequential {}

pub struct Sequential2 {}

pub struct GenerateProblems {}

pub struct GenerateParamProblems {}

pub struct GenerateAndSolveLevel {}

// pub struct GenerateProblem {}

pub struct StateToPredicate {}

pub struct ParamStateToPredicate {}

pub struct LowLevelSequential<'ctx> {
pub ctx: &'ctx ContextZ3
}

pub struct Compositional {}

pub struct Compositional2 {}

pub struct Concatenate {}

pub struct ActivateNextParam {}

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

impl ParamTransition {
    pub fn new(n: &str, v: Vec<Variable>, g: &Vec<(&str, Predicate)>, u: &Vec<(&str, Predicate)>) -> ParamTransition {
        ParamTransition { n: n.to_string(),
                     v: v.clone(),
                     g: g.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
                     u: u.iter().map(|x| (x.0.to_string(), x.1.clone())).collect()
        }
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

impl ParamPlanningProblem {
    pub fn new(name: String,
               vars: Vec<Variable>,
               params: Vec<(&str, bool)>,
               initial: Vec<(&str, Predicate)>,
               goal: Vec<(&str, Predicate)>,
               trans: Vec<ParamTransition>,
               ltl_specs: Predicate,
            //    level: u32,
            //    concat: u32,
               max_steps: u32) -> ParamPlanningProblem {
        ParamPlanningProblem {
            name: name.to_string(),
            vars: vars,
            params: params.iter().map(|x| (x.0.to_string(), x.1)).collect(),
            initial: initial.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
            goal: goal.iter().map(|x| (x.0.to_string(), x.1.clone())).collect(),
            trans: trans,
            ltl_specs: ltl_specs,
            // level: level,
            // concat: concat,
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
            let result = GetPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
            result
        } else {
            let model = FreshModelZ3::new(&ctx);
            let result = GetPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found);
            result
        }              
    }   
}

// accepts param_planning_problem that accepts param transitions
impl ParamSequential {
    pub fn new(p: &ParamPlanningProblem, params: &Vec<(&str, bool)>, level: u32, concat: u32) -> ParamPlanningResult {

        // println!("parameterized_trans: {:?}", p.trans[0]);

        // resolve transitions based on parameters:
        let mut resolved_trans = vec!();
        for t in &p.trans {
            let mut resolved_guard = vec!();
            let mut resolved_update = vec!();
            for param in params {
                for t_guard in &t.g {
                    if t_guard.0 == param.0 && param.1 {
                        resolved_guard.push(t_guard.1.clone())
                    }
                };
                for t_update in &t.u {
                    if t_update.0 == param.0 && param.1 {
                        resolved_update.push(t_update.1.clone())
                    }
                };
            }

            resolved_guard.sort();
            resolved_guard.dedup();

            resolved_update.sort();
            resolved_update.dedup();

            resolved_trans.push(
                Transition::new(
                    t.n.as_str(), 
                    t.v.clone(),
                    &Predicate::AND(resolved_guard),
                    &Predicate::AND(resolved_update)
            )
        )
        }

        // println!("resolved_trans: {:?}", resolved_trans[0]);

        // resolve initial state:
        let mut initial_pred_vec = vec!();
        for pred in &p.initial {
            for param in params {
                if pred.0 == param.0 && param.1 {
                    initial_pred_vec.push(pred.1.clone())
                }
            }
        }

        let initial = Predicate::AND(initial_pred_vec);

        // resolve goal state:
        let mut goal_pred_vec = vec!();
        for pred in &p.goal {
            for param in params {
                if pred.0 == param.0 && param.1 {
                    goal_pred_vec.push(pred.1.clone())
                }
            }
        }

        let goal = Predicate::AND(goal_pred_vec);

        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);
    
        slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &initial, 0));

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, 0));
        slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &goal, 0));

        let now = Instant::now();
        let mut plan_found: bool = false;

        let mut step: u32 = 0;

        while step < p.max_steps + 1 {
            step = step + 1;
            if SlvCheckZ3::new(&ctx, &slv) != 1 {
                SlvPopZ3::new(&ctx, &slv, 1);

                let mut all_trans = vec!();
                for t in &resolved_trans {
                    let name = format!("{}_t{}", &t.n, step);
                    let guard = PredicateToAstZ3::new(&ctx, &t.g, step - 1);
                    let updates = UpdatePredicateToAstZ3::new(&ctx, &t.u, step);
                    let keeps = KeepVariableValues::new(&ctx, &p.vars, t, step);

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, updates, keeps)));
                }

                slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
                SlvPushZ3::new(&ctx, &slv);
                slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
                // slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.ltl_specs, step));
                slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &goal, step));
        
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
            let result = GetParamPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found, level, concat);
            result
        } else {
            let model = FreshModelZ3::new(&ctx);
            let result = GetParamPlanningResultZ3::new(&ctx, model, step, planning_time, plan_found, level, concat);
            result
        }              
    }   
}

// 

impl ActivateNextParam {
    pub fn new(params: &Vec<(String, bool)>, order: &Vec<&str>) -> Vec<(String, bool)> {
        let mut index_to_update = 0;
        match params.iter().all(|x| x.1) {
            true => panic!("ActivateNextParam: all parameters activated!"),
            false => {
                for ord in order {
                    for p in params {
                        if &p.0 == ord && !p.1 {
                            match params.iter().position(|x| x == p) {
                                Some(x) => index_to_update = x,
                                None => panic!("ActivateNextParam: element fail.")
                            }
                            break;
                        }
                    }
                };
                let mut pclone = params.clone();
                let mut to_update = pclone.remove(index_to_update);
                to_update.1 = true;
                pclone.push((to_update.0, to_update.1));
                pclone.iter().map(|x| (x.0.to_string(), x.1)).collect()
            }
        }
    }
}

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

impl ParamStateToPredicate {
    pub fn new(state: &Vec<&str>, p: &ParamPlanningProblem) -> Vec<(String, Predicate)> {
        let mut pred_vec: Vec<(String, Predicate)> = vec!();
        for s in state {
            // let pred: (&str, Predicate)
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
            let mut activator: String = String::from("");
            for param in &p.params {
                if var.n.ends_with(&param.0) {
                    activator = param.0.to_string()
                }
            };
            
            pred_vec.push((activator, Predicate::EQVAL(var, String::from(val))));
        }
        pred_vec
    }
}

// generates unrefined problems that are trivial to solve
// refinement comes later in the lower levels
impl GenerateProblems {
    pub fn new(r: &PlanningResult, p: &PlanningProblem) -> Vec<PlanningProblem> {
        let mut new_problems: Vec<PlanningProblem> = vec!();

        match r.plan_found {
            false => panic!("No plan at GenerateProblems::new"),
            true => match r.plan_length == 0 {
                false => {
                    for i in 0..r.trace.len() - 1 {
                        new_problems.push(
                            PlanningProblem::new(
                                String::from("some"), 
                                p.vars.clone(), 
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
                        p.vars.clone(), 
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

impl GenerateParamProblems {
    pub fn new(r: &PlanningResult, p: &ParamPlanningProblem, uv: &Vec<Variable>) -> Vec<ParamPlanningProblem> {
        let mut new_problems: Vec<ParamPlanningProblem> = vec!();

        match r.plan_found {
            false => panic!("No plan at GenerateProblems::new"),
            true => match r.plan_length == 0 {
                false => {
                    for i in 0..r.trace.len() - 1 {
                        new_problems.push(
                            ParamPlanningProblem::new(
                                String::from("some"), 
                                uv.clone(),
                                p.params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
                                ParamStateToPredicate::new(&r.trace[i].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                ParamStateToPredicate::new(&r.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                p.trans.clone(),
                                p.ltl_specs.clone(),
                                p.max_steps)
                        )
                    }
                },
                true => new_problems.push(
                    ParamPlanningProblem::new(
                        String::from("some"), 
                        uv.clone(),
                        p.params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
                        ParamStateToPredicate::new(&r.trace[0].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                        ParamStateToPredicate::new(&r.trace[0].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                        p.trans.clone(),
                        p.ltl_specs.clone(),
                        p.max_steps)
                )
            }
        }
        new_problems
    }
}

impl GenerateAndSolveLevel {

    // here we get a trace, we have to refine the problem, generate and solve
    // how to solve the goal to init inheritance?
    pub fn new(r: &ParamPlanningResult, p: &ParamPlanningProblem, params: &Vec<(String, bool)>, level: u32) -> Vec<ParamPlanningResult> {

        let mut solved_problems: Vec<ParamPlanningResult> = vec!();
        let mut problem = ParamPlanningProblem::new(String::from("dummy"), vec!(), vec!(), vec!(), vec!(), vec!(), Predicate::TRUE, 0);
        let mut concat: u32 = 0;

        let mut inheritance = vec!((String::from("dummy"), Predicate::TRUE));

        match r.plan_found {
            false => panic!("No plan at GenerateProblems::new"),
            true => match r.plan_length == 0 {
                false => {
                    for i in 0..=r.trace.len() - 1 {
                        if i == 0 {
                            let goal: Vec<(String, Predicate)> = ParamStateToPredicate::new(&r.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.to_string(), x.1.clone())).collect();
                            problem = ParamPlanningProblem::new(
                                String::from("some"), 
                                p.vars.clone(),
                                p.params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                p.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                goal.clone().iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                p.trans.clone(),
                                p.ltl_specs.clone(),
                                p.max_steps);
                            // println!("i==0, {:?}, {:?}", i, inheritance);
                            inheritance = goal.iter().map(|x| (x.0.to_string(), x.1.clone())).collect();
                        } else if i == r.trace.len() - 1 {
                            problem = ParamPlanningProblem::new(
                                String::from("some"), 
                                p.vars.clone(),
                                p.params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                inheritance.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                p.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                p.trans.clone(),
                                p.ltl_specs.clone(),
                                p.max_steps);
                            // println!("i == r.trace.len() - 1, {:?}, {:?}", i, inheritance);
                        } else {
                            let goal: Vec<(String, Predicate)> = ParamStateToPredicate::new(&r.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.to_string(), x.1.clone())).collect();
                            problem = ParamPlanningProblem::new(
                                String::from("some"), 
                                p.vars.clone(),
                                p.params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                inheritance.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                goal.clone().iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                                p.trans.clone(),
                                p.ltl_specs.clone(),
                                p.max_steps);
                            inheritance = goal.iter().map(|x| (x.0.to_string(), x.1.clone())).collect();
                            // println!("i==between, {:?}, {:?}", i, inheritance);
                        }
                        solved_problems.push(ParamSequential::new(&problem, &params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(), level, concat));
                        concat = concat + 1;
                    }
                },
                true => {
                    // println!("lasdhflaksdfhlaksdjfhlasdkjfhasldkjfhsaldkfjh");
                    problem = ParamPlanningProblem::new(
                        String::from("some"), 
                        p.vars.clone(),
                        p.params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
                        ParamStateToPredicate::new(&r.trace[0].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                        ParamStateToPredicate::new(&r.trace[0].state.iter().map(|x| x.as_str()).collect(), &p).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                        p.trans.clone(),
                        p.ltl_specs.clone(),
                        p.max_steps);
                    solved_problems.push(ParamSequential::new(&problem, &params.iter().map(|x| (x.0.as_str(), x.1)).collect(), level, concat));
                    concat = concat + 1;
                }
            }
        }
        solved_problems
    }
}

// solve level 0 and fwd to compositional
// results should already contain the level0 result
impl Compositional {
    pub fn new(r: &ParamPlanningResult, // level 0 result
               p: &ParamPlanningProblem, // level 0 problem
               params: &Vec<(String, bool)>, 
               ord: &Vec<&str>, 
               results: &Vec<ParamPlanningResult>, // contains only level 0 result
               curr_level: u32) -> Vec<ParamPlanningResult> { // curently, level 0
        
    let mut all_res: Vec<ParamPlanningResult> = vec!();
    all_res.extend(results.iter().cloned());
    println!("current level: {:?}", curr_level);
    println!("current results: {:?}", results);

    let mut level: u32 = curr_level;

    if !params.iter().all(|x| x.1) {
        // println!{"111111111111111111111"};
        level = level + 1;
        let act = &ActivateNextParam::new(&params, &ord);
        let refined = ParamPlanningProblem::new(
            String::from("dummy"), 
            p.vars.clone(), 
            act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 
            p.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(), 
            p.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(), 
            p.trans.clone(),
            p.ltl_specs.clone(),
            p.max_steps);
        let results = GenerateAndSolveLevel::new(r, &refined, &act, level);
        all_res.extend(results.clone());
        for res in results {
            Compositional::new(&res, &refined, &act, &ord, &all_res, level);
        }
        
    } else {
        // println!{"2222222222222222222222222"};
        // let act = &ActivateNextParam::new(&params, &ord);
        all_res.extend(GenerateAndSolveLevel::new(r, &p, &params, level));
        }
        all_res.clone()
    }
}

// impl Compositional2 {
//     pub fn new(curr_problem: &ParamPlanningProblem,
//                parameters: &Vec<(String, bool)>, 
//                order: &Vec<&str>, 
//                cumulative_results: &Vec<ParamPlanningResult>,
//                curr_level: u32,
//                curr_concat: u32) -> Vec<ParamPlanningResult> {
    
//         let mut level: u32 = curr_level;
//         let mut concat: u32 = curr_concat;
//         let mut all_results: Vec<ParamPlanningResult> = cumulative_results.clone();

//         if !parameters.iter().all(|x| x.1) {
//             let act = &ActivateNextParam::new(&parameters, &order);
//             let problem = ParamPlanningProblem::new(
//                 format!("problem_l{:?}_c{:?}", level, concat), 
//                 curr_problem.vars.clone(),
//                 act.clone().iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                 curr_problem.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                 curr_problem.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                 curr_problem.trans.clone(), 
//                 curr_problem.ltl_specs.clone(),
//                 30
//             );
//             let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), level, concat);
//             all_results.push(result);



//         } else {

//         }
//         all_results
//     }
// }

// impl Compositional2 {
//     pub fn new(curr_problem: &ParamPlanningProblem,
//                parameters: &Vec<(String, bool)>, 
//                order: &Vec<&str>, 
//                cumulative_results: &Vec<ParamPlanningResult>,
//                curr_level: u32,
//                curr_concat: u32) -> Vec<ParamPlanningResult> {
    
//         let mut level: u32 = curr_level;
//         let mut concat: u32 = curr_concat;
//         let mut all_results: Vec<ParamPlanningResult> = cumulative_results.clone();

//         for s in all_results.clone() {

//             println!("level: {:?}", level);
//             println!("concat: {:?}", concat);
//             println!("plan_found: {:?}", s.plan_found);
//             println!("plan_lenght: {:?}", s.plan_length);
//             println!("time_to_solve: {:?}", s.time_to_solve);
//             println!("trace: ");
    
//             for t in &s.trace{
            
//                 println!("state: {:?}", t.state);
//                 println!("trans: {:?}", t.trans);
//                 println!("=========================");
//             }
//         }

//         println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

//         if !parameters.iter().all(|x| !x.1) {
            
//             if !parameters.iter().all(|x| x.1) {
//                 level = level + 1;
//                 concat = 0;
//                 println!("11111111111111111111111111");
//                 let result = ParamSequential::new(&curr_problem, &parameters.iter().map(|x| (x.0.as_str(), x.1)).collect(), level, concat);
//                 all_results.push(result.clone());

//                 let refined_params = &ActivateNextParam::new(&parameters, &order);

//                 if result.plan_found {
//                     if result.plan_length != 0 {
//                         for i in 0..result.trace.len() - 1 {

//                             let problem = ParamPlanningProblem::new(
//                                 format!("problem_l{:?}_c{:?}", level, concat), 
//                                 curr_problem.vars.clone(),
//                                 refined_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                                 ParamStateToPredicate::new(&result.trace[i].state.iter().map(|x| x.as_str()).collect(), &curr_problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 ParamStateToPredicate::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &curr_problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                                 curr_problem.trans.clone(),
//                                 curr_problem.ltl_specs.clone(),
//                                 curr_problem.max_steps);
                            
//                             concat = concat + 1;
//                             Compositional2::new(&problem, &refined_params, order, &all_results, level, concat);
//                         }
//                     } else {

//                         let problem = ParamPlanningProblem::new(
//                             format!("problem_l{:?}_c{:?}", level, concat), 
//                             curr_problem.vars.clone(),
//                             refined_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
//                             ParamStateToPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &curr_problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                             ParamStateToPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &curr_problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
//                             curr_problem.trans.clone(),
//                             curr_problem.ltl_specs.clone(),
//                             curr_problem.max_steps);
                        
//                         concat = concat + 1;
//                         Compositional2::new(&problem, &refined_params, order, &all_results, level, concat);
//                     }
//                 } else {
//                     panic!("No plan found at level: {:?}, concat: {:?}", level, concat);
//                 }

//             } else if parameters.iter().all(|x| x.1) {
//                 println!("22222222222222222222222");
//                 let result = ParamSequential::new(&curr_problem, &parameters.iter().map(|x| (x.0.as_str(), x.1)).collect(), level, concat);
//                 all_results.push(result.clone());
//             }

//         } else if parameters.iter().all(|x| !x.1) {
//             println!("33333333333333333333333333");
//             let refined_params = &ActivateNextParam::new(&parameters, &order);
//             Compositional2::new(&curr_problem, &refined_params, order, &all_results, level, concat);
//         }
//         all_results
//     }
// }

// pub struct ParamPlanningResult {
//     pub plan_found: bool,
//     pub plan_length: u32,
//     pub level: u32,
//     pub concat: u32,
//     pub trace: Vec<PlanningFrame>,
//     pub time_to_solve: std::time::Duration,
// }

impl Concatenate {
    pub fn new(results: &Vec<ParamPlanningResult>) -> ParamPlanningResult {
        let conc_plan_found = match results.iter().all(|x| x.plan_found) {
            true => true,
            false => false
        };
        let conc_plan_lenght = results.iter().map(|x| x.plan_length).sum();
        let conc_plan_level = results[1].level;
        let conc_plan_concat:u32 = 123456789;
        let mut conc_plan_trace: Vec<PlanningFrame> = vec!();

        for res in results {
            if results.iter().position(|x| x == res).unwrap() == 0 {
                for tr in res.trace.clone() {
                    conc_plan_trace.push(tr)
                }
            } else {
                for tr in res.trace.clone() {
                    if res.trace.iter().position(|x| *x == tr).unwrap() != 0 {
                        
                        conc_plan_trace.push(tr)
                    }
                }
            }
        };
        let conc_plan_duration = results.iter().map(|x| x.time_to_solve).sum();

        ParamPlanningResult {
            plan_found: conc_plan_found,
            plan_length: conc_plan_lenght,
            level: conc_plan_level,
            concat: conc_plan_concat,
            trace: conc_plan_trace,
            time_to_solve: conc_plan_duration
        }
    }
}


impl Compositional2 {
    pub fn new(result: &ParamPlanningResult,
               problem: &ParamPlanningProblem,
               params: &Vec<(String, bool)>, 
               order: &Vec<&str>, 
               all_results: &Vec<ParamPlanningResult>,
               level: u32) -> Vec<ParamPlanningResult> {
    
        let mut all_results: Vec<ParamPlanningResult> = vec!();

        let current_level = level + 1;
        if !params.iter().all(|x| x.1) {
            
            if result.plan_found {
                let mut inheritance: Vec<String> = vec!() ;
                let mut level_results = vec!();
                let activated_params = &ActivateNextParam::new(&params, &order);
                let mut concat = 0;
                for i in 0..=result.trace.len() - 1 {
                    if i == 0 {
                        let new_problem = ParamPlanningProblem::new(
                            format!("problem_l{:?}_c{:?}", current_level, concat), 
                            problem.vars.clone(),
                            activated_params.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            problem.initial.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            ParamStateToPredicate::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            problem.trans.clone(),
                            problem.ltl_specs.clone(),
                            problem.max_steps);
                        let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
                        
                        // assuming plan is found, handle no plan found later
                        level_results.push(new_result.clone());
                        match new_result.trace.last() {
                            Some(x) => inheritance = x.state.clone(),
                            None => panic!("No tail in the plan!")
                        }
                        concat = concat + 1;                         
                    } else if i == result.trace.len() - 1 {
                        let new_problem = ParamPlanningProblem::new(
                            format!("problem_l{:?}_c{:?}", current_level, concat), 
                            problem.vars.clone(),
                            activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
                            ParamStateToPredicate::new(&inheritance.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            problem.goal.iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            problem.trans.clone(),
                            problem.ltl_specs.clone(),
                            problem.max_steps);
                        let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);
                        
                        // assuming plan is found, handle no plan found later
                        level_results.push(new_result.clone());
                        concat = concat + 1;
                    } else {
                        let new_problem = ParamPlanningProblem::new(
                            format!("problem_l{:?}_c{:?}", current_level, concat), 
                            problem.vars.clone(),
                            activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(),
                            ParamStateToPredicate::new(&inheritance.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            ParamStateToPredicate::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &problem).iter().map(|x| (x.0.as_str(), x.1.clone())).collect(),
                            problem.trans.clone(),
                            problem.ltl_specs.clone(),
                            problem.max_steps);
                        let new_result = ParamSequential::new(&new_problem, &activated_params.iter().map(|x| (x.0.as_str(), x.1)).collect(), current_level, concat);

                        level_results.push(new_result.clone());
                        match new_result.trace.last() {
                            Some(x) => inheritance = x.state.clone(),
                            None => panic!("No tail in the plan!")
                        }
                        concat = concat + 1;   
                    }
                }

                for result in &level_results {
                    // println!("{:?}", level_result);
                    println!("level: {:?}", result.level);
                    println!("concat: {:?}", result.concat);
                    println!("plan_found: {:?}", result.plan_found);
                    println!("plan_lenght: {:?}", result.plan_length);
                    println!("time_to_solve: {:?}", result.time_to_solve);
                    println!("trace: ");
//              
                    for t in &result.trace{
    //              
                        println!("state: {:?}", t.state);
                        println!("trans: {:?}", t.trans);
                        println!("=========================");
                    }
                }

                let concat_res = Concatenate::new(&level_results);
                println!("level: {:?}", concat_res.level);
                    println!("concat: {:?}", concat_res.concat);
                    println!("plan_found: {:?}", concat_res.plan_found);
                    println!("plan_lenght: {:?}", concat_res.plan_length);
                    println!("time_to_solve: {:?}", concat_res.time_to_solve);
                    println!("trace: ");
//              
                for t in &concat_res.trace{
    //             
                    println!("state: {:?}", t.state);
                    println!("trans: {:?}", t.trans);
                    println!("=========================");
                }

                Compositional2::new(&concat_res, &problem, &activated_params, &order, &all_results, current_level);
            }
        }
        all_results
    }
}

impl <'ctx> GetPlanningResultZ3<'ctx> {
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

        // println!("{:#?}", model_vec);

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

impl <'ctx> GetParamPlanningResultZ3<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, model: Z3_model, nr_steps: u32, 
    planning_time: std::time::Duration, plan_found: bool, level: u32, concat: u32) -> ParamPlanningResult {
        let model_str = ModelToStringZ3::new(&ctx, model);
        let mut model_vec = vec!();

        let num = ModelGetNumConstsZ3::new(&ctx, model);
        let mut lines = model_str.lines();
        let mut i: u32 = 0;

        while i < num {
            model_vec.push(lines.next().unwrap_or(""));
            i = i + 1;
        }

        // println!("{:#?}", model_vec);

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

        ParamPlanningResult {
            plan_found: plan_found,
            plan_length: nr_steps - 1,
            level: level,
            concat:concat,
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
fn test_idea_1_iteration_6(){
    
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

    // var group cube, have to think of something better
    let buffer = Variable::new("buffer_cube", "buffer", buffer_domain.clone());
    let gripper = Variable::new("gripper_cube", "gripper", gripper_domain.clone());
    let table = Variable::new("table_cube", "table", table_domain.clone());

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

    let t1 = ParamTransition::new(
        "start_activate",
        vec!(ref_stat.clone()),
        &vec!(
            ("stat", not_stat_active.clone()),
            ("stat", not_set_stat_active.clone())
        ),
        &vec!(
            ("stat", set_stat_active.clone())
        )
    );

    let t2 = ParamTransition::new(
        "finish_activate",
        vec!(act_stat.clone()),
        &vec!(
            ("stat", set_stat_active.clone()),
            ("stat", not_stat_active.clone())
        ),
        &vec!(
            ("stat", stat_active.clone())
        )
    );

    let t3 = ParamTransition::new(
        "start_deactivate",
        vec!(ref_stat.clone()),
        &vec!(
            ("stat", not_stat_idle.clone()),
            ("stat", not_set_stat_idle.clone())
        ),
        &vec!(
            ("stat", set_stat_idle.clone())
        )
    );

    let t4 = ParamTransition::new(
        "finish_deactivate",
        vec!(act_stat.clone()),
        &vec!(
            ("stat", not_stat_idle.clone()),
            ("stat", set_stat_idle.clone())
        ),
        &vec!(
            ("stat", stat_idle.clone())
        )
    );

    let t5 = ParamTransition::new(
        "start_move_to_buffer",
        vec!(ref_pos.clone()),
        &vec!(
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_stable.clone()),
            ("pos", not_pos_buffer.clone()),
            ("pos", not_set_pos_buffer.clone())
        ),
        &vec!(
            ("pos", set_pos_buffer.clone())
        )
    );

    let t6 = ParamTransition::new(
        "finish_move_to_buffer",
        vec!(act_pos.clone()),
        &vec!(
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", not_pos_buffer.clone()),
            ("pos", set_pos_buffer.clone())
        ),
        &vec!(
            ("pos", pos_buffer.clone())
        )
    );

    let t7 = ParamTransition::new(
        "start_move_to_table",
        vec!(ref_pos.clone()),
        &vec!(
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_stable.clone()),
            ("pos", not_pos_table.clone()),
            ("pos", not_set_pos_table.clone())
        ),
        &vec!(
            ("pos", set_pos_table.clone())
        )
    );

    let t8 = ParamTransition::new(
        "finish_move_to_table",
        vec!(act_pos.clone()),
        &vec!(
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", not_pos_table.clone()),
            ("pos", set_pos_table.clone())
        ),
        &vec!(
            ("pos", pos_table.clone())
        )
    );

    let t9 = ParamTransition::new(
        "start_move_to_home",
        vec!(ref_pos.clone()),
        &vec!(
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_stable.clone()),
            ("pos", not_pos_home.clone()),
            ("pos", not_set_pos_home.clone())
        ),
        &vec!(
            ("pos", set_pos_home.clone())
        )
    );

    let t10 = ParamTransition::new(
        "finish_move_to_home",
        vec!(act_pos.clone()),
        &vec!(
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", not_pos_home.clone()),
            ("pos", set_pos_home.clone())
        ),
        &vec!(
            ("pos", pos_home.clone())
        )
    );

    let t11 = ParamTransition::new(
        "take_cube_from_buffer",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &vec!(
            ("cube", buffer_cube.clone()),
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_buffer.clone()),
            ("pos", set_pos_buffer.clone())
        ),
        &vec!(
            ("cube", gripper_cube.clone())
        )
    );

    let t12 = ParamTransition::new(
        "take_cube_from_table",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &vec!(
            ("cube", table_cube.clone()),
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_table.clone()),
            ("pos", set_pos_table.clone())
        ),
        &vec!(
            ("cube", gripper_cube.clone())
        )
    );

    let t13 = ParamTransition::new(
        "leave_cube_at_buffer",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &vec!(
            ("cube", gripper_cube.clone()),
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_buffer.clone()),
            ("pos", set_pos_buffer.clone())
        ),
        &vec!(
            ("cube", buffer_cube.clone())
        )
    );

    let t14 = ParamTransition::new(
        "leave_cube_at_table",
        vec!(gripper.clone(), buffer.clone(), table.clone()),
        &vec!(
            ("cube", gripper_cube.clone()),
            ("stat", stat_active.clone()),
            ("stat", set_stat_active.clone()),
            ("pos", pos_table.clone()),
            ("pos", set_pos_table.clone())
        ),
        &vec!(
            ("cube", table_cube.clone())
        )
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
    
    // 4. no ball in the system:
    let s4 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::OR(
                        vec!(
                            buffer_ball.clone(),
                            table_ball.clone(),
                            gripper_ball.clone()
                        )
                    )
                )
            )
        )
    );
    
    let initial = vec!(
        ("pos", pos_stable.clone()),
        ("pos", pos_buffer.clone()),
        ("stat", stat_stable.clone()),
        ("stat", stat_idle.clone()),
        ("cube", table_cube.clone())
    );

    let goal = vec!(
        ("pos", pos_table.clone()),
        ("stat", stat_idle.clone()),
        ("cube", buffer_cube.clone())
    );
    
    let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), true), ("stat".to_string(), true), ("cube".to_string(), true));
    let refining_order: Vec<&str> = vec!("pos", "stat", "cube"); // opposite for some reason? fix this
    let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);
    let specs = Predicate::AND(vec!(s1, s2, s3, s4));

    let mut concat: u32 = 0;
    let mut level: u32 = 0;

    let problem = ParamPlanningProblem::new(
        String::from("param_prob_1"), 
        all_vars.clone(),
        act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
        initial.clone(),
        goal.clone(),
        trans.clone(), 
        specs.clone(),
        30
    );

    let now = Instant::now();

    let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

    let seq_planning_time = now.elapsed();

    println!("level: {:?}", result.level);
    println!("concat: {:?}", result.concat);
    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
    println!("trace: ");

    for t in &result.trace{
 
        println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        println!("=========================");
    }

    let mut act: Vec<(String, bool)> = vec!(("pos".to_string(), false), ("stat".to_string(), false), ("cube".to_string(), true));

    let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

    println!("level: {:?}", result.level);
    println!("concat: {:?}", result.concat);
    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
    println!("trace: ");

    for t in &result.trace{
 
        println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        println!("=========================");
    }

    let now = Instant::now();
        
    let solutions = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

    let comp_planning_time = now.elapsed();

    println!("TOTAL SEQUENTIAL TIME: {:?}", seq_planning_time);
    println!("TOTAL COMPOSITIONAL TIME: {:?}", comp_planning_time);

    // println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    // let act = ActivateNextParam::new(&act, &refining_order);
    // let solutions = GenerateAndSolveLevel::new(&result, &problem, &act, 1);

    // for s in solutions.clone() {
    //     println!("level: {:?}", 1);
    //     println!("concat: {:?}", concat);
    //     println!("plan_found: {:?}", s.plan_found);
    //     println!("plan_lenght: {:?}", s.plan_length);
    //     println!("time_to_solve: {:?}", s.time_to_solve);
    //     println!("trace: ");

    //     for t in &s.trace{
        
    //         println!("state: {:?}", t.state);
    //         println!("trans: {:?}", t.trans);
    //         println!("=========================");
    //     }
    //     concat = concat + 1;
    // }

    // concat = 0;
    // println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    // let mut solutions2 = vec!();
    // let act = ActivateNextParam::new(&act, &refining_order);
    // for sol in solutions.clone() {
    //     solutions2.extend(GenerateAndSolveLevel::new(&sol, &problem, &act, 1));
    // }
    // // let solutions = GenerateAndSolveLevel::new(&result, &problem, &act, 1);

    // for s in solutions2 {
    //     println!("level: {:?}", 2);
    //     println!("concat: {:?}", concat);
    //     println!("plan_found: {:?}", s.plan_found);
    //     println!("plan_lenght: {:?}", s.plan_length);
    //     println!("time_to_solve: {:?}", s.time_to_solve);
    //     println!("trace: ");

    //     for t in &s.trace{
        
    //         println!("state: {:?}", t.state);
    //         println!("trans: {:?}", t.trans);
    //         println!("=========================");
    //     }
    //     concat = concat +1;
    // }
}

#[test]
fn test_idea_1_iteration_7(){
    
    let r1_pose_domain = vec!("buffer", "home", "table_1", "table_2");
    let r2_pose_domain = vec!("buffer", "home", "table_1", "table_2");
    let r1_stat_domain = vec!("active", "idle");
    let r2_stat_domain = vec!("active", "idle");
    let buffer_1_domain = vec!("cube", "ball", "empty");
    let gripper_1_domain = vec!("cube", "ball", "empty");
    let table_1_domain = vec!("cube", "ball", "empty");
    let buffer_2_domain = vec!("cube", "ball", "empty");
    let gripper_2_domain = vec!("cube", "ball", "empty");
    let table_2_domain = vec!("cube", "ball", "empty");

    // r1 var group pos
    let r1_act_pos = Variable::new("r1_act_pos", "r1_pose", r1_pose_domain.clone());
    let r1_ref_pos = Variable::new("r1_ref_pos", "r1_pose", r1_pose_domain.clone());

    // r2 var group pos
    let r2_act_pos = Variable::new("r2_act_pos", "r2_pose", r2_pose_domain.clone());
    let r2_ref_pos = Variable::new("r2_ref_pos", "r2_pose", r2_pose_domain.clone());

    // r1 var group stat
    let r1_act_stat = Variable::new("r1_act_stat", "r1_status", r1_stat_domain.clone());
    let r1_ref_stat = Variable::new("r1_ref_stat", "r1_status", r1_stat_domain.clone());

    // r2 var group stat
    let r2_act_stat = Variable::new("r2_act_stat", "r2_status", r2_stat_domain.clone());
    let r2_ref_stat = Variable::new("r2_ref_stat", "r2_status", r2_stat_domain.clone());

    // var group prod, have to think of something better, maybe also separate groups into ball and cube
    let buffer_1 = Variable::new("buffer_1", "buffer_1", buffer_1_domain.clone());
    let gripper_1 = Variable::new("gripper__1", "gripper_1", gripper_1_domain.clone());
    let table_1 = Variable::new("table_1", "table_1", table_1_domain.clone());
    let buffer_2 = Variable::new("buffer_2", "buffer_2", buffer_2_domain.clone());
    let gripper_2 = Variable::new("gripper_2", "gripper_2", gripper_2_domain.clone());
    let table_2 = Variable::new("table_2", "table_2", table_2_domain.clone());

    // r1 act stat predicates
    let r1_stat_active = Predicate::EQVAL(r1_act_stat.clone(), String::from("active"));
    let r1_stat_idle = Predicate::EQVAL(r1_act_stat.clone(), String::from("idle"));
    let r1_not_stat_active = Predicate::NOT(vec!(r1_stat_active.clone()));
    let r1_not_stat_idle = Predicate::NOT(vec!(r1_stat_idle.clone()));

    // r2 act stat predicates
    let r2_stat_active = Predicate::EQVAL(r2_act_stat.clone(), String::from("active"));
    let r2_stat_idle = Predicate::EQVAL(r2_act_stat.clone(), String::from("idle"));
    let r2_not_stat_active = Predicate::NOT(vec!(r2_stat_active.clone()));
    let r2_not_stat_idle = Predicate::NOT(vec!(r2_stat_idle.clone()));

    // r1 ref stat predicates
    let r1_set_stat_active = Predicate::EQVAL(r1_ref_stat.clone(), String::from("active"));
    let r1_set_stat_idle = Predicate::EQVAL(r1_ref_stat.clone(), String::from("idle"));
    let r1_not_set_stat_active = Predicate::NOT(vec!(r1_set_stat_active.clone()));
    let r1_not_set_stat_idle = Predicate::NOT(vec!(r1_set_stat_idle.clone()));

    // r2 ref stat predicates
    let r2_set_stat_active = Predicate::EQVAL(r2_ref_stat.clone(), String::from("active"));
    let r2_set_stat_idle = Predicate::EQVAL(r2_ref_stat.clone(), String::from("idle"));
    let r2_not_set_stat_active = Predicate::NOT(vec!(r2_set_stat_active.clone()));
    let r2_not_set_stat_idle = Predicate::NOT(vec!(r2_set_stat_idle.clone()));

    // r1 act pos predicates
    let r1_pos_buffer = Predicate::EQVAL(r1_act_pos.clone(), String::from("buffer"));
    let r1_pos_table_1 = Predicate::EQVAL(r1_act_pos.clone(), String::from("table_1"));
    let r1_pos_table_2 = Predicate::EQVAL(r1_act_pos.clone(), String::from("table_2"));
    let r1_pos_home = Predicate::EQVAL(r1_act_pos.clone(), String::from("home"));
    let r1_not_pos_buffer = Predicate::NOT(vec!(r1_pos_buffer.clone()));
    let r1_not_pos_table_1 = Predicate::NOT(vec!(r1_pos_table_1.clone()));
    let r1_not_pos_table_2 = Predicate::NOT(vec!(r1_pos_table_1.clone()));
    let r1_not_pos_home = Predicate::NOT(vec!(r1_pos_home.clone()));

    // r2 act pos predicates
    let r2_pos_buffer = Predicate::EQVAL(r2_act_pos.clone(), String::from("buffer"));
    let r2_pos_table_1 = Predicate::EQVAL(r2_act_pos.clone(), String::from("table_1"));
    let r2_pos_table_2 = Predicate::EQVAL(r2_act_pos.clone(), String::from("table_2"));
    let r2_pos_home = Predicate::EQVAL(r2_act_pos.clone(), String::from("home"));
    let r2_not_pos_buffer = Predicate::NOT(vec!(r2_pos_buffer.clone()));
    let r2_not_pos_table_1 = Predicate::NOT(vec!(r2_pos_table_1.clone()));
    let r2_not_pos_table_2 = Predicate::NOT(vec!(r2_pos_table_2.clone()));
    let r2_not_pos_home = Predicate::NOT(vec!(r2_pos_home.clone()));

    // r1 ref pos predicates
    let r1_set_pos_buffer = Predicate::EQVAL(r1_ref_pos.clone(), String::from("buffer"));
    let r1_set_pos_table_1 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("table_1"));
    let r1_set_pos_table_2 = Predicate::EQVAL(r1_ref_pos.clone(), String::from("table_2"));
    let r1_set_pos_home = Predicate::EQVAL(r1_ref_pos.clone(), String::from("home"));
    let r1_not_set_pos_buffer = Predicate::NOT(vec!(r1_set_pos_buffer.clone()));
    let r1_not_set_pos_table_1 = Predicate::NOT(vec!(r1_set_pos_table_1.clone()));
    let r1_not_set_pos_table_2 = Predicate::NOT(vec!(r1_set_pos_table_2.clone()));
    let r1_not_set_pos_home = Predicate::NOT(vec!(r1_set_pos_home.clone()));

    // r2 ref pos predicates
    let r2_set_pos_buffer = Predicate::EQVAL(r2_ref_pos.clone(), String::from("buffer"));
    let r2_set_pos_table_1 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("table_1"));
    let r2_set_pos_table_2 = Predicate::EQVAL(r2_ref_pos.clone(), String::from("table_2"));
    let r2_set_pos_home = Predicate::EQVAL(r2_ref_pos.clone(), String::from("home"));
    let r2_not_set_pos_buffer = Predicate::NOT(vec!(r2_set_pos_buffer.clone()));
    let r2_not_set_pos_table_1 = Predicate::NOT(vec!(r2_set_pos_table_1.clone()));
    let r2_not_set_pos_table_2 = Predicate::NOT(vec!(r2_set_pos_table_2.clone()));
    let r2_not_set_pos_home = Predicate::NOT(vec!(r2_set_pos_home.clone()));

    // act buffer 1 predicates
    let buffer_1_cube = Predicate::EQVAL(buffer_1.clone(), String::from("cube"));
    let buffer_1_ball = Predicate::EQVAL(buffer_1.clone(), String::from("ball"));
    let buffer_1_empty = Predicate::EQVAL(buffer_1.clone(), String::from("empty"));
    let not_buffer_1_cube = Predicate::NOT(vec!(buffer_1_cube.clone()));
    let not_buffer_1_ball = Predicate::NOT(vec!(buffer_1_ball.clone()));
    let not_buffer_1_empty = Predicate::NOT(vec!(buffer_1_empty.clone()));

    // act buffer 2 predicates
    let buffer_2_cube = Predicate::EQVAL(buffer_2.clone(), String::from("cube"));
    let buffer_2_ball = Predicate::EQVAL(buffer_2.clone(), String::from("ball"));
    let buffer_2_empty = Predicate::EQVAL(buffer_2.clone(), String::from("empty"));
    let not_buffer_2_cube = Predicate::NOT(vec!(buffer_2_cube.clone()));
    let not_buffer_2_ball = Predicate::NOT(vec!(buffer_2_ball.clone()));
    let not_buffer_2_empty = Predicate::NOT(vec!(buffer_2_empty.clone()));
    
    // act gripper 1 predicates
    let gripper_1_cube = Predicate::EQVAL(gripper_1.clone(), String::from("cube"));
    let gripper_1_ball = Predicate::EQVAL(gripper_1.clone(), String::from("ball"));
    let gripper_1_empty = Predicate::EQVAL(gripper_1.clone(), String::from("empty"));
    let not_gripper_1_cube = Predicate::NOT(vec!(gripper_1_cube.clone()));
    let not_gripper_1_ball = Predicate::NOT(vec!(gripper_1_ball.clone()));
    let not_gripper_1_empty = Predicate::NOT(vec!(gripper_1_empty.clone()));

    // act gripper 2 predicates
    let gripper_2_cube = Predicate::EQVAL(gripper_2.clone(), String::from("cube"));
    let gripper_2_ball = Predicate::EQVAL(gripper_2.clone(), String::from("ball"));
    let gripper_2_empty = Predicate::EQVAL(gripper_2.clone(), String::from("empty"));
    let not_gripper_2_cube = Predicate::NOT(vec!(gripper_2_cube.clone()));
    let not_gripper_2_ball = Predicate::NOT(vec!(gripper_2_ball.clone()));
    let not_gripper_2_empty = Predicate::NOT(vec!(gripper_2_empty.clone()));

    // act table 1 predicates
    let table_1_cube = Predicate::EQVAL(table_1.clone(), String::from("cube"));
    let table_1_ball = Predicate::EQVAL(table_1.clone(), String::from("ball"));
    let table_1_empty = Predicate::EQVAL(table_1.clone(), String::from("empty"));
    let not_table_1_cube = Predicate::NOT(vec!(table_1_cube.clone()));
    let not_table_1_ball = Predicate::NOT(vec!(table_1_ball.clone()));
    let not_table_1_empty = Predicate::NOT(vec!(table_1_empty.clone()));

    // act table 2 predicates
    let table_2_cube = Predicate::EQVAL(table_2.clone(), String::from("cube"));
    let table_2_ball = Predicate::EQVAL(table_2.clone(), String::from("ball"));
    let table_2_empty = Predicate::EQVAL(table_2.clone(), String::from("empty"));
    let not_table_2_cube = Predicate::NOT(vec!(table_2_cube.clone()));
    let not_table_2_ball = Predicate::NOT(vec!(table_2_ball.clone()));
    let not_table_2_empty = Predicate::NOT(vec!(table_2_empty.clone()));

    // r1 are ref == act predicates
    let r1_pos_stable = Predicate::EQVAR(r1_act_pos.clone(), r1_ref_pos.clone());
    let r1_stat_stable = Predicate::EQVAR(r1_act_stat.clone(), r1_ref_stat.clone());
    let r1_not_pos_stable = Predicate::NEQVAR(r1_act_pos.clone(), r1_ref_pos.clone());
    let r1_not_stat_stable = Predicate::NEQVAR(r1_act_stat.clone(), r1_ref_stat.clone());

    // r2 are ref == act predicates
    let r2_pos_stable = Predicate::EQVAR(r2_act_pos.clone(), r2_ref_pos.clone());
    let r2_stat_stable = Predicate::EQVAR(r2_act_stat.clone(), r2_ref_stat.clone());
    let r2_not_pos_stable = Predicate::NEQVAR(r2_act_pos.clone(), r2_ref_pos.clone());
    let r2_not_stat_stable = Predicate::NEQVAR(r2_act_stat.clone(), r2_ref_stat.clone());

    // variables in the problem
    let all_vars = vec!(
        r1_act_pos.clone(), 
        r1_ref_pos.clone(), 
        r2_act_pos.clone(), 
        r2_ref_pos.clone(), 
        r1_act_stat.clone(), 
        r1_ref_stat.clone(),
        r2_act_stat.clone(), 
        r2_ref_stat.clone(),
        buffer_1.clone(), 
        gripper_1.clone(), 
        table_1.clone(),
        buffer_2.clone(), 
        gripper_2.clone(), 
        table_2.clone()
    );

    let t1 = ParamTransition::new(
        "r1_start_activate",
        vec!(r1_ref_stat.clone()),
        &vec!(
            ("r1_stat", r1_not_stat_active.clone()),
            ("r1_stat", r1_not_set_stat_active.clone())
        ),
        &vec!(
            ("r1_stat", r1_set_stat_active.clone())
        )
    );

    let t2 = ParamTransition::new(
        "r1_finish_activate",
        vec!(r1_act_stat.clone()),
        &vec!(
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_stat", r1_not_stat_active.clone())
        ),
        &vec!(
            ("r1_stat", r1_stat_active.clone())
        )
    );

    let t3 = ParamTransition::new(
        "r1_start_deactivate",
        vec!(r1_ref_stat.clone()),
        &vec!(
            ("r1_stat", r1_not_stat_idle.clone()),
            ("r1_stat", r1_not_set_stat_idle.clone())
        ),
        &vec!(
            ("r1_stat", r1_set_stat_idle.clone())
        )
    );

    let t4 = ParamTransition::new(
        "r1_finish_deactivate",
        vec!(r1_act_stat.clone()),
        &vec!(
            ("r1_stat", r1_not_stat_idle.clone()),
            ("r1_stat", r1_set_stat_idle.clone())
        ),
        &vec!(
            ("r1_stat", r1_stat_idle.clone())
        )
    );

    let t5 = ParamTransition::new(
        "r1_start_move_to_buffer",
        vec!(r1_ref_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_stable.clone()),
            ("r1_pos", r1_not_pos_buffer.clone()),
            ("r1_pos", r1_not_set_pos_buffer.clone())
        ),
        &vec!(
            ("r1_pos", r1_set_pos_buffer.clone())
        )
    );

    let t6 = ParamTransition::new(
        "r1_finish_move_to_buffer",
        vec!(r1_act_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_not_pos_buffer.clone()),
            ("r1_pos", r1_set_pos_buffer.clone())
        ),
        &vec!(
            ("r1_pos", r1_pos_buffer.clone())
        )
    );

    let t7 = ParamTransition::new(
        "r1_start_move_to_table_1",
        vec!(r1_ref_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_stable.clone()),
            ("r1_pos", r1_not_pos_table_1.clone()),
            ("r1_pos", r1_not_set_pos_table_1.clone())
        ),
        &vec!(
            ("r1_pos", r1_set_pos_table_1.clone())
        )
    );

    let t8 = ParamTransition::new(
        "r1_finish_move_to_table_1",
        vec!(r1_act_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_not_pos_table_1.clone()),
            ("r1_pos", r1_set_pos_table_1.clone())
        ),
        &vec!(
            ("r1_pos", r1_pos_table_1.clone())
        )
    );

    let t9 = ParamTransition::new(
        "r1_start_move_to_table_2",
        vec!(r1_ref_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_stable.clone()),
            ("r1_pos", r1_not_pos_table_2.clone()),
            ("r1_pos", r1_not_set_pos_table_2.clone())
        ),
        &vec!(
            ("r1_pos", r1_set_pos_table_2.clone())
        )
    );

    let t10 = ParamTransition::new(
        "r1_finish_move_to_table_2",
        vec!(r1_act_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_not_pos_table_2.clone()),
            ("r1_pos", r1_set_pos_table_2.clone())
        ),
        &vec!(
            ("r1_pos", r1_pos_table_2.clone())
        )
    );

    let t11 = ParamTransition::new(
        "r1_start_move_to_home",
        vec!(r1_ref_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_stable.clone()),
            ("r1_pos", r1_not_pos_home.clone()),
            ("r1_pos", r1_not_set_pos_home.clone())
        ),
        &vec!(
            ("r1_pos", r1_set_pos_home.clone())
        )
    );

    let t12 = ParamTransition::new(
        "r1_finish_move_to_home",
        vec!(r1_act_pos.clone()),
        &vec!(
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_not_pos_home.clone()),
            ("r1_pos", r1_set_pos_home.clone())
        ),
        &vec!(
            ("r1_pos", r1_pos_home.clone())
        )
    );

    let t13 = ParamTransition::new(
        "r1_take_cube_from_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", buffer_1_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_buffer.clone()),
            ("r1_pos", r1_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", gripper_1_cube.clone())
        )
    );

    let t14 = ParamTransition::new(
        "r1_take_cube_from_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_1_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_1.clone()),
            ("r1_pos", r1_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", gripper_1_cube.clone())
        )
    );

    let t15 = ParamTransition::new(
        "r1_take_cube_from_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_2_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_2.clone()),
            ("r1_pos", r1_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", gripper_1_cube.clone())
        )
    );

    let t16 = ParamTransition::new(
        "r1_leave_cube_at_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_1_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_buffer.clone()),
            ("r1_pos", r1_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", buffer_1_cube.clone())
        )
    );

    let t17 = ParamTransition::new(
        "r1_leave_cube_at_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_1_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_1.clone()),
            ("r1_pos", r1_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", table_1_cube.clone())
        )
    );

    let t18 = ParamTransition::new(
        "r1_leave_cube_at_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_1_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_2.clone()),
            ("r1_pos", r1_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", table_2_cube.clone())
        )
    );

    let t19 = ParamTransition::new(
        "r2_start_activate",
        vec!(r1_ref_stat.clone()),
        &vec!(
            ("r2_stat", r2_not_stat_active.clone()),
            ("r2_stat", r2_not_set_stat_active.clone())
        ),
        &vec!(
            ("r2_stat", r2_set_stat_active.clone())
        )
    );

    let t20 = ParamTransition::new(
        "r2_finish_activate",
        vec!(r2_act_stat.clone()),
        &vec!(
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_stat", r2_not_stat_active.clone())
        ),
        &vec!(
            ("r2_stat", r2_stat_active.clone())
        )
    );

    let t21 = ParamTransition::new(
        "r2_start_deactivate",
        vec!(r2_ref_stat.clone()),
        &vec!(
            ("r2_stat", r2_not_stat_idle.clone()),
            ("r2_stat", r2_not_set_stat_idle.clone())
        ),
        &vec!(
            ("r2_stat", r2_set_stat_idle.clone())
        )
    );

    let t22 = ParamTransition::new(
        "r2_finish_deactivate",
        vec!(r2_act_stat.clone()),
        &vec!(
            ("r2_stat", r2_not_stat_idle.clone()),
            ("r2_stat", r2_set_stat_idle.clone())
        ),
        &vec!(
            ("r2_stat", r2_stat_idle.clone())
        )
    );

    let t23 = ParamTransition::new(
        "r2_start_move_to_buffer",
        vec!(r2_ref_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_stable.clone()),
            ("r2_pos", r2_not_pos_buffer.clone()),
            ("r2_pos", r2_not_set_pos_buffer.clone())
        ),
        &vec!(
            ("r2_pos", r2_set_pos_buffer.clone())
        )
    );

    let t24 = ParamTransition::new(
        "r2_finish_move_to_buffer",
        vec!(r2_act_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_not_pos_buffer.clone()),
            ("r2_pos", r2_set_pos_buffer.clone())
        ),
        &vec!(
            ("r2_pos", r2_pos_buffer.clone())
        )
    );

    let t25 = ParamTransition::new(
        "r2_start_move_to_table_1",
        vec!(r2_ref_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_stable.clone()),
            ("r2_pos", r2_not_pos_table_1.clone()),
            ("r2_pos", r2_not_set_pos_table_1.clone())
        ),
        &vec!(
            ("r2_pos", r2_set_pos_table_1.clone())
        )
    );

    let t26 = ParamTransition::new(
        "r2_finish_move_to_table_1",
        vec!(r2_act_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_not_pos_table_1.clone()),
            ("r2_pos", r2_set_pos_table_1.clone())
        ),
        &vec!(
            ("r2_pos", r2_pos_table_1.clone())
        )
    );

    let t27 = ParamTransition::new(
        "r2_start_move_to_table_2",
        vec!(r2_ref_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_stable.clone()),
            ("r2_pos", r2_not_pos_table_2.clone()),
            ("r2_pos", r2_not_set_pos_table_2.clone())
        ),
        &vec!(
            ("r2_pos", r2_set_pos_table_2.clone())
        )
    );

    let t28 = ParamTransition::new(
        "r2_finish_move_to_table_2",
        vec!(r2_act_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_not_pos_table_2.clone()),
            ("r2_pos", r2_set_pos_table_2.clone())
        ),
        &vec!(
            ("r2_pos", r2_pos_table_2.clone())
        )
    );

    let t29 = ParamTransition::new(
        "r2_start_move_to_home",
        vec!(r2_ref_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_stable.clone()),
            ("r2_pos", r2_not_pos_home.clone()),
            ("r2_pos", r2_not_set_pos_home.clone())
        ),
        &vec!(
            ("r2_pos", r2_set_pos_home.clone())
        )
    );

    let t30 = ParamTransition::new(
        "r2_finish_move_to_home",
        vec!(r2_act_pos.clone()),
        &vec!(
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_not_pos_home.clone()),
            ("r2_pos", r2_set_pos_home.clone())
        ),
        &vec!(
            ("r2_pos", r2_pos_home.clone())
        )
    );

    let t31 = ParamTransition::new(
        "r2_take_cube_from_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", buffer_2_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_buffer.clone()),
            ("r2_pos", r2_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", gripper_2_cube.clone())
        )
    );

    let t32 = ParamTransition::new(
        "r2_take_cube_from_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_1_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_1.clone()),
            ("r2_pos", r2_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", gripper_2_cube.clone())
        )
    );

    let t33 = ParamTransition::new(
        "r2_take_cube_from_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_2_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_2.clone()),
            ("r2_pos", r2_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", gripper_2_cube.clone())
        )
    );

    let t34 = ParamTransition::new(
        "r2_leave_cube_at_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_2_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_buffer.clone()),
            ("r2_pos", r2_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", buffer_2_cube.clone())
        )
    );

    let t35 = ParamTransition::new(
        "r2_leave_cube_at_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_2_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_1.clone()),
            ("r2_pos", r2_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", table_1_cube.clone())
        )
    );

    let t36 = ParamTransition::new(
        "r2_leave_cube_at_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_2_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_2.clone()),
            ("r2_pos", r2_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", table_2_cube.clone())
        )
    );

    let t37 = ParamTransition::new(
        "r2_take_ball_from_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", buffer_2_ball.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_buffer.clone()),
            ("r2_pos", r2_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", gripper_2_ball.clone())
        )
    );

    let t38 = ParamTransition::new(
        "r2_take_ball_from_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_1_ball.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_1.clone()),
            ("r2_pos", r2_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", gripper_2_ball.clone())
        )
    );

    let t39 = ParamTransition::new(
        "r2_take_ball_from_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_2_ball.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_2.clone()),
            ("r2_pos", r2_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", gripper_2_ball.clone())
        )
    );

    let t40 = ParamTransition::new(
        "r2_leave_ball_at_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_2_ball.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_buffer.clone()),
            ("r2_pos", r2_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", buffer_2_ball.clone())
        )
    );

    let t41 = ParamTransition::new(
        "r2_leave_ball_at_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_2_ball.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_1.clone()),
            ("r2_pos", r2_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", table_1_ball.clone())
        )
    );

    let t42 = ParamTransition::new(
        "r2_leave_ball_at_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_2_cube.clone()),
            ("r2_stat", r2_stat_active.clone()),
            ("r2_stat", r2_set_stat_active.clone()),
            ("r2_pos", r2_pos_table_2.clone()),
            ("r2_pos", r2_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", table_2_ball.clone())
        )
    );

    let t43 = ParamTransition::new(
        "r1_take_ball_from_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", buffer_1_ball.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_buffer.clone()),
            ("r1_pos", r1_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", gripper_1_ball.clone())
        )
    );

    let t44 = ParamTransition::new(
        "r1_take_ball_from_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_1_ball.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_1.clone()),
            ("r1_pos", r1_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", gripper_1_ball.clone())
        )
    );

    let t45 = ParamTransition::new(
        "r1_take_ball_from_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", table_2_ball.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_2.clone()),
            ("r1_pos", r1_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", gripper_1_ball.clone())
        )
    );

    let t46 = ParamTransition::new(
        "r1_leave_ball_at_buffer",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_1_ball.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_buffer.clone()),
            ("r1_pos", r1_set_pos_buffer.clone())
        ),
        &vec!(
            ("prod", buffer_1_ball.clone())
        )
    );

    let t47 = ParamTransition::new(
        "r1_leave_ball_at_table_1",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_1_ball.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_1.clone()),
            ("r1_pos", r1_set_pos_table_1.clone())
        ),
        &vec!(
            ("prod", table_1_ball.clone())
        )
    );

    let t48 = ParamTransition::new(
        "r1_leave_ball_at_table_2",
        vec!(gripper_1.clone(), buffer_1.clone(), table_1.clone(), gripper_2.clone(), buffer_2.clone(), table_2.clone()),
        &vec!(
            ("prod", gripper_1_cube.clone()),
            ("r1_stat", r1_stat_active.clone()),
            ("r1_stat", r1_set_stat_active.clone()),
            ("r1_pos", r1_pos_table_2.clone()),
            ("r1_pos", r1_set_pos_table_2.clone())
        ),
        &vec!(
            ("prod", table_2_ball.clone())
        )
    );

    // 1. r1 has to go through the "home" pose:
    let s1 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("table_1"))
                        ),
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("buffer"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 1. r1 has to go through the "home" pose:
    let s2 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("table_2"))
                        ),
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("buffer"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );
    
    // 1. r1 has to go through the "home" pose:
    let s3 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("buffer"))
                        ),
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("table_1"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 1. r1 has to go through the "home" pose:
    let s4 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("buffer"))
                        ),
                        vec!(
                            Predicate::EQVAL(r1_act_pos.clone(), String::from("table_2"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 1. r2 has to go through the "home" pose:
    let s5 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("table_1"))
                        ),
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("buffer"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 1. r12 has to go through the "home" pose:
    let s6 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("table_2"))
                        ),
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("buffer"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );
    
    // 1. r2 has to go through the "home" pose:
    let s7 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("buffer"))
                        ),
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("table_1"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );

    // 1. r2 has to go through the "home" pose:
    let s8 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AFTER(
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("buffer"))
                        ),
                        vec!(
                            Predicate::EQVAL(r2_act_pos.clone(), String::from("table_2"))
                        ),
                        2 // how can this be improved so that the plan also holds for bigger a number
                    )
                )
            )
        )
    );
    
    // one cube in the system:
    let s9 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    buffer_1_cube.clone(),
                    buffer_2_cube.clone(),
                    gripper_1_cube.clone(),
                    gripper_2_cube.clone(),
                    table_1_cube.clone(),
                    table_2_cube.clone()
                ),
                1
            )
        )
    );

    // one ball in the system:
    let s10 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    buffer_1_ball.clone(),
                    buffer_2_ball.clone(),
                    gripper_1_ball.clone(),
                    gripper_2_ball.clone(),
                    table_1_ball.clone(),
                    table_2_ball.clone()
                ),
                1
            )
        )
    );

    // ball and cube can't be in the same place at the same time:
    let s11 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    buffer_1_cube.clone(),
                    buffer_1_ball.clone()
                ),
                1
            )
        )
    );

    let s12 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    buffer_2_cube.clone(),
                    buffer_2_ball.clone()
                ),
                1
            )
        )
    );

    let s13 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    table_1_cube.clone(),
                    table_1_ball.clone()
                ),
                1
            )
        )
    );

    let s14 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    table_2_cube.clone(),
                    table_2_ball.clone()
                ),
                1
            )
        )
    );

    let s15 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    gripper_1_cube.clone(),
                    gripper_1_ball.clone()
                ),
                1
            )
        )
    );

    let s16 = Predicate::GLOB(
        vec!(
            Predicate::PBEQ(
                vec!(
                    gripper_2_cube.clone(),
                    gripper_2_ball.clone()
                ),
                1
            )
        )
    );

    let initial = vec!(
        // ("r1_pos", r1_pos_stable.clone()),
        // ("r1_pos", r1_pos_buffer.clone()),
        // ("r2_pos", r2_pos_stable.clone()),
        // ("r2_pos", r2_pos_buffer.clone()),
        // ("r1_stat", r1_stat_stable.clone()),
        // ("r1_stat", r1_stat_idle.clone()),
        // ("r2_stat", r2_stat_stable.clone()),
        // ("r2_stat", r2_stat_idle.clone()),
        ("prod", table_1_cube.clone()),
        ("prod", table_2_ball.clone())
    );

    let goal = vec!(
        // ("r1_pos", r1_pos_table_1.clone()),
        // ("r1_stat", r1_stat_idle.clone()),
        // ("r2_pos", r2_pos_table_2.clone()),
        // ("r2_stat", r2_stat_idle.clone()),
        ("prod", buffer_1_cube.clone()),
        ("prod", buffer_2_ball.clone())
    );
    
    let mut act: Vec<(String, bool)> = vec!(
        ("r1_pos".to_string(), true), 
        ("r1_stat".to_string(), true), 
        ("r2_pos".to_string(), true), 
        ("r2_stat".to_string(), true), 
        ("prod".to_string(), true));

    let refining_order: Vec<&str> = vec!("r1_pos", "r2_pos", "r1_stat", "r2_stat", "prod"); // opposite for some reason? fix this
    let trans = vec!(
        t1, t2, t3, t4, t5, t6, t7, t8, t9, t10,
        t11, t12, t13, t14, t15, t16, t17, t18, t19, t20,
        t21, t22, t23, t24, t25, t26, t27, t28, t29, t30,
        t31, t32, t33, t34, t35, t36, t37, t38, t39, t40,
        t41, t42, t43, t44, t45, t46, t47, t48);

    let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16));

    let mut concat: u32 = 0;
    let mut level: u32 = 0;

    let problem = ParamPlanningProblem::new(
        String::from("param_prob_1"), 
        all_vars.clone(),
        act.clone().iter().map(|x| (x.0.as_str(), x.1)).collect(),
        initial.clone(),
        goal.clone(),
        trans.clone(), 
        specs.clone(),
        50
    );

    let result = ParamSequential::new(&problem, &act.iter().map(|x| (x.0.as_str(), x.1)).collect(), 0, 0);

    println!("level: {:?}", result.level);
    println!("concat: {:?}", result.concat);
    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
    println!("trace: ");

    for t in &result.trace{
 
        println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        println!("=========================");
    }

    // let solutions = Compositional2::new(&result, &problem, &act, &refining_order, &vec!(result.clone()), level);

}