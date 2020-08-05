use std::time::{Instant};
use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct StateToPredicate {
    pub state: Vec<String>,
    pub prob: PlanningProblem,
    pub pred: Predicate
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct VerifySafety {
    pub prob: PlanningProblem,
    pub res: PlanningResult,
    pub safe: bool
}

impl StateToPredicate {
    pub fn new(state: &Vec<&str>, prob: &PlanningProblem) -> Predicate {
        let mut pred_vec: Vec<Predicate> = vec!();
        let prob_vars = GetProblemVars::new(prob);
        for s in state {
            let sep: Vec<&str> = s.split(" -> ").collect();
            let mut var: EnumVariable = EnumVariable::default();
            for v in &prob_vars {
                match v.name == sep[0] {
                    true => var = v.to_owned(),
                    false => ()
                };
            };
            // hackidy hack
            if !(var.name == "EMPTY") {
                pred_vec.push(Predicate::EQRL(var, String::from(sep[1])));
            };
        } 
        Predicate::AND(pred_vec)
    }
}

impl VerifySafety {
    pub fn new(prob: &PlanningProblem, res: &PlanningResult, forb: &Predicate) -> bool {
        let mut founds = vec!();
        for t in &res.trace {
            let new_prob = PlanningProblem::new(
                prob.name.as_str(), 
                &StateToPredicate::new(&t.state.iter().map(|x| x.as_str()).collect(), &prob), 
                forb, 
                &prob.trans, 
                &prob.ltl_specs, 
                &prob.max_steps);
            founds.push(Incremental::new(&new_prob).plan_found);
        }
        founds.iter().all(|x| *x)
    }
}

