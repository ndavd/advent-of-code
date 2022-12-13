mod handheld_device {
    use std::collections::VecDeque;

    const A_U8: u8 = 'a' as u8;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Position {
        pub x: usize,
        pub y: usize,
    }

    impl Position {
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Move {
        Up,
        Down,
        Left,
        Right,
    }

    impl Move {
        pub const VALUES: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

        pub fn pos(&self, pos: &Position, max_x: usize, max_y: usize) -> Option<Position> {
            let (x, y) = (pos.x, pos.y);
            match self {
                Move::Up => {
                    if y == 0 {
                        None
                    } else {
                        Some(Position::new(x, y - 1))
                    }
                }
                Move::Down => {
                    if y == max_y {
                        None
                    } else {
                        Some(Position::new(x, y + 1))
                    }
                }
                Move::Left => {
                    if x == 0 {
                        None
                    } else {
                        Some(Position::new(x - 1, y))
                    }
                }
                Move::Right => {
                    if x == max_x {
                        None
                    } else {
                        Some(Position::new(x + 1, y))
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct HeightMap {
        // Positions are indices in the map array of arrays
        pub origin: Position,
        pub destination: Position,
        pub history: Vec<Position>,
        pub map: Vec<Vec<u8>>,
    }

    impl std::fmt::Display for HeightMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for y in 0..self.map.len() {
                for x in 0..self.map[0].len() {
                    let h = self.map[y][x];
                    write!(
                        f,
                        "{}",
                        if (x, y) == (self.origin.x, self.origin.y) {
                            '█'
                        } else if (x, y) == (self.destination.x, self.destination.y) {
                            '⌷'
                        } else {
                            (h + A_U8) as char
                        }
                    )
                    .unwrap();
                }
                if y != self.map.len() - 1 {
                    writeln!(f).unwrap();
                }
            }
            Ok(())
        }
    }

    impl HeightMap {
        pub fn new(input: &Vec<String>) -> Result<Self, &str> {
            let mut char_map: Vec<Vec<char>> = Vec::new();
            for i in 0..input.len() {
                let s = &input[i];
                char_map.push(s.chars().collect::<Vec<_>>());
            }
            let mut origin_pos: Option<Position> = None;
            let mut destination_pos: Option<Position> = None;
            'outer: for y in 0..char_map.len() {
                for x in 0..char_map[y].len() {
                    let c = &mut char_map[y][x];
                    if *c == 'S' {
                        *c = 'a';
                        origin_pos = Some(Position { x, y });
                    }
                    if *c == 'E' {
                        *c = 'z';
                        destination_pos = Some(Position { x, y });
                    }
                    if origin_pos != None && destination_pos != None {
                        break 'outer;
                    }
                }
            }
            if origin_pos == None || destination_pos == None {
                return Err("HeightMap::new::Invalid input");
            }
            let map = char_map
                .iter()
                .map(|char_vec| char_vec.iter().map(|c| *c as u8 - A_U8).collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>();

            Ok(HeightMap {
                origin: origin_pos.unwrap(),
                destination: destination_pos.unwrap(),
                history: vec![],
                map,
            })
        }

        // Using backtracing
        pub fn smallest_path(&self) -> Vec<Position> {
            let map = &self.map;
            let mut paths: Vec<Vec<Position>> = Vec::new();
            let mut history: Vec<Position> = Vec::new();
            let mut unexplored: VecDeque<(Position, Vec<Position>)> = VecDeque::new();
            unexplored.push_back((self.origin.clone(), Vec::new()));
            while let Some((curr_pos, path)) = unexplored.pop_front() {
                if history.contains(&curr_pos) {
                    continue;
                }
                if curr_pos == self.destination {
                    paths.push(path.clone());
                    continue;
                }
                history.push(curr_pos.clone());
                for m in Move::VALUES {
                    let next_pos_option =
                        m.pos(&curr_pos, map[curr_pos.y].len() - 1, map.len() - 1);
                    if let Some(next_pos) = next_pos_option {
                        let curr_elevation = map[curr_pos.y][curr_pos.x];
                        let next_elevation = map[next_pos.y][next_pos.x];
                        if !history.contains(&next_pos) && curr_elevation + 1 >= next_elevation {
                            let mut new_path = path.clone();
                            new_path.push(curr_pos.clone());
                            unexplored.push_back((next_pos, new_path));
                        }
                    }
                }
            }
            paths
                .iter()
                .min_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .to_vec()
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize, usize> {
    let height_map = handheld_device::HeightMap::new(&input).unwrap();

    aoc::Answer(height_map.smallest_path().len(), 0)
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(12, 31, 0).compute(&get_answer)
}
