use mini_sp_tools::*;
use mini_sp_examples::gripper::incremental;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let trans = incremental();

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let init = Predicate::AND(
        vec!(
            Predicate::EQRL(EnumVariable::new("r1", "r1", &robot_pos_domain, None).clone(), String::from("a")).clone(),
            Predicate::EQRL(EnumVariable::new("gl", "gl", &gripper_domain, None).clone(), String::from("e")).clone(),
            Predicate::EQRL(EnumVariable::new("gr", "gr", &gripper_domain, None).clone(), String::from("e")).clone(),
            Predicate::EQRL(EnumVariable::new("b1", "b1", &ball_pos_domain, None).clone(), String::from("a")).clone(),
            Predicate::EQRL(EnumVariable::new("b2", "b2", &ball_pos_domain, None).clone(), String::from("a")).clone(),
            Predicate::EQRL(EnumVariable::new("b3", "b3", &ball_pos_domain, None).clone(), String::from("a")).clone(),
            Predicate::EQRL(EnumVariable::new("b4", "b4", &ball_pos_domain, None).clone(), String::from("a")).clone(),
        )
    );

    let goal = Predicate::AND(
        vec!(
            Predicate::EQRL(EnumVariable::new("b1", "b1", &ball_pos_domain, None).clone(), String::from("b")).clone(),
            Predicate::EQRL(EnumVariable::new("b2", "b2", &ball_pos_domain, None).clone(), String::from("b")).clone(),
            Predicate::EQRL(EnumVariable::new("b3", "b3", &ball_pos_domain, None).clone(), String::from("b")).clone(),
            Predicate::EQRL(EnumVariable::new("b4", "b4", &ball_pos_domain, None).clone(), String::from("b")).clone()
        )
    );

    let problem = PlanningProblem::new("problem_1", &init, &goal, &trans, &Predicate::TRUE, &max_steps);
    
    let result = Incremental::new(&problem);

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