//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(ajrust::add(1, 1), 2);
}

#[wasm_bindgen_test]
fn simple() {
    assert_eq!(ajrust::convert("mm", "cm", 1000.0), 100.0);
}

#[wasm_bindgen_test]
fn arrayAdd() {
    assert_eq!(
        ajrust::addList(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
        vec![2, 4, 6, 8, 10]
    );
}
