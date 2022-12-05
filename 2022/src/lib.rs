pub type Input = Vec<String>;
pub type Answer = (i32, i32);

#[derive(Debug)]
pub struct AoC {
    pub day: u32,
    pub test_answer: Answer,
}

impl AoC {
    pub fn new(day: u32, test_answer_1: i32, test_answer_2: i32) -> Self {
        Self {
            day,
            test_answer: (test_answer_1, test_answer_2),
        }
    }

    pub fn compute(&self, get_answer: &dyn Fn(Input) -> Answer) -> Result<(), ()> {
        let test_input = self.read_input(true)?;
        assert_eq!(get_answer(test_input), self.test_answer, "AoC::Test computation output doesn't match test answer input. You haven't got it yet 😉");

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

    fn print_answer(&self, answer: Answer) {
        println!("{}", "🎄".repeat(self.day.try_into().unwrap()));
        println!("Answer from day {}: {:?}", self.day, answer);
    }
}
