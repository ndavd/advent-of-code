use std::fmt::{Debug, Display};

pub type Input = Vec<String>;
#[derive(Debug, PartialEq)]
pub struct Answer<T: Display, B: Display>(pub T, pub B);

#[derive(Debug)]
pub struct AoC<T: Display, B: Display> {
    pub day: u32,
    pub test_answer: Answer<T, B>,
}

impl<T: Display + Debug + PartialEq, B: Display + Debug + PartialEq> Display for Answer<T, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n    Part 1: {}\n    Part 2: {}\n}}", self.0, self.1)
    }
}

impl<T: Display + Debug + PartialEq, B: Display + Debug + PartialEq> AoC<T, B> {
    pub fn new(day: u32, test_answer_1: T, test_answer_2: B) -> Self {
        Self {
            day,
            test_answer: Answer(test_answer_1, test_answer_2),
        }
    }

    pub fn test_compute(&self, get_answer: &dyn Fn(Input) -> Answer<T, B>) -> Result<(), ()> {
        let test_input = self.read_input(true)?;
        assert_eq!(get_answer(test_input), self.test_answer, "AoC::Test computation output doesn't match test answer input. You haven't got it yet 😉");

        Ok(())
    }

    pub fn compute(&self, get_answer: &dyn Fn(Input) -> Answer<T, B>) -> Result<(), ()> {
        self.test_compute(get_answer).unwrap();

        let input = self.read_input(false)?;
        self.print_answer(get_answer(input));

        Ok(())
    }

    fn read_input(&self, is_test_data: bool) -> Result<Input, ()> {
        Ok(std::fs::read_to_string(format!(
            "./src/data/day{}.{}.txt",
            self.day,
            if is_test_data { "test" } else { "input" }
        ))
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect())
    }

    fn print_answer(&self, answer: Answer<T, B>) {
        println!("{}", "🎄".repeat(self.day.try_into().unwrap()));
        println!("Answer from day {}: {}", self.day, answer);
    }
}
