use mini_sp_tools::*;
use mini_sp_examples::sticks::incremental_sticks;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();
    let nr_sticks: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let mut stuff = vec!();
    for b in 0..=nr_sticks {
        stuff.push(format!("{}", b))
    };

    let sticks = stuff.iter().map(|x| x.as_str()).collect();

    let trans = incremental_sticks(&sticks);

    let turn_domain = vec!("c", "k");
    let tf_domain = vec!("t", "f");

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k")),
        Predicate::EQRL(EnumVariable::new("sticks", "sticks", &sticks, None), String::from(format!("{}", nr_sticks)))
    );

    let init = Predicate::AND(init_predicates);

    let mut goal_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k")),
        Predicate::EQRL(EnumVariable::new("sticks", "sticks", &sticks, None), String::from("0"))
    );

    let goal = Predicate::AND(goal_predicates);

    let problem = PlanningProblem::new("problem_1", &init, &goal, &trans, &Predicate::TRUE, &max_steps);
    
    // let result = Incremental::new(&problem);
    // let result2 = IncrementalDenial::new(&problem, &vec!(&result));
    // let result3 = IncrementalDenial::new(&problem, &vec!(&result, &result2));
    // let result4 = IncrementalDenial::new(&problem, &vec!(&result, &result2, &result3));
    // let result5 = IncrementalDenial::new(&problem, &vec!(&result, &result2, &result3, &result4));
    // let result6 = IncrementalDenial::new(&problem, &vec!(&result, &result2, &result3, &result4, &result5));
    // let result7 = IncrementalDenial::new(&problem, &vec!(&result, &result2, &result3, &result4, &result5, &result6));
    let all = IncrementalAll::new(&problem, 10);

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT               ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT 2             ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result2.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT 3             ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result3.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT 4             ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result4.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT 5             ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result5.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT 5             ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result6.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    // println!("\n");
    // println!("============================================");
    // println!("              PLANNING RESULT 5             ");
    // println!("============================================");
    // println!("trace: ");
    // for t in &result7.raw_trace{
 
    //     // println!("state: {:?}", t.state);
    //     println!("trans: {:?}", t.trans);
    //     // println!("=========================");
    // }

    println!("\n");
    println!("============================================");
    println!("             ALL PLANNING RESULTS           ");
    println!("============================================");
    for p in all {
        println!("trace: ");
        for t in &p.trace{
        
            // println!("state: {:?}", t.state);
            println!("trans: {:?}", t.trans);
            // println!("=========================");
        }
    }

}