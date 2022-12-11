use num::{CheckedAdd, CheckedDiv, CheckedMul, Zero};

use crate::types::{Frac, RecurseNum};

pub fn op_add(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    x1.value.checked_add(&x2.value).map(|x| RecurseNum {
        value: x,
        repr: format!("({} + {})", x1.repr, x2.repr),
    })
}

pub fn op_mul(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    x1.value.checked_mul(&x2.value).map(|x| RecurseNum {
        value: x,
        repr: format!("({} * {})", x1.repr, x2.repr),
    })
}

pub fn op_div(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    x1.value.checked_div(&x2.value).map(|x| RecurseNum {
        value: x,
        repr: format!("({} / {})", x1.repr, x2.repr),
    })
}

pub fn op_pow(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    if !x2.value.is_integer() {
        None
    } else {
        std::panic::catch_unwind(|| RecurseNum {
            value: if x2.value.is_zero() {
                Frac::from(1)
            } else if x1.value.is_zero() {
                Frac::from(0)
            } else {
                x1.value.pow(x2.value.to_integer())
            },
            repr: format!("({} ** {})", x1.repr, x2.repr),
        })
        .ok()
    }
}

pub const OPS: &'static [fn(&RecurseNum, &RecurseNum) -> Option<RecurseNum>] =
    &[op_add, op_mul, op_div, op_pow];
