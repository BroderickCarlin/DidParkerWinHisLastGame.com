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

pub fn puzzle1(input: &str) -> i32 {
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

pub fn puzzle2(input: &str) -> i32 {
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
