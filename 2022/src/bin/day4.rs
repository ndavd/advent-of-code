mod elf_pair {
    use std::ops::RangeInclusive;

    #[derive(Debug)]
    pub struct ElfPair(pub RangeInclusive<i32>, pub RangeInclusive<i32>);

    impl ElfPair {
        pub fn new(input: &String) -> Result<Self, &str> {
            let elves: Vec<String> = input.split(',').map(|a| a.to_string()).collect();
            if elves.len() != 2 {
                return Err("ElfPair::new::There must be 2 elves in each input");
            }

            Ok(ElfPair(
                Self::get_range(&elves[0]).unwrap(),
                Self::get_range(&elves[1]).unwrap(),
            ))
        }

        pub fn fully_contains(&self) -> bool {
            self.1.end() <= self.0.end() && self.1.start() >= self.0.start()
                || self.0.end() <= self.1.end() && self.1.start() <= self.0.start()
        }

        pub fn partially_contains(&self) -> bool {
            self.1.end() >= self.0.start() && self.0.end() >= self.1.start()
        }

        fn get_range(s: &String) -> Result<RangeInclusive<i32>, &str> {
            let elf_range: Vec<String> = s.split('-').map(|a| a.to_string()).collect();

            let wrong_input_msg = "ElfPair::get_range::Wrongly formatted input";
            if elf_range.len() != 2 {
                return Err(wrong_input_msg);
            }

            let range_val_1 = match elf_range[0].parse::<i32>() {
                Ok(val) => val,
                Err(_) => return Err(wrong_input_msg),
            };
            let range_val_2 = match elf_range[1].parse::<i32>() {
                Ok(val) => val,
                Err(_) => return Err(wrong_input_msg),
            };

            if range_val_2 < range_val_1 {
                return Err(wrong_input_msg);
            }

            Ok(range_val_1..=range_val_2)
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize, usize> {
    let assignments: Vec<elf_pair::ElfPair> = input
        .iter()
        .map(|a| elf_pair::ElfPair::new(a).unwrap())
        .collect();
    let assignments_iter = assignments.iter();

    let fully_overlapping = assignments_iter
        .clone()
        .filter(|x| x.fully_contains())
        .collect::<Vec<_>>()
        .len();

    let partially_overlapping = assignments_iter
        .filter(|x| x.partially_contains())
        .collect::<Vec<_>>()
        .len();

    aoc::Answer(fully_overlapping, partially_overlapping)
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(4, 2, 4).compute(&get_answer)
}
