mod utils;

use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;
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
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn addList(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut retVal = Vec::new();
    for (i, a1) in a.iter().enumerate() {
        retVal.push(a[i] + b[i]);
    }
    return retVal;
}

#[wasm_bindgen]
pub fn convert(from: &str, to: &str, value: f64) -> f64 {
    let units: HashMap<&str, f64> =
        HashMap::<_, _>::from_iter(IntoIter::new([("mm", 1.0 / 1000.0), ("cm", 1.0 / 100.0)]));

    let from = units[from];
    let to = units[to];
    return value * from / to;
}
