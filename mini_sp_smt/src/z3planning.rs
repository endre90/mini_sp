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

// #[derive(Hash, Eq, PartialEq, Clone, Debug)]
// pub struct Assignment {
//     var: Variable,
//     val: String,
// }

#[derive(PartialEq, Clone, Debug)]
pub struct Transition {
    n: String,
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
    specs: Predicate,
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
    NOT(Box<Predicate>),
    EQVAL(Variable, String),
    NEQVAL(Variable, String),
    EQVAR(Variable, Variable),
    NEQVAR(Variable, Variable),
    TRUE,
    FALSE
}

pub struct PredicateToAstZ3<'ctx> {
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

impl Variable {
    /// Creates a new Variable
    pub fn new(n: &str, t: &str, d: &Vec<&str>) -> Variable {
        Variable { n: n.to_string(), 
                   t: t.to_string(),
                   d: d.iter().map(|x| x.to_string()).collect::<Vec<String>>()}
    }
}

// impl Assignment {
//     pub fn new(var: &Variable, val: &str) -> Assignment {
//         Assignment { 
//             var: Variable {
//                 n: var.n.clone(),
//                 t: var.t.clone(),
//                 d: var.d.clone()
//             },  
//             val: val.to_string() 
//         }
//     }
// }

impl Transition {
    pub fn new(n: &str, g: &Predicate, u: &Predicate) -> Transition {
        Transition { n: n.to_string(),
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
               specs: Predicate,
               max_steps: u32) -> PlanningProblem {
        PlanningProblem {
            name: name.to_string(),
            vars: vars,
            initial: initial,
            goal: goal,
            trans: trans,
            specs: specs,
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

// impl Predicate {
//     pub fn new(&mut self, state: &Vec<Assignment>) {
//         match self {
//             Predicate::AND(x) => x.iter_mut().for_each(|p| p.new(state)),
//             Predicate::OR(x) => x.iter_mut().for_each(|p| p.new(state)),
//             Predicate::NOT(x) => x.new(state),
//             _=> panic!("flatten values")
//         }
//     }
// }

impl <'ctx> PredicateToAstZ3<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, pred: &Predicate, step: u32) -> Z3_ast {
        match pred {
            Predicate::TRUE => BoolZ3::new(&ctx, true),
            Predicate::FALSE => BoolZ3::new(&ctx, false),
            Predicate::NOT(p) => NOTZ3::new(&ctx, PredicateToAstZ3::new(&ctx, p, step)),
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
            }
        }
    }
}

// impl <'ctx> AssignmentToAstZ3<'ctx> {
//     pub fn new(ctx: &'ctx ContextZ3, init: &Vec<Assignment>, step: u32) -> Z3_ast {
//         let mut to_assign = vec!();
//         for a in init {
//             let sort = EnumSortZ3::new(&ctx, &a.var.t, a.var.d.iter().map(|x| x.as_str()).collect());
//             let elems = &sort.enum_asts;
//             let index = a.var.d.iter().position(|r| *r == a.val.to_string()).unwrap();
//             let var = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", a.var.n.to_string(), step).as_str());
//             to_assign.push(
//                 EQZ3::new(&ctx,var, elems[index])
//             )
//         }
//         ANDZ3::new(&ctx, to_assign)
//     }
// }

impl Sequential {
    pub fn new(p: &PlanningProblem) -> PlanningResult {
    
        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);
    
        slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.specs, 0));
        
        let initial_state = PredicateToAstZ3::new(&ctx, &p.initial, 0);
        slv_assert_z3!(&ctx, &slv, initial_state);

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        let goal_state = PredicateToAstZ3::new(&ctx, &p.goal, 0);
        slv_assert_z3!(&ctx, &slv, goal_state);

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
                    // let name = &t.n;
                    let guard = PredicateToAstZ3::new(&ctx, &t.g, step - 1);
                    let updates = PredicateToAstZ3::new(&ctx, &t.u, step);

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, updates)));
                }

                slv_assert_z3!(&ctx, &slv, PredicateToAstZ3::new(&ctx, &p.specs, step));

                slv_assert_z3!(&ctx, &slv, ORZ3::new(&ctx, all_trans));

                SlvPushZ3::new(&ctx, &slv);

                let goal_state = PredicateToAstZ3::new(&ctx, &p.goal, step);
                slv_assert_z3!(&ctx, &slv, goal_state);
        
            } else {
                plan_found = true;
                break;
            }
        }

        let planning_time = now.elapsed();
        
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
                        _ => frame.state.push(sep[1].to_string()),
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
fn test_var(){
    let domain = vec!("home", "buffer", "table");
    let mut v = Variable::new("robot_pose", "pose", &domain);
    println!("{:?}", v);
    v.t = "blah".to_string();
    println!("{:?}", v);
}

#[test]
fn test_coll(){
    let pose_domain = vec!("home", "buffer", "table");
    let status_domain = vec!("idle", "active");

    let v1 = Variable::new("robot_pose", "pose", &pose_domain);
    let v2 = Variable::new("robot_status", "status", &status_domain);

    let a1 = Predicate::EQVAL(v1, String::from("home"));
    let a2 = Predicate::EQVAL(v2, String::from("active"));

    let initial_state = Predicate::AND(vec!(a1, a2));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    println!("{}", ast_to_string_z3!(&ctx, PredicateToAstZ3::new(&ctx, &initial_state, 0)));
}

#[test]
fn test_predicate(){
    let pose_domain = vec!("home", "buffer", "table");
    let status_domain = vec!("idle", "active");

    let act_pos = Variable::new("act_pos", "pose", &pose_domain);
    let ref_pos = Variable::new("ref_pos", "pose", &pose_domain);
    let status = Variable::new("robot_status", "status", &status_domain);

    let enabled = Predicate::AND(vec!(Predicate::EQVAR(act_pos.clone(), ref_pos.clone()), Predicate::EQVAL(status.clone(), String::from("active"))));

    let mut step: u32 = 7;
    
    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let z3_pred = PredicateToAstZ3::new(&ctx, &enabled, step);
    println!("{}", ast_to_string_z3!(&ctx, z3_pred));
}

#[test]
fn test_initial_state(){
    let pose_domain = vec!("home", "buffer", "table");
    let status_domain = vec!("idle", "active");

    let act_pos = Variable::new("act_pos", "pose", &pose_domain);
    let ref_pos = Variable::new("ref_pos", "pose", &pose_domain);
    let status = Variable::new("robot_status", "status", &status_domain);

    // let vars = Variables::new(&vec!(v1.clone(), ref_pos.clone(), v2.clone()));
    let vars = vec!(act_pos.clone(), status.clone());

    // buffer idle state
    let a1 = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
    let a2 = Predicate::EQVAL(status.clone(), String::from("idle"));

    let cfg = ConfigZ3::new();
    let ctx = ContextZ3::new(&cfg);
    let slv = SolverZ3::new(&ctx);

    let buffer_idle = Predicate::AND(vec!(a1, a2));
    // let init_z3 = AssignmentToAstZ3::new(&ctx, &buffer_idle, 0);
    let init_z3 = PredicateToAstZ3::new(&ctx, &buffer_idle, 0);
    println!("{}", ast_to_string_z3!(&ctx, init_z3));
}

#[test]
fn test_problem(){
    let pose_domain = vec!("home", "buffer", "table");
    let status_domain = vec!("idle", "active");

    let act_pos = Variable::new("act_pos", "pose", &pose_domain);
    let ref_pos = Variable::new("ref_pos", "pose", &pose_domain);
    let status = Variable::new("robot_status", "status", &status_domain);

    // let vars = Variables::new(&vec!(v1.clone(), ref_pos.clone(), v2.clone()));
    let vars = vec!(act_pos.clone(), status.clone());

    // buffer idle state
    let a1 = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
    let a2 = Predicate::EQVAL(status.clone(), String::from("idle"));

    // buffer idle state
    let a3 = Predicate::EQVAL(act_pos.clone(), String::from("buffer"));
    let a4 = Predicate::EQVAL(status.clone(), String::from("active"));

    // home activated state
    let a5 = Predicate::EQVAL(act_pos.clone(), String::from("home"));
    let a6 = Predicate::EQVAL(status.clone(), String::from("active"));

    // table activated state
    let a7 = Predicate::EQVAL(act_pos.clone(), String::from("table"));
    let a8 = Predicate::EQVAL(status.clone(), String::from("active"));

    // table idle state
    let a9 = Predicate::EQVAL(act_pos.clone(), String::from("table"));
    let a10 = Predicate::EQVAL(status.clone(), String::from("idle"));

    let buffer_idle = Predicate::AND(vec!(a1, a2));
    let buffer_active = Predicate::AND(vec!(a3, a4));
    let home_active = Predicate::AND(vec!(a5, a6));
    let table_active = Predicate::AND(vec!(a7, a8));
    let table_idle = Predicate::AND(vec!(a9, a10));

    let enabled = Predicate::AND(vec!(Predicate::EQVAR(act_pos.clone(), ref_pos.clone()), Predicate::EQVAL(status.clone(), String::from("active"))));
    let executing = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
    let idle = Predicate::EQVAL(status, String::from("idle"));

    let t1 = Transition::new("activate", &idle, &buffer_active);
    let t2 = Transition::new("buffer_to_home", &enabled, &home_active);
    let t3 = Transition::new("home_to_table", &enabled, &table_active);
    let t4 = Transition::new("deactivate", &enabled, &table_idle);

    let trans = vec!(t1, t2, t3, t4);

    let problem = PlanningProblem::new(String::from("robot1"), vars, buffer_idle, table_idle, trans, Predicate::TRUE, 5);
    let result = Sequential::new(&problem);
    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
    println!("trace: ");
    // 
    for t in result.trace{
        
        println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        println!("=========================");
    } 
}

// #[test]
// fn test_model(){
//     let name = "robots";
//     let initial = vec!((String::from("pose1"), true), (String::from("pose2"), false), (String::from("pose3"), false));
//     let goal = vec!((String::from("pose1"), false), (String::from("pose2"), false), (String::from("pose3"), true));

//     let t1_guard = vec!((String::from("pose1"), true));
//     let t1_update = vec!((String::from("pose2"), true));
//     let t1 = (t1_guard, t1_update);

//     let t2_guard = vec!((String::from("pose2"), true));
//     let t2_update = vec!((String::from("pose3"), true));
//     let t2 = (t2_guard, t2_update);

//     let trans = vec!(t1, t2);

//     let specs = vec!((String::from("spec"), true));
//     let max_steps = 5;

//     let p = Problem::new(name, initial, goal, trans, specs, max_steps);
//     println!("{:?}", p);
// }