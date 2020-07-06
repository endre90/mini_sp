use mini_sp_tools::*;
use mini_sp_examples::gripper::incremental_grip;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();
    let nr_balls: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let balls = match nr_balls {
        1 => vec!("1"),
        2 => vec!("1", "2"),
        3 => vec!("1", "2", "3"),
        4 => vec!("1", "2", "3", "4"),
        5 => vec!("1", "2", "3", "4", "5"),
        6 => vec!("1", "2", "3", "4", "5", "6"),
        7 => vec!("1", "2", "3", "4", "5", "6", "7"),
        8 => vec!("1", "2", "3", "4", "5", "6", "7", "8"),
        9 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9"),
        10 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10"),
        _ => panic!("Too many balls"),
    };

    let trans = incremental_grip(
        &vec!("r1"),
        &balls,
        &vec!("a", "b"),
        &vec!("gl", "gr")
    );

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("r1", "r1", &robot_pos_domain, None).clone(), String::from("a")).clone(),
        Predicate::EQRL(EnumVariable::new("gl", "gl", &gripper_domain, None).clone(), String::from("e")).clone(),
        Predicate::EQRL(EnumVariable::new("gr", "gr", &gripper_domain, None).clone(), String::from("e")).clone(),
    );

    for b in &balls {
        init_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("b{}", b), &format!("b{}", b), &ball_pos_domain, None).clone(), String::from("a")).clone(),
        )
    } 

    let init = Predicate::AND(init_predicates);

    let mut goal_predicates = vec!();

    for b in &balls {
        goal_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("b{}", b), &format!("b{}", b), &ball_pos_domain, None).clone(), String::from("b")).clone(),
        )
    } 

    let goal = Predicate::AND(goal_predicates);

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