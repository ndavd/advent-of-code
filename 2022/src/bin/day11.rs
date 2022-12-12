mod mitm {
    use std::collections::VecDeque;

    #[derive(Debug, Clone)]
    pub enum Operation {
        Add(usize),
        Sub(usize),
        Mul(usize),
        Pow,
    }

    impl Operation {
        pub fn exec(&self, x: usize) -> usize {
            match self {
                Self::Add(v) => x + v,
                Self::Sub(v) => x - v,
                Self::Mul(v) => x * v,
                Self::Pow => x * x,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Monkey {
        pub holding_items: VecDeque<usize>,
        pub operation: Operation,
        pub test_threshold: usize,
        pub test_true: usize,
        pub test_false: usize,
    }

    #[derive(Debug, Clone)]
    pub struct Monkeys(pub Vec<Monkey>);
    impl std::fmt::Display for Monkeys {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let monkeys = &self.0;
            let monkeys_len = monkeys.len();
            for i in 0..monkeys_len {
                write!(f, "{:?}", monkeys[i]).unwrap();
                if i != monkeys_len - 1 {
                    writeln!(f).unwrap();
                }
            }
            Ok(())
        }
    }

    impl Monkeys {
        pub fn new(input: &Vec<String>) -> Result<Self, &str> {
            let err = Err("Monkeys::new::Invalid input");
            let mut monkeys: Vec<Monkey> = Vec::new();

            for i in (0..input.len()).step_by(7) {
                if input[i].split(" ").collect::<Vec<&str>>()[0] != "Monkey" {
                    return err;
                }

                let starting_items: Vec<&str> = input[i + 1].trim().split(": ").collect();
                if starting_items[0] != "Starting items" {
                    return err;
                }
                let holding_items = starting_items[1]
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<VecDeque<_>>();

                let operation: Vec<&str> = input[i + 2].trim().split(": ").collect();
                if operation[0] != "Operation" {
                    return err;
                }
                let operation_items: Vec<&str> = operation[1].split(" ").collect();
                let operation_items_len = operation_items.len();
                let operation: Operation;
                let operation_val = operation_items[operation_items_len - 1];
                if operation_val == "old" {
                    operation = Operation::Pow;
                } else {
                    let v: usize = operation_val.parse().unwrap();
                    operation = match operation_items[operation_items_len - 2] {
                        "+" => Operation::Add(v),
                        "-" => Operation::Sub(v),
                        "*" => Operation::Mul(v),
                        _ => return err,
                    }
                }

                let test: Vec<&str> = input[i + 3].trim().split(": ").collect();
                if test[0] != "Test" {
                    return err;
                }
                let test_items = test[1].split(" ").collect::<Vec<_>>();
                let test_threshold = test_items[test_items.len() - 1].parse::<usize>().unwrap();

                let test_if_true: Vec<&str> = input[i + 4].trim().split(": ").collect();
                if test_if_true[0] != "If true" {
                    return err;
                }
                let test_if_true_items = test_if_true[1].split(" ").collect::<Vec<_>>();
                let test_true = test_if_true_items[test_if_true_items.len() - 1]
                    .parse::<usize>()
                    .unwrap();

                let test_if_false: Vec<&str> = input[i + 5].trim().split(": ").collect();
                if test_if_false[0] != "If false" {
                    return err;
                }
                let test_if_false_items = test_if_false[1].split(" ").collect::<Vec<_>>();
                let test_false = test_if_false_items[test_if_false_items.len() - 1]
                    .parse::<usize>()
                    .unwrap();

                monkeys.push(Monkey {
                    holding_items,
                    operation,
                    test_threshold,
                    test_true,
                    test_false,
                })
            }

            Ok(Monkeys(monkeys))
        }

        pub fn inspect(&mut self, rounds: usize, relieved: bool) -> Vec<usize> {
            let monkeys = &mut self.0;
            // All the test_thresholds are relatively prime and the Least Common
            // Multiple of relatively prime numbers is just their product
            let lcm: usize = monkeys.iter().map(|m| m.test_threshold).product();
            let monkeys_len = monkeys.len();
            let mut monkeys_inspections_count = vec![0; monkeys_len];
            for _ in 0..rounds {
                for monkey_i in 0..monkeys_len {
                    let inspections_count = monkeys[monkey_i].holding_items.len();
                    for _ in 0..inspections_count {
                        let mut item_worry_level =
                            monkeys[monkey_i].holding_items.pop_front().unwrap();
                        item_worry_level = monkeys[monkey_i].operation.exec(item_worry_level);

                        // We need to keep the item_worry_level low or weird
                        // things start to happen when testing it against the
                        // test_threshold. To achieve that we can mod them
                        // against their LCM
                        item_worry_level %= lcm;

                        if !relieved {
                            item_worry_level /= 3;
                        }

                        let passed_test = item_worry_level % monkeys[monkey_i].test_threshold == 0;

                        let receiving_monkey_index = if passed_test {
                            monkeys[monkey_i].test_true
                        } else {
                            monkeys[monkey_i].test_false
                        };

                        monkeys[receiving_monkey_index]
                            .holding_items
                            .push_back(item_worry_level);
                    }
                    monkeys_inspections_count[monkey_i] += inspections_count;
                }
            }
            monkeys_inspections_count
        }

        pub fn monkey_business(&mut self, rounds: usize, relieved: bool) -> usize {
            let monkeys_len = self.0.len();
            let mut monkeys_inspections_count = self.inspect(rounds, relieved);
            monkeys_inspections_count.sort();
            monkeys_inspections_count[monkeys_len - 1] * monkeys_inspections_count[monkeys_len - 2]
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize, usize> {
    aoc::Answer(
        mitm::Monkeys::new(&input)
            .unwrap()
            .monkey_business(20, false),
        mitm::Monkeys::new(&input)
            .unwrap()
            .monkey_business(10000, true),
    )
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(11, 10605, 2713310158).compute(&get_answer)
}
