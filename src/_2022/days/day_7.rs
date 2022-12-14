use crate::solve;

pub fn run() {
    let day_number = 7;
    let year = 2022;
    solve!(&year, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    data.directories
        .clone()
        .into_iter()
        .filter_map(|dir| {
            let size = size_of(&*dir, &data.files);
            if size < 100000 {
                return Some(size);
            }
            None
        })
        .sum::<usize>()
        .to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let total_space = 70000000;
    let required_space = 30000000;
    let total_used = size_of("/", &data.files);
    let unused_space = total_space - total_used;

    data.directories
        .clone()
        .into_iter()
        .filter_map(|dir| {
            let size = size_of(&*dir, &data.files);
            if unused_space + size > required_space {
                return Some(size);
            }
            None
        })
        .min()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct ParsedInput {
    files: Vec<File>,
    directories: Vec<String>,
}

fn size_of(path: &str, files: &Vec<File>) -> usize {
    files
        .into_iter()
        .filter_map(|file| {
            if file.path.contains(path) {
                return Some(file.size);
            }
            None
        })
        .sum::<usize>()
}

#[derive(Debug, Clone)]

struct File {
    path: String,
    size: usize,
}

fn parse(input: &str) -> ParsedInput {
    let mut files = vec![];
    let mut current_path = vec![];
    let mut directories = vec![];
    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        match split[0] {
            "$" => match split[1] {
                "cd" => {
                    match split[2] {
                        ".." => {
                            // move up one dir
                            current_path.pop();
                        }
                        "/" => current_path = vec!["/"],
                        _ => {
                            // move into specific dir
                            current_path.push(split[2]);
                            directories.push(current_path.join("/"));
                        }
                    }
                }
                _ => {
                    // do nothing
                }
            },
            "dir" => {
                // do nothing
            }
            _ => {
                // add file to current dir
                files.push(File {
                    size: split[0].parse::<usize>().unwrap(),
                    path: current_path.join("/"),
                })
            }
        }
    });

    ParsedInput { files, directories }
}

#[test]
fn test() {
    let test_input = String::from(
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    );
    let parsed = parse(&test_input);
    assert_eq!(task_1(&parsed), "95437");
    assert_eq!(task_2(&parsed), "24933642");
}
