struct PasswordEntry<'a> {
    min: usize,
    max: usize,
    char: char,
    password: &'a str,
}

fn parse_input(input: &str) -> Vec<PasswordEntry> {
    let mut output = vec![];
    for line in input.lines() {
        
        let line: Vec<&str> = line.split(' ').collect();
        let min_max:Vec<&str> = line[0].split('-').collect();
        let char = line[1].chars().next().unwrap();

        output.push(
            PasswordEntry {
                min: min_max[0].parse().unwrap(), 
                max: min_max[1].parse().unwrap(), 
                char: char, 
                password: line[2]
            });
    }
    output
}

pub fn puzzle1(input: &str) -> i32 {
    let input = parse_input(input);
    let mut count = 0;

    for password in input {
        let mut char_count = 0;
        for c in password.password.chars() {
            if c == password.char {
                char_count += 1;
            }
        }
        if char_count >= password.min && char_count <= password.max {
            count += 1;
        }
    }
    count
}

pub fn puzzle2(input: &str) -> i32 {
    let input = parse_input(input);
    let mut count = 0;

    for password in input {
        if (password.password.chars().nth(password.min - 1).unwrap() == password.char &&
           password.password.chars().nth(password.max - 1).unwrap() != password.char) ||
           (password.password.chars().nth(password.min - 1).unwrap() != password.char &&
            password.password.chars().nth(password.max - 1).unwrap() == password.char) {
            count += 1;
        } 
    }
    count
}
