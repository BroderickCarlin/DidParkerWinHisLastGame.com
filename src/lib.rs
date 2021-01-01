use wasm_bindgen::prelude::*;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

// Day 3
#[wasm_bindgen]
pub fn day3puzzle1(input: &str) -> i64 {
    day3::puzzle1(input)
}

#[wasm_bindgen]
pub fn day3puzzle2(input: &str) -> i64 {
    day3::puzzle2(input)
}

// Day 4
#[wasm_bindgen]
pub fn day4puzzle1(input: &str) -> i64 {
    day4::puzzle1(input)
}

#[wasm_bindgen]
pub fn day4puzzle2(input: &str) -> i64 {
    day4::puzzle2(input)
}

// Day 5
#[wasm_bindgen]
pub fn day5puzzle1(input: &str) -> i64 {
    day5::puzzle1(input)
}

#[wasm_bindgen]
pub fn day5puzzle2(input: &str) -> i64 {
    day5::puzzle2(input)
}

// Day 6
#[wasm_bindgen]
pub fn day6puzzle1(input: &str) -> i64 {
    day6::puzzle1(input)
}

#[wasm_bindgen]
pub fn day6puzzle2(input: &str) -> i64 {
    day6::puzzle2(input)
}