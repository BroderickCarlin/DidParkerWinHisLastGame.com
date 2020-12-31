use wasm_bindgen::prelude::*;
mod day1;
mod day2;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Day 1
#[wasm_bindgen]
pub fn day1puzzle1(input: &str) -> i32 {
    day1::puzzle1(input)
}

#[wasm_bindgen]
pub fn day1puzzle2(input: &str) -> i32 {
    day1::puzzle2(input)
}

// Day 2
#[wasm_bindgen]
pub fn day2puzzle1(input: &str) -> i32 {
    day2::puzzle1(input)
}

#[wasm_bindgen]
pub fn day2puzzle2(input: &str) -> i32 {
    day2::puzzle2(input)
}