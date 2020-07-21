use mini_sp_tools::*;
use mini_sp_examples::movie::incremental_movie;
use std::env;

fn main() {

    let mut param_order: Vec<String> = env::args().collect();

    param_order.drain(0..1);
    let max_steps: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();
    let nr_stuff: u32 = param_order.drain(0..1).collect::<Vec<String>>()[0].parse().unwrap();

    let mut stuff = vec!();
    for b in 1..=nr_stuff {
        stuff.push(format!("{}", b))
    };

    let stuff2 = stuff.iter().map(|x| x.as_str()).collect();

    let trans = incremental_movie(&stuff2);

    let domain = vec!("t", "f");

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("counter_at_other_than_two_hours", "counter_at_other_than_two_hours", &domain, None), String::from("t")),
        Predicate::EQRL(EnumVariable::new("counter_at_two_hours", "counter_at_two_hours", &domain, None), String::from("f")),
        Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, None), String::from("f")),
        Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, None), String::from("f"))
    );

    for b in &stuff {
        init_predicates.extend(
            vec!(
                Predicate::EQRL(EnumVariable::new(&format!("chips_{}", b), &format!("chips_{}", b), &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new(&format!("dip_{}", b), &format!("dip_{}", b), &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new(&format!("pop_{}", b), &format!("pop_{}", b), &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new(&format!("cheese_{}", b), &format!("cheese_{}", b), &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new(&format!("crackers_{}", b), &format!("crackers_{}", b), &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new("have_chips", "have_chips", &domain, None), String::from("f")),
                Predicate::EQRL(EnumVariable::new("have_dip", "have_dip", &domain, None), String::from("f")),
                Predicate::EQRL(EnumVariable::new("have_pop", "have_pop", &domain, None), String::from("f")),
                Predicate::EQRL(EnumVariable::new("have_cheese", "have_cheese", &domain, None), String::from("f")),
                Predicate::EQRL(EnumVariable::new("have_crackers", "have_crackers", &domain, None), String::from("f"))
            )
        )
    } 

    let init = Predicate::AND(init_predicates);

    let mut goal_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, None), String::from("t")),
        Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, None), String::from("t"))
    );

    for b in &stuff {
        goal_predicates.extend(
            vec!(
                Predicate::EQRL(EnumVariable::new("have_chips", "have_chips", &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new("have_dip", "have_dip", &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new("have_pop", "have_pop", &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new("have_cheese", "have_cheese", &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new("have_crackers", "have_crackers", &domain, None), String::from("t"))
            )
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
 
        // println!("state: {:?}", t.state);
        println!("trans: {:?}", t.trans);
        // println!("=========================");
    }

    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
}