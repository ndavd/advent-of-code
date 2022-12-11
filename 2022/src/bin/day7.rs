mod device {
    use std::collections::HashSet;

    pub const TOTAL_SPACE: usize = 70000000;
    pub const MIN_FREE_SPACE_FOR_UPGRADE: usize = 30000000;

    #[derive(Debug)]
    pub struct File {
        pub absolute_path: Vec<String>,
        pub size: usize,
    }

    #[derive(Clone, Debug)]
    pub struct Directory(pub String, pub usize);

    #[derive(Debug)]
    pub struct DirectoryCollection(pub Vec<Directory>);

    #[derive(Debug)]
    pub struct FileSystem(Vec<File>);

    impl std::fmt::Display for FileSystem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.iter().for_each(|file| {
                let mut file_path = String::from("/");
                file_path.push_str(&file.absolute_path.join("/"));
                writeln!(f, "{} ({})", file_path, file.size).unwrap();
            });

            Ok(())
        }
    }

    impl FileSystem {
        pub fn new(commands: &Vec<String>) -> Result<Self, &str> {
            let mut fs = Self(Vec::new());

            let mut curr_path: Vec<&str> = Vec::new();
            for command in commands {
                let command_args = command.split(" ").collect::<Vec<&str>>();
                let args_count = command_args.len();

                if args_count == 3 {
                    match command_args[args_count - 1] {
                        ".." => {
                            match curr_path.pop() {
                                Some(_) => {}
                                None => return Err(
                                    "FileSystem::new::Cannot move out from the outermost directory",
                                ),
                            }
                        }
                        "/" => curr_path = Vec::new(),
                        x => curr_path.push(x),
                    }
                }

                match command_args[0] {
                    "$" => {}
                    "dir" => {}
                    size => {
                        let file_size = match size.parse::<usize>() {
                            Ok(v) => v,
                            Err(_) => return Err("FileSystem::new::File size is not parseable"),
                        };

                        let mut file_absolute_path: Vec<String> =
                            curr_path.iter().map(|x| x.to_string()).collect();
                        file_absolute_path.push(command_args[1].to_string());

                        fs.0.push(File {
                            absolute_path: file_absolute_path,
                            size: file_size,
                        })
                    }
                }
            }

            Ok(fs)
        }

        pub fn ls_dirs(&self, max_size: Option<usize>) -> DirectoryCollection {
            let fs = &self.0;
            let mut dirs_vec: Vec<String> = Vec::new();

            for file in fs {
                let file_path = &file.absolute_path;
                let file_path_len = file_path.len();
                if file_path_len == 1 {
                    continue;
                }

                for index in 0..file_path_len {
                    let p = file_path[0..=index].join("/");
                    dirs_vec.push(p);
                }
            }

            let dirs_hashset = dirs_vec.iter().collect::<HashSet<_>>();

            let mut dirs: DirectoryCollection = DirectoryCollection(Vec::new());

            for dir in dirs_hashset {
                let mut dir_size: usize = 0;
                for file in fs {
                    let file_path = &file.absolute_path;
                    let joined_path = file_path[0..file_path.len()].join("/");
                    if joined_path.len() > dir.len() && joined_path[0..dir.len()] == dir.to_string()
                    {
                        dir_size += file.size;
                    }
                }
                if max_size == None || dir_size <= max_size.unwrap() {
                    dirs.0.push(Directory(dir.to_string(), dir_size));
                }
            }

            let base_size = fs.iter().map(|f| f.size).sum::<_>();
            if max_size == None || base_size <= max_size.unwrap() {
                dirs.0.push(Directory("/".to_string(), base_size));
            }

            dirs
        }

        pub fn optimal_dir_to_delete(&self) -> Option<Directory> {
            let dirs = self.ls_dirs(None).0;

            let occupied_space = dirs.iter().find(|d| d.0 == "/").unwrap().1;
            let free_space = TOTAL_SPACE - occupied_space;

            if free_space >= MIN_FREE_SPACE_FOR_UPGRADE {
                return None;
            }

            Some(
                dirs.iter()
                    .filter(|d| d.1 > MIN_FREE_SPACE_FOR_UPGRADE - free_space)
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .unwrap()
                    .clone(),
            )
        }
    }

    impl DirectoryCollection {
        pub fn sum_sizes(&self) -> usize {
            self.0.iter().map(|d| d.1).sum()
        }
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<usize, usize> {
    let fs = device::FileSystem::new(&input).unwrap();

    aoc::Answer(
        fs.ls_dirs(Some(100000)).sum_sizes(),
        fs.optimal_dir_to_delete().unwrap().1,
    )
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(7, 95437, 24933642).compute(&get_answer)
}
