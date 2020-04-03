//! Z3 sorts for SP

use std::ffi::{CStr, CString};
use std::collections::HashMap;
use z3_sys::*;
use super::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Variable {
    n: String,    
    t: String,     
    d: Vec<String>  
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Assignment {
    var: Variable,
    val: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Transition {
    n: String,
    g: Predicate,
    u: Vec<Assignment>
}

#[derive(Debug)]
pub struct PlanningProblem {
    name: String,
    vars: Vec<Variable>,
    initial: Vec<Assignment>,
    goal: Vec<Assignment>,
    trans: Vec<Transition>,
    specs: Vec<Predicate>,
    max_steps: u32
}

#[derive(Debug)]
pub struct PlanningFrame {
    state: Vec<Assignment>,
    transition: Transition,
}

#[derive(Debug)]
pub struct PlanningResult {
    plan_found: bool,
    plan_length: u32,
    trace: Vec<PlanningFrame>,
    time_to_solve: std::time::Duration,
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

pub struct PredicateToAstZ3<'ctx>{
    pub ctx: &'ctx ContextZ3,
    pub pred: Predicate,
    pub step: u32,
    pub r: Z3_ast
}

#[derive(Debug)]
pub struct Sequential {
    p: PlanningProblem,
    r: PlanningResult
}

impl Variable {
    /// Creates a new Variable
    pub fn new(n: &str, t: &str, d: &Vec<&str>) -> Variable {
        Variable { n: n.to_string(), 
                   t: t.to_string(),
                   d: d.iter().map(|x| x.to_string()).collect::<Vec<String>>()}
    }
}

impl Assignment {
    pub fn new(var: &Variable, val: &str) -> Assignment {
        Assignment { 
            var: Variable {
                n: var.n.clone(),
                t: var.t.clone(),
                d: var.d.clone()
            },  
            val: val.to_string() 
        }
    }
}

impl Transition {
    pub fn new(n: &str, g: &Predicate, u: Vec<Assignment>) -> Transition {
        Transition { n: n.to_string(),
                     g: g.clone(),
                     u: u }
    }
}

impl PlanningProblem {
    pub fn new(name: String,
               vars: Vec<Variable>,
               initial: Vec<Assignment>,
               goal: Vec<Assignment>,
               trans: Vec<Transition>,
               specs: Vec<Predicate>,
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

impl Predicate {
    pub fn new(&mut self, state: &Vec<Assignment>) {
        match self {
            Predicate::AND(x) => x.iter_mut().for_each(|p| p.new(state)),
            Predicate::OR(x) => x.iter_mut().for_each(|p| p.new(state)),
            Predicate::NOT(x) => x.new(state),
            _=> panic!("flatten values")
        }
    }
}

// impl Sequential {
//     pub fn new(p: &PlanningProblem) -> PlanningResult {

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

    let a1 = Assignment::new(&v1, "home");
    let a2 = Assignment::new(&v2, "active");

    let initial_state = vec!(a1, a2);

    let pred = Predicate::AND(
        vec!(
            Predicate::EQVAL(v1, String::from("home")),
            Predicate::EQVAL(v2, String::from("active"))
        )   
    );

    println!("{:?}", pred);
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
fn test_problem(){
    let pose_domain = vec!("home", "buffer", "table");
    let status_domain = vec!("idle", "active");

    let act_pos = Variable::new("act_pos", "pose", &pose_domain);
    let ref_pos = Variable::new("ref_pos", "pose", &pose_domain);
    let status = Variable::new("robot_status", "status", &status_domain);

    // let vars = Variables::new(&vec!(v1.clone(), ref_pos.clone(), v2.clone()));
    let vars = vec!(act_pos.clone(), status.clone());

    // buffer idle state
    let a1 = Assignment::new(&act_pos, "buffer");
    let a2 = Assignment::new(&status, "idle");

    // buffer activated state
    let a3 = Assignment::new(&act_pos, "buffer");
    let a4 = Assignment::new(&status, "active");

    // home activated state
    let a5 = Assignment::new(&act_pos, "home");
    let a6 = Assignment::new(&status, "active");

    // table activated state
    let a7 = Assignment::new(&act_pos, "table");
    let a8 = Assignment::new(&status, "active");

    // table idle state
    let a9 = Assignment::new(&act_pos, "table");
    let a10 = Assignment::new(&status, "active");

    let buffer_idle = vec!(a1, a2);
    let buffer_active = vec!(a3, a4);
    let home_active = vec!(a5, a6);
    let table_active = vec!(a7, a8);
    let table_idle = vec!(a9, a10);

    let enabled = Predicate::AND(vec!(Predicate::EQVAR(act_pos.clone(), ref_pos.clone()), Predicate::EQVAL(status.clone(), String::from("active"))));
    let executing = Predicate::NEQVAR(act_pos.clone(), ref_pos.clone());
    let idle = Predicate::EQVAL(status, String::from("idle"));

    let t1 = Transition::new("activate", &idle, buffer_active.clone());
    let t2 = Transition::new("buffer_to_home", &enabled, home_active.clone());
    let t3 = Transition::new("home_to_table", &enabled, table_active.clone());
    let t4 = Transition::new("deactivate", &enabled, table_idle.clone());

    let trans = vec!(t1, t2, t3, t4);

    let problem = PlanningProblem::new(String::from("robot1"), vars, buffer_idle, table_idle, trans, vec!(Predicate::TRUE), 20);
    println!("{:?}", problem);
    // for t in problem.trans{
    //     println!("{:?}", t);
    // } 
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