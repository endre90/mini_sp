use z3_sys::*;
use mini_sp_smt::*;
use super::*;

pub struct GetPredicateVars {
    pub pred: Predicate,
    pub vars: Vec<EnumVariable>
}

pub struct GetParamPredicateVars {
    pub pred: ParamPredicate,
    pub vars: Vec<EnumVariable>
}

pub struct GetProblemVars {
    pub pred: PlanningProblem,
    pub vars: Vec<EnumVariable>
}

pub struct GetParamProblemVars {
    pub pred: ParamPlanningProblem,
    pub vars: Vec<EnumVariable>
}

pub trait IterOps<T, I>: IntoIterator<Item = T>
    where I: IntoIterator<Item = T>,
          T: PartialEq {
    fn intersect(self, other: I) -> Vec<T>;
    fn difference(self, other: I) -> Vec<T>;
}

impl<T, I> IterOps<T, I> for I
    where I: IntoIterator<Item = T>,
          T: PartialEq
{
    fn intersect(self, other: I) -> Vec<T> {
        let mut common = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                common.push(e1);
                v_other.remove(pos);
            }
        }

        common
    }

    fn difference(self, other: I) -> Vec<T> {
        let mut diff = Vec::new();
        let mut v_other: Vec<_> = other.into_iter().collect();

        for e1 in self.into_iter() {
            if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
                v_other.remove(pos);
            } else {
                diff.push(e1);
            }
        }

        diff.append(&mut v_other);
        diff
    }
}

impl GetPredicateVars {
    pub fn new(pred: &Predicate) -> Vec<EnumVariable> {
        let mut s = Vec::new();
        match pred {
            Predicate::TRUE => {},
            Predicate::FALSE => {},
            Predicate::AND(x) => s.extend(x.iter().flat_map(|p| GetPredicateVars::new(p))),
            Predicate::OR(x) => s.extend(x.iter().flat_map(|p| GetPredicateVars::new(p))),
            Predicate::NOT(x) => s.extend(GetPredicateVars::new(x)),
            Predicate::EQRL(x, _) => s.push(x.clone()),
            Predicate::EQRR(x, y) => {
                s.push(x.clone());
                s.push(y.clone());
            },
            Predicate::EQLR(_, x) => s.push(x.clone()),
            Predicate::EQPP(x, y) => {
                s.extend(GetPredicateVars::new(x));
                s.extend(GetPredicateVars::new(y));
            },
            Predicate::NEQRL(x, _) => s.push(x.clone()),
            Predicate::NEQRR(x, y) => {
                s.push(x.clone());
                s.push(y.clone());
            },
            Predicate::NEQLR(_, x) => s.push(x.clone()),
            Predicate::NEQPP(x, y) => {
                s.extend(GetPredicateVars::new(x));
                s.extend(GetPredicateVars::new(y));
            },
            Predicate::PBEQ(x, _) => s.extend(x.iter().flat_map(|p| GetPredicateVars::new(p))),
            Predicate::NEXT(x) => s.extend(GetPredicateVars::new(x)),
            Predicate::ALWAYS(x) => s.extend(GetPredicateVars::new(x)),
            Predicate::NEVER(x) => s.extend(GetPredicateVars::new(x)),
            Predicate::EVENTUALLY(x) => s.extend(GetPredicateVars::new(x)),
            Predicate::UNTIL(x, y) => {
                s.extend(GetPredicateVars::new(x));
                s.extend(GetPredicateVars::new(y));
            },
            Predicate::RELEASE(x, y) => {
                s.extend(GetPredicateVars::new(x));
                s.extend(GetPredicateVars::new(y));
            },
            Predicate::AFTER(x, y) => {
                s.extend(GetPredicateVars::new(x));
                s.extend(GetPredicateVars::new(y));
            },
            // Predicate::SAFTER(x, y, _) => {
            //     s.extend(GetPredicateVars::new(x));
            //     s.extend(GetPredicateVars::new(y));
            // },
            Predicate::TPBEQ(x, _) => s.extend(GetPredicateVars::new(x))
        }
        s.sort();
        s.dedup();
        s
    }
}

impl GetParamPredicateVars {
    pub fn new(ppred: &ParamPredicate) -> Vec<EnumVariable> {
        let mut s = Vec::new();
        for p in &ppred.preds {
            s.extend(GetPredicateVars::new(&p));
        }
        s.sort();
        s.dedup();
        s
    }
}

impl GetProblemVars {
    pub fn new(prob: &PlanningProblem) -> Vec<EnumVariable> {
        let mut s = Vec::new();
        for t in &prob.trans {
            s.extend(GetPredicateVars::new(&t.guard));
            s.extend(GetPredicateVars::new(&t.update));
        }
        s.sort();
        s.dedup();
        s
    }
}

impl GetParamProblemVars {
    pub fn new(prob: &ParamPlanningProblem) -> Vec<EnumVariable> {
        let mut s = Vec::new();
        for t in &prob.trans {
            s.extend(GetParamPredicateVars::new(&t.guard));
            s.extend(GetParamPredicateVars::new(&t.update));
        }
        s.sort();
        s.dedup();
        s
    }
}

// maybe write some more tests for this fn
#[test]
fn test_get_predicate_vars(){

    let x = EnumVariable::new("x", "letters", &vec!("a", "b", "c", "d"), None);
    let y = EnumVariable::new("y", "letters", &vec!("a", "b", "c", "d"), None);
    let z = EnumVariable::new("z", "letters", &vec!("a", "b", "c", "d"), None);

    let n = Predicate::AND(vec!(Predicate::EQRR(x, y.clone()), Predicate::EQRR(y, z)));

    println!("predicate: {:?}", n);
    let vars = GetPredicateVars::new(&n);
    for var in vars {
        println!("var: {:?}", var);
    }
}