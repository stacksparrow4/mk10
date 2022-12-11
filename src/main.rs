mod ops;
mod panic;
mod perms;
mod types;

use ops::{perform_op, OPS};
use perms::gen_perms;
use text_io::read;

use crate::types::Frac;

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
                    let new_res = perform_op(op, &n, i);

                    nodes.extend(new_res.into_iter());
                }
            }
        }
    }

    return (sol, num_sols);
}

fn main() {
    //// Brute force every combination
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

    // Input mode
    println!("Enter 4 numbers seperated with spaces");

    let res = solve(&[read!(), read!(), read!(), read!()]);

    println!("{}\nNumber of solutions: {}", res.0, res.1);
}
