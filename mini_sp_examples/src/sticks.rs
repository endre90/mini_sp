// Stick picking game:
// Two players, c and k, alternate to pick one, two or three sticks
// from an initial pool of n sticks. The player who picks the last stick wins. Player
// k makes the first turn. To demonstrate different types of requirements that can be
// included in the SCT framework, this advanced variant of the game comes with
// three additional rules:
// 1. A player cannot pick one stick from the pool if the other player just picked
// one stick.
// 2. Player c cannot end a turn such that there are six sticks in the pool.
// 3. Player k can pick three sticks at most once.
// The challenge is to calculate a game plan for player c to always win, irre-
// spectively of how k plays. This is accomplished by synthesizing a supervisor
// that represents the game plan for c.

use mini_sp_tools::*;

pub fn incremental_sticks(sticks: &Vec<&str>) -> Vec<Transition> {

    let turn_domain = vec!("c", "k");
    let tf_domain = vec!("t", "f");

    let mut trans = vec!();
    for stick in sticks {
        if stick.to_string().parse::<i32>().unwrap() >= 3 {
            trans.push(
                Transition::new(
                    &format!("{}_sticks_c_takes_3", stick),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("c")),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("2")))),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("1")))),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("0")))),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from(stick.to_owned()))
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), (stick.to_string().parse::<i32>().unwrap() - 3).to_string()),
                            Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("6")))),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k"))
                        )
                    )
                )
            )
        }
    }

    for stick in sticks {
        if stick.to_string().parse::<i32>().unwrap() >= 2 {
            trans.push(
                Transition::new(
                    &format!("{}_sticks_c_takes_2", stick),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("c")),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("1")))),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("0")))),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from(stick.to_owned()))
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), (stick.to_string().parse::<i32>().unwrap() - 2).to_string()),
                            Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("6")))),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k"))
                        )
                    )
                )
            )
        }
    }

    for stick in sticks {
        if stick.to_string().parse::<i32>().unwrap() >= 1 {
            trans.push(
                Transition::new(
                    &format!("{}_sticks_c_takes_1", stick),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("c")),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("0")))),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from(stick.to_owned()))
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("t")),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), (stick.to_string().parse::<i32>().unwrap() - 1).to_string()),
                            Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("6")))),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k"))
                        )
                    )
                )
            )
        }
    }

    for stick in sticks {
        if stick.to_string().parse::<i32>().unwrap() >= 3 {
            trans.push(
                Transition::new(
                    &format!("{}_sticks_k_takes_3", stick),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("k_took_3", "k_took_3", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k")),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("2")))),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("1")))),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("0")))),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from(stick.to_owned()))
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("k_took_3", "k_took_3", &tf_domain, None), String::from("t")),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), (stick.to_string().parse::<i32>().unwrap() - 3).to_string()),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("c"))
                        )
                    )
                )
            )
        }
    }

    for stick in sticks {
        if stick.to_string().parse::<i32>().unwrap() >= 2 {
            trans.push(
                Transition::new(
                    &format!("{}_sticks_k_takes_2", stick),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k")),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("1")))),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("0")))),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from(stick.to_owned()))
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), (stick.to_string().parse::<i32>().unwrap() - 2).to_string()),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("c"))
                        )
                    )
                )
            )
        }
    }   

    for stick in sticks {
        if stick.to_string().parse::<i32>().unwrap() >= 1 {
            trans.push(
                Transition::new(
                    &format!("{}_sticks_k_takes_1", stick),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("f")),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("k")),
                            // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from("0")))),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), String::from(stick.to_owned()))
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new("picked_1", "picked_1", &tf_domain, None), String::from("t")),
                            Predicate::EQRL(EnumVariable::new("sticks", "sticks", sticks, None), (stick.to_string().parse::<i32>().unwrap() - 1).to_string()),
                            Predicate::EQRL(EnumVariable::new("turn", "turn", &turn_domain, None), String::from("c"))
                        )
                    )
                )
            )
        }
    }

    trans
}

