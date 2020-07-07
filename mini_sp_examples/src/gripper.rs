// There is a robot with two grippers. It can carry a ball in each. 
// The goal is to take N balls from one room to another; N rises 
// with problem number. Some planners treat the two grippers 
// asymmetrically, giving rise to an unnecessary combinatorial 
// explosion. This domain was created by Jana Koehler.

// (define (domain gripper-strips)
// (:predicates (room ?r)
//      (ball ?b)
//      (gripper ?g)
//      (at-robby ?r)
//      (at ?b ?r)
//      (free ?g)
//      (carry ?o ?g))

// (:action move
//     :parameters  (?from ?to)
//     :precondition (and  (room ?from) (room ?to) (at-robby ?from))
//     :effect (and  (at-robby ?to)
//           (not (at-robby ?from))))



// (:action pick
//     :parameters (?obj ?room ?gripper)
//     :precondition  (and  (ball ?obj) (room ?room) (gripper ?gripper)
//              (at ?obj ?room) (at-robby ?room) (free ?gripper))
//     :effect (and (carry ?obj ?gripper)
//          (not (at ?obj ?room)) 
//          (not (free ?gripper))))


// (:action drop
//     :parameters  (?obj  ?room ?gripper)
//     :precondition  (and  (ball ?obj) (room ?room) (gripper ?gripper)
//              (carry ?obj ?gripper) (at-robby ?room))
//     :effect (and (at ?obj ?room)
//          (free ?gripper)
//          (not (carry ?obj ?gripper)))))

use mini_sp_tools::*;

pub fn incremental_grip(robots: &Vec<&str>, balls: &Vec<&str>, rooms: &Vec<&str>, grippers: &Vec<&str>) -> Vec<Transition> {

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let mut move_to_transitions = vec!();
    //for robot in vec!("r1"){
    //    for room in vec!("a", "b") {
    for robot in robots {
        for room in rooms {
            move_to_transitions.push(
                Transition::new(
                    &format!("move_{}_to_{}", robot, room),
                    &Predicate::AND(
                        vec!(
                            Predicate::NOT(
                                Box::new(Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room.to_owned()))
                                )
                            )
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room.to_owned()))
                        )
                    )
                )
            )
        }
    }

    let mut pick_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    pick_transitions.push(
                        Transition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("e")), 
                                    Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room.to_owned())),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room.to_owned()))
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("f")),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(gripper.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut drop_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    drop_transitions.push(
                        Transition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_from_gripper_{}", robot, ball, room, gripper),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("f")),
                                    Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room.to_owned())),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(gripper.to_owned()))
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("e")),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut trans = vec!();
    for t in vec!(move_to_transitions, pick_transitions, drop_transitions) {
        trans.extend(t)
    }
    trans
}

pub fn compositional_grip_g1(robots: &Vec<&str>, balls: &Vec<&str>, rooms: &Vec<&str>, grippers: &Vec<&str>) -> Vec<ParamTransition> {

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let ball_param = Parameter::new("b", &false);
    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let mut move_to_transitions = vec!();
    for robot in robots {
        for room in rooms {
            move_to_transitions.push(
                ParamTransition::new(
                    &format!("move_{}_to_{}", robot, room),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::NOT(
                                Box::new(Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned()))
                                )
                            )
                        )
                    ),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned()))
                        )
                    )
                )
            )
        }
    }

    let mut pick_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    pick_transitions.push(
                        ParamTransition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")), 
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned())),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(room.to_owned()))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room)))),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(gripper.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut drop_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    drop_transitions.push(
                        ParamTransition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_from_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned())),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(gripper.to_owned()))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")),
                                    // Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(room.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut trans = vec!();
    for t in vec!(move_to_transitions, pick_transitions, drop_transitions) {
        trans.extend(t)
    }
    trans
}

pub fn compositional_grip_g2(robots: &Vec<&str>, balls: &Vec<&str>, rooms: &Vec<&str>, grippers: &Vec<&str>) -> Vec<ParamTransition> {

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let mut move_to_transitions = vec!();
    for robot in robots {
        for room in rooms {
            move_to_transitions.push(
                ParamTransition::new(
                    &format!("move_{}_to_{}", robot, room),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::NOT(
                                Box::new(Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned()))
                                )
                            )
                        )
                    ),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned()))
                        )
                    )
                )
            )
        }
    }

    let mut pick_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    pick_transitions.push(
                        ParamTransition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")), 
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned())),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(room.to_owned()))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room)))),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(gripper.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut drop_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    drop_transitions.push(
                        ParamTransition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_from_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned())),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(gripper.to_owned()))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")),
                                    // Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(room.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut trans = vec!();
    for t in vec!(move_to_transitions, pick_transitions, drop_transitions) {
        trans.extend(t)
    }
    trans
}

pub fn compositional_grip_g3(robots: &Vec<&str>, balls: &Vec<&str>, rooms: &Vec<&str>, grippers: &Vec<&str>) -> Vec<ParamTransition> {

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let mut move_to_transitions = vec!();
    for robot in robots {
        for room in rooms {
            move_to_transitions.push(
                ParamTransition::new(
                    &format!("move_{}_to_{}", robot, room),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::NOT(
                                Box::new(Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned()))
                                )
                            )
                        )
                    ),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned()))
                        )
                    )
                )
            )
        }
    }

    let mut pick_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    pick_transitions.push(
                        ParamTransition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")), 
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned())),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(room.to_owned()))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room)))),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(gripper.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut drop_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for gripper in grippers {
                for ball in balls {
                    drop_transitions.push(
                        ParamTransition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_from_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room.to_owned())),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(gripper.to_owned()))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")),
                                    // Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&Parameter::new(&format!("b{}", ball), &false))), String::from(room.to_owned()))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut trans = vec!();
    for t in vec!(move_to_transitions, pick_transitions, drop_transitions) {
        trans.extend(t)
    }
    trans
}