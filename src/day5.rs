struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    fn from_input(input: &str) -> Self {
        let mut row = 0;
        let mut column = 0;
        let (row_code, column_code) = input.split_at(7);

        for char in row_code.chars() {
            row <<= 1;
            row |= if char == 'B' { 1 } else { 0 };
        }

        for char in column_code.chars() {
            column <<= 1;
            column |= if char == 'R' { 1 } else { 0 };
        }

        BoardingPass {row, column}
    }

    fn id(&self) -> i64 {
        (self.row as i64 * 8) + self.column as i64
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut highest = 0;
    for line in input.lines() {
        let pass = BoardingPass::from_input(line);
        if highest < pass.id() {
            highest = pass.id();
        }
    }
    highest
}

pub fn puzzle2(input: &str) -> i64 {
    let mut seats = [false; 128 * 8];

    for line in input.lines() {
        seats[BoardingPass::from_input(line).id() as usize] = true;
    }

    (seats.windows(3).position(|frame| frame[0] && !frame[1] && frame[2]).unwrap() + 1) as i64
}
