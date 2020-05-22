use std::time::{Duration, Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct Activate {
    pub params: Vec<Parameter>
}

pub struct StateToParamPredicate {
    pub state: Vec<String>,
    pub prob: ParamPlanningProblem,
    pub ppred: ParamPredicate
}

pub struct Concatenate {
    pub results: Vec<ParamPlanningResult>
}

pub struct RemoveLoops {
    pub results: Vec<ParamPlanningResult>
}

pub struct Compositional {
    pub prob: ParamPlanningProblem,
    pub params: Vec<Parameter>,
    pub level: u32
}

impl Activate {
    pub fn new(params: &Vec<&Parameter>) -> Vec<Parameter> {
        let mut new_params: Vec<Parameter> = vec!();
        let mut activated_ff: bool = false;
        match params.iter().all(|x| x.value) {
            true => panic!("Error 830d4128-68b0-42f2-9ec6-64717fd17b74: All parameters active in Activate."),
            false => {
                for param in params {
                    if param.value | activated_ff {
                        new_params.push(param.to_owned().to_owned())
                    } else {
                        let activated = Parameter::new(&param.name, &true);
                        activated_ff = true;
                        new_params.push(activated)
                    }
                }
            }
        }
        new_params
    }
}

impl StateToParamPredicate {
    pub fn new(state: &Vec<&str>, prob: &ParamPlanningProblem) -> ParamPredicate {
        let mut pred_vec: Vec<Predicate> = vec!();
        let prob_vars = GetParamProblemVars::new(prob);
        for s in state {
            let sep: Vec<&str> = s.split(" -> ").collect();
            let mut var: EnumVariable = EnumVariable::default();
            for v in &prob_vars {
                match v.name == sep[0] {
                    true => var = v.to_owned(),
                    false => ()
                };
            };
            pred_vec.push(Predicate::EQRL(var, String::from(sep[1])));
        } 
        ParamPredicate::new(&pred_vec.iter().map(|x| x).collect())
    }
}

impl Concatenate {
    pub fn new(results: &Vec<&ParamPlanningResult>) -> ParamPlanningResult {
        match results.len() == 0 {
            true => panic!("Error e1146f6a-9f3e-46df-8ec1-14f9b2bdd820: No results to concatenate."),
            false => {
                ParamPlanningResult {
                    plan_found: results.iter().all(|x| x.plan_found),
                    plan_length: results.iter().map(|x| x.plan_length).sum(),
                    level: results[0].level,
                    concat: 123456789,
                    trace: {
                        let mut conc_plan_trace: Vec<PlanningFrame> = vec!();
                        for res in results {
                            conc_plan_trace.extend(res.trace.to_owned());
                        }
                        conc_plan_trace
                    },
                    time_to_solve: results.iter().map(|x| x.time_to_solve).sum()
                }
            }
        }
    }
}

// messy, will need to make state and assignment structs... also have to be tested
// can we avoud making loops in the first place?
impl RemoveLoops {
    pub fn new(result: &ParamPlanningResult) -> ParamPlanningResult {
        let mut duplicates: Vec<(PlanningFrame, usize, usize)> = vec!();
        let mut sorted_trace: Vec<PlanningFrame> = vec!();

        for r in &result.trace {
            let mut sorted_state = r.state.clone();
            sorted_state.sort();
            let frame: PlanningFrame = PlanningFrame::new(&sorted_state.iter().map(|x| x.as_str()).collect(), &r.trans);
            sorted_trace.push(frame);
        };

        for tr in &sorted_trace {
            let start = match sorted_trace.iter().position(|x| x.state == tr.state) {
                Some(y) => y as usize,
                None => 666
            };
            let finish = match sorted_trace.iter().rposition(|x| x.state == tr.state) {
                Some(y) => y as usize,
                None => 666
            };
            if start != finish && start != 666 && finish != 666 {
                if !duplicates.iter().any(|x| x.0.state == tr.state) {
                    duplicates.push((tr.to_owned(), start, finish))
                }   
            }
        }

        duplicates.sort();
        duplicates.dedup();

        let mut fixed: Vec<PlanningFrame> = vec!();

        while duplicates.len() != 0 {
            println!("DUPLICATES: {:?}\n", duplicates);
            if duplicates[0].1 != 123456789 {
                fixed = sorted_trace.drain(duplicates[0].1 + 1..duplicates[0].2 + 1).collect();
                duplicates.remove(0);
                if duplicates.len() != 0 {
                    duplicates[0].1 = match sorted_trace.iter().position(|x| x.state == duplicates[0].0.state) {
                        Some(y) => y as usize,
                        None => 123456789
                    };
                    duplicates[0].2 = match sorted_trace.iter().rposition(|x| x.state == duplicates[0].0.state) {
                        Some(y) => y as usize,
                        None => 123456789
                    };
                }
            } else {
                duplicates.remove(0);
                if duplicates.len() != 0 {
                    duplicates[0].1 = match sorted_trace.iter().position(|x| x.state == duplicates[0].0.state) {
                        Some(y) => y as usize,
                        None => 123456789
                    };
                    duplicates[0].2 = match sorted_trace.iter().rposition(|x| x.state == duplicates[0].0.state) {
                        Some(y) => y as usize,
                        None => 123456789
                    };
                }
            }
        }

        ParamPlanningResult {
            plan_found: result.plan_found,
            plan_length: sorted_trace.len() as u32 - 1,
            level: result.level,
            concat: result.concat,
            // trace: sorted_trace,
            trace: fixed,
            time_to_solve: result.time_to_solve
        }        
    }
}

impl Compositional {
    pub fn new(prob: &ParamPlanningProblem, params: &Vec<&Parameter>, level: &u32) -> ParamPlanningResult {
        let return_result = match params.iter().all(|x| !x.value) {
            true => {
                let first_params = Activate::new(params);
                let first_result = ParamIncremental::new(&prob, &first_params.iter().map(|x| x).collect(), &level, &0);
                recursive_subfn(&first_result, &prob, &params, &level)
            },
            false => {
                let first_result = ParamIncremental::new(&prob, &params.iter().map(|&x| x).collect(), &level, &0);
                recursive_subfn(&first_result, &prob, &params, &level)
            }
        };

        fn recursive_subfn(result: &ParamPlanningResult, prob: &ParamPlanningProblem, params: &Vec<&Parameter>, level: &u32) -> ParamPlanningResult {
            let level = level + 1;
            let mut final_result: ParamPlanningResult = result.to_owned();
            if !params.iter().all(|x| x.value) {
                if result.plan_found {
                    let mut inheritance: Vec<String> = vec!() ;
                    let mut level_subresults = vec!();
                    let activated_params = Activate::new(&params);
                    let mut concat: u32 = 0;
                    if result.plan_length != 0 {
                        for i in 0..=result.trace.len() - 1 {
                            if i == 0 {
                                let next_prob = ParamPlanningProblem::new(
                                    &format!("problem_l{:?}_c{:?}", level, concat),
                                    params,
                                    &prob.init,
                                    &StateToParamPredicate::new(&result.trace[i + 1].state.iter().map(|x| x.as_str()).collect(), &prob),
                                    &prob.trans,
                                    &prob.ltl_specs,
                                    &prob.max_steps
                                );
                                let next_result = ParamIncremental::new(&next_prob, &activated_params.iter().map(|x| x).collect(), &level, &concat);
                                if next_result.plan_found {
                                    level_subresults.push(next_result.to_owned());
                                    match next_result.trace.last() {
                                        Some(x) => inheritance = x.state.clone(),
                                        None => panic!("No tail in the plan! 1")
                                    }
                                } else {
                                    panic!("NO PLAN FOUND 1 !")
                                }
                                concat = concat + 1;                       
                            }
                        }
                    }
                    let level_result = Concatenate::new(&level_subresults.iter().map(|x| x).collect());
                    let delooped_and_sorted = RemoveLoops::new(&final_result);
                    let final_level_result = recursive_subfn(&delooped_and_sorted, &prob, &activated_params.iter().map(|x| x).collect(), &level);
                }
            }
            final_result
        }
        return_result
    }
}
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


#[test]
fn test_activate() {
    let param_a = Parameter::new("a", &true);
    let param_b = Parameter::new("b", &false);
    let param_c = Parameter::new("c", &false);
    let params = vec!(&param_a, &param_b, &param_c);
    assert_eq!("[Parameter { name: \"a\", value: true }, Parameter { name: \"b\", value: false }, Parameter { name: \"c\", value: false }]", format!("{:?}", params));
    let new_params = Activate::new(&params);
    assert_eq!("[Parameter { name: \"a\", value: true }, Parameter { name: \"b\", value: true }, Parameter { name: \"c\", value: false }]", format!("{:?}", new_params));
}

#[test]
#[should_panic(expected = "Error 830d4128-68b0-42f2-9ec6-64717fd17b74: All parameters active in Activate.")]
fn test_activate_panic() {
    let param_a = Parameter::new("a", &true);
    let param_b = Parameter::new("b", &true);
    let param_c = Parameter::new("c", &true);
    let params = vec!(&param_a, &param_b, &param_c);
    Activate::new(&params);
}