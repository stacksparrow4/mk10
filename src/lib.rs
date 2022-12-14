mod math;
mod ops;
mod perms;
mod solve;
mod types;

use solve::solve;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve_problem(a: i32, b: i32, c: i32, d: i32) -> String {
    console_error_panic_hook::set_once();

    let v = vec![a, b, c, d];
    let (sol, _) = solve(&v, 10);

    sol
}
