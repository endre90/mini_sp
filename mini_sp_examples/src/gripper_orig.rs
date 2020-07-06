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

pub fn incremental_grip_orig(robots: &Vec<&str>, balls: &Vec<&str>, rooms: &Vec<&str>, grippers: &Vec<&str>) -> Vec<Transition> {

    let tf_domain = vec!("t", "f");

    let mut move_to_transitions = vec!();
    for robot in robots {
        for room1 in rooms {
            for room2 in rooms {
                if room1 != room2 {
                    move_to_transitions.push(
                        Transition::new(
                            &format!("move_{}_from_{}_to_{}", robot, room1, room2),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room1), &format!("robot_{}_at_{}", robot, room1), &tf_domain, None), "t".to_string())
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room1), &format!("robot_{}_at_{}", robot, room1), &tf_domain, None), "f".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room2), &format!("robot_{}_at_{}", robot, room2), &tf_domain, None), "t".to_string())
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut pick_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for ball in balls {
                for gripper in grippers {
                    pick_transitions.push(
                        Transition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_{}", ball, room), &format!("ball_{}_at_{}", ball, room), &tf_domain, None), "t".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room), &format!("robot_{}_at_{}", robot, room), &tf_domain, None), "t".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("gripper_{}_free", gripper), &format!("gripper_{}_free", gripper), &tf_domain, None), "t".to_string())
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_{}", ball, room), &format!("ball_{}_at_{}", ball, room), &tf_domain, None), "f".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_carries_ball_{}", robot, ball), &format!("robot_{}_carries_ball_{}", robot, ball), &tf_domain, None), "t".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("gripper_{}_free", gripper), &format!("gripper_{}_free", gripper), &tf_domain, None), "f".to_string())
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
            for ball in balls {
                for gripper in grippers {
                    drop_transitions.push(
                        Transition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room), &format!("robot_{}_at_{}", robot, room), &tf_domain, None), "t".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_carries_ball_{}", robot, ball), &format!("robot_{}_carries_ball_{}", robot, ball), &tf_domain, None), "t".to_string()),
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_{}", ball, room), &format!("ball_{}_at_{}", ball, room), &tf_domain, None), "t".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("robot_{}_carries_ball_{}", robot, ball), &format!("robot_{}_carries_ball_{}", robot, ball), &tf_domain, None), "f".to_string()),
                                    Predicate::EQRL(EnumVariable::new(&format!("gripper_{}_free", gripper), &format!("gripper_{}_free", gripper), &tf_domain, None), "t".to_string())
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

pub fn compositional_grip_orig(robots: &Vec<&str>, balls: &Vec<&str>, rooms: &Vec<&str>, grippers: &Vec<&str>) -> Vec<ParamTransition> {

    let ball_param = Parameter::new("b", &false);
    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let tf_domain = vec!("t", "f");

    let mut move_to_transitions = vec!();
    for robot in robots {
        for room1 in rooms {
            for room2 in rooms {
                if room1 != room2 {
                    move_to_transitions.push(
                        ParamTransition::new(
                            &format!("move_{}_from_{}_to_{}", robot, room1, room2),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room1), &format!("robot_{}_at_{}", robot, room1), &tf_domain, Some(&robot_param)), "t".to_string())
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room1), &format!("robot_{}_at_{}", robot, room1), &tf_domain, Some(&robot_param)), "f".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room2), &format!("robot_{}_at_{}", robot, room2), &tf_domain, Some(&robot_param)), "t".to_string())
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut pick_transitions = vec!();
    for robot in robots {
        for room in rooms {
            for ball in balls {
                for gripper in grippers {
                    pick_transitions.push(
                        ParamTransition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_{}", ball, room), &format!("ball_{}_at_{}", ball, room), &tf_domain, Some(&ball_param)), "t".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room), &format!("robot_{}_at_{}", robot, room), &tf_domain, Some(&robot_param)), "t".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("gripper_{}_free", gripper), &format!("gripper_{}_free", gripper), &tf_domain, Some(&gripper_param)), "t".to_string())
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_{}", ball, room), &format!("ball_{}_at_{}", ball, room), &tf_domain, Some(&ball_param)), "f".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_carries_ball_{}", robot, ball), &format!("robot_{}_carries_ball_{}", robot, ball), &tf_domain, Some(&ball_param)), "t".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("gripper_{}_free", gripper), &format!("gripper_{}_free", gripper), &tf_domain, Some(&gripper_param)), "f".to_string())
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
            for ball in balls {
                for gripper in grippers {
                    drop_transitions.push(
                        ParamTransition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_at_{}", robot, room), &format!("robot_{}_at_{}", robot, room), &tf_domain, Some(&robot_param)), "t".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_carries_ball_{}", robot, ball), &format!("robot_{}_carries_ball_{}", robot, ball), &tf_domain, Some(&ball_param)), "t".to_string()),
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(&format!("ball_{}_at_{}", ball, room), &format!("ball_{}_at_{}", ball, room), &tf_domain, Some(&ball_param)), "t".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("robot_{}_carries_ball_{}", robot, ball), &format!("robot_{}_carries_ball_{}", robot, ball), &tf_domain, Some(&ball_param)), "f".to_string()),
                                    &Predicate::EQRL(EnumVariable::new(&format!("gripper_{}_free", gripper), &format!("gripper_{}_free", gripper), &tf_domain, Some(&gripper_param)), "t".to_string())
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