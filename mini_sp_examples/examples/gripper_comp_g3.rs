use mini_sp_tools::*;
use mini_sp_examples::gripper::compositional_grip_g3;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();
    let nr_balls: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let mut balls = vec!();
    for b in 1..=nr_balls {
        let mut pair = ("".to_string(), "".to_string());
        pair.0 = format!("{}", b);
        if b % 2 == 0 {
            pair.1 = format!("{}", b / 2)
        } else {
            pair.1 = format!("{}", (b / 2) + 1)
        }
        balls.push(pair)
    };

    println!("{:?}", balls);

    let trans = compositional_grip_g3(
        &vec!("r1"),
        &balls.iter().map(|x| (x.0.as_str(), x.1.as_str())).collect(),
        &vec!("a", "b"),
        &vec!("gl", "gr")
    );

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let balls_param = Parameter::new("b", &false); // unrolled for individual balls separatelly
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
            Parameter::new(&format!("b{}", b.1), &false)
        );
        init_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("b{}", b.0), &format!("b{}", b.0), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", b.1), &false))).clone(), String::from("a")).clone(),
        )
    }

    ball_params.sort();
    ball_params.dedup();
    println!("{:?}", ball_params);

    let init = ParamPredicate::new(&init_predicates.iter().map(|x| x).collect());

    let mut goal_predicates = vec!();

    for b in &balls {
        goal_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("b{}", b.0), &format!("b{}", b.0), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", b.1), &false))).clone(), String::from("b")).clone(),
        )
    } 

    let goal = ParamPredicate::new(&goal_predicates.iter().map(|x| x).collect());

    let params = vec!(gripper_param, robot_param, balls_param);
    // // params.extend(ball_params);
    let mut params_sorted = vec!();
    for po in param_order {
        for p in &params {
            if po == p.name {
                if po != "b".to_string() {
                    params_sorted.push(p)
                } else {
                    params_sorted.extend(&ball_params)
                }
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
 
        // println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        // println!("=========================");
    }
    println!("============================================");
    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
}