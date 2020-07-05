use mini_sp_tools::*;

pub fn incremental() -> Vec<Transition> {

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let mut move_to_transitions = vec!();
    for robot in vec!("r1"){
        for room in vec!("a", "b") {
            move_to_transitions.push(
                Transition::new(
                    &format!("move_{}_to_{}", robot, room),
                    &Predicate::AND(
                        vec!(
                            Predicate::NOT(
                                Box::new(Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room))
                                )
                            )
                        )
                    ),
                    &Predicate::AND(
                        vec!(
                            Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room))
                        )
                    )
                )
            )
        }
    }

    let mut pick_transitions = vec!();
    for robot in vec!("r1") {
        for room in vec!("a", "b") {
            for gripper in vec!("gl", "gr") {
                for ball in vec!("1", "2", "3", "4") {
                    pick_transitions.push(
                        Transition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("e")), 
                                    Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room))
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("f")),
                                    // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room)))),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(gripper))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut drop_transitions = vec!();
    for robot in vec!("r1") {
        for room in vec!("a", "b") {
            for gripper in vec!("gl", "gr") {
                for ball in vec!("1", "2", "3", "4") {
                    drop_transitions.push(
                        Transition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_from_gripper_{}", robot, ball, room, gripper),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("f")),
                                    Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(gripper))
                                )
                            ),
                            &Predicate::AND(
                                vec!(
                                    Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, None), String::from("e")),
                                    // Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room))
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

pub fn compositional() -> Vec<ParamTransition> {

    let ball_pos_domain = vec!("a", "b", "gl", "gr");
    let robot_pos_domain = vec!("a", "b");
    let gripper_domain = vec!("e", "f");

    let ball_param = Parameter::new("b", &false);
    let robot_param = Parameter::new("r", &false);
    let gripper_param = Parameter::new("g", &false);

    let mut move_to_transitions = vec!();
    for robot in vec!("r1"){
        for room in vec!("a", "b") {
            move_to_transitions.push(
                ParamTransition::new(
                    &format!("move_{}_to_{}", robot, room),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::NOT(
                                Box::new(Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room))
                                )
                            )
                        )
                    ),
                    &ParamPredicate::new(
                        &vec!(
                            &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room))
                        )
                    )
                )
            )
        }
    }

    let mut pick_transitions = vec!();
    for robot in vec!("r1") {
        for room in vec!("a", "b") {
            for gripper in vec!("gl", "gr") {
                for ball in vec!("1", "2", "3", "4") {
                    pick_transitions.push(
                        ParamTransition::new(
                            &format!("{}_pick_ball_{}_in_room_{}_with_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")), 
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room)),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(room))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    // Predicate::NOT(Box::new(Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, None), String::from(room)))),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(gripper))
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    let mut drop_transitions = vec!();
    for robot in vec!("r1") {
        for room in vec!("a", "b") {
            for gripper in vec!("gl", "gr") {
                for ball in vec!("1", "2", "3", "4") {
                    drop_transitions.push(
                        ParamTransition::new(
                            &format!("{}_drop_ball_{}_in_room_{}_from_gripper_{}", robot, ball, room, gripper),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("f")),
                                    &Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, Some(&robot_param)), String::from(room)),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(gripper))
                                )
                            ),
                            &ParamPredicate::new(
                                &vec!(
                                    &Predicate::EQRL(EnumVariable::new(gripper, gripper, &gripper_domain, Some(&gripper_param)), String::from("e")),
                                    // Predicate::EQRL(EnumVariable::new(robot, robot, &robot_pos_domain, None), String::from(room)),
                                    &Predicate::EQRL(EnumVariable::new(&format!("b{}", ball), &format!("b{}", ball), &ball_pos_domain, Some(&ball_param)), String::from(room))
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