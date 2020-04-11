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
    GLOB(Vec<Predicate>),
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
            Predicate::GLOB(x) => GloballyZ3::new(&ctx, &x, step)
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

// convert an ast formula (predicate) to cnf and remove clauses that don't contain any variable from the filtering vector
impl <'ctx> Abstract<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, v: &Vec<Variable>, p: Z3_ast) -> Z3_ast {

        if v.len() != 0 {
            let cnf = GetCnfVectorZ3::new(&ctx, vec!(p));
            let mut filtered: Vec<Z3_ast> = vec!();

            for a in cnf {
                for var in v {
                    if ast_to_string_z3!(&ctx, a).contains(&var.n){
                        filtered.push(a)
                    }
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

// refine a predicate by ANDing it with a vector of variables that have values from the original problem
// this ia basically applied to init, goal and invariants
// does this have to be done on the z3 level because od the to_cnf function? yes, then its
// basically going to be the abstract but with a bigger abstraction vector
// impl Refine {
//     pub fn new(predicate: &Predicate,
//                old_vector: &Vec<Variable>, 
//                refinement_vector: &Vec<Variable>, 
//                original_problem: &PlanningProblem) -> Predicate {

//                    let mut refinement = vec!();
//                     for r_var in refinement_vector {
//                         if !old_vector.contains(r_var) {
//                             refinement.push(r_var);
//                         }
//                     }
//                 Predicate::AND(refinement.push(predicate))
//                }
// }

// name: String,
//     vars: Vec<Variable>,
//     initial: Predicate,
//     goal: Predicate,
//     trans: Vec<Transition>,
//     ltl_specs: Predicate,
//     max_steps: u32

// let act_pos = Variable::new("act_pos", "pose", pose_domain.clone());

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

#[test]
fn test_sequential_2(){
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

    let vars = vec!(act_pos.clone(), ref_pos.clone(), act_stat.clone(), ref_stat.clone(),
        buffer.clone(), gripper.clone(), table.clone());

    let move_enabled = Predicate::EQVAR(act_pos.clone(), ref_pos.clone());
    let stat_enabled = Predicate::EQVAR(act_stat.clone(), ref_stat.clone());
    let move_executing = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
    let stat_executing = Predicate::NEQVAR(act_stat.clone(), ref_stat.clone());

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
        &stat_enabled,
        &Predicate::EQVAL(ref_stat.clone(), String::from("active"))
    );

    let t2 = Transition::new(
        "finish_activate", 
        vec!(act_stat.clone()), 
        &stat_executing,
        &Predicate::EQVAL(act_stat.clone(), String::from("active"))
    );

    let t3 = Transition::new(
        "start_deactivate", 
        vec!(ref_stat.clone()), 
        &stat_enabled,
        &Predicate::EQVAL(ref_stat.clone(), String::from("idle"))
    );
    
    let t4 = Transition::new(
        "finish_deactivate", 
        vec!(act_stat.clone()), 
        &stat_executing,
        &Predicate::EQVAL(act_stat.clone(), String::from("idle"))
    );

    // moving the robot
    let t5 = Transition::new(
        "start_move_to_buffer",
        vec!(ref_pos.clone()),
        &move_enabled,
        &Predicate::EQVAL(ref_pos.clone(), String::from("buffer"))
    );

    let t6 = Transition::new(
        "finish_move_to_buffer",
        vec!(act_pos.clone()),
        &move_executing,
        &Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
    );

    let t7 = Transition::new(
        "start_move_to_home",
        vec!(ref_pos.clone()),
        &move_enabled,
        &Predicate::EQVAL(ref_pos.clone(), String::from("home"))
    );

    let t8 = Transition::new(
        "finish_move_to_home",
        vec!(act_pos.clone()),
        &move_executing,
        &Predicate::EQVAL(act_pos.clone(), String::from("home"))
    );

    let t9 = Transition::new(
        "start_move_to_table",
        vec!(ref_pos.clone()),
        &move_enabled,
        &Predicate::EQVAL(ref_pos.clone(), String::from("table"))
    );

    let t10 = Transition::new(
        "finish_move_to_table",
        vec!(act_pos.clone()),
        &move_executing,
        &Predicate::EQVAL(act_pos.clone(), String::from("table"))
    );

    // moving the cube
    let t11 = Transition::new(
        "take_cube_from_table",
        vec!(gripper.clone(), table.clone()),
        &Predicate::AND(
            vec!(
                cube_at_table.clone(), 
                Predicate::EQVAL(act_pos.clone(), String::from("table")),
                Predicate::EQVAL(ref_pos.clone(), String::from("table"))
            )
        ),
        &Predicate::AND(
            vec!(
                cube_at_gripper.clone(), table_empty.clone()
            )
        )  
    );

    let t12 = Transition::new(
        "leave_cube_at_table",
        vec!(table.clone(), gripper.clone()),
        &Predicate::AND(
            vec!(
                cube_at_gripper.clone(), 
                Predicate::EQVAL(act_pos.clone(), String::from("table")),
                Predicate::EQVAL(ref_pos.clone(), String::from("table"))
            )
        ),
        &Predicate::AND(
            vec!(
                cube_at_table.clone(), gripper_empty.clone()
            )
        )
    );

    let t13 = Transition::new(
        "take_cube_from_buffer",
        vec!(gripper.clone(), buffer.clone()),
        &Predicate::AND(
            vec!(
                cube_at_buffer.clone(), 
                Predicate::EQVAL(act_pos.clone(), String::from("buffer")),
                Predicate::EQVAL(ref_pos.clone(), String::from("buffer"))
            )
        ),
        &Predicate::AND(
            vec!(
                cube_at_gripper.clone(), buffer_empty.clone()
            )
        )
    );

    let t14 = Transition::new(
        "leave_cube_at_buffer",
        vec!(buffer.clone(), gripper.clone()),
        &Predicate::AND(
            vec!(
                cube_at_gripper.clone(), 
                Predicate::EQVAL(act_pos.clone(), String::from("buffer")),
                Predicate::EQVAL(ref_pos.clone(), String::from("buffer"))
            )
        ),
        &Predicate::AND(
            vec!(
                cube_at_buffer.clone(), gripper_empty.clone()
            )
        )
    );

    let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);

    // ltl specs (real ugly, need some macros):
    // 1. can't move if not active:
    let s1 = Predicate::GLOB(
        vec!(
            Predicate::NOT(
                vec!(
                    Predicate::AND(
                        vec!(
                            Predicate::NEQVAR(act_pos.clone(), ref_pos.clone()),
                            Predicate::OR(
                                vec!(
                                    Predicate::EQVAL(act_stat.clone(), String::from("idle")),
                                    Predicate::EQVAL(ref_stat.clone(), String::from("idle"))
                                )
                            )
                        )
                    )
                )
            )
        )
    );

    // 2a. has to go through the home pos (next is inherently global):
    let s2 = Predicate::NOT(
        vec!(
            Predicate::NEXT(
                vec!(
                    Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                ), 
                vec!(
                    Predicate::EQVAL(ref_pos.clone(), String::from("table"))
                )
            )
        )
    );

    // 2b. has to go through the home pos (next is inherently global):
    let s3 = Predicate::NOT(
        vec!(
            Predicate::NEXT(
                vec!(
                    Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                ), 
                vec!(
                    Predicate::EQVAL(act_pos.clone(), String::from("table"))
                )
            )
        )
    );

    // 2c. has to go through the home pos (next is inherently global):
    let s4 = Predicate::NOT(
        vec!(
            Predicate::NEXT(
                vec!(
                    Predicate::EQVAL(act_pos.clone(), String::from("table"))
                ), 
                vec!(
                    Predicate::EQVAL(ref_pos.clone(), String::from("buffer"))
                )
            )
        )
    );

    // 2d. has to go through the home pos (next is inherently global):
    let s5 = Predicate::NOT(
        vec!(
            Predicate::NEXT(
                vec!(
                    Predicate::EQVAL(act_pos.clone(), String::from("table"))
                ), 
                vec!(
                    Predicate::EQVAL(act_pos.clone(), String::from("buffer"))
                )
            )
        )
    );

    // 3. there is only one cube in the system (implement pbeq in the future):
    let s6 = Predicate::GLOB(
        vec!(
            Predicate::OR(
                vec!(
                    Predicate::AND(
                        vec!(
                            Predicate::EQVAL(buffer.clone(), String::from("cube")),
                            Predicate::NEQVAL(gripper.clone(), String::from("cube")),
                            Predicate::NEQVAL(table.clone(), String::from("cube"))
                        )
                    ),
                    Predicate::AND(
                        vec!(
                            Predicate::EQVAL(gripper.clone(), String::from("cube")),
                            Predicate::NEQVAL(buffer.clone(), String::from("cube")),
                            Predicate::NEQVAL(table.clone(), String::from("cube"))
                        )
                    ),
                    Predicate::AND(
                        vec!(
                            Predicate::EQVAL(table.clone(), String::from("cube")),
                            Predicate::NEQVAL(gripper.clone(), String::from("cube")),
                            Predicate::NEQVAL(buffer.clone(), String::from("cube"))
                        )
                    )
                )        
            )
        )
    );

    // 4. item can be taken/left only when robot is there:
    // actually, its easier just to add it in the trans guards
    // let s5 = Predicate::GLOB(
    //     vec!(

    //     )
    // );
    
    
    
    let specs = Predicate::AND(vec!(s1, s2, s3, s4, s5, s6));

    // initial:
    let initial = Predicate::AND(
        vec!(
            Predicate::EQVAL(ref_stat.clone(), String::from("idle")),
            Predicate::EQVAL(ref_pos.clone(), String::from("buffer")), 
            Predicate::EQVAL(act_pos.clone(), String::from("buffer")), 
            Predicate::EQVAL(act_stat.clone(), String::from("idle")),
            Predicate::EQVAL(table.clone(), String::from("cube"))
        )
    );

    // goal:
    let goal = Predicate::AND(
        vec!(
            Predicate::EQVAL(act_pos.clone(), String::from("table")), 
            Predicate::EQVAL(act_stat.clone(), String::from("idle")),
            Predicate::EQVAL(buffer.clone(), String::from("cube"))
        )
    );

    let problem = PlanningProblem::new(String::from("robot1"), vars, initial, goal, trans, specs, 20);
    let result = Sequential::new(&problem, &vec!());

    let vars3 = vec!(act_stat.clone());
    let vars4 = vec!(ref_stat.clone());
    let vars2 = vec!(act_pos.clone(), ref_pos.clone());
    let result2 = Sequential::new(&problem, &vars3);

    // println!("plan_found: {:?}", result.plan_found);
    // println!("plan_lenght: {:?}", result.plan_length);
    // println!("time_to_solve: {:?}", result.time_to_solve);
    // println!("trace: ");
    // 
    // for t in result.trace{
        
    //     println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     println!("=========================");
    // }

    println!("plan_found2: {:?}", result2.plan_found);
    println!("plan_lenght2: {:?}", result2.plan_length);
    println!("time_to_solve2: {:?}", result2.time_to_solve);
    println!("trace2: ");
    
    for t in &result2.trace{
        
        println!("state2: {:?}", t.state);
        println!("trans2: {:?}", t.trans);
        println!("=========================");
    }

    let probs = GenerateProblems::new(&result2, &problem, &vars4);
    for p in &probs {
        println!("init : {:?}", p.initial);
        println!("goal : {:?}", p.goal);
    }
    // println!("{:?}", probs);

}