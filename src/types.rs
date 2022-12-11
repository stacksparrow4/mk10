use num::rational::Rational32;

pub type Frac = Rational32;

#[derive(Clone)]
pub struct RecurseNum {
    pub value: Frac,
    pub repr: String,
}

pub type RecurseState = Vec<RecurseNum>;
