use crate::ops::{perform_op, OPS};
use crate::perms::gen_perms;

use crate::types::Frac;

pub fn solve(problem: &[i32], target: i32) -> (String, u32) {
    let mut nodes = gen_perms(&problem);

    let mut num_sols = 0u32;
    let mut sol = String::from("No solution");

    while let Some(n) = nodes.pop() {
        if n.len() == 1 {
            if n[0].value == Frac::from(target) {
                // Solved!
                num_sols += 1;
                if num_sols == 1 {
                    sol = format!("Possible solution: {}", n[0].repr);
                }
            }
        } else {
            for i in 0..=(n.len() - 2) {
                for op in OPS {
                    let new_res = perform_op(op, &n, i);

                    nodes.extend(new_res.into_iter());
                }
            }
        }
    }

    return (sol, num_sols);
}
