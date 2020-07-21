use mini_sp_tools::*;
use mini_sp_examples::movie::compositional_movie;
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

    let trans = compositional_movie(&stuff2);

    let domain = vec!("t", "f");
    let movie_param = Parameter::new("m", &false);

    let mut init_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("counter_at_other_than_two_hours", "counter_at_other_than_two_hours", &domain, Some(&movie_param)), String::from("t")),
        Predicate::EQRL(EnumVariable::new("counter_at_two_hours", "counter_at_two_hours", &domain, Some(&movie_param)), String::from("f")),
        Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, Some(&movie_param)), String::from("f")),
        Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, Some(&movie_param)), String::from("f"))
    );

    for s in &stuff {
        for f in vec!("c", "d", "p", "s", "k") {
            init_predicates.extend(
                vec!(
                    Predicate::EQRL(EnumVariable::new(&format!("{}_{}", f, s), &format!("{}_{}", f, s), &domain, Some(&Parameter::new(&format!("{}", f), &false))), String::from("t")),
                    Predicate::EQRL(EnumVariable::new(&format!("have_{}", f), &format!("have_{}", f), &domain, Some(&Parameter::new(&format!("{}", f), &false))), String::from("f"))
                )
            )
        }
    } 

    let init = ParamPredicate::new(&init_predicates.iter().map(|x| x).collect());

    let mut goal_predicates = vec!(
        Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, Some(&movie_param)), String::from("t")),
        Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, Some(&movie_param)), String::from("t"))
    );

    for s in &stuff {
        for f in vec!("c", "d", "p", "s", "k") {
            goal_predicates.extend(
                vec!(
                    Predicate::EQRL(EnumVariable::new(&format!("have_{}", f), &format!("have_{}", f), &domain, Some(&Parameter::new(&format!("{}", f), &false))), String::from("t"))
                )
            )
        }
    } 

    let goal = ParamPredicate::new(&goal_predicates.iter().map(|x| x).collect());

    let mut params = vec!();
    for f in vec!("c", "d", "p", "s", "k", "m") {
        params.push(Parameter::new(&format!("{}", f), &false))
    }
    
    let mut params_sorted = vec!();
    for po in param_order {
        for p in &params {
            if po == p.name {
                params_sorted.push(p)
                // if po != "b".to_string() {
                //     params_sorted.push(p)
                // } else {
                //     params_sorted.extend(&ball_params)
                // }
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

    println!("plan_found: {:?}", result.plan_found);
    println!("plan_lenght: {:?}", result.plan_length);
    println!("time_to_solve: {:?}", result.time_to_solve);
}