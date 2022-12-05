mod supply_stacks {
    #[derive(Debug)]
    pub struct DrawingComponents {
        pub stacks: Vec<String>,
        pub instructions: Vec<String>,
    }

    impl DrawingComponents {
        pub fn new(input: Vec<String>) -> Result<Self, &'static str> {
            let split_index = match input.iter().position(|a| a.as_str() == "") {
                Some(val) => val,
                None => return Err("DrawingComponents::new::Bad format"),
            };

            Ok(Self {
                stacks: input[..split_index].to_vec(),
                instructions: input[split_index + 1..].to_vec(),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct SupplyStacks(Vec<Vec<char>>);

    impl std::fmt::Display for SupplyStacks {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (i, char_vec) in self.0.iter().enumerate() {
                write!(f, "{}: ", i + 1).unwrap();

                for char in char_vec {
                    write!(f, "[{}] ", char).unwrap();
                }

                if i + 1 < self.0.len() {
                    writeln!(f, "").unwrap();
                }
            }

            Ok(())
        }
    }

    impl SupplyStacks {
        pub fn new(stacks_drawing: Vec<String>) -> Result<Self, &'static str> {
            let mut stacks_drawing_clone = stacks_drawing.clone();

            if stacks_drawing_clone.len() < 2 {
                return Err("SupplyStacks::new::You have no stacks");
            }

            let stack_count: usize = match stacks_drawing_clone
                .pop()
                .unwrap()
                .split("   ")
                .last()
                .unwrap()
                .trim()
                .parse()
            {
                Ok(val) => val,
                Err(_) => return Err("SupplyStacks::new::Could not get stack count from drawing"),
            };

            let mut stacks: Vec<Vec<char>> = Vec::new();

            for _ in 0..stack_count {
                stacks.push(Vec::new())
            }

            stacks_drawing_clone.iter().rev().for_each(|line| {
                for i in 0..stack_count {
                    let crate_label = line.chars().nth((i * 4) + 1).unwrap();
                    if crate_label != ' ' {
                        stacks[i].push(crate_label);
                    }
                }
            });

            Ok(Self(stacks))
        }

        pub fn operate_crane(
            &mut self,
            instructions: &Vec<String>,
            model: usize,
        ) -> Result<&Self, &str> {
            let bad_instruction_msg = "SupplyStacks::operate_crane::Bad instruction";

            for instruction in instructions {
                let instruction_items: Vec<&str> = instruction.split(" ").collect();

                let quantity_to_move = match instruction_items[1].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return Err(bad_instruction_msg),
                };
                let origin = match instruction_items[3].parse::<usize>() {
                    Ok(v) => v - 1,
                    Err(_) => return Err(bad_instruction_msg),
                };
                let destination: usize = match instruction_items[5].parse::<usize>() {
                    Ok(v) => v - 1,
                    Err(_) => return Err(bad_instruction_msg),
                };

                let stacks = &mut self.0;
                match model {
                    // Moves one crate
                    9000 => {
                        for _ in 0..quantity_to_move {
                            let moving_crate = match stacks[origin].pop() {
                                Some(v) => v,
                                None => return Err("SupplyStacks::operate_crane::[CrateMover 9000]The stack you're trying to move a crate from is empty"),
                            };
                            stacks[destination].push(moving_crate);
                        }
                    }
                    // Moves multiple crates at once
                    9001 => {
                        let origin_size = stacks[origin].len();
                        if origin_size < quantity_to_move {
                            return Err("SupplyStacks::operate_crane::[CrateMover 9001]The stack you're trying to move crates from doesn't have enough crates");
                        }
                        let mut moving_crates =
                            stacks[origin].split_off(origin_size - quantity_to_move);
                        stacks[destination].append(&mut moving_crates);
                    }
                    _ => {
                        return Err(
                            "SupplyStacks::operate_crane::We don't have that crane model available",
                        )
                    }
                }
            }

            Ok(self)
        }

        pub fn list_top_crates(&self) -> String {
            self.0
                .iter()
                .map(|stack| stack.last().unwrap())
                .into_iter()
                .collect()
        }

        #[allow(dead_code)]
        pub fn display(&self) -> &Self {
            println!("{}\n", self);
            self
        }
    }
}
pub fn get_answer(input: aoc::Input) -> aoc::Answer<String> {
    let drawing_contents = supply_stacks::DrawingComponents::new(input).unwrap();

    let mut supply_stacks = supply_stacks::SupplyStacks::new(drawing_contents.stacks).unwrap();

    aoc::Answer(
        supply_stacks
            .clone()
            .operate_crane(&drawing_contents.instructions, 9000)
            .unwrap()
            .list_top_crates(),
        supply_stacks
            .operate_crane(&drawing_contents.instructions, 9001)
            .unwrap()
            .list_top_crates(),
    )
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(5, "CMZ".to_string(), "MCD".to_string()).compute(&get_answer)
}
