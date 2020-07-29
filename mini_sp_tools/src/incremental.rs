use std::time::{Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Transition {
    pub name: String,
    pub guard: Predicate,
    pub update: Predicate
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct PlanningProblem {
    pub name: String,
    pub init: Predicate,
    pub goal: Predicate,
    pub trans: Vec<Transition>,
    pub ltl_specs: Predicate,
    pub max_steps: u32
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct MultGoalsPlanningProblem {
    pub name: String,
    pub init: Predicate,
    pub goals: Vec<(Predicate, Predicate)>,
    pub trans: Vec<Transition>,
    pub ltl_specs: Predicate,
    pub max_steps: u32
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Incremental {
    pub prob: PlanningProblem
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct IncrementalDenial {
    pub prob: PlanningProblem
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct IncrementalAll {
    pub prob: PlanningProblem
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct MultGoalsIncremental {
    pub prob: PlanningProblem
}

#[derive(Clone)]
pub struct KeepVariableValues<'ctx> {
    pub ctx: &'ctx ContextZ3
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct PlanningFrame {
    pub state: Vec<String>,
    pub trans: String,
}

// maybe implement in the future when all works
// #[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
// pub struct PlanningFrame {
//     pub state: State,
//     pub trans: Transition,
// }

pub struct GetPlanningResultZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub model: Z3_model,
    pub nr_steps: u32,
    pub frames: PlanningResult
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct PlanningResult {
    pub plan_found: bool,
    pub plan_length: u32,
    pub trace: Vec<PlanningFrame>,
    pub raw_trace: Vec<PlanningFrame>,
    pub time_to_solve: std::time::Duration,
}

impl Transition {
    pub fn new(name: &str, guard: &Predicate, update: &Predicate) -> Transition {
        Transition { name: name.to_string(),
                     guard: guard.to_owned(),
                     update: update.to_owned() }
    }
}

impl PlanningProblem {
    pub fn new(name: &str, init: &Predicate, goal: &Predicate, trans: &Vec<Transition>,
        ltl_specs: &Predicate, max_steps: &u32) -> PlanningProblem {
        PlanningProblem {
            name: name.to_string(),
            init: init.to_owned(),
            goal: goal.to_owned(),
            trans: trans.to_owned(),
            ltl_specs: ltl_specs.to_owned(),
            max_steps: max_steps.to_owned()
        }
    }
}

impl MultGoalsPlanningProblem {
    pub fn new(name: &str, init: &Predicate, goals: &Vec<(&Predicate, Option<&Predicate>)>, trans: &Vec<Transition>,
        ltl_specs: &Predicate, max_steps: &u32) -> MultGoalsPlanningProblem {
        MultGoalsPlanningProblem {
            name: name.to_string(),
            init: init.to_owned(),
            goals: goals.iter().map(|x| (x.0.to_owned(), match x.1 {
                Some(x) => x.to_owned(),
                None => Predicate::TRUE
            })).collect(),
            trans: trans.to_owned(),
            ltl_specs: ltl_specs.to_owned(),
            max_steps: max_steps.to_owned()
        }
    }
}

impl <'ctx> KeepVariableValues<'ctx> {
    pub fn new(ctx: &'ctx ContextZ3, vars: &Vec<EnumVariable>, trans: &Transition, step: &u32) -> Z3_ast {

        let changed = GetPredicateVars::new(&trans.update);
        let unchanged = IterOps::difference(vars, &changed);
        let mut assert_vec = vec!();
        for u in unchanged {
            let sort = EnumSortZ3::new(&ctx, &u.r#type, u.domain.iter().map(|x| x.as_str()).collect());
            let v_1 = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", u.name.to_string(), step).as_str());
            let v_2 = EnumVarZ3::new(&ctx, sort.r, format!("{}_s{}", u.name.to_string(), step - 1).as_str());
            assert_vec.push(EQZ3::new(&ctx, v_1, v_2));
        }
        ANDZ3::new(&ctx, assert_vec)
    }
}

impl Incremental {
    pub fn new(prob: &PlanningProblem) -> PlanningResult {

        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);

        let problem_vars = GetProblemVars::new(&prob);

        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.init, "state", &0));

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &0));
        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.goal, "specs", &0));

        let now = Instant::now();
        let mut plan_found: bool = false;

        let mut step: u32 = 0;

        while step < prob.max_steps + 1 {
            step = step + 1;
            if SlvCheckZ3::new(&ctx, &slv) != 1 {
                SlvPopZ3::new(&ctx, &slv, 1);

                let mut all_trans = vec!();
                for t in &prob.trans {
                    let name = format!("{}_t{}", &t.name, step);
                    let guard = PredicateToAstZ3::new(&ctx, &t.guard, "guard", &(step - 1));
                    let update = PredicateToAstZ3::new(&ctx, &t.update, "update", &(step));
                    let keeps = KeepVariableValues::new(&ctx, &problem_vars, &t, &step);

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, update, keeps)));
                }

                SlvAssertZ3::new(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
                SlvPushZ3::new(&ctx, &slv);
                SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &step));
                SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.goal, "specs", &step));
                
            } else {
                plan_found = true;
                break;
            }
        }

        let planning_time = now.elapsed();

        // let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
        // let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
        // for asrt in asrtvec {
        //     println!("{}", AstToStringZ3::new(&ctx, asrt));
        // }
        // let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
        
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

impl IncrementalDenial {
    pub fn new(prob: &PlanningProblem, deny: &Vec<&PlanningResult>) -> PlanningResult {

        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);

        let problem_vars = GetProblemVars::new(&prob);

        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.init, "state", &0));
        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.init, "state", &0));

        // deny previous solutions:
        let mut denied = vec!();
        for den in deny {
            let mut predicate = vec!();
            for tr in &den.raw_trace{
                predicate.push(
                    EQZ3::new(&ctx, 
                        BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), tr.trans.as_str()), 
                        BoolZ3::new(&ctx, true)
                    )
                )
            }
            denied.push(NOTZ3::new(&ctx, ANDZ3::new(&ctx, predicate)))
        } 

        SlvAssertZ3::new(&ctx, &slv, ANDZ3::new(&ctx, denied));

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &0));
        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.goal, "specs", &0));

        let now = Instant::now();
        let mut plan_found: bool = false;

        let mut step: u32 = 0;

        while step < prob.max_steps + 1 {
            step = step + 1;
            if SlvCheckZ3::new(&ctx, &slv) != 1 {
                SlvPopZ3::new(&ctx, &slv, 1);

                let mut all_trans = vec!();
                for t in &prob.trans {
                    let name = format!("{}_t{}", &t.name, step);
                    let guard = PredicateToAstZ3::new(&ctx, &t.guard, "guard", &(step - 1));
                    let update = PredicateToAstZ3::new(&ctx, &t.update, "update", &(step));
                    let keeps = KeepVariableValues::new(&ctx, &problem_vars, &t, &step);

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, update, keeps)));
                }

                SlvAssertZ3::new(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
                SlvPushZ3::new(&ctx, &slv);
                SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &step));
                SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.goal, "specs", &step));
                
            } else {
                plan_found = true;
                break;
            }
        }

        let planning_time = now.elapsed();

        // let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
        // let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
        // for asrt in asrtvec {
        //     println!("{}", AstToStringZ3::new(&ctx, asrt));
        // }
        // let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
        
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

impl IncrementalAll {
    pub fn new(prob: &PlanningProblem, many: u32) -> Vec<PlanningResult> {
        let mut results = vec!();
        let first_result = Incremental::new(&prob);
        results.push(&first_result);

        fn recursive_subfn(prob: &PlanningProblem, deny: &Vec<&PlanningResult>, many: u32) -> Vec<PlanningResult> {
            let mut results = deny.to_owned();
            let add_res = IncrementalDenial::new(&prob, &results);
            let mut i: u32 = 0;
            if !deny.iter().any(|x| x.plan_found == false) && i < many {
                i = i + 1;
                results.push(&add_res);
                println!("{:?}", results);
                recursive_subfn(&prob, &results, many);
            }
            results.iter().map(|&x| x.to_owned()).collect()
        }

        recursive_subfn(&prob, &results, many)
    }
}

// impl IncrementalAll {
//     pub fn new(prob: &PlanningProblem) -> Vec<PlanningResult> {

//         let mut results = vec!();
//         let first_result = Incremental::new(&prob);
//         while re
//         while 

//         let cfg = ConfigZ3::new();
//         let ctx = ContextZ3::new(&cfg);
//         let slv = SolverZ3::new(&ctx);

//         let problem_vars = GetProblemVars::new(&prob);

//         SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.init, "state", &0));

//         SlvPushZ3::new(&ctx, &slv); // create backtracking point
//         SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &0));
//         SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.goal, "specs", &0));

//         let now = Instant::now();
//         let mut plan_found: bool = false;

//         let mut step: u32 = 0;

//         while step < prob.max_steps + 1 {
//             step = step + 1;
//             if SlvCheckZ3::new(&ctx, &slv) != 1 {
//                 SlvPopZ3::new(&ctx, &slv, 1);

//                 let mut all_trans = vec!();
//                 for t in &prob.trans {
//                     let name = format!("{}_t{}", &t.name, step);
//                     let guard = PredicateToAstZ3::new(&ctx, &t.guard, "guard", &(step - 1));
//                     let update = PredicateToAstZ3::new(&ctx, &t.update, "update", &(step));
//                     let keeps = KeepVariableValues::new(&ctx, &problem_vars, &t, &step);

//                     all_trans.push(ANDZ3::new(&ctx, 
//                         vec!(EQZ3::new(&ctx, 
//                             BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
//                             BoolZ3::new(&ctx, true)),
//                         guard, update, keeps)));
//                 }

//                 SlvAssertZ3::new(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
//                 SlvPushZ3::new(&ctx, &slv);
//                 SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &step));
//                 SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.goal, "specs", &step));
                
//             } else {
//                 plan_found = true;
//                 break;
//             }
//         }

//         let planning_time = now.elapsed();

//         // let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
//         // let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
//         // for asrt in asrtvec {
//         //     println!("{}", AstToStringZ3::new(&ctx, asrt));
//         // }
//         // let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
        
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

impl MultGoalsIncremental {
    pub fn new(prob: &MultGoalsPlanningProblem) -> PlanningResult {

        let cfg = ConfigZ3::new();
        let ctx = ContextZ3::new(&cfg);
        let slv = SolverZ3::new(&ctx);

        let problem_vars = GetProblemVars::new(&PlanningProblem::new(
            prob.name.as_str(), 
            &Predicate::TRUE, // since I extract only from trans...
            &Predicate::TRUE,
            &prob.trans,
            &prob.ltl_specs,
            &prob.max_steps)
        );

        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.init, "state", &0));

        SlvPushZ3::new(&ctx, &slv); // create backtracking point
        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &0));
        for g in &prob.goals {
            SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &Predicate::EVENTUALLY(Box::new(g.0.to_owned())), "state", &0));
            match g.1 {
                Predicate::TRUE => {
                    
                },
                _ => SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &Predicate::UNTIL(Box::new(g.1.to_owned()), Box::new(g.0.to_owned())), "specs", &0))
            }
        }

        let now = Instant::now();
        let mut plan_found: bool = false;

        let mut step: u32 = 0;

        while step < prob.max_steps + 1 {
            step = step + 1;
            if SlvCheckZ3::new(&ctx, &slv) != 1 {
                SlvPopZ3::new(&ctx, &slv, 1);

                let mut all_trans = vec!();
                for t in &prob.trans {
                    let name = format!("{}_t{}", &t.name, step);
                    let guard = PredicateToAstZ3::new(&ctx, &t.guard, "guard", &(step - 1));
                    let update = PredicateToAstZ3::new(&ctx, &t.update, "update", &(step));
                    let keeps = KeepVariableValues::new(&ctx, &problem_vars, &t, &step);

                    all_trans.push(ANDZ3::new(&ctx, 
                        vec!(EQZ3::new(&ctx, 
                            BoolVarZ3::new(&ctx, &BoolSortZ3::new(&ctx), name.as_str()), 
                            BoolZ3::new(&ctx, true)),
                        guard, update, keeps)));
                }

                SlvAssertZ3::new(&ctx, &slv, ORZ3::new(&ctx, all_trans));
                
                SlvPushZ3::new(&ctx, &slv);
                SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &prob.ltl_specs, "specs", &step));
                for g in &prob.goals {
                    SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &Predicate::EVENTUALLY(Box::new(g.0.to_owned())), "state", &step));
                    match g.1 {
                        Predicate::TRUE => {
                            
                        },
                        _ => 
                        SlvAssertZ3::new(&ctx, &slv, PredicateToAstZ3::new(&ctx, &Predicate::UNTIL(Box::new(g.1.to_owned()), Box::new(g.0.to_owned())), "specs", &step))
                    }
                }
        
            } else {
                plan_found = true;
                break;
            }
        }

        let planning_time = now.elapsed();

        // let asserts = SlvGetAssertsZ3::new(&ctx, &slv);
        // let asrtvec = Z3AstVectorToVectorAstZ3::new(&ctx, asserts);
        // for asrt in asrtvec {
        //     println!("{}", AstToStringZ3::new(&ctx, asrt));
        // }
        // let cnf = GetCnfVectorZ3::new(&ctx, asrtvec);
        
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

// impl PlanningFrame {
//     pub fn new(state: Vec<&str>, trans: &str) -> PlanningFrame {
//         PlanningFrame {
//             state: state.iter().map(|x| x.to_string()).collect(),
//             trans: trans.to_string()
//         }
//     }
// }

impl PlanningFrame {
    pub fn new(state: &Vec<&str>, trans: &str) -> PlanningFrame {
        PlanningFrame {
            state: state.iter().map(|x| x.to_string()).collect(),
            trans: trans.to_string()
        }
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
        let mut raw_trace: Vec<PlanningFrame> = vec!();
        
        for i in 0..nr_steps {
            let mut frame: PlanningFrame = PlanningFrame::new(&vec!(), "");
            let mut raw_frame: PlanningFrame = PlanningFrame::new(&vec!(), "");
            for j in &model_vec {
                let sep: Vec<&str> = j.split(" -> ").collect();
                if sep[0].ends_with(&format!("_s{}", i)){
                    // raw_frame.state.push(j.to_string());
                    let trimmed_state = sep[0].trim_end_matches(&format!("_s{}", i));
                    match sep[1] {
                        "false" => {
                            frame.state.push(sep[0].to_string());
                            // raw_frame.state.push(j.to_string());
                        },
                        "true" => {
                            frame.state.push(sep[0].to_string());
                            // raw_frame.state.push(j.to_string());
                        },
                        _ => {
                            frame.state.push(format!("{} -> {}", trimmed_state, sep[1]));
                            // raw_frame.state.push(j.to_string());
                        }
                    }
                } else if sep[0].ends_with(&format!("_t{}", i)) && sep[1] == "true" {
                    let trimmed_trans = sep[0].trim_end_matches(&format!("_t{}", i));
                    frame.trans = trimmed_trans.to_string();  
                    raw_frame.trans = sep[0].to_string();
                }
            }
            if model_vec.len() != 0 {
                trace.push(frame);
                raw_trace.push(raw_frame);
            }
        }

        PlanningResult {
            plan_found: plan_found,
            plan_length: nr_steps - 1,
            trace: trace,
            raw_trace: raw_trace,
            time_to_solve: planning_time,
        }
    }
}

#[test]
fn test_incremental_1(){

    let max_steps: u32 = 60;

    let color_domain = vec!("red", "green");
    let pose_domain = vec!("buffer", "home", "table");
    let stat_domain = vec!("active", "idle");
    let buffer_domain = vec!("cube", "ball", "empty");
    let gripper_domain = vec!("cube", "ball", "empty");
    let table_domain = vec!("cube", "ball", "empty");

    let color = EnumVariable::new("color", "color", &color_domain, None);

    let act_pos = EnumVariable::new("act_pos", "pose", &pose_domain, None);
    let ref_pos = EnumVariable::new("ref_pos", "pose", &pose_domain, None);

    let act_stat = EnumVariable::new("act_stat", "status", &stat_domain, None);
    let ref_stat = EnumVariable::new("ref_stat", "status", &stat_domain, None);

    let buffer = EnumVariable::new("buffer_cube", "buffer", &buffer_domain, None);
    let gripper = EnumVariable::new("gripper_cube", "gripper", &gripper_domain, None);
    let table = EnumVariable::new("table_cube", "table", &table_domain, None);

    // colors
    let red = Predicate::EQRL(color.clone(), String::from("red"));
    let green = Predicate::EQRL(color.clone(), String::from("green"));

    // act stat predicates
    let stat_active = Predicate::EQRL(act_stat.clone(), String::from("active"));
    let stat_idle = Predicate::EQRL(act_stat.clone(), String::from("idle"));
    let not_stat_active = Predicate::NOT(Box::new(stat_active.clone()));
    let not_stat_idle = Predicate::NOT(Box::new(stat_idle.clone()));

    // ref stat predicates
    let set_stat_active = Predicate::EQRL(ref_stat.clone(), String::from("active"));
    let set_stat_idle = Predicate::EQRL(ref_stat.clone(), String::from("idle"));
    let not_set_stat_active = Predicate::NOT(Box::new(set_stat_active.clone()));
    let not_set_stat_idle = Predicate::NOT(Box::new(set_stat_idle.clone()));

    // act pos predicates
    let pos_buffer = Predicate::EQRL(act_pos.clone(), String::from("buffer"));
    let pos_table = Predicate::EQRL(act_pos.clone(), String::from("table"));
    let pos_home = Predicate::EQRL(act_pos.clone(), String::from("home"));
    let not_pos_buffer = Predicate::NOT(Box::new(pos_buffer.clone()));
    let not_pos_table = Predicate::NOT(Box::new(pos_table.clone()));
    let not_pos_home = Predicate::NOT(Box::new(pos_home.clone()));

    // ref pos predicates
    let set_pos_buffer = Predicate::EQRL(ref_pos.clone(), String::from("buffer"));
    let set_pos_table = Predicate::EQRL(ref_pos.clone(), String::from("table"));
    let set_pos_home = Predicate::EQRL(ref_pos.clone(), String::from("home"));
    let not_set_pos_buffer = Predicate::NOT(Box::new(set_pos_buffer.clone()));
    let not_set_pos_table = Predicate::NOT(Box::new(set_pos_table.clone()));
    let not_set_pos_home = Predicate::NOT(Box::new(set_pos_home.clone()));

    // act buffer predicates
    let buffer_cube = Predicate::EQRL(buffer.clone(), String::from("cube"));
    let buffer_ball = Predicate::EQRL(buffer.clone(), String::from("ball"));
    let buffer_empty = Predicate::EQRL(buffer.clone(), String::from("empty"));
    let _not_buffer_cube = Predicate::NOT(Box::new(buffer_cube.clone()));
    let _not_buffer_ball = Predicate::NOT(Box::new(buffer_ball.clone()));
    let _not_buffer_empty = Predicate::NOT(Box::new(buffer_empty.clone()));
    
    // act gripper predicates
    let gripper_cube = Predicate::EQRL(gripper.clone(), String::from("cube"));
    let gripper_ball = Predicate::EQRL(gripper.clone(), String::from("ball"));
    let gripper_empty = Predicate::EQRL(gripper.clone(), String::from("empty"));
    let _not_gripper_cube = Predicate::NOT(Box::new(gripper_cube.clone()));
    let _not_gripper_ball = Predicate::NOT(Box::new(gripper_ball.clone()));
    let _not_gripper_empty = Predicate::NOT(Box::new(gripper_empty.clone()));

    // act table predicates
    let table_cube = Predicate::EQRL(table.clone(), String::from("cube"));
    let table_ball = Predicate::EQRL(table.clone(), String::from("ball"));
    let table_empty = Predicate::EQRL(table.clone(), String::from("empty"));
    let _not_table_cube = Predicate::NOT(Box::new(table_cube.clone()));
    let _not_table_ball = Predicate::NOT(Box::new(table_ball.clone()));
    let _not_table_empty = Predicate::NOT(Box::new(table_empty.clone()));

    // are ref == act predicates
    let pos_stable = Predicate::EQRR(act_pos.clone(), ref_pos.clone());
    let stat_stable = Predicate::EQRR(act_stat.clone(), ref_stat.clone());
    let _not_pos_stable = Predicate::EQRR(act_pos.clone(), ref_pos.clone());
    let _not_stat_stable = Predicate::EQRR(act_stat.clone(), ref_stat.clone());

    let t1 = Transition::new(
        "start_activate", 
        &Predicate::AND(
            vec!(
                not_stat_active.clone(),
                not_set_stat_active.clone()
            )
        ),
        &set_stat_active
    );

    let t2 = Transition::new(
        "finish_activate", 
        &Predicate::AND(
            vec!(
                set_stat_active.clone(),
                not_stat_active.clone()
            )
        ),
        &stat_active
    );

    let t3 = Transition::new(
        "start_deactivate", 
        &Predicate::AND(
            vec!(
                not_stat_idle.clone(),
                not_set_stat_idle.clone()
            )
        ),
        &set_stat_idle
    );

    let t4 = Transition::new(
        "finish_deactivate", 
        &Predicate::AND(
            vec!(
                not_stat_idle.clone(),
                set_stat_idle.clone()
            )
        ),
        &stat_idle
    );

    let t5 = Transition::new(
        "start_move_to_buffer",
        &Predicate::AND(
            vec!(
                stat_active.clone(),
                set_stat_active.clone(),
                pos_stable.clone(),
                not_pos_buffer.clone(),
                not_set_pos_buffer.clone()
            )
        ),
        &set_pos_buffer
    );

    let t6 = Transition::new(
        "finish_move_to_buffer",
        &Predicate::AND(
            vec!(
                stat_active.clone(),
                set_stat_active.clone(),
                not_pos_buffer.clone(),
                set_pos_buffer.clone()
            )
        ),
        &pos_buffer
    );

    let t7 = Transition::new(
        "start_move_to_table",
        &Predicate::AND(
            vec!(
                stat_active.clone(),
                set_stat_active.clone(),
                pos_stable.clone(),
                not_pos_table.clone(),
                not_set_pos_table.clone()
            )
        ),
        &set_pos_table
    );

    let t8 = Transition::new(
        "finish_move_to_table",
        &Predicate::AND(
            vec!(
                stat_active.clone(),
                set_stat_active.clone(),
                not_pos_table.clone(),
                set_pos_table.clone()
            )
        ),
        &pos_table
    );

    let t9 = Transition::new(
        "start_move_to_home",
        &Predicate::AND(
            vec!(
                stat_active.clone(),
                set_stat_active.clone(),
                pos_stable.clone(),
                not_pos_home.clone(),
                not_set_pos_home.clone()
            )
        ),
        &set_pos_home
    );

    let t10 = Transition::new(
        "finish_move_to_home",
        &Predicate::AND(
            vec!(
                stat_active.clone(),
                set_stat_active.clone(),
                not_pos_home.clone(),
                set_pos_home.clone()
            )
        ),
        &pos_home
    );

    let t11 = Transition::new(
        "take_cube_from_buffer",
        &Predicate::AND(
            vec!(
                buffer_cube.clone(),
                stat_active.clone(),
                set_stat_active.clone(),
                pos_buffer.clone(),
                set_pos_buffer.clone()
            )
        ),
        &Predicate::AND(
            vec!(
                gripper_cube.clone(),
                table_empty.clone(),
                buffer_empty.clone()
            )
        )
    );

    let t12 = Transition::new(
        "take_cube_from_table",
        &Predicate::AND(
            vec!(
                table_cube.clone(),
                stat_active.clone(),
                set_stat_active.clone(),
                pos_table.clone(),
                set_pos_table.clone()
            )
        ),
        &Predicate::AND(
            vec!(
                gripper_cube.clone(),
                table_empty.clone(),
                buffer_empty.clone()
            )
        )
    );

    let t13 = Transition::new(
        "leave_cube_at_buffer",
        &Predicate::AND(
            vec!(
                gripper_cube.clone(),
                stat_active.clone(),
                set_stat_active.clone(),
                pos_buffer.clone(),
                set_pos_buffer.clone()
            )
        ),
        &Predicate::AND(
            vec!(
                gripper_empty.clone(),
                table_empty.clone(),
                buffer_cube.clone()
            )
        )
    );

    let t14 = Transition::new(
        "leave_cube_at_table",
        &Predicate::AND(
            vec!(
                gripper_cube.clone(),
                stat_active.clone(),
                set_stat_active.clone(),
                pos_table.clone(),
                set_pos_table.clone()
            )
        ),
        &Predicate::AND(
            vec!(
                gripper_empty.clone(),
                table_cube.clone(),
                buffer_empty.clone()
            )
        )
    );

    let s1 = Predicate::ALWAYS(
        Box::new(
            Predicate::PBEQ(
                vec!(
                    gripper_cube.clone(),
                    table_cube.clone(),
                    buffer_cube.clone()
                ),
                1
            )
        )
    );

    let s2 = Predicate::ALWAYS(
        Box::new(
            Predicate::AND(
                vec!(
                    Predicate::NOT(Box::new(gripper_ball.clone())),
                    Predicate::NOT(Box::new(table_ball.clone())),
                    Predicate::NOT(Box::new(buffer_ball.clone())),
                )
            )
        )
    );

    let s3 = Predicate::NEVER(
        Box::new(
            Predicate::AFTER(
                Box::new(pos_table.clone()),
                Box::new(pos_buffer.clone())
            )
        )
    );

    let s4 = Predicate::NEVER(
        Box::new(
            Predicate::AFTER(
                Box::new(pos_buffer.clone()),
                Box::new(pos_table.clone())
            )
        )
    );

    let init = Predicate::AND(
        vec!(
            pos_stable.clone(),
            pos_buffer.clone(),
            stat_stable.clone(),
            stat_idle.clone(),
            table_cube.clone()
        )
    );

    let goal1 = Predicate::AND(
        vec!(
            pos_table.clone(),
            stat_idle.clone(),
            buffer_cube.clone(),
            red.clone()
        )
    );

    let goal2 = Predicate::AND(
        vec!(
            pos_buffer.clone(),
            stat_idle.clone(),
            table_cube.clone()
        )
    );

    let goal3 = Predicate::AND(
        vec!(
            pos_table.clone(),
            stat_idle.clone(),
            buffer_cube.clone(),
            green.clone()
        )
    );

    let gspec = Predicate::SEQUENCE(
        vec!(
            goal1.clone(),
            goal2.clone(),
            // goal3.clone()
        )  
    );

    let specs = Predicate::AND(
        vec!(
            s1, s2, s3, s4, gspec
        )
    );

    let goals = vec!((&goal1, None), (&goal2, None)); //, (&goal3, None));

    let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);

    let problem = MultGoalsPlanningProblem::new("problem_1", &init, &goals, &trans, &specs, &max_steps);
    
    let result = MultGoalsIncremental::new(&problem);

    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
    println!("trace: ");

    for t in &result.trace{
 
        println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        println!("=========================");
    }
}