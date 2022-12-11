mod rope_bridge {
    use std::collections::HashSet;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /* Assuming the following coordinate system:
     * +y
     *  ^
     *  |
     *  O--> +x
     */
    pub struct KnotPosition {
        pub x: isize,
        pub y: isize,
    }

    impl KnotPosition {
        pub fn new(x: isize, y: isize) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug)]
    pub struct RopePosition(Vec<KnotPosition>);

    impl std::fmt::Display for RopePosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let knots_positions = &self.0;
            let knots_positions_len = knots_positions.len();

            for i in 0..knots_positions_len {
                let pos = &knots_positions[i];
                let name = if i == 0 {
                    'H'
                } else if i == knots_positions_len - 1 {
                    'T'
                } else {
                    std::char::from_digit(i.try_into().unwrap(), 10).unwrap_or('*')
                };
                write!(f, "{}{{ {} {} }} ", name, pos.x, pos.y).unwrap();
            }
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct RopePath(pub Vec<RopePosition>);

    impl RopePath {
        pub fn visited_tail_positions(&self) -> HashSet<KnotPosition> {
            self.0.iter().map(|pos| pos.0[pos.0.len() - 1]).collect()
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
    // A Rope is defined by a sequence of head steps and how many knots it has
    pub struct Rope {
        pub head_steps: Vec<KnotStep>,
        pub knot_count: usize,
    }

    impl Rope {
        pub fn new(input: &Vec<String>, knot_count: usize) -> Result<Self, &str> {
            if knot_count < 2 {
                return Err("Rope::new::A rope must have at least 2 knots");
            }

            let mut head_steps: Vec<KnotStep> = Vec::new();
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
                    head_steps.push(step);
                }
            }

            Ok(Rope {
                head_steps,
                knot_count,
            })
        }

        fn get_new_pos(pos: &KnotPosition, step: &KnotStep) -> KnotPosition {
            match step {
                KnotStep::Up => KnotPosition::new(pos.x, pos.y + 1),
                KnotStep::Down => KnotPosition::new(pos.x, pos.y - 1),
                KnotStep::Left => KnotPosition::new(pos.x - 1, pos.y),
                KnotStep::Right => KnotPosition::new(pos.x + 1, pos.y),
                KnotStep::UpLeft => KnotPosition::new(pos.x - 1, pos.y + 1),
                KnotStep::UpRight => KnotPosition::new(pos.x + 1, pos.y + 1),
                KnotStep::DownLeft => KnotPosition::new(pos.x - 1, pos.y - 1),
                KnotStep::DownRight => KnotPosition::new(pos.x + 1, pos.y - 1),
            }
        }

        fn get_tail_pos(previous_tail_pos: &KnotPosition, head_pos: &KnotPosition) -> KnotPosition {
            let distance_x = head_pos.x - previous_tail_pos.x;
            let distance_y = head_pos.y - previous_tail_pos.y;

            // Should move tail
            if distance_x.abs() > 1 || distance_y.abs() > 1 {
                let pos = previous_tail_pos;
                fn norm(x: isize) -> i8 {
                    if x == 0 {
                        0
                    } else {
                        (x / x.abs()).try_into().unwrap()
                    }
                }
                return match (norm(distance_x), norm(distance_y)) {
                    (0, 1) => Self::get_new_pos(pos, &KnotStep::Up),
                    (0, -1) => Self::get_new_pos(pos, &KnotStep::Down),
                    (-1, 0) => Self::get_new_pos(pos, &KnotStep::Left),
                    (1, 0) => Self::get_new_pos(pos, &KnotStep::Right),
                    (-1, 1) => Self::get_new_pos(pos, &KnotStep::UpLeft),
                    (1, 1) => Self::get_new_pos(pos, &KnotStep::UpRight),
                    (-1, -1) => Self::get_new_pos(pos, &KnotStep::DownLeft),
                    (1, -1) => Self::get_new_pos(pos, &KnotStep::DownRight),
                    _ => {
                        println!("{:?}", (distance_x, distance_y));
                        panic!("Rope::get_tail_pos::Something has gone extremely wrong!")
                    }
                };
            }

            previous_tail_pos.clone()
        }

        pub fn get_path(&self, origin: Option<isize>) -> Result<RopePath, &str> {
            let o = origin.unwrap_or(0);
            let head_steps = &self.head_steps;
            let mut path = vec![vec![KnotPosition::new(o, o); self.knot_count]];

            for i in 0..head_steps.len() {
                let previous_rope_pos = &path[i];
                let mut new_rope_pos: Vec<KnotPosition> = Vec::new();

                let head_pos = Self::get_new_pos(&previous_rope_pos[0], &head_steps[i]);
                new_rope_pos.push(head_pos);

                let mut relative_head_pos = head_pos;
                for k in 1..self.knot_count {
                    let tail_pos = Self::get_tail_pos(&previous_rope_pos[k], &relative_head_pos);
                    new_rope_pos.push(tail_pos);

                    relative_head_pos = tail_pos;
                }

                path.push(new_rope_pos);
            }

            Ok(RopePath(
                path.iter().map(|p| RopePosition(p.to_vec())).collect(),
            ))
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize, usize> {
    let simple_rope_path = rope_bridge::Rope::new(&input, 2)
        .unwrap()
        .get_path(None)
        .unwrap();

    let complex_rope_path = rope_bridge::Rope::new(&input, 10)
        .unwrap()
        .get_path(None)
        .unwrap();

    aoc::Answer(
        simple_rope_path.visited_tail_positions().len(),
        complex_rope_path.visited_tail_positions().len(),
    )
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(9, 13, 1).compute(&get_answer)
}
