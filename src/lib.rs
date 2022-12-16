mod math;
mod ops;
mod perms;
mod solve;
mod types;

use solve::solve;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_problem(prob: &[i32]) -> String {
    console_error_panic_hook::set_once();

    let (sol, _) = solve(prob, 10);

    sol
}
