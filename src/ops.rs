use num::{CheckedAdd, CheckedDiv, CheckedMul, Zero};

use crate::types::{Frac, RecurseNum, RecurseState};

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
            repr: format!("(({}) ** {})", x1.repr, x2.repr),
        })
        .ok()
    }
}

pub fn perform_op(op: &Op, state: &RecurseState, op_index: usize) -> Vec<RecurseState> {
    let mut ret: Vec<RecurseState> = Vec::new();

    let x1 = &state[op_index];
    let x2 = &state[op_index + 1];
    if let Some(op_res) = (op.op)(x1, x2) {
        // With negation (only for some operations)
        if op.needs_negate {
            if let Some(negated) = std::panic::catch_unwind(|| {
                let mut new_node: RecurseState = Vec::new();
                new_node.extend_from_slice(&state[0..op_index]);
                new_node.push(RecurseNum {
                    value: -op_res.value,
                    repr: format!("-{}", op_res.repr),
                });
                new_node.extend_from_slice(&state[(op_index + 2)..]);
                new_node
            })
            .ok()
            {
                ret.push(negated);
            }
        }
        // Original
        let mut new_node: RecurseState = Vec::new();
        new_node.extend_from_slice(&state[0..op_index]);
        new_node.push(op_res.clone());
        new_node.extend_from_slice(&state[(op_index + 2)..]);
        ret.push(new_node);
    }

    ret
}

pub struct Op {
    pub op: fn(&RecurseNum, &RecurseNum) -> Option<RecurseNum>,
    pub needs_negate: bool,
}

pub const OPS: &[Op] = &[
    Op {
        op: op_add,
        needs_negate: false,
    },
    Op {
        op: op_mul,
        needs_negate: false,
    },
    Op {
        op: op_div,
        needs_negate: false,
    },
    Op {
        op: op_pow,
        needs_negate: true,
    },
];
