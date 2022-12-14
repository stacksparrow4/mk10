mod math;
mod ops;
mod perms;
mod solve;
mod types;

use solve::solve;

fn main() {
    for a in 0..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                for d in 0..=9 {
                    // println!("{}{}{}{}", a, b, c, d);
                    solve(&vec![a, b, c, d], 10);
                }
            }
            println!("{}%", a * 10 + b + 1);
        }
    }
}
