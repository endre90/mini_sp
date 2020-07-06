use mini_sp_tools::*;
use mini_sp_examples::gripper_orig::incremental_grip_orig;
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

    let trans = incremental_grip_orig(
        &vec!("r1"),
        &balls,
        &vec!("a", "b"),
        &vec!("gl", "gr")
    );

    // let ball_pos_domain = vec!("a", "b", "gl", "gr");
    // let robot_pos_domain = vec!("a", "b");
    // let gripper_domain = vec!("e", "f");
    let tf_domain = vec!("t", "f");

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("robot_r1_at_a", "robot_r1_at_a", &tf_domain, None), "t".to_string()),
        Predicate::EQRL(EnumVariable::new("robot_r1_at_b", "robot_r1_at_b", &tf_domain, None), "f".to_string()),
        Predicate::EQRL(EnumVariable::new("gripper_gr_free","gripper_gr_free", &tf_domain, None), "t".to_string()),
        Predicate::EQRL(EnumVariable::new("gripper_gl_free","gripper_gl_free", &tf_domain, None), "t".to_string()),
    );

    for b in &balls {
        init_predicates.extend(
            vec!(
                Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_b", b), &format!("ball_{}_at_b", b), &tf_domain, None), "f".to_string()),
                Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_a", b), &format!("ball_{}_at_a", b), &tf_domain, None), "t".to_string()),
                Predicate::EQRL(EnumVariable::new(&format!("robot_r1_carries_ball_{}", b), &format!("robot_r1_carries_ball_{}", b), &tf_domain, None), "f".to_string())
            )
        )
    } 

    let init = Predicate::AND(init_predicates);

    // let mut goal_predicates = vec!();

    let mut goal_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("robot_r1_at_b", "robot_r1_at_b", &tf_domain, None), "t".to_string())
    );

    for b in &balls {
        goal_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_b", b), &format!("ball_{}_at_b", b), &tf_domain, None), "t".to_string()),
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