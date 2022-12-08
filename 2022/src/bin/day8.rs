mod quadcopter {
    #[derive(Debug)]
    pub struct TreeMap(Vec<Vec<u8>>);

    #[derive(Debug)]
    pub struct Tree {
        pub height: u8,
        pub pos_x: usize,
        pub pos_y: usize,
        pub visible: bool,
        pub scenic_score: usize,
    }

    impl std::fmt::Display for TreeMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (i, v) in self.0.iter().enumerate() {
                for c in v {
                    write!(f, "{}", c).unwrap();
                }
                if i < self.0.len() - 1 {
                    writeln!(f).unwrap();
                }
            }
            Ok(())
        }
    }

    impl TreeMap {
        pub fn new(input: &Vec<String>) -> Result<Self, &str> {
            let mut tree_map: Vec<Vec<u8>> = Vec::new();

            let input_len = input.len();
            for line in input {
                let chars: Vec<char> = line.chars().collect();
                if chars.len() != input_len {
                    return Err("TreeMap::new::Bad input");
                }
                tree_map.push(
                    chars
                        .iter()
                        .map(|c| c.clone() as u8 - '0' as u8)
                        .collect::<Vec<u8>>(),
                )
            }

            Ok(Self(tree_map))
        }

        pub fn get_tree_details(&self) -> Vec<Tree> {
            let mut trees: Vec<Tree> = Vec::new();

            let map_enumerate = self.0.iter().enumerate();

            let cols: Vec<Vec<u8>> = (0..self.0[0].len())
                .map(|index| self.0.iter().map(|row| row[index]).collect::<Vec<u8>>())
                .collect();

            for (y, row) in map_enumerate {
                for (x, val) in row.iter().enumerate() {
                    let mut tree = Tree {
                        height: *val,
                        pos_x: x,
                        pos_y: y,
                        visible: false,
                        scenic_score: 1,
                    };
                    let mut is_visible = || tree.visible = true;

                    let left = &row[0..x];
                    let right = &row[x + 1..];
                    let up = &cols[x][0..y];
                    let down = &cols[x][y + 1..];

                    // Check left
                    if left.iter().all(|v| v < val) {
                        is_visible();
                        tree.scenic_score *= left.len();
                    } else {
                        tree.scenic_score *= match left
                            .iter()
                            .rev()
                            .enumerate()
                            .find(|(_, v)| v.clone() >= val)
                        {
                            Some(v) => v.0 + 1,
                            None => 1,
                        };
                    }

                    // Check right
                    if right.iter().all(|v| v < val) {
                        is_visible();
                        tree.scenic_score *= right.len();
                    } else {
                        tree.scenic_score *=
                            match right.iter().enumerate().find(|(_, v)| v.clone() >= val) {
                                Some(v) => v.0 + 1,
                                None => 1,
                            };
                    }

                    // Check up
                    if up.iter().all(|v| v < val) {
                        is_visible();
                        tree.scenic_score *= up.len();
                    } else {
                        tree.scenic_score *=
                            match up.iter().rev().enumerate().find(|(_, v)| v.clone() >= val) {
                                Some(v) => v.0 + 1,
                                None => 1,
                            };
                    }

                    // Check down
                    if down.iter().all(|v| v < val) {
                        is_visible();
                        tree.scenic_score *= down.len();
                    } else {
                        tree.scenic_score *=
                            match down.iter().enumerate().find(|(_, v)| v.clone() >= val) {
                                Some(v) => v.0 + 1,
                                None => 1,
                            };
                    }

                    trees.push(tree);
                }
            }

            trees
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize> {
    let forest_map = quadcopter::TreeMap::new(&input).unwrap();

    let trees = forest_map.get_tree_details();

    aoc::Answer(
        trees
            .iter()
            .filter(|tree| tree.visible)
            .collect::<Vec<_>>()
            .len(),
        trees
            .iter()
            .max_by(|tree_a, tree_b| tree_a.scenic_score.cmp(&tree_b.scenic_score))
            .unwrap()
            .scenic_score,
    )
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(8, 21, 8).compute(&get_answer)
}
