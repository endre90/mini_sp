use mini_sp_tools::*;
use mini_sp_examples::gripper_orig::compositional_grip_orig;
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

    let trans = compositional_grip_orig(
        &vec!("r1"),
        &balls,
        &vec!("a", "b"),
        &vec!("gl", "gr")
    );

    let ball_param = Parameter::new("b", &false);
    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let tf_domain = vec!("t", "f");

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("robot_r1_at_a", "robot_r1_at_a", &tf_domain, Some(&robot_param)), "t".to_string()),
        Predicate::EQRL(EnumVariable::new("robot_r1_at_b", "robot_r1_at_b", &tf_domain, Some(&robot_param)), "f".to_string()),
        Predicate::EQRL(EnumVariable::new("gripper_gr_free","gripper_gr_free", &tf_domain, Some(&gripper_param)), "t".to_string()),
        Predicate::EQRL(EnumVariable::new("gripper_gl_free","gripper_gl_free", &tf_domain, Some(&gripper_param)), "t".to_string()),
    );

    for b in &balls {
        init_predicates.extend(
            vec!(
                Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_b", b), &format!("ball_{}_at_b", b), &tf_domain, Some(&ball_param)), "f".to_string()),
                Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_a", b), &format!("ball_{}_at_a", b), &tf_domain, Some(&ball_param)), "t".to_string()),
                Predicate::EQRL(EnumVariable::new(&format!("robot_r1_carries_ball_{}", b), &format!("robot_r1_carries_ball_{}", b), &tf_domain, Some(&ball_param)), "f".to_string())
            )
        )
    } 

    let init = ParamPredicate::new(&init_predicates.iter().map(|x| x).collect());

    let mut goal_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("robot_r1_at_b", "robot_r1_at_b", &tf_domain, Some(&robot_param)), "t".to_string())
    );

    for b in &balls {
        goal_predicates.push(
            Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_b", b), &format!("ball_{}_at_b", b), &tf_domain, Some(&ball_param)), "t".to_string()),
        )
    } 

    let goal = ParamPredicate::new(&goal_predicates.iter().map(|x| x).collect());

    let params = vec!(gripper_param, ball_param, robot_param);
    let mut params_sorted = vec!();
    for po in param_order {
        for p in &params {
            if po == p.name {
                params_sorted.push(p)
            }
        }
    }

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