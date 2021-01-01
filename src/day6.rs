struct SurveyResults<'a> {
    results: Vec<&'a str>,
}

impl<'a> SurveyResults<'a> {
    fn from_input(input: &'a str) -> SurveyResults<'a> {
        let results = input.lines().collect();
        SurveyResults { results }
    }

    fn unique_answers(&self) -> i64 {
        let mut answers: u32 = 0;

        for &result in &self.results {
            for c in result.chars() {
                answers |= 0b1 << (c as u8 - 'a' as u8);
            }
        }

        answers.count_ones() as i64
    }

    fn common_answers(&self) -> i64 {
        let mut answers: u32 = 0xFFFFFFFF;

        for &result in &self.results {
            let mut answer_map: u32 = 0;
            for c in result.chars() {
                answer_map |= 0b1 << (c as u8 - 'a' as u8);
            }
            answers &= answer_map;
        }

        answers.count_ones() as i64
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut sum = 0;

    for result in input.split("\n\n") {
        sum += SurveyResults::from_input(result).unique_answers();
    }

    sum
}

pub fn puzzle2(input: &str) -> i64 {
    let mut sum = 0;

    for result in input.split("\n\n") {
        sum += SurveyResults::from_input(result).common_answers();
    }

    sum
}
