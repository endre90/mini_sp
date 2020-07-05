use mini_sp_tools::*;
use mini_sp_examples::gripper::compositional;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let trans = compositional();

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let ball_param = Parameter::new("b", &false);
    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let init = ParamPredicate::new(
        &vec!(
            &Predicate::EQRL(EnumVariable::new("r1", "r1", &robot_pos_domain, Some(&robot_param)).clone(), String::from("a")).clone(),
            &Predicate::EQRL(EnumVariable::new("gl", "gl", &gripper_domain, Some(&gripper_param)).clone(), String::from("e")).clone(),
            &Predicate::EQRL(EnumVariable::new("gr", "gr", &gripper_domain, Some(&gripper_param)).clone(), String::from("e")).clone(),
            &Predicate::EQRL(EnumVariable::new("b1", "b1", &ball_pos_domain, Some(&ball_param)).clone(), String::from("a")).clone(),
            &Predicate::EQRL(EnumVariable::new("b2", "b2", &ball_pos_domain, Some(&ball_param)).clone(), String::from("a")).clone(),
            &Predicate::EQRL(EnumVariable::new("b3", "b3", &ball_pos_domain, Some(&ball_param)).clone(), String::from("a")).clone(),
            &Predicate::EQRL(EnumVariable::new("b4", "b4", &ball_pos_domain, Some(&ball_param)).clone(), String::from("a")).clone(),
        )
    );

    let goal = ParamPredicate::new(
        &vec!(
            &Predicate::EQRL(EnumVariable::new("b1", "b1", &ball_pos_domain, Some(&ball_param)).clone(), String::from("b")).clone(),
            &Predicate::EQRL(EnumVariable::new("b2", "b2", &ball_pos_domain, Some(&ball_param)).clone(), String::from("b")).clone(),
            &Predicate::EQRL(EnumVariable::new("b3", "b3", &ball_pos_domain, Some(&ball_param)).clone(), String::from("b")).clone(),
            &Predicate::EQRL(EnumVariable::new("b4", "b4", &ball_pos_domain, Some(&ball_param)).clone(), String::from("b")).clone()
        )
    );

    let params = vec!(gripper_param, ball_param, robot_param);
    let mut params_sorted = vec!();
    for po in param_order {
        for p in &params {
            if po == p.name {
                params_sorted.push(p)
            }
        }
    }
    println!("{:?}", params_sorted);

    let problem = ParamPlanningProblem::new("problem_1", &params_sorted, &init, &goal, &trans, &Predicate::TRUE, &max_steps);
    
    let result = Compositional::new(&problem, &params_sorted);

    println!("\n");
    println!("============================================");
    println!("              PLANNING RESULT               ");
    println!("============================================");
    println!("trace: ");
    for t in &result.trace{
 
        println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        println!("=========================");
    }

    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
}