use std::time::{Duration, Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct GeneratePredicate {
    pub params: Vec<Parameter>,
    pub ppreds: ParamPredicate,
    pub pred: Predicate
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct GenerateTransitions {
    pub params: Vec<Parameter>,
    pub ptrans: Vec<ParamTransition>,
    pub trans: Vec<Transition>
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct ParamTransition {
    pub name: String,
    pub guard: ParamPredicate,
    pub update: ParamPredicate
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct ParamPlanningProblem {
    pub name: String,
    pub params: Vec<Parameter>,
    pub init: ParamPredicate,
    pub goal: ParamPredicate,
    pub trans: Vec<ParamTransition>,
    // pub ltl_specs: ParamPredicate,
    pub ltl_specs: Predicate,
    pub max_steps: u32
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct ParamIncremental {
    pub prob: ParamPlanningProblem
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

pub struct GetParamPlanningResultZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub model: Z3_model,
    pub level: u32,
    pub concat: u32,
    pub nr_steps: u32,
    pub frames: PlanningResult
}

impl GeneratePredicate {
    pub fn new(params: &Vec<&Parameter>, ppred: &ParamPredicate) -> Predicate {
        let mut p_own = params.to_owned();
        let default_param = Parameter::default();
        p_own.push(&default_param);
        let mut pred_vec = vec!();
        for pred in &ppred.preds {
            let pred_vars: Vec<EnumVariable> = GetPredicateVars::new(&pred);
            for param in &p_own {
                if pred_vars.iter().any(|x| x.param.name == param.name) && param.value {
                    pred_vec.push(pred.to_owned())
                }
            }
        }
        pred_vec.sort();
        pred_vec.dedup();
        Predicate::AND(pred_vec)
    }
}

impl GenerateTransitions {
    pub fn new(params: &Vec<&Parameter>, ptrans: &Vec<ParamTransition>) -> Vec<Transition> {
        let mut trans_vec = vec!();
        for pt in ptrans {
            let guard = GeneratePredicate::new(&params, &pt.guard);
            let update = GeneratePredicate::new(&params, &pt.update);
            trans_vec.push(
                Transition::new(pt.name.as_str(), &guard, &update)
            )
        }
        trans_vec
    }
}

impl ParamTransition {
    pub fn new(name: &str, guard: &ParamPredicate, update: &ParamPredicate) -> ParamTransition {
        ParamTransition {
            name: name.to_string(),
            guard: guard.to_owned(),
            update: update.to_owned()
        }
    }
}

impl ParamPlanningProblem {
    pub fn new(name: &str, params: &Vec<&Parameter>, init: &ParamPredicate, goal: &ParamPredicate, 
        trans: &Vec<ParamTransition>, ltl_specs: &Predicate, max_steps: &u32) -> ParamPlanningProblem {
        ParamPlanningProblem {
            name: name.to_string(),
            params: params.iter().map(|&x| x.clone()).collect(),
            init: init.to_owned(),
            goal: goal.to_owned(),
            trans: trans.to_owned(),
            ltl_specs: ltl_specs.to_owned(),
            max_steps: max_steps.to_owned()
        }
    }
}

impl ParamIncremental {
    pub fn new(prob: &ParamPlanningProblem, params: &Vec<&Parameter>, level: &u32, concat: &u32) -> ParamPlanningResult {
        let generated_init = GeneratePredicate::new(&params, &prob.init);
        let generated_goal = GeneratePredicate::new(&params, &prob.goal);
        let generated_trans = GenerateTransitions::new(&params, &prob.trans);

        let generated_prob = PlanningProblem::new(
            prob.name.as_str(), 
            &generated_init, 
            &generated_goal, 
            &generated_trans, 
            &prob.ltl_specs,
            &prob.max_steps
        );

        let inc_result = Incremental::new(&generated_prob);

        ParamPlanningResult {
            plan_found: inc_result.plan_found,
            plan_length: inc_result.plan_length,
            level: *level,
            concat: *concat,
            trace: inc_result.trace,
            time_to_solve: inc_result.time_to_solve
        }
    }
}

#[test]
fn test_generate_predicate(){
    let pose_param = Parameter::new("pose", &true);
    let stat_param = Parameter::new("stat", &true);

    let pose_domain = vec!("buffer", "home", "table");
    let stat_domain = vec!("active", "idle");

    let act_pos = EnumVariable::new("act_pos", "pose", &pose_domain, Some(&pose_param));
    let act_stat = EnumVariable::new("act_stat", "status", &stat_domain, Some(&stat_param));

    let stat_active = Predicate::EQRL(act_stat.clone(), String::from("active"));
    let stat_idle = Predicate::EQRL(act_stat.clone(), String::from("idle"));

    let pos_buffer = Predicate::EQRL(act_pos.clone(), String::from("buffer"));
    let pos_table = Predicate::EQRL(act_pos.clone(), String::from("table"));

    let pp1 = ParamPredicate::new(
        &vec!(
            &stat_active,
            &stat_idle,
            &pos_buffer
        )
    );

    // TODO: MORE SOPHISTICATED FILTERING FOR COMPLEX PREDICATES
    let pp2 = ParamPredicate::new(
        &vec!(
            &Predicate::OR(vec!(stat_active, pos_buffer)),
            &stat_idle,
            &pos_table
        )
    );

    let pose_param = Parameter::new("pose", &false);
    let stat_param = Parameter::new("stat", &false);
    let params = vec!(&pose_param, &stat_param);
    let gen1 = GeneratePredicate::new(&params, &pp1);
    assert_eq!("AND([])", &format!("{:?}", gen1));

    let pose_param = Parameter::new("pose", &true);
    let stat_param = Parameter::new("stat", &false);
    let params = vec!(&pose_param, &stat_param);
    let gen2 = GeneratePredicate::new(&params, &pp1);
    assert_eq!("AND([EQRL(EnumVariable { name: \"act_pos\", type: \"pose\", domain: [\"buffer\", \"home\", \"table\"], param: Parameter { name: \"pose\", value: true } }, \"buffer\")])", 
        &format!("{:?}", gen2));

    let pose_param = Parameter::new("pose", &false);
    let stat_param = Parameter::new("stat", &true);
    let params = vec!(&pose_param, &stat_param);
    let gen3 = GeneratePredicate::new(&params, &pp1);
    assert_eq!("AND([EQRL(EnumVariable { name: \"act_stat\", type: \"status\", domain: [\"active\", \"idle\"], param: Parameter { name: \"stat\", value: true } }, \"active\"), EQRL(EnumVariable { name: \"act_stat\", type: \"status\", domain: [\"active\", \"idle\"], param: Parameter { name: \"stat\", value: true } }, \"idle\")])", 
        &format!("{:?}", gen3));

    let pose_param = Parameter::new("pose", &true);
    let stat_param = Parameter::new("stat", &true);
    let params = vec!(&pose_param, &stat_param);
    let gen4 = GeneratePredicate::new(&params, &pp1);
    assert_eq!("AND([EQRL(EnumVariable { name: \"act_pos\", type: \"pose\", domain: [\"buffer\", \"home\", \"table\"], param: Parameter { name: \"pose\", value: true } }, \"buffer\"), EQRL(EnumVariable { name: \"act_stat\", type: \"status\", domain: [\"active\", \"idle\"], param: Parameter { name: \"stat\", value: true } }, \"active\"), EQRL(EnumVariable { name: \"act_stat\", type: \"status\", domain: [\"active\", \"idle\"], param: Parameter { name: \"stat\", value: true } }, \"idle\")])", 
        &format!("{:?}", gen4));

}

#[test]
fn test_paramincremental_1(){

    let max_steps: u32 = 30;

    let pose_param = Parameter::new("pose", &true);
    let stat_param = Parameter::new("stat", &false);
    let cube_param = Parameter::new("cube", &true);

    let pose_domain = vec!("buffer", "home", "table");
    let stat_domain = vec!("active", "idle");
    let buffer_domain = vec!("cube", "ball", "empty");
    let gripper_domain = vec!("cube", "ball", "empty");
    let table_domain = vec!("cube", "ball", "empty");

    let act_pos = EnumVariable::new("act_pos", "pose", &pose_domain, None);
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
    let not_buffer_cube = Predicate::NOT(Box::new(buffer_cube.clone()));
    let not_buffer_ball = Predicate::NOT(Box::new(buffer_ball.clone()));
    let not_buffer_empty = Predicate::NOT(Box::new(buffer_empty.clone()));
    
    // act gripper predicates
    let gripper_cube = Predicate::EQRL(gripper.clone(), String::from("cube"));
    let gripper_ball = Predicate::EQRL(gripper.clone(), String::from("ball"));
    let gripper_empty = Predicate::EQRL(gripper.clone(), String::from("empty"));
    let not_gripper_cube = Predicate::NOT(Box::new(gripper_cube.clone()));
    let not_gripper_ball = Predicate::NOT(Box::new(gripper_ball.clone()));
    let not_gripper_empty = Predicate::NOT(Box::new(gripper_empty.clone()));

    // act table predicates
    let table_cube = Predicate::EQRL(table.clone(), String::from("cube"));
    let table_ball = Predicate::EQRL(table.clone(), String::from("ball"));
    let table_empty = Predicate::EQRL(table.clone(), String::from("empty"));
    let not_table_cube = Predicate::NOT(Box::new(table_cube.clone()));
    let not_table_ball = Predicate::NOT(Box::new(table_ball.clone()));
    let not_table_empty = Predicate::NOT(Box::new(table_empty.clone()));

    // are ref == act predicates
    let pos_stable = Predicate::EQRR(act_pos.clone(), ref_pos.clone());
    let stat_stable = Predicate::EQRR(act_stat.clone(), ref_stat.clone());
    let not_pos_stable = Predicate::EQRR(act_pos.clone(), ref_pos.clone());
    let not_stat_stable = Predicate::EQRR(act_stat.clone(), ref_stat.clone());

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


    let params = vec!(&pose_param, &stat_param, &cube_param);

    let problem = ParamPlanningProblem::new("problem_1", &params, &init, &goal, &trans, &specs, &max_steps);
    
    let level: u32 = 0;
    let concat: u32 = 0;

    let result = ParamIncremental::new(&problem, &params, &level, &concat);

    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
    println!("trace: ");

    for t in &result.trace{
 
        println!("state: {:?}", t.state);
        println!("ppred: {:?}", StateToParamPredicate::new(&t.state.iter().map(|x| x.as_str()).collect(), &problem));
        println!("trans: {:?}", t.trans);
        println!("=========================");
    }
}