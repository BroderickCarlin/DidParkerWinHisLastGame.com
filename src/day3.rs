struct Map<'a> {
    slice: Vec<&'a str>
}

impl<'a> Map<'a> {
    fn from_input(input: &'a str) -> Map<'a> {
        Map {slice: input.lines().collect()}
    }

    fn slide(&self, x: usize, y: usize) -> i64 {
        let mut count = 0;
        let mut position = (0, 0);

        loop {

            if self.slice[position.1].chars().nth(position.0) == Some('#') {
                count += 1;
            }

            position.0 = (position.0 + x) % self.slice[0].len();
            position.1 += y;

            if position.1 >= self.slice.len() {
                return count;
            }
        }
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let map = Map::from_input(input);
    map.slide(3, 1)
}

pub fn puzzle2(input: &str) -> i64 {
    let map = Map::from_input(input);
    let slope1 = map.slide(1, 1);
    let slope2 = map.slide(3, 1);
    let slope3 = map.slide(5, 1);
    let slope4 = map.slide(7, 1);
    let slope5 = map.slide(1, 2);

    slope1 * slope2 * slope3 * slope4 * slope5
}
