use std::iter;

use num::{rational::Rational32, CheckedAdd, CheckedDiv, CheckedMul, Signed};
use text_io::read;

type Frac = Rational32;

#[derive(Clone)]
struct RecurseNum {
    value: Frac,
    repr: String,
}

type RecurseState = Vec<RecurseNum>;

const TARGET: i32 = 10;

fn op_add(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    x1.value.checked_add(&x2.value).map(|x| RecurseNum {
        value: x,
        repr: format!("({} + {})", x1.repr, x2.repr),
    })
}

fn op_mul(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    x1.value.checked_mul(&x2.value).map(|x| RecurseNum {
        value: x,
        repr: format!("({} * {})", x1.repr, x2.repr),
    })
}

fn op_div(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    x1.value.checked_div(&x2.value).map(|x| RecurseNum {
        value: x,
        repr: format!("({} / {})", x1.repr, x2.repr),
    })
}

fn op_pow(x1: &RecurseNum, x2: &RecurseNum) -> Option<RecurseNum> {
    if !x2.value.is_integer()
        || (x1.value != Frac::from(1)
            && x2.value != Frac::from(1)
            && (x1.value.abs() > Frac::from(10) || x2.value.abs() > Frac::from(5)))
    {
        return None;
    }

    let repr = format!("({} ** {})", x1.repr, x2.repr);

    if x2.value == Frac::from(0) {
        return Some(RecurseNum {
            value: Frac::from(1),
            repr: repr,
        });
    }
    if x1.value == Frac::from(0) {
        return Some(RecurseNum {
            value: Frac::from(0),
            repr: repr,
        });
    }

    return Some(RecurseNum {
        value: x1.value.pow(x2.value.to_integer()),
        repr: repr,
    });
}

const OPS: &'static [fn(&RecurseNum, &RecurseNum) -> Option<RecurseNum>] =
    &[op_add, op_mul, op_div, op_pow];

fn recurse_join_perms(data: &[i32]) -> Vec<Vec<i32>> {
    if data.len() > 1 {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let next = recurse_join_perms(&data[1..]);

        ret.extend(
            next.iter()
                .map(|d| iter::once(data[0]).chain(d.iter().map(|x| *x)).collect()),
        );
        ret.extend(next.iter().map(|d| {
            iter::once(i32::from_str_radix(&format!("{}{}", data[0], d[0]), 10).unwrap())
                .chain(d.iter().skip(1).map(|x| *x))
                .collect()
        }));

        ret
    } else {
        vec![Vec::from(data)]
    }
}

fn recurse_signs(data: &[i32]) -> Vec<Vec<i32>> {
    if data.len() > 1 {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let next = recurse_signs(&data[1..]);

        ret.extend(
            next.iter()
                .map(|d| iter::once(data[0]).chain(d.iter().map(|x| *x)).collect()),
        );
        ret.extend(
            next.iter()
                .map(|d| iter::once(-data[0]).chain(d.iter().map(|x| *x)).collect()),
        );

        ret
    } else if data.len() == 1 {
        vec![vec![data[0]], vec![-data[0]]]
    } else {
        Vec::new()
    }
}

fn gen_perms(data: &[i32]) -> Vec<RecurseState> {
    assert!(data.len() > 0);

    recurse_join_perms(data)
        .into_iter()
        .flat_map(|d| recurse_signs(&d))
        .map(|d| {
            d.iter()
                .map(|val| RecurseNum {
                    value: Frac::from(*val),
                    repr: format!("{}", *val),
                })
                .collect()
        })
        .collect()
}

fn solve(problem: &[i32]) -> (String, u32) {
    let mut nodes = gen_perms(&problem);

    let mut num_sols = 0u32;
    let mut sol = String::from("No solution");

    while let Some(n) = nodes.pop() {
        if n.len() == 1 {
            if n[0].value == Frac::from(TARGET) {
                // Solved!
                num_sols += 1;
                if num_sols == 1 {
                    sol = n[0].repr.clone();
                }
            }
        } else {
            for i in 0..=(n.len() - 2) {
                for op in OPS {
                    let x1 = &n[i];
                    let x2 = &n[i + 1];
                    if let Some(op_res) = op(x1, x2) {
                        // With negation
                        let mut new_node: RecurseState = Vec::new();
                        new_node.extend_from_slice(&n[0..i]);
                        new_node.push(RecurseNum {
                            value: -op_res.value,
                            repr: format!("-{}", op_res.repr),
                        });
                        new_node.extend_from_slice(&n[(i + 2)..]);
                        nodes.push(new_node);
                        // Original
                        let mut new_node: RecurseState = Vec::new();
                        new_node.extend_from_slice(&n[0..i]);
                        new_node.push(op_res.clone());
                        new_node.extend_from_slice(&n[(i + 2)..]);
                        nodes.push(new_node);
                    }
                }
            }
        }
    }

    return (sol, num_sols);
}

fn main() {
    println!("Enter 4 numbers seperated with spaces");

    let res = solve(&[read!(), read!(), read!(), read!()]);

    println!("{}\nNumber of solutions: {}", res.0, res.1);
}
