mod ops;
mod perms;
mod types;

use ops::OPS;
use perms::gen_perms;
use text_io::read;

use crate::types::{Frac, RecurseNum, RecurseState};

const TARGET: i32 = 10;

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
                        // With negation (powers only)
                        if let Some(negated) = std::panic::catch_unwind(|| {
                            let mut new_node: RecurseState = Vec::new();
                            new_node.extend_from_slice(&n[0..i]);
                            new_node.push(RecurseNum {
                                value: -op_res.value,
                                repr: format!("-{}", op_res.repr),
                            });
                            new_node.extend_from_slice(&n[(i + 2)..]);
                            new_node
                        })
                        .ok()
                        {
                            nodes.push(negated);
                        }
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
    std::panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    // for a in 0..10 {
    //     for b in 0..10 {
    //         for c in 0..10 {
    //             for d in 0..10 {
    //                 let res = solve(&[a, b, c, d]);
    //                 println!("{}\t{} {} {} {}", res.1, a, b, c, d);
    //             }
    //         }
    //     }
    // }

    println!("Enter 4 numbers seperated with spaces");

    let res = solve(&[read!(), read!(), read!(), read!()]);

    println!("{}\nNumber of solutions: {}", res.0, res.1);
}
