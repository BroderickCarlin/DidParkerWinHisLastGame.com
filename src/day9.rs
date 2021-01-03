fn find_invalid(input: &[i64]) -> i64 {
    'outer: for i in 0..(input.len() - 26) {
        let preamble = &input[i..(i+25)];
        let number = input[i+25];


        for i in 0..(preamble.len() - 1) {
            for j in (i + 1)..preamble.len() {
                if preamble[i] + preamble[j] == number {
                    continue 'outer;
                }
            }
        }

        return number;
        
    }
    0
}


pub fn puzzle1(input: &str) -> i64 {
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    find_invalid(&nums)
}

pub fn puzzle2(input: &str) -> i64 {
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let invalid = find_invalid(&nums);

    let mut min_idx = 0;
    let mut max_idx = 2; 

    loop {
        let sum: i64 = nums[min_idx..max_idx].iter().sum();
        if sum == invalid {
            return nums[min_idx..max_idx].iter().min().unwrap() + nums[min_idx..max_idx].iter().max().unwrap();
        } else if sum > invalid {
            min_idx += 1;
        } else {
            max_idx += 1;
        }
    }
}
