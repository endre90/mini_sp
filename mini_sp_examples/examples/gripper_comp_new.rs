use mini_sp_tools::*;
use mini_sp_examples::gripper::compositional_new_grip;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();
    let nr_balls: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let mut balls = vec!();
    for b in 0..=nr_balls {
        balls.push(format!("{}", b))
    };

    // let balls = match nr_balls {
    //     1 => vec!("1"),
    //     2 => vec!("1", "2"),
    //     3 => vec!("1", "2", "3"),
    //     4 => vec!("1", "2", "3", "4"),
    //     5 => vec!("1", "2", "3", "4", "5"),
    //     6 => vec!("1", "2", "3", "4", "5", "6"),
    //     7 => vec!("1", "2", "3", "4", "5", "6", "7"),
    //     8 => vec!("1", "2", "3", "4", "5", "6", "7", "8"),
    //     9 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9"),
    //     10 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10"),
    //     11 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"),
    //     12 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"),
    //     13 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"),
    //     14 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14"),
    //     15 => vec!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"),
    //     _ => panic!("Too many balls"),
    // };

    let trans = compositional_new_grip(
        &vec!("r1"),
        &balls.iter().map(|x| x.as_str()).collect(),
        &vec!("a", "b"),
        &vec!("gl", "gr")
    );

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("r1", "r1", &robot_pos_domain, Some(&robot_param)).clone(), String::from("a")).clone(),
        Predicate::EQRL(EnumVariable::new("gl", "gl", &gripper_domain, Some(&gripper_param)).clone(), String::from("e")).clone(),
        Predicate::EQRL(EnumVariable::new("gr", "gr", &gripper_domain, Some(&gripper_param)).clone(), String::from("e")).clone(),
    );

    let mut ball_params = vec!();
    for b in &balls {
        ball_params.push(
            Parameter::new(&format!("b{}", b), &false)
        );
        init_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("b{}", b), &format!("b{}", b), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", b), &false))).clone(), String::from("a")).clone(),
        )
    } 

    let init = ParamPredicate::new(&init_predicates.iter().map(|x| x).collect());

    let mut goal_predicates = vec!();

    for b in &balls {
        goal_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("b{}", b), &format!("b{}", b), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", b), &false))).clone(), String::from("b")).clone(),
        )
    } 

    let goal = ParamPredicate::new(&goal_predicates.iter().map(|x| x).collect());

    let mut params = vec!(gripper_param, robot_param);
    params.extend(ball_params);
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
    println!("============================================");
    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
}