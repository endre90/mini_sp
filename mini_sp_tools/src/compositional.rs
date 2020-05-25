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
                        conc_plan_trace
                    },
                    time_to_solve: results.iter().map(|x| x.time_to_solve).sum()
                }
            }
        }
    }
}

// messy, will need to make state and assignment structs... also have to be tested
// can we avoid making loops in the first place?
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

        while duplicates.len() != 0 {
            // println!("DUPLICATES: {:?}\n", duplicates);
            if duplicates[0].1 != 123456789 {
                sorted_trace.drain(duplicates[0].1 + 1..duplicates[0].2 + 1).for_each(drop);
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
            trace: sorted_trace,
            time_to_solve: result.time_to_solve
        }        
    }
}

impl Compositional {
    pub fn new(prob: &ParamPlanningProblem, params: &Vec<&Parameter>) -> ParamPlanningResult {
        let return_result = match params.iter().all(|x| !x.value) {
            true => {
                let first_params = Activate::new(params);
                let first_result = ParamIncremental::new(&prob, &first_params.iter().map(|x| x).collect(), &0, &0);
                recursive_subfn(&first_result, &prob, &params, &0)
            },
            false => {
                let first_result = ParamIncremental::new(&prob, &params.iter().map(|&x| x).collect(), &0, &0);
                recursive_subfn(&first_result, &prob, &params, &0)
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
                            } else if i == result.trace.len() - 1 {
                                let next_prob = ParamPlanningProblem::new(
                                    &format!("problem_l{:?}_c{:?}", level, concat),
                                    params,
                                    &StateToParamPredicate::new(&inheritance.iter().map(|x| x.as_str()).collect(), &prob),
                                    &prob.goal,
                                    &prob.trans,
                                    &prob.ltl_specs,
                                    &prob.max_steps
                                );
                                let next_result = ParamIncremental::new(&next_prob, &activated_params.iter().map(|x| x).collect(), &level, &concat);
                                
                                if next_result.plan_found {
                                    level_subresults.push(next_result.clone());
                                } else {
                                    panic!("NO PLAN FOUND 2 !")
                                }
                                concat = concat + 1;
                            } else {
                                let next_prob = ParamPlanningProblem::new(
                                    &format!("problem_l{:?}_c{:?}", level, concat),
                                    params,
                                    &StateToParamPredicate::new(&inheritance.iter().map(|x| x.as_str()).collect(), &prob),
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
                                    panic!("NO PLAN FOUND 3 !")
                                }
                                concat = concat + 1;   
                            }
                        } 
                    } else {
                        // have to investigate this step more... now it feels like a hack
                        let activated_params = Activate::new(&params);
                        let next_prob = ParamPlanningProblem::new(
                            &format!("problem_l{:?}_c{:?}", level, concat),
                            params,
                            &prob.init,
                            &prob.goal,
                            // &StateToParamPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &prob),
                            // &StateToParamPredicate::new(&result.trace[0].state.iter().map(|x| x.as_str()).collect(), &prob),
                            &prob.trans,
                            &prob.ltl_specs,
                            &prob.max_steps
                        );
                        let next_result = ParamIncremental::new(&next_prob, &activated_params.iter().map(|x| x).collect(), &level, &concat);
                        if next_result.plan_found {
                            level_subresults.push(next_result.to_owned());
                            // match next_result.trace.last() {
                            //     Some(x) => inheritance = x.state.clone(),
                            //     None => panic!("No tail in the plan! 3")
                            // }
                        } else {
                            panic!("NO PLAN FOUND 4 !")
                        }
                        // concat = concat + 1;   
                    }
                    let level_result = Concatenate::new(&level_subresults.iter().map(|x| x).collect());
                    for t in 0..level_result.trace.len() {
 
                        println!("only_concat: {:?} : {:?}", t, level_result.trace[t].state);
                        println!("only_concat: {:?} : {:?}", t, level_result.trace[t].trans);
                        println!("=========================");
                    }
                    let delooped_and_sorted = RemoveLoops::new(&level_result);

                    for t in 0..delooped_and_sorted.trace.len() {
 
                        println!("delooped: {:?} : {:?}", t, delooped_and_sorted.trace[t].state);
                        println!("delooped: {:?} : {:?}", t, delooped_and_sorted.trace[t].trans);
                        println!("=========================");
                    }
                    final_result = recursive_subfn(&delooped_and_sorted, &prob, &activated_params.iter().map(|x| x).collect(), &level);
                }
            }
            final_result
        }
        return_result
    }
}
  
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

#[test]
fn test_compositional_1(){

    let max_steps: u32 = 30;

    let pose_param = Parameter::new("pose", &false);
    let stat_param = Parameter::new("stat", &false);
    let cube_param = Parameter::new("cube", &false);

    let pose_domain = vec!("buffer", "home", "table");
    let stat_domain = vec!("active", "idle");
    let buffer_domain = vec!("cube", "ball", "empty");
    let gripper_domain = vec!("cube", "ball", "empty");
    let table_domain = vec!("cube", "ball", "empty");

    let act_pos = EnumVariable::new("act_pos", "pose", &pose_domain, Some(&pose_param));
    let ref_pos = EnumVariable::new("ref_pos", "pose", &pose_domain, Some(&pose_param));

    let act_stat = EnumVariable::new("act_stat", "status", &stat_domain, Some(&stat_param));
    let ref_stat = EnumVariable::new("ref_stat", "status", &stat_domain, Some(&stat_param));

    let buffer = EnumVariable::new("buffer_cube", "buffer", &buffer_domain, Some(&cube_param));
    let gripper = EnumVariable::new("gripper_cube", "gripper", &gripper_domain, Some(&cube_param));
    let table = EnumVariable::new("table_cube", "table", &table_domain, Some(&cube_param));

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

    let t1 = ParamTransition::new(
        "start_activate",
        &ParamPredicate::new(
            &vec!(
                &not_stat_active,
                &not_set_stat_active
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &set_stat_active
            )
        )
    );

    let t2 = ParamTransition::new(
        "finish_activate",
        &ParamPredicate::new(
            &vec!(
                &set_stat_active,
                &not_stat_active
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &stat_active
            )
        )
    );

    let t3 = ParamTransition::new(
        "start_deactivate",
        &ParamPredicate::new(
            &vec!(
                &not_stat_idle,
                &not_set_stat_idle
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &set_stat_idle
            )
        )
    );

    let t4 = ParamTransition::new(
        "finish_deactivate",
        &ParamPredicate::new(
            &vec!(
                &not_stat_idle,
                &set_stat_idle
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &stat_idle
            )
        )
    );
    
    let t5 = ParamTransition::new(
        "start_move_to_buffer",
        &ParamPredicate::new(
            &vec!(
                &stat_active,
                &set_stat_active,
                &pos_stable,
                &not_pos_buffer,
                &not_set_pos_buffer
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &set_pos_buffer
            )
        )
    );

    let t6 = ParamTransition::new(
        "finish_move_to_buffer",
        &ParamPredicate::new(
            &vec!(
                &stat_active,
                &set_stat_active,
                &not_pos_buffer,
                &set_pos_buffer
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &pos_buffer
            )
        )
    );

    let t7 = ParamTransition::new(
        "start_move_to_table",
        &ParamPredicate::new(
            &vec!(
                &stat_active,
                &set_stat_active,
                &pos_stable,
                &not_pos_table,
                &not_set_pos_table
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &set_pos_table
            )
        )
    );

    let t8 = ParamTransition::new(
        "finish_move_to_table",
        &ParamPredicate::new(
            &vec!(
                &stat_active,
                &set_stat_active,
                &not_pos_table,
                &set_pos_table
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &pos_table
            )
        )
    );

    let t9 = ParamTransition::new(
        "start_move_to_home",
        &ParamPredicate::new(
            &vec!(
                &stat_active,
                &set_stat_active,
                &pos_stable,
                &not_pos_home,
                &not_set_pos_home
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &set_pos_home
            )
        )
    );

    let t10 = ParamTransition::new(
        "finish_move_to_home",
        &ParamPredicate::new(
            &vec!(
                &stat_active,
                &set_stat_active,
                &not_pos_home,
                &set_pos_home
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &pos_home
            )
        )
    );

    let t11 = ParamTransition::new(
        "take_cube_from_buffer",
        &ParamPredicate::new(
            &vec!(
                &buffer_cube,
                &stat_active,
                &set_stat_active,
                &pos_buffer,
                &set_pos_buffer
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &gripper_cube,
                &buffer_empty,
                &table_empty
            )
        )
    );

    let t12 = ParamTransition::new(
        "take_cube_from_table",
        &ParamPredicate::new(
            &vec!(
                &table_cube,
                &stat_active,
                &set_stat_active,
                &pos_table,
                &set_pos_table
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &gripper_cube,
                &buffer_empty,
                &table_empty
            )
        )
    );

    let t13 = ParamTransition::new(
        "leave_cube_at_buffer",
        &ParamPredicate::new(
            &vec!(
                &gripper_cube,
                &stat_active,
                &set_stat_active,
                &pos_buffer,
                &set_pos_buffer
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &gripper_empty,
                &buffer_cube,
                &table_empty
            )
        )
    );

    let t14 = ParamTransition::new(
        "leave_cube_at_table",
        &ParamPredicate::new(
            &vec!(
                &gripper_cube,
                &stat_active,
                &set_stat_active,
                &pos_table,
                &set_pos_table
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &gripper_empty,
                &buffer_empty,
                &table_cube
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

    let init = ParamPredicate::new(
        &vec!(
            &stat_stable,
            &stat_idle,
            &pos_stable,
            &pos_buffer,
            &table_cube
        )
    );
    
    let goal = ParamPredicate::new(
        &vec!(
            &stat_idle,
            &pos_table,
            &buffer_cube
        )
    );

    let specs = Predicate::AND(
        vec!(
            s1, s2, s3, s4
        )
    );

    let trans = vec!(t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14);


    let params = vec!(&cube_param, &pose_param, &stat_param);

    let problem = ParamPlanningProblem::new("problem_1", &params, &init, &goal, &trans, &specs, &max_steps);

    let result = Compositional::new(&problem, &params);


    println!("=========================");
    println!("=========================");
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