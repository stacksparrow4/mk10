mod math;
mod ops;
mod perms;
mod solve;
mod types;

use solve::solve;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_problem(a: i32, b: i32, c: i32, d: i32) -> String {
    console_error_panic_hook::set_once();

    let v = vec![a, b, c, d];
    let (sol, _) = solve(&v, 10);

    sol
}
