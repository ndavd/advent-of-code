mod handheld_device {
    #[derive(Debug)]
    pub enum CpuInstruction {
        Noop,
        AddX(isize),
    }

    impl std::fmt::Display for CpuInstruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let name = match self {
                CpuInstruction::Noop => "NOOP".to_string(),
                CpuInstruction::AddX(x) => format!("ADD_X {}", x),
            };
            write!(f, "{}", name)
        }
    }

    #[derive(Debug)]
    pub struct ProgramInstructions(Vec<CpuInstruction>);

    impl std::fmt::Display for ProgramInstructions {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let instructions = &self.0;
            for i in 0..instructions.len() {
                write!(f, "{}", instructions[i]).unwrap();
                if i != instructions.len() - 1 {
                    writeln!(f).unwrap();
                }
            }
            Ok(())
        }
    }

    impl ProgramInstructions {
        pub fn new(input: &Vec<String>) -> Result<Self, &str> {
            let bad_input_msg = "CpuInstruction::new::Bad input";
            let mut program: Vec<CpuInstruction> = Vec::new();

            for line in input {
                let parsed_line = line.split(' ').collect::<Vec<_>>();

                if parsed_line[0] == "noop" {
                    program.push(CpuInstruction::Noop);
                    continue;
                }

                if parsed_line[0] == "addx" {
                    let value = match parsed_line[1].parse::<isize>() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(bad_input_msg);
                        }
                    };
                    program.push(CpuInstruction::AddX(value));
                    continue;
                }

                return Err(bad_input_msg);
            }

            Ok(Self(program))
        }
    }

    #[derive(Debug)]
    pub struct CpuRegisterRecord {
        x: isize,
    }

    impl std::fmt::Display for CpuRegisterRecord {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "X: {}", self.x)
        }
    }

    impl CpuRegisterRecord {
        pub fn new(x: isize) -> Self {
            Self { x }
        }
    }

    #[derive(Debug)]
    pub struct Program(Vec<CpuRegisterRecord>);

    impl std::fmt::Display for Program {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let program = &self.0;
            for i in 0..program.len() {
                write!(f, "{}: {}", i, program[i]).unwrap();
                if i != program.len() {
                    writeln!(f).unwrap();
                }
            }
            Ok(())
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct CRTScreenOutput(pub Vec<Vec<char>>);

    pub const LOWER_HALF_BLOCK: &str = "▃";
    pub const DARK_BLOCK: &str = " ";
    pub const LIGHT_BLOCK: &str = "▓";
    pub const BLOCK: &str = "░";

    impl std::fmt::Display for CRTScreenOutput {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let screen = &self.0;
            let screen_height = screen.len();
            let monitor_width = screen[0].len();

            writeln!(f, "\n{}", LOWER_HALF_BLOCK.repeat(monitor_width + 2)).unwrap();
            for i in 0..screen_height {
                writeln!(
                    f,
                    "{1}{0}{1}",
                    screen[i].iter().cloned().collect::<String>(),
                    BLOCK
                )
                .unwrap();
            }
            writeln!(f, "{}", BLOCK.repeat(monitor_width + 2)).unwrap();
            let model = " Communication device 9000 ";
            write!(
                f,
                "{2}{3}{3}{0}{1}{2}",
                model,
                LOWER_HALF_BLOCK
                    .repeat(monitor_width - model.len() - 2),
                BLOCK,
                LOWER_HALF_BLOCK
            )
        }
    }

    impl Program {
        pub fn new(instructions: &ProgramInstructions) -> Self {
            let initial_x = 1;
            let mut program: Vec<CpuRegisterRecord> = vec![CpuRegisterRecord::new(initial_x)];
            let instr = &instructions.0;
            let mut curr_x = program[0].x;
            for i in 0..instr.len() {
                match instr[i] {
                    CpuInstruction::Noop => {
                        program.push(CpuRegisterRecord::new(curr_x));
                    }
                    CpuInstruction::AddX(v) => {
                        program.push(CpuRegisterRecord::new(curr_x));
                        program.push(CpuRegisterRecord::new(curr_x));
                        curr_x += v;
                    }
                }
            }
            Self(program)
        }

        pub fn signal_strength(&self, cycle: usize) -> Result<isize, &str> {
            if cycle > self.0.len() - 1 {
                return Err("Program::signal_strength::Program does not run on that cycle");
            }
            Ok(cycle as isize * self.0[cycle].x)
        }

        pub fn execute(&self) -> CRTScreenOutput {
            let mut screen: Vec<Vec<char>> = Vec::new();
            let mut screen_line: Vec<char> = Vec::new();
            let program = &self.0;
            for cycle in 1..program.len() {
                let sprite_start = program[cycle].x - 1;
                let sprite = sprite_start..=sprite_start + 2;
                let pixel_pos = (cycle - 1) % 40;
                let draw_pixel = sprite.contains(&(pixel_pos as isize));

                screen_line.push(if draw_pixel {
                    LIGHT_BLOCK.chars().next().unwrap()
                } else {
                    DARK_BLOCK.chars().next().unwrap()
                });

                if screen_line.len() == 40 {
                    screen.push(screen_line.clone());
                    screen_line = Vec::new();
                }
            }

            CRTScreenOutput(screen)
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<isize, handheld_device::CRTScreenOutput> {
    let instructions = handheld_device::ProgramInstructions::new(&input).unwrap();
    let program = handheld_device::Program::new(&instructions);

    let strength = (20..=220)
        .step_by(40)
        .collect::<Vec<usize>>()
        .iter()
        .map(|cycle| program.signal_strength(*cycle).unwrap())
        .sum();

    let crt_out = program.execute();

    aoc::Answer(strength, crt_out)
}

fn main() -> Result<(), ()> {
    let raw_crt_out: Vec<String> = vec![
        "##..##..##..##..##..##..##..##..##..##..",
        "###...###...###...###...###...###...###.",
        "####....####....####....####....####....",
        "#####.....#####.....#####.....#####.....",
        "######......######......######......####",
        "#######.......#######.......#######.....",
    ]
    .iter()
    .map(|s| {
        s.replace("#", handheld_device::LIGHT_BLOCK)
            .replace(".", handheld_device::DARK_BLOCK)
    })
    .collect();

    let crt_out = handheld_device::CRTScreenOutput(
        raw_crt_out
            .iter()
            .map(|e| e.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    aoc::AoC::new(10, 13140, crt_out).compute(&get_answer)
}
