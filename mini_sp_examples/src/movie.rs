// In this domain, the goal is always the same (to have lots of snacks in order to watch a movie), 
// but the number of constants increases with problem number. Some planners have combinatorial 
// problems in such cases. This domain was created by Corin Anderson.

// (define (domain movie-strips)
//   (:predicates (movie-rewound)
//                (counter-at-two-hours)
// 	       (counter-at-other-than-two-hours)
//                (counter-at-zero)
//                (have-chips)
//                (have-dip)
//                (have-pop)
//                (have-cheese)
//                (have-crackers)
//                (chips ?x)
//                (dip ?x)
//                (pop ?x)
//                (cheese ?x)
//                (crackers ?x))
  
//   (:action rewind-movie-2
//            :parameters ()
// 	   :precondition (counter-at-two-hours)
//            :effect (movie-rewound))
  
//   (:action rewind-movie
//            :parameters ()
// 	   :precondition (counter-at-other-than-two-hours)
//            :effect (and (movie-rewound)
//                         ;; Let's assume that the movie is 2 hours long
//                         (not (counter-at-zero))))

//   (:action reset-counter
//            :parameters ()
//            :precondition (and)
//            :effect (counter-at-zero))


//   ;;; Get the food and snacks for the movie
//   (:action get-chips

//            :parameters (?x)
//            :precondition (chips ?x)
//            :effect (have-chips))
  
//   (:action get-dip
//            :parameters (?x)
//            :precondition (dip ?x)
//            :effect (have-dip))

//   (:action get-pop
//            :parameters (?x)
//            :precondition (pop ?x)
//            :effect (have-pop))
  
//   (:action get-cheese
//            :parameters (?x)
//            :precondition (cheese ?x)
//            :effect (have-cheese))
  
//   (:action get-crackers
//            :parameters (?x)
//            :precondition (crackers ?x)
//            :effect (have-crackers)))

use mini_sp_tools::*;

pub fn incremental_movie(stuff: &Vec<&str>) -> Vec<Transition> {

    let domain = vec!("t", "f");

    let t0 = Transition::new(
        "rewind_movie_2",
        &Predicate::EQRL(EnumVariable::new("counter_at_two_hours", "counter_at_two_hours", &domain, None), String::from("t")),
        &Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, None), String::from("t"))
    );

    let t1 = Transition::new(
        "rewind_movie",
        &Predicate::EQRL(EnumVariable::new("counter_at_other_than_two_hours", "counter_at_other_than_two_hours", &domain, None), String::from("t")),
        &Predicate::AND(
            vec!(
                Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, None), String::from("t")),
                Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, None), String::from("f"))
            )
        )
    );

    let t2 = Transition::new(
        "reset_counter",
        &Predicate::TRUE,
        &Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, None), String::from("t"))
    );

    let mut food_transitions = vec!();
    for f in vec!("chips", "dip", "pop", "cheese", "crackers") {
        for s in stuff {
            food_transitions.push(
                Transition::new(
                    &format!("get_{}_{}", f, s),
                    &Predicate::EQRL(EnumVariable::new(&format!("{}_{}", f, s), &format!("{}_{}", f, s), &domain, None), String::from("t")),
                    &Predicate::EQRL(EnumVariable::new(&format!("have_{}", f), &format!("have_{}", f), &domain, None), String::from("t"))
                )
            )
        }
    }

    let mut trans = vec!(t0, t1, t2);
    trans.extend(food_transitions);
    trans

}

pub fn compositional_movie(stuff: &Vec<&str>) -> Vec<ParamTransition> {

    let domain = vec!("t", "f");
    let movie_param = Parameter::new("m", &false);

    let t0 = ParamTransition::new(
        "rewind_movie_2",
        &ParamPredicate::new(
            &vec!(
                &Predicate::EQRL(EnumVariable::new("counter_at_two_hours", "counter_at_two_hours", &domain, Some(&movie_param)), String::from("t")),
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, Some(&movie_param)), String::from("t")),
            )
        )
    );

    let t1 = ParamTransition::new(
        "rewind_movie",
        &ParamPredicate::new(
            &vec!(
                &Predicate::EQRL(EnumVariable::new("counter_at_other_than_two_hours", "counter_at_other_than_two_hours", &domain, Some(&movie_param)), String::from("t")),
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &Predicate::AND(
                    vec!(
                        Predicate::EQRL(EnumVariable::new("movie_rewound", "movie_rewound", &domain, Some(&movie_param)), String::from("t")),
                        Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, Some(&movie_param)), String::from("f")),
                    )
                )
            )
        )
    );

    let t2 = ParamTransition::new(
        "rewind_movie_2",
        &ParamPredicate::new(
            &vec!(
                &Predicate::TRUE
            )
        ),
        &ParamPredicate::new(
            &vec!(
                &Predicate::EQRL(EnumVariable::new("counter_at_zero", "counter_at_zero", &domain, Some(&movie_param)), String::from("t")),
            )
        )
    );

    let mut food_transitions = vec!();
    for f in vec!("c", "d", "p", "s", "k") {
        for s in stuff {
            food_transitions.push(
                ParamTransition::new(
                    &format!("get_{}_{}", f, s),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::EQRL(EnumVariable::new(&format!("{}_{}", f, s), &format!("{}_{}", f, s), &domain, Some(&Parameter::new(&format!("{}", f), &false))), String::from("t")),
                        )
                    ),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::EQRL(EnumVariable::new(&format!("have_{}", f), &format!("have_{}", f), &domain, Some(&Parameter::new(&format!("{}", f), &false))), String::from("t")),
                        )
                    )
                )
            )
        }
    }

    let mut trans = vec!(t0, t1, t2);
    trans.extend(food_transitions);
    trans
}