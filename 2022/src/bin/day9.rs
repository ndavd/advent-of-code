use std::collections::HashSet;

mod rope_bridge {
    #[derive(Debug)]
    /* Assuming the following coordinate system:
     * +y
     *  ^
     *  |
     *  O--> +x
     */
    pub struct RopePosition {
        pub head_x: isize,
        pub head_y: isize,
        pub tail_x: isize,
        pub tail_y: isize,
    }

    impl RopePosition {
        pub fn new(head_x: isize, head_y: isize, tail_x: isize, tail_y: isize) -> Self {
            Self {
                head_x,
                head_y,
                tail_x,
                tail_y,
            }
        }
    }

    #[derive(Debug)]
    pub struct RopePath(Vec<RopePosition>);

    impl std::fmt::Display for RopePath {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let path = &self.0;
            let path_len = path.len();
            for i in 0..path_len {
                let pos = &path[i];
                write!(
                    f,
                    "T{{ {} {} }} H{{ {} {} }}",
                    pos.tail_x, pos.tail_y, pos.head_x, pos.head_y
                )
                .unwrap();
                if i != path_len - 1 {
                    writeln!(f).unwrap();
                }
            }
            Ok(())
        }
    }

    impl RopePath {
        pub fn visited_tail_positions(&self) -> Vec<(isize, isize)> {
            self.0.iter().map(|pos| (pos.tail_x, pos.tail_y)).collect()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum KnotStep {
        Up,
        Down,
        Left,
        Right,
        UpLeft,
        UpRight,
        DownLeft,
        DownRight,
    }

    #[derive(Debug)]
    // A Rope is defined by a sequence of head steps
    pub struct Rope(Vec<KnotStep>);

    impl Rope {
        pub fn new(input: &Vec<String>) -> Result<Self, &str> {
            let mut rope: Vec<KnotStep> = Vec::new();
            let bad_input_msg = "Rope::new::Bad input";

            for line in input {
                let parsed_line = line.split(' ').collect::<Vec<_>>();
                let step: KnotStep = match parsed_line[0] {
                    "U" => KnotStep::Up,
                    "D" => KnotStep::Down,
                    "L" => KnotStep::Left,
                    "R" => KnotStep::Right,
                    _ => return Err(bad_input_msg),
                };
                let count = match parsed_line[1].parse::<isize>() {
                    Ok(v) => v,
                    Err(_) => return Err(bad_input_msg),
                };

                for _ in 0..count {
                    rope.push(step);
                }
            }

            Ok(Rope(rope))
        }

        fn get_new_pos(pos: (isize, isize), step: &KnotStep) -> (isize, isize) {
            let (x, y) = pos;
            match step {
                KnotStep::Up => (x, y + 1),
                KnotStep::Down => (x, y - 1),
                KnotStep::Left => (x - 1, y),
                KnotStep::Right => (x + 1, y),
                KnotStep::UpLeft => (x - 1, y + 1),
                KnotStep::UpRight => (x + 1, y + 1),
                KnotStep::DownLeft => (x - 1, y - 1),
                KnotStep::DownRight => (x + 1, y - 1),
            }
        }

        fn get_tail_pos(
            previous_pos: &RopePosition,
            head_x: isize,
            head_y: isize,
        ) -> (isize, isize) {
            let x = previous_pos.tail_x;
            let y = previous_pos.tail_y;

            let distance_x = head_x - x;
            let distance_y = head_y - y;

            // Should move tail
            if distance_x.abs() > 1 || distance_y.abs() > 1 {
                let pos = (x, y);
                return match (distance_x, distance_y) {
                    (0, 2) => Self::get_new_pos(pos, &KnotStep::Up),
                    (0, -2) => Self::get_new_pos(pos, &KnotStep::Down),
                    (-2, 0) => Self::get_new_pos(pos, &KnotStep::Left),
                    (2, 0) => Self::get_new_pos(pos, &KnotStep::Right),
                    (-1, 2) | (-2, 1) => Self::get_new_pos(pos, &KnotStep::UpLeft),
                    (1, 2) | (2, 1) => Self::get_new_pos(pos, &KnotStep::UpRight),
                    (-1, -2) | (-2, -1) => Self::get_new_pos(pos, &KnotStep::DownLeft),
                    (1, -2) | (2, -1) => Self::get_new_pos(pos, &KnotStep::DownRight),
                    _ => {
                        println!("{:?}", (distance_x, distance_y));
                        panic!("Rope::get_tail_pos::Something has gone extremely wrong!")
                    }
                };
            }

            (x, y)
        }

        pub fn get_path(&self, origin: Option<isize>) -> Result<RopePath, &str> {
            let o = origin.unwrap_or(0);
            let steps = &self.0;
            let mut path = vec![RopePosition::new(o, o, o, o)];

            for i in 0..steps.len() {
                let previous_pos = &path[i];

                let (head_x, head_y) =
                    Self::get_new_pos((previous_pos.head_x, previous_pos.head_y), &steps[i]);

                let (tail_x, tail_y) = Self::get_tail_pos(previous_pos, head_x, head_y);

                path.push(RopePosition {
                    head_x,
                    head_y,
                    tail_x,
                    tail_y,
                });
            }

            Ok(RopePath(path))
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize> {
    let rope = rope_bridge::Rope::new(&input).unwrap();
    let path = rope.get_path(None).unwrap();

    let tail_positions = path.visited_tail_positions();

    aoc::Answer(tail_positions.iter().collect::<HashSet<_>>().len(), 0)
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(9, 13, 0).compute(&get_answer)
}
