use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn parse_input<T>(input: &str) -> Result<Vec<T>, String> 
where
    T: std::str::FromStr 
{
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().map_err(|_| format!("Failed to parse \"{:?}\" as an i32", line))?);
    }
    Ok(output)
}

#[wasm_bindgen]
pub fn day1puzzle1(input: &str) -> i32 {
    let input = parse_input::<i32>(input).unwrap();
    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    0
}

#[wasm_bindgen]
pub fn day1puzzle2(input: &str) -> i32 {
    let input = parse_input::<i32>(input).unwrap();
    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            for k in (j+1)..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}


// #[wasm_bindgen]
// pub fn day2puzzle1(input: &str) -> i32 {
//     input.len() as i32
// }

// #[wasm_bindgen]
// pub fn day2puzzle2(_input: &str) -> i32 {
//     1
// }